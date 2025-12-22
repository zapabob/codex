// Enhanced GUI-CLI Bridge with Windows 11 25H2 MCP Integration
// Provides bidirectional communication between GUI and CLI with MCP support

import { EventEmitter } from 'events'

export interface BridgeMessage {
  id: string
  type: BridgeMessageType
  payload: unknown
  timestamp: number
  source: 'gui' | 'cli'
  target: 'gui' | 'cli' | 'mcp'
}

export enum BridgeMessageType {
  // Core communication
  HANDSHAKE = 'handshake',
  HEARTBEAT = 'heartbeat',
  COMMAND_EXEC = 'command_exec',
  COMMAND_RESULT = 'command_result',

  // MCP integration
  MCP_DISCOVER = 'mcp_discover',
  MCP_CONNECT = 'mcp_connect',
  MCP_DISCONNECT = 'mcp_disconnect',

  // VR/AR integration
  VR_ENTER = 'vr_enter',
  VR_EXIT = 'vr_exit',
  VR_COMMIT_SELECT = 'vr_commit_select',
  VR_NAVIGATE = 'vr_navigate',

  // State synchronization
  STATE_SYNC = 'state_sync',
  STATE_UPDATE = 'state_update',

  // Error handling
  ERROR = 'error',
  RECONNECT = 'reconnect',
}

export interface BridgeConfig {
  websocketUrl: string
  mcpRegistryUrl?: string
  reconnectInterval: number
  maxRetries: number
  heartbeatInterval: number
}

export class DualBridge extends EventEmitter {
  private ws: WebSocket | null = null
  private mcpRegistry: MCPRegistry
  private webxrManager: WebXRManager
  private config: BridgeConfig
  private reconnectTimer: NodeJS.Timeout | null = null
  private heartbeatTimer: NodeJS.Timeout | null = null
  private messageQueue: BridgeMessage[] = []
  private isConnected = false
  private messageId = 0
  private retryCount = 0

  constructor(config: BridgeConfig) {
    super()
    this.config = config
    this.mcpRegistry = new MCPRegistry(config.mcpRegistryUrl)
    this.webxrManager = new WebXRManager()

    this.initializeEventHandlers()
  }

  private initializeEventHandlers() {
    // WebXR events
    this.webxrManager.on('commitSelected', (commit: unknown) => {
      this.sendMessage({
        type: BridgeMessageType.VR_COMMIT_SELECT,
        payload: { commit },
        target: 'cli'
      })
    })

    this.webxrManager.on('navigation', (position: unknown) => {
      this.sendMessage({
        type: BridgeMessageType.VR_NAVIGATE,
        payload: { position },
        target: 'cli'
      })
    })

    // MCP events
    this.mcpRegistry.on('serverDiscovered', (servers: unknown[]) => {
      this.sendMessage({
        type: BridgeMessageType.MCP_DISCOVER,
        payload: { servers },
        target: 'gui'
      })
    })

    this.mcpRegistry.on('serverConnected', (serverId: string) => {
      this.sendMessage({
        type: BridgeMessageType.MCP_CONNECT,
        payload: { serverId },
        target: 'gui'
      })
    })
  }

  async connect(): Promise<void> {
    if (this.isConnected) return

    return new Promise((resolve, reject) => {
      try {
        this.ws = new WebSocket(this.config.websocketUrl)

        this.ws.onopen = () => {
          this.retryCount = 0
          this.isConnected = true
          this.startHeartbeat()
          this.flushMessageQueue()
          this.performHandshake()
          resolve()
        }

        this.ws.onmessage = (event) => {
          try {
            const message: BridgeMessage = JSON.parse(event.data)
            this.handleMessage(message)
          } catch (error) {
            console.error('Dual Bridge: Failed to parse message', error)
          }
        }

        this.ws.onclose = () => {
          this.isConnected = false
          this.stopHeartbeat()
          this.scheduleReconnect()
        }

        this.ws.onerror = (error) => {
          console.error('Dual Bridge: Connection error', error)
          reject(error)
        }

        // Initialize MCP registry if URL provided
        if (this.config.mcpRegistryUrl) {
          void this.initializeMCPRegistry()
        }
      } catch (error) {
        reject(error)
      }
    })
  }

