export interface Worktree {
  id: string;
  path: string;
  branch: string;
  repository: string;
  status: "idle" | "running" | "error" | "busy";
  lastActivity: Date;
  createdAt: Date;
  pinned: boolean;
  tags: string[];
}

export interface TaskStatus {
  taskId: string;
  worktreeId: string;
  status: "pending" | "running" | "completed" | "failed";
  startTime: Date;
  endTime?: Date;
  output?: string;
  error?: string;
}

export interface Task {
  id: string;
  name: string;
  command: string;
  workingDirectory?: string;
  env?: Record<string, string>;
  onOutput?: (output: string) => void;
  onComplete?: (exitCode: number) => void;
  onError?: (error: string) => void;
}

export interface WorktreeConfig {
  autoSync: boolean;
  conflictPrevention: boolean;
  maxWorktrees: number;
  cleanupAfterDays: number;
}

export interface ConflictInfo {
  type: "branch" | "file" | "resource";
  message: string;
  resolution?: string[];
  requiresManualResolution: boolean;
}
