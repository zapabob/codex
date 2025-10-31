/**
 * Transport layer for orchestrator protocol client
 */

import * as net from 'net';
import * as fs from 'fs/promises';
import * as path from 'path';
import { EventEmitter } from 'events';
import { Envelope } from './types';

export interface TransportConfig {
  socketPath?: string;
  tcpHost?: string;
  tcpPort?: number;
  reconnectInterval?: number;
  maxReconnectAttempts?: number;
}

export enum TransportState {
  DISCONNECTED = 'disconnected',
  CONNECTING = 'connecting',
  CONNECTED = 'connected',
  RECONNECTING = 'reconnecting',
}

/**
 * Transport manages the connection to the orchestrator server
 */
export class Transport extends EventEmitter {
  private config: TransportConfig;
  private socket?: net.Socket;
  private state: TransportState = TransportState.DISCONNECTED;
  private reconnectAttempts = 0;
  private reconnectTimer?: NodeJS.Timeout;
  private buffer = '';

  constructor(config: TransportConfig = {}) {
    super();
    this.config = {
      socketPath: config.socketPath || '.codex/orchestrator.sock',
      tcpHost: config.tcpHost || '127.0.0.1',
      reconnectInterval: config.reconnectInterval || 5000,
      maxReconnectAttempts: config.maxReconnectAttempts || 10,
      ...config,
    };
  }

  /**
   * Connect to the orchestrator server
   */
  async connect(): Promise<void> {
    if (this.state === TransportState.CONNECTED || this.state === TransportState.CONNECTING) {
      return;
    }

    this.state = TransportState.CONNECTING;
    this.emit('connecting');

    try {
      // Try Unix socket first (on Unix-like systems)
      if (process.platform !== 'win32' && this.config.socketPath) {
        await this.connectUnixSocket();
      } else {
        // Fall back to TCP
        await this.connectTcp();
      }
    } catch (error) {
      this.handleConnectionError(error);
    }
  }

  /**
   * Connect via Unix Domain Socket
   */
  private async connectUnixSocket(): Promise<void> {
    const socketPath = this.config.socketPath!;
    
    // Check if socket exists
    try {
      await fs.access(socketPath);
    } catch {
      // Socket doesn't exist, try TCP fallback
      return this.connectTcp();
    }

    return new Promise((resolve, reject) => {
      this.socket = net.createConnection({ path: socketPath }, () => {
        this.onConnected();
        resolve();
      });

      this.socket.on('error', (error) => {
        reject(error);
      });

      this.setupSocketHandlers();
    });
  }

  /**
   * Connect via TCP
   */
  private async connectTcp(): Promise<void> {
    // Try to read port from .codex/orchestrator.port
    let port = this.config.tcpPort;
    
    if (!port) {
      try {
        const portFile = '.codex/orchestrator.port';
        const portStr = await fs.readFile(portFile, 'utf-8');
        port = parseInt(portStr.trim(), 10);
      } catch {
        throw new Error('No TCP port configured and .codex/orchestrator.port not found');
      }
    }

    return new Promise((resolve, reject) => {
      this.socket = net.createConnection(
        { host: this.config.tcpHost, port },
        () => {
          this.onConnected();
          resolve();
        }
      );

      this.socket.on('error', (error) => {
        reject(error);
      });

      this.setupSocketHandlers();
    });
  }

  /**
   * Setup socket event handlers
   */
  private setupSocketHandlers(): void {
    if (!this.socket) return;

    this.socket.setEncoding('utf-8');

    this.socket.on('data', (data: string) => {
      this.handleData(data);
    });

    this.socket.on('close', () => {
      this.handleDisconnect();
    });

    this.socket.on('error', (error) => {
      this.emit('error', error);
    });
  }

  /**
   * Handle incoming data
   */
  private handleData(data: string): void {
    this.buffer += data;

    // Process complete lines (JSON Lines format)
    const lines = this.buffer.split('\n');
    this.buffer = lines.pop() || '';

    for (const line of lines) {
      if (line.trim()) {
        try {
          const envelope: Envelope = JSON.parse(line);
          this.emit('message', envelope);
        } catch (error) {
          this.emit('error', new Error(`Failed to parse message: ${error}`));
        }
      }
    }
  }

  /**
   * Handle connection established
   */
  private onConnected(): void {
    this.state = TransportState.CONNECTED;
    this.reconnectAttempts = 0;
    this.emit('connected');
  }

  /**
   * Handle disconnection
   */
  private handleDisconnect(): void {
    if (this.state === TransportState.CONNECTED) {
      this.state = TransportState.DISCONNECTED;
      this.emit('disconnected');
      this.attemptReconnect();
    }
  }

  /**
   * Handle connection error
   */
  private handleConnectionError(error: any): void {
    this.state = TransportState.DISCONNECTED;
    this.emit('error', error);
    this.attemptReconnect();
  }

  /**
   * Attempt to reconnect
   */
  private attemptReconnect(): void {
    if (this.reconnectAttempts >= this.config.maxReconnectAttempts!) {
      this.emit('error', new Error('Max reconnect attempts reached'));
      return;
    }

    this.reconnectAttempts++;
    this.state = TransportState.RECONNECTING;
    this.emit('reconnecting', this.reconnectAttempts);

    // Add jitter to reconnect interval
    const jitter = Math.random() * 1000;
    const interval = this.config.reconnectInterval! + jitter;

    this.reconnectTimer = setTimeout(() => {
      this.connect().catch((error) => {
        this.emit('error', error);
      });
    }, interval);
  }

  /**
   * Send a message to the server
   */
  async send(envelope: Envelope): Promise<void> {
    if (this.state !== TransportState.CONNECTED || !this.socket) {
      throw new Error('Not connected');
    }

    const json = JSON.stringify(envelope);
    const line = json + '\n';

    return new Promise((resolve, reject) => {
      this.socket!.write(line, (error) => {
        if (error) {
          reject(error);
        } else {
          resolve();
        }
      });
    });
  }

  /**
   * Disconnect from the server
   */
  disconnect(): void {
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
      this.reconnectTimer = undefined;
    }

    if (this.socket) {
      this.socket.destroy();
      this.socket = undefined;
    }

    this.state = TransportState.DISCONNECTED;
    this.emit('disconnected');
  }

  /**
   * Get current state
   */
  getState(): TransportState {
    return this.state;
  }

  /**
   * Check if connected
   */
  isConnected(): boolean {
    return this.state === TransportState.CONNECTED;
  }
}