  private async initializeMCPRegistry() {
    try {
      await this.mcpRegistry.connect(this.config.mcpRegistryUrl!)
      await Promise.all([
        this.mcpRegistry.registerSystemServer('filesystem'),
        this.mcpRegistry.registerSystemServer('windowing'),
        this.mcpRegistry.registerSystemServer('wsl')
      ])
    } catch (error) {
      console.error('Dual Bridge: Failed to initialize MCP Registry', error)
    }
  }

  private performHandshake() {
    this.sendMessage({
      type: BridgeMessageType.HANDSHAKE,
      payload: {
        version: '1.0.0',
        capabilities: ['mcp', 'vr', 'state-sync'],
        platform: 'windows-11-25h2'
      },
      target: 'cli'
    })
  }

  private startHeartbeat() {
    this.heartbeatTimer = setInterval(() => {
      if (this.isConnected) {
        this.sendMessage({
          type: BridgeMessageType.HEARTBEAT,
          payload: { timestamp: Date.now() },
          target: 'cli'
        })
      }
    }, this.config.heartbeatInterval)
  }

  private stopHeartbeat() {
    if (this.heartbeatTimer) {
      clearInterval(this.heartbeatTimer)
      this.heartbeatTimer = null
    }
  }

  private scheduleReconnect() {
    if (this.reconnectTimer || this.retryCount >= this.config.maxRetries) return

    this.reconnectTimer = setTimeout(async () => {
      try {
        this.retryCount += 1
        await this.connect()
        this.reconnectTimer = null
      } catch (error) {
        console.error('Dual Bridge: Reconnection failed', error)
        this.scheduleReconnect()
      }
    }, this.config.reconnectInterval)
  }

  sendMessage(message: Partial<BridgeMessage>): string {
    const fullMessage: BridgeMessage = {
      id: this.generateMessageId(),
      timestamp: Date.now(),
      source: 'gui',
      ...message,
      target: message.target ?? 'cli'
    }

    if (this.isConnected && this.ws) {
      this.ws.send(JSON.stringify(fullMessage))
    } else {
      this.messageQueue.push(fullMessage)
    }

    return fullMessage.id
  }

  private flushMessageQueue() {
    while (this.messageQueue.length > 0 && this.isConnected && this.ws) {
      const message = this.messageQueue.shift()!
      this.ws.send(JSON.stringify(message))
    }
  }

  private handleMessage(message: BridgeMessage) {
    this.emit('message', message)

    switch (message.type) {
      case BridgeMessageType.HANDSHAKE:
        this.handleHandshake(message)
        break
      case BridgeMessageType.HEARTBEAT:
        break
      case BridgeMessageType.COMMAND_RESULT:
        this.handleCommandResult(message)
        break
      case BridgeMessageType.STATE_UPDATE:
        this.handleStateUpdate(message)
        break
      case BridgeMessageType.ERROR:
        this.handleError(message)
        break
      default:
        this.emit(message.type, message.payload)
    }
  }

  private handleHandshake(message: BridgeMessage) {
    this.emit('handshake', message.payload)
  }

  private handleCommandResult(message: BridgeMessage) {
    this.emit('commandResult', message.payload)
  }

  private handleStateUpdate(message: BridgeMessage) {
    this.emit('stateUpdate', message.payload)
  }

  private handleError(message: BridgeMessage) {
    this.emit('error', message.payload)
  }

  async executeCommand(command: string, args: Record<string, unknown> = {}): Promise<{ exitCode: number; stdout: string; stderr: string }> {
    return new Promise((resolve, reject) => {
      const messageId = this.sendMessage({
        type: BridgeMessageType.COMMAND_EXEC,
        payload: { command, args },
        target: 'cli'
      })

      const responseHandler = (result: { messageId?: string; exitCode?: number; stdout?: string; stderr?: string }) => {
        if (result.messageId === messageId) {
          this.off('commandResult', responseHandler)
          resolve({
            exitCode: result.exitCode ?? 0,
            stdout: result.stdout ?? '',
            stderr: result.stderr ?? ''
          })
        }
      }

      this.on('commandResult', responseHandler)

      setTimeout(() => {
        this.off('commandResult', responseHandler)
        reject(new Error('Command execution timeout'))
      }, 30000)
    })
  }

