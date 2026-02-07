import { useEffect, useState } from 'react';
import { bridge } from '../lib/api/Bridge';

export function useBridge() {
  const [connected, setConnected] = useState(false);

  useEffect(() => {
    bridge.connect()
      .then(() => setConnected(true))
      .catch((err) => console.error("Bridge connection failed:", err));
  }, []);

  return { connected, bridge };
}
