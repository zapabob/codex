// Enhanced GUI-CLI Bridge with Windows 11 25H2 MCP Integration
// Provides bidirectional communication between GUI and CLI with MCP support

import { EventEmitter } from 'events'
import { CodexAPIClient } from '../api/client'
import { MCPRegistry } from './mcp-registry'
import { WebXRManager } from './webxr-manager'

export interface BridgeMessage {
  id: string
  type: BridgeMessageType
  payload: any
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
  private apiClient: CodexAPIClient
  private mcpRegistry: MCPRegistry
  private webxrManager: WebXRManager
  private config: BridgeConfig
  private reconnectTimer: NodeJS.Timeout | null = null
  private heartbeatTimer: NodeJS.Timeout | null = null
  private messageQueue: BridgeMessage[] = []
  private isConnected = false
  private messageId = 0

  constructor(config: BridgeConfig) {
    super()
    this.config = config
    this.apiClient = new CodexAPIClient()
    this.mcpRegistry = new MCPRegistry()
    this.webxrManager = new WebXRManager()

    this.initializeEventHandlers()
  }

  private initializeEventHandlers() {
    // WebXR events
    this.webxrManager.on('commitSelected', (commit: any) => {
      this.sendMessage({
        type: BridgeMessageType.VR_COMMIT_SELECT,
        payload: { commit },
        target: 'cli'
      })
    })

    this.webxrManager.on('navigation', (position: any) => {
      this.sendMessage({
        type: BridgeMessageType.VR_NAVIGATE,
        payload: { position },
        target: 'cli'
      })
    })

    // MCP events
    this.mcpRegistry.on('serverDiscovered', (servers: any[]) => {
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
    return new Promise((resolve, reject) => {
      try {
        this.ws = new WebSocket(this.config.websocketUrl)

        this.ws.onopen = () => {
          console.log('Dual Bridge: Connected to CLI server')
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
          console.log('Dual Bridge: Connection closed')
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
          this.initializeMCPRegistry()
        }

      } catch (error) {
        reject(error)
      }
    })
  }

  private async initializeMCPRegistry() {
    try {
      await this.mcpRegistry.connect(this.config.mcpRegistryUrl!)

      // Register Windows system servers
      await this.mcpRegistry.registerSystemServer('filesystem')
      await this.mcpRegistry.registerSystemServer('windowing')
      await this.mcpRegistry.registerSystemServer('wsl')

      console.log('Dual Bridge: MCP Registry initialized')
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
    if (this.reconnectTimer) return

    this.reconnectTimer = setTimeout(async () => {
      console.log('Dual Bridge: Attempting reconnection...')
      try {
        await this.connect()
        this.reconnectTimer = null
      } catch (error) {
        console.error('Dual Bridge: Reconnection failed', error)
        this.scheduleReconnect()
      }
    }, this.config.reconnectInterval)
  }

  sendMessage(message: Partial<BridgeMessage>): void {
    const fullMessage: BridgeMessage = {
      id: this.generateMessageId(),
      timestamp: Date.now(),
      source: 'gui',
      ...message
    }

    if (this.isConnected && this.ws) {
      this.ws.send(JSON.stringify(fullMessage))
    } else {
      // Queue message for later sending
      this.messageQueue.push(fullMessage)
    }
  }

  private flushMessageQueue() {
    while (this.messageQueue.length > 0 && this.isConnected && this.ws) {
      const message = this.messageQueue.shift()!
      this.ws.send(JSON.stringify(message))
    }
  }

  private handleMessage(message: BridgeMessage) {
    // Emit message for external listeners
    this.emit('message', message)

    // Handle specific message types
    switch (message.type) {
      case BridgeMessageType.HANDSHAKE:
        this.handleHandshake(message)
        break

      case BridgeMessageType.HEARTBEAT:
        // Heartbeat response - connection is healthy
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
        // Forward to specific handlers
        this.emit(message.type, message.payload)
    }
  }

  private handleHandshake(message: BridgeMessage) {
    console.log('Dual Bridge: Handshake received', message.payload)
    this.emit('handshake', message.payload)
  }

  private handleCommandResult(message: BridgeMessage) {
    this.emit('commandResult', message.payload)
  }

  private handleStateUpdate(message: BridgeMessage) {
    this.emit('stateUpdate', message.payload)
  }

  private handleError(message: BridgeMessage) {
    console.error('Dual Bridge: Error received', message.payload)
    this.emit('error', message.payload)
  }

  // Public API methods
  async executeCommand(command: string, args: any = {}): Promise<any> {
    return new Promise((resolve, reject) => {
      const messageId = this.sendMessage({
        type: BridgeMessageType.COMMAND_EXEC,
        payload: { command, args },
        target: 'cli'
      })

      // Set up response handler
      const responseHandler = (result: any) => {
        if (result.messageId === messageId) {
          this.off('commandResult', responseHandler)
          resolve(result)
        }
      }

      this.on('commandResult', responseHandler)

      // Timeout after 30 seconds
      setTimeout(() => {
        this.off('commandResult', responseHandler)
        reject(new Error('Command execution timeout'))
      }, 30000)
    })
  }

  async discoverMCPServers(capabilities?: string[]): Promise<any[]> {
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

  syncState(state: any): void {
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

// MCP Registry wrapper for GUI
class MCPRegistry {
  private registryUrl: string = ''

  on(event: string, handler: Function) {
    // Event handling implementation
  }

  async connect(url: string) {
    this.registryUrl = url
    // Connection logic
  }

  async registerSystemServer(type: string) {
    // Register Windows system server
  }

  async discoverServers(capabilities?: string[]) {
    // Discover MCP servers
    return []
  }

  async connectToServer(serverId: string) {
    // Connect to MCP server
  }

  disconnect() {
    // Cleanup connections
  }
}

// WebXR Manager wrapper
class WebXRManager {
  on(event: string, handler: Function) {
    // Event handling implementation
  }

  enterVR() {
    // Enter VR mode
  }

  exitVR() {
    // Exit VR mode
  }

  cleanup() {
    // Cleanup WebXR resources
  }
}
