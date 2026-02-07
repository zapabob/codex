import { bridge } from './Bridge';

export interface WorktreeInfo {
  name: string;
  path: string;
  branch: string;
  agent: string;
}

export class WorktreeApi {
  static async list(repoPath: string): Promise<WorktreeInfo[]> {
    const response = await bridge.request('worktree/list', { repoPath });
    return response.worktrees;
  }

  static async create(repoPath: string, agentName: string, taskId: string): Promise<WorktreeInfo> {
    const response = await bridge.request('worktree/create', { agentName, taskId, repoPath });
    return response.worktree;
  }

  static async remove(repoPath: string, worktreeName: string): Promise<boolean> {
    const response = await bridge.request('worktree/remove', { worktreeName, repoPath });
    return response.success;
  }

  static async merge(repoPath: string, worktree: WorktreeInfo, targetBranch: string): Promise<boolean> {
    const response = await bridge.request('worktree/merge', { worktree, targetBranch, repoPath });
    return response.success;
  }
}