  async discoverMCPServers(capabilities?: string[]): Promise<unknown[]> {
    return this.mcpRegistry.discoverServers(capabilities)
  }

  async connectToMCPServer(serverId: string): Promise<void> {
    return this.mcpRegistry.connectToServer(serverId)
  }

  enterVR(): void {
    this.webxrManager.enterVR()
    this.sendMessage({
      type: BridgeMessageType.VR_ENTER,
      payload: {},
      target: 'cli'
    })
  }

  exitVR(): void {
    this.webxrManager.exitVR()
    this.sendMessage({
      type: BridgeMessageType.VR_EXIT,
      payload: {},
      target: 'cli'
    })
  }

  syncState(state: unknown): void {
    this.sendMessage({
      type: BridgeMessageType.STATE_SYNC,
      payload: state,
      target: 'cli'
    })
  }

  private generateMessageId(): string {
    return `msg_${++this.messageId}_${Date.now()}`
  }

  disconnect(): void {
    this.isConnected = false
    this.stopHeartbeat()

    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer)
      this.reconnectTimer = null
    }

    if (this.ws) {
      this.ws.close()
      this.ws = null
    }

    this.mcpRegistry.disconnect()
    this.webxrManager.cleanup()
  }
}

class MCPRegistry extends EventEmitter {
  private registryUrl: string | undefined
  private activeSockets = new Map<string, WebSocket>()

  constructor(registryUrl?: string) {
    super()
    this.registryUrl = registryUrl
  }

  async connect(url: string) {
    this.registryUrl = url
    await this.discoverServers()
  }

  async registerSystemServer(type: string) {
    if (!this.registryUrl) return
    const response = await fetch(`${this.registryUrl}/servers`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ type })
    })

    if (!response.ok) {
      throw new Error(`MCP registry rejected system server registration: ${response.statusText}`)
    }
  }

  async discoverServers(capabilities?: string[]) {
    if (!this.registryUrl) return []
    const query = capabilities?.length ? `?capabilities=${encodeURIComponent(capabilities.join(','))}` : ''
    const response = await fetch(`${this.registryUrl}/servers${query}`)
    if (!response.ok) {
      throw new Error(`Failed to discover MCP servers: ${response.statusText}`)
    }

    const servers = await response.json()
    this.emit('serverDiscovered', servers)
    return servers
  }

  async connectToServer(serverId: string) {
    if (!this.registryUrl) throw new Error('MCP registry URL not configured')
    const response = await fetch(`${this.registryUrl}/servers/${serverId}/connect`, { method: 'POST' })
    if (!response.ok) {
      throw new Error(`Failed to connect to MCP server ${serverId}: ${response.statusText}`)
    }

    const { wsUrl } = await response.json()
    if (wsUrl) {
      const socket = new WebSocket(wsUrl)
      this.activeSockets.set(serverId, socket)
      socket.onopen = () => this.emit('serverConnected', serverId)
      socket.onclose = () => this.activeSockets.delete(serverId)
    }
  }

  disconnect() {
    this.activeSockets.forEach((socket) => socket.close())
    this.activeSockets.clear()
  }
}

class WebXRManager extends EventEmitter {
  private session: XRSession | null = null

  enterVR() {
    if (typeof navigator === 'undefined' || !navigator.xr) {
      return
    }

    void navigator.xr.isSessionSupported('immersive-vr').then((supported) => {
      if (!supported) return
      return navigator.xr?.requestSession('immersive-vr').then((session) => {
        this.session = session
        this.emit('enter', session)
        session.addEventListener('end', () => this.emit('exit'))
      })
    })
  }

  exitVR() {
    if (this.session) {
      void this.session.end()
      this.session = null
    }
  }

  cleanup() {
    if (this.session) {
      void this.session.end()
      this.session = null
    }
  }
}
