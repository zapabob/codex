import { create } from "zustand";
import { persist, devtools } from "zustand/middleware";
import type {
  Worktree,
  TaskStatus,
  Task,
  WorktreeConfig,
} from "../types/worktree";

interface WorktreeState {
  worktrees: Worktree[];
  activeWorktreeId: string | null;
  pinnedThreads: string[];
  runningTasks: Map<string, TaskStatus>;
  config: WorktreeConfig;

  // Actions
  createWorktree: (repo: string, branch: string) => Promise<Worktree>;
  deleteWorktree: (id: string) => Promise<void>;
  setActiveWorktree: (id: string | null) => void;
  pinThread: (threadId: string) => void;
  unpinThread: (threadId: string) => void;
  runTask: (task: Task, worktreeId: string) => Promise<string>;
  updateTaskStatus: (taskId: string, status: Partial<TaskStatus>) => void;
  completeTask: (
    taskId: string,
    success: boolean,
    output?: string,
    error?: string,
  ) => void;
  updateConfig: (config: Partial<WorktreeConfig>) => void;
  getWorktree: (id: string) => Worktree | undefined;
  getActiveWorktree: () => Worktree | undefined;
}

const defaultConfig: WorktreeConfig = {
  autoSync: true,
  conflictPrevention: true,
  maxWorktrees: 10,
  cleanupAfterDays: 7,
};

export const useWorktreeStore = create<WorktreeState>()(
  devtools(
    persist(
      (set, get) => ({
        worktrees: [],
        activeWorktreeId: null,
        pinnedThreads: [],
        runningTasks: new Map(),
        config: defaultConfig,

        createWorktree: async (repo: string, branch: string) => {
          const worktreeId = `wt-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;

          const worktree: Worktree = {
            id: worktreeId,
            path: `.worktrees/${worktreeId}`,
            branch,
            repository: repo,
            status: "idle",
            lastActivity: new Date(),
            createdAt: new Date(),
            pinned: false,
            tags: [],
          };

          // In real implementation, this would call the backend
          // const result = await window.codex.git.createWorktree({ repo, branch, worktreeId });

          set((state) => ({
            worktrees: [...state.worktrees, worktree],
            activeWorktreeId: worktreeId,
          }));

          return worktree;
        },

        deleteWorktree: async (id: string) => {
          // In real implementation: await window.codex.git.deleteWorktree({ id });

          set((state) => ({
            worktrees: state.worktrees.filter((wt) => wt.id !== id),
            activeWorktreeId:
              state.activeWorktreeId === id ? null : state.activeWorktreeId,
          }));
        },

        setActiveWorktree: (id: string | null) => {
          set({ activeWorktreeId: id });
        },

        pinThread: (threadId: string) => {
          set((state) => {
            if (state.pinnedThreads.includes(threadId)) {
              return state;
            }
            return { pinnedThreads: [...state.pinnedThreads, threadId] };
          });
        },

        unpinThread: (threadId: string) => {
          set((state) => ({
            pinnedThreads: state.pinnedThreads.filter((id) => id !== threadId),
          }));
        },

        runTask: async (task: Task, worktreeId: string) => {
          const taskId = `task-${Date.now()}`;

          const taskStatus: TaskStatus = {
            taskId,
            worktreeId,
            status: "running",
            startTime: new Date(),
          };

          set((state) => {
            const newTasks = new Map(state.runningTasks);
            newTasks.set(taskId, taskStatus);
            return { runningTasks: newTasks };
          });

          // In real implementation, this would execute via backend
          // const result = await window.codex.terminal.runTask({ task, worktreeId });

          return taskId;
        },

        updateTaskStatus: (taskId: string, status: Partial<TaskStatus>) => {
          set((state) => {
            const newTasks = new Map(state.runningTasks);
            const existing = newTasks.get(taskId);
            if (existing) {
              newTasks.set(taskId, { ...existing, ...status });
            }
            return { runningTasks: newTasks };
          });
        },

        completeTask: (
          taskId: string,
          success: boolean,
          output?: string,
          error?: string,
        ) => {
          set((state) => {
            const newTasks = new Map(state.runningTasks);
            const existing = newTasks.get(taskId);
            if (existing) {
              newTasks.set(taskId, {
                ...existing,
                status: success ? "completed" : "failed",
                endTime: new Date(),
                output,
                error,
              });
            }
            return { runningTasks: newTasks };
          });
        },

        updateConfig: (config: Partial<WorktreeConfig>) => {
          set((state) => ({
            config: { ...state.config, ...config },
          }));
        },

        getWorktree: (id: string) => {
          return get().worktrees.find((wt) => wt.id === id);
        },

        getActiveWorktree: () => {
          const { activeWorktreeId, worktrees } = get();
          return activeWorktreeId
            ? worktrees.find((wt) => wt.id === activeWorktreeId)
            : undefined;
        },
      }),
      {
        name: "codex-worktree-store",
        partialize: (state) => ({
          worktrees: state.worktrees.slice(-50),
          pinnedThreads: state.pinnedThreads,
          config: state.config,
        }),
      },
    ),
    { name: "WorktreeStore" },
  ),
);
