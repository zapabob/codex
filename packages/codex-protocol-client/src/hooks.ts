/**
 * React hook for using the protocol client in React applications
 */

import { useEffect, useState, useCallback, useRef } from 'react';
import { ProtocolClient, ProtocolClientConfig } from './client';
import { Topic } from './types';

export interface UseProtocolOptions extends ProtocolClientConfig {
  autoConnect?: boolean;
  subscribeTopics?: Topic[];
}

export interface ProtocolState {
  connected: boolean;
  connecting: boolean;
  error?: Error;
}

export interface UseProtocolResult {
  client: ProtocolClient | null;
  state: ProtocolState;
  connect: () => Promise<void>;
  disconnect: () => void;
  subscribe: (topics: Topic[]) => Promise<void>;
  unsubscribe: (topics: Topic[]) => Promise<void>;
}

/**
 * React hook for using the protocol client
 */
export function useProtocol(options: UseProtocolOptions = {}): UseProtocolResult {
  const [state, setState] = useState<ProtocolState>({
    connected: false,
    connecting: false,
  });

  const clientRef = useRef<ProtocolClient | null>(null);

  // Initialize client
  useEffect(() => {
    const client = new ProtocolClient(options);
    clientRef.current = client;

    // Setup event handlers
    client.on('connected', () => {
      setState({ connected: true, connecting: false });
    });

    client.on('disconnected', () => {
      setState({ connected: false, connecting: false });
    });

    client.on('error', (error: Error) => {
      setState(prev => ({ ...prev, error }));
    });

    // Auto-connect if requested
    if (options.autoConnect !== false) {
      setState({ connected: false, connecting: true });
      client.connect().catch((error: Error) => {
        setState({ connected: false, connecting: false, error });
      });
    }

    // Auto-subscribe to topics if provided
    if (options.subscribeTopics && options.subscribeTopics.length > 0) {
      client.on('connected', () => {
        client.subscribe(options.subscribeTopics!).catch((error: Error) => {
          console.error('Failed to subscribe to topics:', error);
        });
      });
    }

    // Cleanup
    return () => {
      client.disconnect();
      client.removeAllListeners();
      clientRef.current = null;
    };
  }, []); // Only run once on mount

  const connect = useCallback(async () => {
    if (!clientRef.current) return;
    
    setState(prev => ({ ...prev, connecting: true }));
    try {
      await clientRef.current.connect();
    } catch (error) {
      setState(prev => ({ ...prev, connecting: false, error: error as Error }));
      throw error;
    }
  }, []);

  const disconnect = useCallback(() => {
    if (!clientRef.current) return;
    clientRef.current.disconnect();
  }, []);

  const subscribe = useCallback(async (topics: Topic[]) => {
    if (!clientRef.current) {
      throw new Error('Client not initialized');
    }
    await clientRef.current.subscribe(topics);
  }, []);

  const unsubscribe = useCallback(async (topics: Topic[]) => {
    if (!clientRef.current) {
      throw new Error('Client not initialized');
    }
    await clientRef.current.unsubscribe(topics);
  }, []);

  return {
    client: clientRef.current,
    state,
    connect,
    disconnect,
    subscribe,
    unsubscribe,
  };
}

/**
 * Hook for subscribing to protocol events
 */
export function useProtocolEvent(
  client: ProtocolClient | null,
  topic: Topic,
  handler: (data: any) => void
): void {
  useEffect(() => {
    if (!client) return;

    const eventHandler = (data: any) => handler(data);
    client.on(`event:${topic}`, eventHandler);

    return () => {
      client.off(`event:${topic}`, eventHandler);
    };
  }, [client, topic, handler]);
}
