import { create } from 'zustand';
import { devtools } from 'zustand/middleware';
import type { AgentStatus, Task, TaskPriority, TaskStatus } from '../types/mcp';

interface AgentState {
  agents: AgentStatus[];
  tasks: Map<string, Task>;
  taskQueue: string[];
  activeTaskId: string | null;
  isAgentRunning: boolean;
  agentLogs: Map<string, string[]>;
  a2aEndpoints: Map<string, string>;
  isLoading: boolean;
  error: string | null;

  setAgents: (agents: AgentStatus[]) => void;
  updateAgent: (agentId: string, updates: Partial<AgentStatus>) => void;

  setTasks: (tasks: Task[]) => void;
  addTask: (task: Task) => void;
  updateTask: (taskId: string, updates: Partial<Task>) => void;
  removeTask: (taskId: string) => void;
  setActiveTask: (taskId: string | null) => void;

  addTaskToQueue: (taskId: string, priority?: TaskPriority) => void;
  removeTaskFromQueue: (taskId: string) => void;
  reorderQueue: (taskIds: string[]) => void;

  addAgentLog: (agentId: string, log: string) => void;
  clearAgentLogs: (agentId: string) => void;

  registerA2AEndpoint: (agentId: string, endpoint: string) => void;
  unregisterA2AEndpoint: (agentId: string) => void;

  setIsAgentRunning: (running: boolean) => void;
  setIsLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;

  getActiveTask: () => Task | null;
  getQueuedTasks: () => Task[];
  getTasksByStatus: (status: TaskStatus) => Task[];
}

export const useAgentStore = create<AgentState>()(
  devtools(
    (set, get) => ({
      agents: [],
      tasks: new Map(),
      taskQueue: [],
      activeTaskId: null,
      isAgentRunning: false,
      agentLogs: new Map(),
      a2aEndpoints: new Map(),
      isLoading: false,
      error: null,

      setAgents: (agents) => set({ agents }),
      updateAgent: (agentId, updates) => set((state) => ({
        agents: state.agents.map((a) =>
          a.id === agentId ? { ...a, ...updates } : a
        ),
      })),

      setTasks: (tasks) => set((state) => {
        const newTasks = new Map(state.tasks);
        tasks.forEach((t) => newTasks.set(t.id, t));
        return { tasks: newTasks };
      }),
      addTask: (task) => set((state) => {
        const newTasks = new Map(state.tasks);
        newTasks.set(task.id, task);
        return { tasks: newTasks };
      }),
      updateTask: (taskId, updates) => set((state) => {
        const newTasks = new Map(state.tasks);
        const existing = newTasks.get(taskId);
        if (existing) {
          newTasks.set(taskId, { ...existing, ...updates });
        }
        return { tasks: newTasks };
      }),
      removeTask: (taskId) => set((state) => {
        const newTasks = new Map(state.tasks);
        newTasks.delete(taskId);
        return {
          tasks: newTasks,
          activeTaskId: state.activeTaskId === taskId ? null : state.activeTaskId,
        };
      }),
      setActiveTask: (taskId) => set({ activeTaskId: taskId }),

      addTaskToQueue: (taskId, priority = 'medium') => set((state) => {
        const newQueue = [...state.taskQueue];
        const task = state.tasks.get(taskId);
        if (task) {
          const insertIndex = priority === 'high'
            ? 0
            : priority === 'low'
              ? newQueue.length
              : Math.floor(newQueue.length / 2);
          newQueue.splice(insertIndex, 0, taskId);
        }
        return { taskQueue: newQueue };
      }),
      removeTaskFromQueue: (taskId) => set((state) => ({
        taskQueue: state.taskQueue.filter((id) => id !== taskId),
      })),
      reorderQueue: (taskIds) => set({ taskQueue: taskIds }),

      addAgentLog: (agentId, log) => set((state) => {
        const newLogs = new Map(state.agentLogs);
        const existing = newLogs.get(agentId) || [];
        const timestamp = new Date().toISOString();
        newLogs.set(agentId, [...existing, `[${timestamp}] ${log}`]);
        return { agentLogs: newLogs };
      }),
      clearAgentLogs: (agentId) => set((state) => {
        const newLogs = new Map(state.agentLogs);
        newLogs.set(agentId, []);
        return { agentLogs: newLogs };
      }),

      registerA2AEndpoint: (agentId, endpoint) => set((state) => {
        const newEndpoints = new Map(state.a2aEndpoints);
        newEndpoints.set(agentId, endpoint);
        return { a2aEndpoints: newEndpoints };
      }),
      unregisterA2AEndpoint: (agentId) => set((state) => {
        const newEndpoints = new Map(state.a2aEndpoints);
        newEndpoints.delete(agentId);
        return { a2aEndpoints: newEndpoints };
      }),

      setIsAgentRunning: (running) => set({ isAgentRunning: running }),
      setIsLoading: (loading) => set({ isLoading: loading }),
      setError: (error) => set({ error }),

      getActiveTask: () => {
        const state = get();
        return state.activeTaskId
          ? state.tasks.get(state.activeTaskId) || null
          : null;
      },
      getQueuedTasks: () => {
        const state = get();
        return state.taskQueue
          .map((id) => state.tasks.get(id))
          .filter((t): t is Task => t !== undefined);
      },
      getTasksByStatus: (status) => {
        const state = get();
        return Array.from(state.tasks.values()).filter((t) => t.status === status);
      },
    }),
    { name: 'AgentStore' }
  )
);
