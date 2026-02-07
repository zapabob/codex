import { useEffect } from 'react';
import { a2aBus, type A2AMessage } from '../lib/api/A2ABus';

export function useA2A(onMessage?: (msg: A2AMessage) => void) {
  useEffect(() => {
    if (onMessage) {
      // Wrapper to ensure stable function reference if needed, 
      // but simpler to just use onMessage if user wraps it in useCallback
      a2aBus.subscribe(onMessage);
      return () => {
        a2aBus.unsubscribe(onMessage);
      };
    }
  }, [onMessage]);

  const broadcast = (msg: Omit<A2AMessage, 'timestamp' | 'from'> & { from?: string }) => {
    a2aBus.broadcast(msg);
  };

  return { broadcast, history: a2aBus.getHistory() };
}
