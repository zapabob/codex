'use client';

import { useEffect, useState, useCallback } from 'react';
import { apiClient } from '@/lib/api/client';

export interface LockStatus {
  holder: string;
  pid: number;
  hostname: string;
  since: number;
  stale: boolean;
}

export interface AgentStatus {
  id: string;
  name: string;
  status: string;
  tasks_completed: number;
  tasks_failed: number;
}

export interface TokenUsage {
  prompt_tokens: number;
  completion_tokens: number;
  total_tokens: number;
  last_updated: number;
}

export interface TokenStatus {
  used: number;
  budget: number;
  by_agent: Array<[string, TokenUsage]>;
}

export interface PairSession {
  session_id: string;
  participants: string[];
  roles: string[];
  started_at: number;
  ended_at?: number;
  notes: string;
}

export interface OrchestratorStatus {
  lock?: LockStatus;
  agents: AgentStatus[];
  tokens: TokenStatus;
  sessions: PairSession[];
}

export interface UseOrchestratorStatusOptions {
  /**
   * Polling interval in milliseconds
   * @default 5000
   */
  pollingInterval?: number;

  /**
   * Enable or disable polling
   * @default true
   */
  enabled?: boolean;

  /**
   * Auto-retry on error
   * @default true
   */
  autoRetry?: boolean;
}

/**
 * Hook to fetch and monitor orchestrator status
 */
export function useOrchestratorStatus(
  options: UseOrchestratorStatusOptions = {}
) {
  const {
    pollingInterval = 5000,
    enabled = true,
    autoRetry = true,
  } = options;

  const [status, setStatus] = useState<OrchestratorStatus | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  const fetchStatus = useCallback(async () => {
    try {
      setIsLoading(true);
      const data = await apiClient.getOrchestratorStatus();
      setStatus(data);
      setError(null);
    } catch (err) {
      const error = err instanceof Error ? err : new Error('Failed to fetch orchestrator status');
      setError(error);
      
      if (!autoRetry) {
        setIsLoading(false);
      }
    } finally {
      setIsLoading(false);
    }
  }, [autoRetry]);

  useEffect(() => {
    if (!enabled) {
      return;
    }

    // Initial fetch
    fetchStatus();

    // Set up polling
    const intervalId = setInterval(fetchStatus, pollingInterval);

    return () => {
      clearInterval(intervalId);
    };
  }, [enabled, pollingInterval, fetchStatus]);

  return {
    status,
    isLoading,
    error,
    refetch: fetchStatus,
  };
}
