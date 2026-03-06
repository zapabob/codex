import { useState, useEffect, useCallback, useRef } from 'react';
import { getBridge, resetBridge, MCPBridge } from '../services/mcpBridge';
import type { MCPClientInfo, AgentStatus, Notification } from '../types/mcp';

interface UseMCPBridgeOptions {
  autoConnect?: boolean;
  clientInfo?: Partial<MCPClientInfo>;
  onConnectionChange?: (connected: boolean) => void;
  onAgentStatusChange?: (agents: AgentStatus[]) => void;
  onNotification?: (notification: Notification) => void;
}

export function useMCPBridge(options: UseMCPBridgeOptions = {}) {
  const {
    autoConnect = true,
    clientInfo,
    onConnectionChange,
    onAgentStatusChange,
    onNotification,
  } = options;

  const [connected, setConnected] = useState(false);
  const [connecting, setConnecting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [serverInfo, setServerInfo] = useState<{ name: string; version: string } | null>(null);
  const [agentStatuses, setAgentStatuses] = useState<AgentStatus[]>([]);

  const bridgeRef = useRef<MCPBridge | null>(null);

  useEffect(() => {
    bridgeRef.current = getBridge({
      clientInfo: clientInfo
        ? { name: clientInfo.name || 'codex-gui', version: clientInfo.version || '3.0.0' }
        : undefined,
    });

    const unsubConnection = bridgeRef.current.onConnectionChanged((isConnected) => {
      setConnected(isConnected);
      setConnecting(false);
      onConnectionChange?.(isConnected);
    });

    const unsubAgentStatus = bridgeRef.current.onAgentStatusChanged((agents) => {
      setAgentStatuses(agents);
      onAgentStatusChange?.(agents);
    });

    const unsubNotification = bridgeRef.current.onNotificationReceived((notification) => {
      onNotification?.(notification);
    });

    if (autoConnect) {
      setConnecting(true);
      bridgeRef.current
        .connect()
        .catch((err) => {
          setError(err instanceof Error ? err.message : 'Connection failed');
          setConnecting(false);
        });
    }

    return () => {
      unsubConnection();
      unsubAgentStatus();
      unsubNotification();
    };
  }, [autoConnect, clientInfo, onConnectionChange, onAgentStatusChange, onNotification]);

  const connect = useCallback(async () => {
    if (!bridgeRef.current) return;

    setConnecting(true);
    setError(null);

    try {
      await bridgeRef.current.connect();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Connection failed');
      setConnecting(false);
      throw err;
    }
  }, []);

  const disconnect = useCallback(() => {
    bridgeRef.current?.disconnect();
  }, []);

  const reconnect = useCallback(async () => {
    disconnect();
    await new Promise((resolve) => setTimeout(resolve, 1000));
    await connect();
  }, [connect, disconnect]);

  return {
    bridge: bridgeRef.current,
    connected,
    connecting,
    error,
    serverInfo,
    agentStatuses,
    connect,
    disconnect,
    reconnect,
  };
}

export function resetMCPBridge() {
  resetBridge();
}
