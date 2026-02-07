import React, { useEffect, useState } from 'react';
import { WorktreeApi } from '../../lib/api/WorktreeApi';
import type { WorktreeInfo } from '../../lib/api/WorktreeApi';
import { GitBranch, Plus, Trash2, GitMerge, Layout, Activity } from 'lucide-react';

interface WorktreeDashboardProps {
  repoPath: string;
}

export const WorktreeDashboard: React.FC<WorktreeDashboardProps> = ({ repoPath }) => {
  const [worktrees, setWorktrees] = useState<WorktreeInfo[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchWorktrees = React.useCallback(async () => {
    setLoading(true);
    try {
      const data = await WorktreeApi.list(repoPath);
      setWorktrees(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, [repoPath]);

  useEffect(() => {
    if (repoPath) {
      fetchWorktrees();
    }
  }, [repoPath, fetchWorktrees]);

  const handleCreate = async () => {
    const agentName = prompt("Enter agent name:");
    const taskId = prompt("Enter task ID:");
    if (agentName && taskId) {
      try {
        await WorktreeApi.create(repoPath, agentName, taskId);
        fetchWorktrees();
      } catch (err) {
        alert(err instanceof Error ? err.message : String(err));
      }
    }
  };

  const handleRemove = async (name: string) => {
    if (confirm(`Are you sure you want to remove worktree ${name}?`)) {
      try {
        await WorktreeApi.remove(repoPath, name);
        fetchWorktrees();
      } catch (err) {
        alert(err instanceof Error ? err.message : String(err));
      }
    }
  };

  return (
    <div className="flex flex-col h-full bg-[#0a0a0a] text-gray-200 p-6 overflow-auto">
      <div className="flex items-center justify-between mb-8">
        <div className="flex items-center gap-3">
          <div className="p-2 bg-indigo-500/10 rounded-lg">
            <Layout className="w-6 h-6 text-indigo-400" />
          </div>
          <div>
            <h2 className="text-xl font-semibold text-white">Parallel Orchestration</h2>
            <p className="text-sm text-gray-400">Manage development worktrees and agents</p>
          </div>
        </div>
        <button 
          onClick={handleCreate}
          className="flex items-center gap-2 px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg transition-colors shadow-lg shadow-indigo-500/20"
        >
          <Plus className="w-4 h-4" />
          <span>New Worktree</span>
        </button>
      </div>

      {loading && worktrees.length === 0 ? (
        <div className="flex-1 flex flex-col items-center justify-center text-gray-400 gap-4">
          <Activity className="w-8 h-8 animate-pulse text-indigo-400" />
          <p className="animate-pulse">Analyzing repository state...</p>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {worktrees.map((wt) => (
            <div 
              key={wt.name} 
              className="bg-[#1a1a1a] border border-gray-800 rounded-xl p-5 hover:border-indigo-500/50 transition-all group"
            >
              <div className="flex items-start justify-between mb-4">
                <div className="flex items-center gap-3">
                  <div className="w-10 h-10 rounded-full bg-indigo-500/10 flex items-center justify-center text-indigo-400 font-bold">
                    {wt.agent[0].toUpperCase()}
                  </div>
                  <div>
                    <h3 className="font-medium text-white group-hover:text-indigo-400 transition-colors uppercase tracking-wider text-xs">
                      {wt.agent} Agent
                    </h3>
                    <div className="text-sm font-semibold mt-0.5 truncate max-w-[150px]">
                      {wt.name}
                    </div>
                  </div>
                </div>
                <div className="flex gap-1">
                  <button 
                    onClick={() => handleRemove(wt.name)}
                    className="p-2 hover:bg-red-500/10 hover:text-red-400 text-gray-500 rounded-lg transition-colors"
                    title="Remove Worktree"
                  >
                    <Trash2 className="w-4 h-4" />
                  </button>
                </div>
              </div>

              <div className="space-y-3 mb-6">
                <div className="flex items-center gap-2 text-sm text-gray-400">
                  <GitBranch className="w-4 h-4" />
                  <span className="truncate">{wt.branch}</span>
                </div>
                <div className="flex items-center gap-2 text-sm text-gray-400 bg-black/30 p-2 rounded-md">
                  <span className="text-[10px] text-gray-500 font-mono">PATH:</span>
                  <span className="truncate flex-1 font-mono text-[11px]">{wt.path}</span>
                </div>
              </div>

              <div className="flex gap-2">
                <button className="flex-1 flex items-center justify-center gap-2 py-2 bg-gray-800 hover:bg-gray-700 text-white rounded-lg text-sm font-medium transition-colors border border-gray-700">
                  <Activity className="w-4 h-4 text-emerald-400" />
                  Monitor
                </button>
                <button className="flex-1 flex items-center justify-center gap-2 py-2 bg-indigo-600/10 hover:bg-indigo-600/20 text-indigo-400 rounded-lg text-sm font-medium transition-colors border border-indigo-500/20">
                  <GitMerge className="w-4 h-4" />
                  Merge
                </button>
              </div>
            </div>
          ))}

          {worktrees.length === 0 && !loading && (
            <div className="col-span-full py-12 flex flex-col items-center justify-center text-gray-500 border-2 border-dashed border-gray-800 rounded-2xl">
              <Layout className="w-12 h-12 mb-4 opacity-20" />
              <p>No active worktrees found</p>
              <p className="text-sm opacity-50">Create a new worktree to start parallel development</p>
            </div>
          )}
        </div>
      )}

      {error && (
        <div className="mt-8 p-4 bg-red-500/10 border border-red-500/20 rounded-xl text-red-400 text-sm flex items-center gap-3">
          <div className="w-2 h-2 rounded-full bg-red-500 animate-pulse" />
          {error}
        </div>
      )}
    </div>
  );
};
