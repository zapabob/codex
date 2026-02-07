import { bridge } from './Bridge';

export interface WorktreeInfo {
  name: string;
  path: string;
  branch: string;
  agent: string;
}

export class WorktreeApi {
  static async list(repoPath: string): Promise<WorktreeInfo[]> {
    const response = (await bridge.request('worktree/list', { repoPath })) as {
      worktrees: WorktreeInfo[];
    };
    return response.worktrees;
  }

  static async create(repoPath: string, agentName: string, taskId: string): Promise<WorktreeInfo> {
    const response = (await bridge.request('worktree/create', { agentName, taskId, repoPath })) as {
      worktree: WorktreeInfo;
    };
    return response.worktree;
  }

  static async remove(repoPath: string, worktreeName: string): Promise<boolean> {
    const response = (await bridge.request('worktree/remove', { worktreeName, repoPath })) as {
      success: boolean;
    };
    return response.success;
  }

  static async merge(repoPath: string, worktree: WorktreeInfo, targetBranch: string): Promise<boolean> {
    const response = (await bridge.request('worktree/merge', {
      worktree,
      targetBranch,
      repoPath,
    })) as { success: boolean };
    return response.success;
  }
}
