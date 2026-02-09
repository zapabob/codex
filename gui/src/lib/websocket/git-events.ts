// SPDX-License-Identifier: Apache-2.0

"use client";

import { EventEmitter } from "events";
import { useState, useEffect, useRef } from "react";
import { loadConfig, WebSocketConfig } from "@/lib/config";
import type { GitEvent } from "@/lib/github";

export interface WSMessage {
  type: "event" | "subscribe" | "unsubscribe" | "error" | "ping" | "pong";
  payload: Record<string, unknown>;
  timestamp: string;
}

export interface WebSocketClientConfig {
  server_url?: string;
  reconnect_interval?: number;
  max_reconnect_attempts?: number;
}

export class GitEventWebSocket extends EventEmitter {
  private ws: WebSocket | null = null;
  private config: WebSocketConfig;
  private reconnectAttempts = 0;
  private reconnectTimer: ReturnType<typeof setTimeout> | null = null;
  private pingTimer: ReturnType<typeof setInterval> | null = null;
  private subscribedRepos = new Set<string>();
  private isIntentionallyClosed = false;

  constructor(config?: WebSocketClientConfig) {
    super();
    const fullConfig = loadConfig();

    this.config = {
      server_url: config?.server_url || fullConfig.websocket.server_url,
      reconnect_interval:
        config?.reconnect_interval || fullConfig.websocket.reconnect_interval,
      max_reconnect_attempts:
        config?.max_reconnect_attempts ||
        fullConfig.websocket.max_reconnect_attempts,
    };
  }

  connect(): void {
    if (this.ws?.readyState === WebSocket.OPEN) {
      console.log("[WS] Already connected");
      return;
    }

    this.isIntentionallyClosed = false;
    this.createConnection();
  }

  private createConnection(): void {
    try {
      console.log(`[WS] Connecting to ${this.config.server_url}`);
      this.ws = new WebSocket(this.config.server_url);

      this.ws.onopen = () => {
        console.log("[WS] Connected");
        this.reconnectAttempts = 0;
        this.startPing();
        this.resubscribeAll();
        this.emit("open");
      };

      this.ws.onmessage = (event) => {
        try {
          const message: WSMessage = JSON.parse(event.data);
          this.handleMessage(message);
        } catch (error) {
          console.error("[WS] Failed to parse message:", error);
        }
      };

      this.ws.onclose = (event) => {
        console.log(`[WS] Disconnected (code: ${event.code})`);
        this.stopPing();
        this.emit("close", event);

        if (!this.isIntentionallyClosed) {
          this.attemptReconnect();
        }
      };

      this.ws.onerror = (error) => {
        console.error("[WS] Error:", error);
        this.emit("error", error);
      };
    } catch (error) {
      console.error("[WS] Failed to create connection:", error);
      this.attemptReconnect();
    }
  }

  private handleMessage(message: WSMessage): void {
    switch (message.type) {
      case "event":
        const event = message.payload as GitEvent;
        this.emit("event", event);
        this.emit(event.type, event);
        break;

      case "pong":
        this.emit("pong", message.timestamp);
        break;

      case "error":
        console.error("[WS] Server error:", message.payload);
        this.emit("server_error", message.payload);
        break;

      default:
        this.emit("message", message);
    }
  }

  private startPing(): void {
    this.pingTimer = setInterval(() => {
      this.send({
        type: "ping",
        payload: {},
        timestamp: new Date().toISOString(),
      });
    }, 30000);
  }

  private stopPing(): void {
    if (this.pingTimer) {
      clearInterval(this.pingTimer);
      this.pingTimer = null;
    }
  }

  private attemptReconnect(): void {
    if (this.reconnectAttempts >= this.config.max_reconnect_attempts) {
      console.error("[WS] Max reconnect attempts reached");
      this.emit("max_reconnect_attempts_reached");
      return;
    }

    this.reconnectAttempts++;
    const delay = this.config.reconnect_interval * this.reconnectAttempts;

    console.log(
      `[WS] Reconnecting in ${delay}ms (attempt ${this.reconnectAttempts})`,
    );
    this.emit("reconnecting", this.reconnectAttempts);

    this.reconnectTimer = setTimeout(() => {
      this.createConnection();
    }, delay);
  }

  private resubscribeAll(): void {
    for (const repo of this.subscribedRepos) {
      this.subscribe(repo);
    }
  }

  subscribe(repo: string): void {
    this.subscribedRepos.add(repo);
    this.send({
      type: "subscribe",
      payload: { repo },
      timestamp: new Date().toISOString(),
    });
  }

  unsubscribe(repo: string): void {
    this.subscribedRepos.delete(repo);
    this.send({
      type: "unsubscribe",
      payload: { repo },
      timestamp: new Date().toISOString(),
    });
  }

  private send(message: WSMessage): void {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(message));
    } else {
      console.warn("[WS] Cannot send, not connected");
    }
  }

  disconnect(): void {
    this.isIntentionallyClosed = true;

    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
      this.reconnectTimer = null;
    }

    this.stopPing();

    if (this.ws) {
      this.ws.close(1000, "Client disconnect");
      this.ws = null;
    }

    this.subscribedRepos.clear();
  }

  isConnected(): boolean {
    return this.ws?.readyState === WebSocket.OPEN;
  }

  getSubscribedRepos(): string[] {
    return Array.from(this.subscribedRepos);
  }
}

export function useGitEventsWS(
  owner: string,
  repo: string,
): {
  events: GitEvent[];
  isConnected: boolean;
  reconnectAttempts: number;
} {
  const [events, setEvents] = useState<GitEvent[]>([]);
  const [isConnected, setIsConnected] = useState(false);
  const [reconnectAttempts, setReconnectAttempts] = useState(0);
  const wsRef = useRef<GitEventWebSocket | null>(null);

  useEffect(() => {
    const ws = new GitEventWebSocket();
    wsRef.current = ws;

    const repoFull = `${owner}/${repo}`;
    ws.subscribe(repoFull);
    ws.connect();

    ws.on("event", (event: GitEvent) => {
      setEvents((prev) => [event, ...prev].slice(0, 200));
    });

    ws.on("open", () => {
      setIsConnected(true);
      setReconnectAttempts(0);
    });

    ws.on("close", () => {
      setIsConnected(false);
    });

    ws.on("reconnecting", (attempts) => {
      setReconnectAttempts(attempts as number);
    });

    ws.on("max_reconnect_attempts_reached", () => {
      console.error("[WS] Max reconnect attempts");
    });

    return () => {
      ws.disconnect();
    };
  }, [owner, repo]);

  return { events, isConnected, reconnectAttempts };
}
