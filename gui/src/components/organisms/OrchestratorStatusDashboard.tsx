'use client';

import React from 'react';
import { Card, CardContent, CardHeader } from '@mui/material';
import { Activity, Lock, Users, Zap } from 'lucide-react';
import { useOrchestratorStatus } from '@/hooks/useOrchestratorStatus';

export function OrchestratorStatusDashboard() {
  const { status, isLoading, error } = useOrchestratorStatus({
    pollingInterval: 5000,
    enabled: true,
  });

  if (isLoading && !status) {
    return (
      <div className="flex items-center justify-center p-8">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900 dark:border-white"></div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg">
        <p className="text-red-800 dark:text-red-200">
          Failed to load orchestrator status: {error.message}
        </p>
      </div>
    );
  }

  if (!status) {
    return null;
  }

  const tokenUsagePercent = (status.tokens.used / status.tokens.budget) * 100;
  const tokenUsageColor = tokenUsagePercent > 80 ? 'text-red-600' : tokenUsagePercent > 60 ? 'text-yellow-600' : 'text-green-600';

  return (
    <div className="space-y-4">
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        {/* Lock Status */}
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <h3 className="text-sm font-medium">Lock Status</h3>
            <Lock className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            {status.lock ? (
              <div>
                <div className="text-2xl font-bold">
                  {status.lock.stale ? 'Stale' : 'Locked'}
                </div>
                <p className="text-xs text-muted-foreground">
                  by {status.lock.hostname} (PID: {status.lock.pid})
                </p>
              </div>
            ) : (
              <div>
                <div className="text-2xl font-bold text-green-600">Free</div>
                <p className="text-xs text-muted-foreground">No active lock</p>
              </div>
            )}
          </CardContent>
        </Card>

        {/* Active Agents */}
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <h3 className="text-sm font-medium">Active Agents</h3>
            <Activity className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{status.agents.length}</div>
            <p className="text-xs text-muted-foreground">
              {status.agents.filter(a => a.status === 'active').length} running
            </p>
          </CardContent>
        </Card>

        {/* Token Usage */}
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <h3 className="text-sm font-medium">Token Usage</h3>
            <Zap className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className={`text-2xl font-bold ${tokenUsageColor}`}>
              {tokenUsagePercent.toFixed(1)}%
            </div>
            <p className="text-xs text-muted-foreground">
              {status.tokens.used.toLocaleString()} / {status.tokens.budget.toLocaleString()}
            </p>
          </CardContent>
        </Card>

        {/* Sessions */}
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <h3 className="text-sm font-medium">Pair Sessions</h3>
            <Users className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{status.sessions.length}</div>
            <p className="text-xs text-muted-foreground">
              {status.sessions.filter(s => !s.ended_at).length} active
            </p>
          </CardContent>
        </Card>
      </div>

      {/* Agent Details */}
      {status.agents.length > 0 && (
        <Card>
          <CardHeader>
            <h3 className="text-lg font-semibold">Agent Details</h3>
          </CardHeader>
          <CardContent>
            <div className="space-y-2">
              {status.agents.map((agent) => (
                <div
                  key={agent.id}
                  className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-800 rounded-lg"
                >
                  <div>
                    <div className="font-medium">{agent.name}</div>
                    <div className="text-sm text-gray-500">
                      {agent.tasks_completed} completed, {agent.tasks_failed} failed
                    </div>
                  </div>
                  <span
                    className={`px-2 py-1 text-xs rounded ${
                      agent.status === 'active'
                        ? 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200'
                        : 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200'
                    }`}
                  >
                    {agent.status}
                  </span>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      )}

      {/* Token Usage by Agent */}
      {status.tokens.by_agent.length > 0 && (
        <Card>
          <CardHeader>
            <h3 className="text-lg font-semibold">Token Usage by Agent</h3>
          </CardHeader>
          <CardContent>
            <div className="space-y-2">
              {status.tokens.by_agent.map(([agentId, usage]) => {
                const agentPercent = (usage.total_tokens / status.tokens.budget) * 100;
                return (
                  <div key={agentId} className="space-y-1">
                    <div className="flex items-center justify-between text-sm">
                      <span className="font-medium">{agentId}</span>
                      <span className="text-gray-500">
                        {usage.total_tokens.toLocaleString()} tokens ({agentPercent.toFixed(1)}%)
                      </span>
                    </div>
                    <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                      <div
                        className="bg-blue-600 h-2 rounded-full"
                        style={{ width: `${Math.min(agentPercent, 100)}%` }}
                      ></div>
                    </div>
                  </div>
                );
              })}
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
