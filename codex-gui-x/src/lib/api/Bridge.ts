export type JSONRPCId = string | number | null;

export interface JSONRPCRequest {
  jsonrpc: "2.0";
  method: string;
  params?: unknown;
  id: JSONRPCId;
}

export interface JSONRPCResponse {
  jsonrpc: "2.0";
  result?: unknown;
  error?: {
    code: number;
    message: string;
    data?: unknown;
  };
  id: JSONRPCId;
}

export interface JSONRPCNotification {
  jsonrpc: "2.0";
  method: string;
  params?: unknown;
}

export type JSONRPCMessage = JSONRPCRequest | JSONRPCResponse | JSONRPCNotification;

export type MessageHandler = (message: JSONRPCResponse | JSONRPCNotification) => void;

export class Bridge {
  private url: string;
  private ws: WebSocket | null = null;
  private nextId = 1;
  private pendingRequests = new Map<JSONRPCId, { resolve: (val: unknown) => void; reject: (err: unknown) => void }>();
  private handlers = new Set<MessageHandler>();
  private notificationHandlers = new Map<string, Set<(params: unknown) => void>>();

  constructor(url: string = "ws://localhost:8787") {
    this.url = url;
  }

  connect() {
    return new Promise<void>((resolve, reject) => {
      this.ws = new WebSocket(this.url);
      this.ws.onopen = () => resolve();
      this.ws.onerror = (err) => reject(err);
      this.ws.onmessage = (event) => this.handleMessage(event.data);
      this.ws.onclose = () => {
        console.warn("Bridge disconnected. Reconnecting in 3s...");
        setTimeout(() => this.connect(), 3000);
      };
    });
  }

  private handleMessage(data: string) {
    try {
      const message: JSONRPCMessage = JSON.parse(data);
      
      if ('id' in message && message.id !== null) {
        // Handle Response
        const pending = this.pendingRequests.get(message.id);
        if (pending) {
          this.pendingRequests.delete(message.id);
          if ('error' in message && message.error) {
            pending.reject(message.error);
          } else if ('result' in message) {
            pending.resolve(message.result);
          }
        }
      } else if ('method' in message) {
        // Handle Notification
        const handlers = this.notificationHandlers.get(message.method);
        if (handlers) {
          handlers.forEach(h => h(message.params));
        }
      }
      
      // Notify all general message handlers
      if (!('id' in message) || message.id === null) {
        // Only notify for notifications or broad messages
        this.handlers.forEach(h => h(message as JSONRPCNotification));
      }
    } catch (e) {
      console.error("Failed to parse message:", e);
    }
  }

  onNotification(method: string, handler: (params: unknown) => void) {
    if (!this.notificationHandlers.has(method)) {
      this.notificationHandlers.set(method, new Set());
    }
    this.notificationHandlers.get(method)!.add(handler);
    return () => this.notificationHandlers.get(method)?.delete(handler);
  }

  request(method: string, params?: unknown): Promise<unknown> {
    return new Promise((resolve, reject) => {
      if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
        return reject(new Error("Bridge not connected"));
      }

      const id = this.nextId++;
      const request: JSONRPCRequest = {
        jsonrpc: "2.0",
        method,
        params,
        id
      };

      this.pendingRequests.set(id, { resolve, reject });
      this.ws.send(JSON.stringify(request));
    });
  }

  notify(method: string, params?: unknown): void {
    if (!this.ws || this.ws.readyState !== WebSocket.OPEN) return;
    const notification: JSONRPCNotification = {
      jsonrpc: "2.0",
      method,
      params
    };
    this.ws.send(JSON.stringify(notification));
  }

  onMessage(handler: MessageHandler) {
    this.handlers.add(handler);
    return () => this.handlers.delete(handler);
  }
}

export const bridge = new Bridge();
