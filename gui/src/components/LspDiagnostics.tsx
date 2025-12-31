'use client';

import React, { useEffect, useState } from 'react';
import { LspClient, LspDiagnostic } from '@/lib/lsp/LspClient';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';

interface LspDiagnosticsProps {
  lspClient: LspClient | null;
  currentFile?: string;
  onDiagnosticClick?: (diagnostic: LspDiagnostic, uri: string) => void;
}

export function LspDiagnostics({
  lspClient,
  currentFile,
  onDiagnosticClick,
}: LspDiagnosticsProps) {
  const [diagnostics, setDiagnostics] = useState<Map<string, LspDiagnostic[]>>(new Map());
  const [filter, setFilter] = useState<'all' | 'error' | 'warning' | 'information' | 'hint'>('all');
  const [sortBy, setSortBy] = useState<'severity' | 'file' | 'line'>('severity');

  useEffect(() => {
    if (!lspClient) return;

    const unsubscribe = lspClient.onDiagnostics((newDiagnostics) => {
      setDiagnostics(newDiagnostics);
    });

    return unsubscribe;
  }, [lspClient]);

  const getFilteredDiagnostics = (): Array<{ uri: string; diagnostic: LspDiagnostic }> => {
    const result: Array<{ uri: string; diagnostic: LspDiagnostic }> = [];

    diagnostics.forEach((diags, uri) => {
      if (currentFile && uri !== currentFile) return;

      diags.forEach((diagnostic) => {
        if (filter === 'all' || diagnostic.severity === filter) {
          result.push({ uri, diagnostic });
        }
      });
    });

    // Sort diagnostics
    result.sort((a, b) => {
      if (sortBy === 'severity') {
        const severityOrder = { error: 0, warning: 1, information: 2, hint: 3 };
        const aSeverity = severityOrder[a.diagnostic.severity] ?? 4;
        const bSeverity = severityOrder[b.diagnostic.severity] ?? 4;
        if (aSeverity !== bSeverity) return aSeverity - bSeverity;
        return a.diagnostic.range.start.line - b.diagnostic.range.start.line;
      } else if (sortBy === 'file') {
        return a.uri.localeCompare(b.uri);
      } else {
        return a.diagnostic.range.start.line - b.diagnostic.range.start.line;
      }
    });

    return result;
  };

  const getSeverityColor = (severity: string): string => {
    switch (severity) {
      case 'error':
        return 'bg-red-500/10 text-red-500 border-red-500/20';
      case 'warning':
        return 'bg-yellow-500/10 text-yellow-500 border-yellow-500/20';
      case 'information':
        return 'bg-blue-500/10 text-blue-500 border-blue-500/20';
      case 'hint':
        return 'bg-gray-500/10 text-gray-500 border-gray-500/20';
      default:
        return 'bg-gray-500/10 text-gray-500 border-gray-500/20';
    }
  };

  const getSeverityIcon = (severity: string): string => {
    switch (severity) {
      case 'error':
        return '✕';
      case 'warning':
        return '⚠';
      case 'information':
        return 'ℹ';
      case 'hint':
        return '💡';
      default:
        return '○';
    }
  };

  const filteredDiagnostics = getFilteredDiagnostics();
  const stats = {
    error: diagnostics.size > 0
      ? Array.from(diagnostics.values())
          .flat()
          .filter((d) => d.severity === 'error').length
      : 0,
    warning: diagnostics.size > 0
      ? Array.from(diagnostics.values())
          .flat()
          .filter((d) => d.severity === 'warning').length
      : 0,
    information: diagnostics.size > 0
      ? Array.from(diagnostics.values())
          .flat()
          .filter((d) => d.severity === 'information').length
      : 0,
    hint: diagnostics.size > 0
      ? Array.from(diagnostics.values())
          .flat()
          .filter((d) => d.severity === 'hint').length
      : 0,
  };

  return (
    <Card className="p-4">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-semibold">LSP Diagnostics</h3>
        <div className="flex gap-2">
          <select
            value={filter}
            onChange={(e) => setFilter(e.target.value as any)}
            className="px-2 py-1 text-sm border rounded"
          >
            <option value="all">All</option>
            <option value="error">Errors</option>
            <option value="warning">Warnings</option>
            <option value="information">Information</option>
            <option value="hint">Hints</option>
          </select>
          <select
            value={sortBy}
            onChange={(e) => setSortBy(e.target.value as any)}
            className="px-2 py-1 text-sm border rounded"
          >
            <option value="severity">Sort by Severity</option>
            <option value="file">Sort by File</option>
            <option value="line">Sort by Line</option>
          </select>
        </div>
      </div>

      <div className="flex gap-2 mb-4">
        <Badge variant="outline" className={stats.error > 0 ? 'text-red-500' : ''}>
          Errors: {stats.error}
        </Badge>
        <Badge variant="outline" className={stats.warning > 0 ? 'text-yellow-500' : ''}>
          Warnings: {stats.warning}
        </Badge>
        <Badge variant="outline">
          Info: {stats.information}
        </Badge>
        <Badge variant="outline">
          Hints: {stats.hint}
        </Badge>
      </div>

      <div className="space-y-2 max-h-96 overflow-y-auto">
        {filteredDiagnostics.length === 0 ? (
          <div className="text-center text-gray-500 py-8">
            No diagnostics found
          </div>
        ) : (
          filteredDiagnostics.map(({ uri, diagnostic }, index) => (
            <div
              key={`${uri}-${index}`}
              className={`p-3 rounded border cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-800 ${getSeverityColor(diagnostic.severity)}`}
              onClick={() => onDiagnosticClick?.(diagnostic, uri)}
            >
              <div className="flex items-start gap-2">
                <span className="text-lg">{getSeverityIcon(diagnostic.severity)}</span>
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2 mb-1">
                    <span className="font-medium">{diagnostic.message}</span>
                    {diagnostic.code && (
                      <Badge variant="outline" className="text-xs">
                        {diagnostic.code}
                      </Badge>
                    )}
                    {diagnostic.source && (
                      <Badge variant="outline" className="text-xs">
                        {diagnostic.source}
                      </Badge>
                    )}
                  </div>
                  <div className="text-sm text-gray-600 dark:text-gray-400">
                    {uri.split('/').pop()}:
                    {diagnostic.range.start.line + 1}:
                    {diagnostic.range.start.character + 1}
                  </div>
                </div>
              </div>
            </div>
          ))
        )}
      </div>
    </Card>
  );
}
