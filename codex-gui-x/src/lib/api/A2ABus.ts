import { bridge } from './Bridge';

export interface A2AMessage {
  from: string;
  to: string;
  type: 'status' | 'request' | 'audit' | 'merge_request' | 'optimize' | 'audit_result' | 'optimization_suggestion';
  content: unknown;
  timestamp: number;
}

export type A2AListener = (msg: A2AMessage) => void;

class A2ABus {
  private listeners: Set<A2AListener> = new Set();
  private history: A2AMessage[] = [];

  constructor() {
    // Listen for incoming A2A notifications from the bridge
    bridge.onNotification('a2a/message', (params: unknown) => {
      this.inject(params as A2AMessage);
    });
  }

  subscribe(listener: A2AListener) {
    this.listeners.add(listener);
    // Send history to new listener
    this.history.forEach(msg => listener(msg));
  }

  unsubscribe(listener: A2AListener) {
    this.listeners.delete(listener);
  }

  async broadcast(msg: Omit<A2AMessage, 'timestamp' | 'from'> & { from?: string }) {
    const fullMsg: A2AMessage = {
      ...msg,
      from: msg.from || 'Frontend-Orchestrator',
      timestamp: Date.now(),
    };
    
    // Inject locally
    this.inject(fullMsg);

    // Synchronize with backend if needed
    try {
      await bridge.request('a2a/broadcast', { message: fullMsg });
    } catch (e) {
      console.warn("A2A Backend Sync failed, continuing in-memory", e);
    }
  }

  private inject(msg: A2AMessage) {
    this.history.push(msg);
    this.listeners.forEach(l => l(msg));
  }

  getHistory() {
    return this.history;
  }
}

export const a2aBus = new A2ABus();
