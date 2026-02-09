import { useCallback } from "react";
import { useWorktreeStore } from "../store/useWorktreeStore";
import type { Worktree, Task, TaskStatus } from "../types/worktree";

export function useWorktree() {
  const {
    worktrees,
    activeWorktreeId,
    pinnedThreads,
    runningTasks,
    config,
    createWorktree,
    deleteWorktree,
    setActiveWorktree,
    pinThread,
    unpinThread,
    runTask,
    updateTaskStatus,
    completeTask,
    updateConfig,
    getWorktree,
    getActiveWorktree,
  } = useWorktreeStore();

  const activeWorktree = getActiveWorktree();
  const activeTasks = Array.from(runningTasks.values()).filter(
    (t) => t.worktreeId === activeWorktreeId,
  );

  const handleCreateWorktree = useCallback(
    async (repo: string, branch: string) => {
      return createWorktree(repo, branch);
    },
    [createWorktree],
  );

  const handleDeleteWorktree = useCallback(
    async (id: string) => {
      return deleteWorktree(id);
    },
    [deleteWorktree],
  );

  const handleRunTask = useCallback(
    async (task: Task, worktreeId?: string) => {
      const targetWorktreeId = worktreeId || activeWorktreeId;
      if (!targetWorktreeId) {
        throw new Error("No active worktree");
      }
      return runTask(task, targetWorktreeId);
    },
    [runTask, activeWorktreeId],
  );

  return {
    worktrees,
    activeWorktree,
    activeWorktreeId,
    pinnedThreads,
    runningTasks,
    activeTasks,
    config,
    createWorktree: handleCreateWorktree,
    deleteWorktree: handleDeleteWorktree,
    setActiveWorktree,
    pinThread,
    unpinThread,
    runTask: handleRunTask,
    updateTaskStatus,
    completeTask,
    updateConfig,
    getWorktree,
  };
}

export function useTask(taskId: string) {
  const { runningTasks, updateTaskStatus, completeTask } = useWorktreeStore();

  const task = runningTasks.get(taskId);

  const updateStatus = useCallback(
    (status: Partial<TaskStatus>) => {
      updateTaskStatus(taskId, status);
    },
    [updateTaskStatus, taskId],
  );

  const complete = useCallback(
    (success: boolean, output?: string, error?: string) => {
      completeTask(taskId, success, output, error);
    },
    [completeTask, taskId],
  );

  return {
    task,
    updateStatus,
    complete,
  };
}
