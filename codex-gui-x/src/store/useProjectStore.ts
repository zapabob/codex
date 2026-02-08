import { create } from 'zustand';
import { devtools, persist } from 'zustand/middleware';
import type { Project, WorktreeInfo, ProjectSettings } from '../types/mcp';

interface ProjectState {
  projects: Project[];
  activeProjectId: string | null;
  worktrees: Map<string, WorktreeInfo[]>;
  projectSettings: Map<string, ProjectSettings>;
  recentProjects: string[];
  favorites: string[];
  isLoading: boolean;
  error: string | null;

  setProjects: (projects: Project[]) => void;
  addProject: (project: Project) => void;
  updateProject: (projectId: string, updates: Partial<Project>) => void;
  deleteProject: (projectId: string) => void;
  setActiveProject: (projectId: string | null) => void;

  setWorktrees: (projectId: string, worktrees: WorktreeInfo[]) => void;
  addWorktree: (projectId: string, worktree: WorktreeInfo) => void;
  updateWorktree: (projectId: string, path: string, updates: Partial<WorktreeInfo>) => void;
  deleteWorktree: (projectId: string, path: string) => void;

  setProjectSettings: (projectId: string, settings: ProjectSettings) => void;
  updateProjectSettings: (projectId: string, settings: Partial<ProjectSettings>) => void;

  addToFavorites: (projectId: string) => void;
  removeFromFavorites: (projectId: string) => void;
  addToRecents: (projectId: string) => void;

  setIsLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;

  getActiveProject: () => Project | null;
  getActiveWorktrees: () => WorktreeInfo[];
  getFavoriteProjects: () => Project[];
  getRecentProjects: () => Project[];
}

export const useProjectStore = create<ProjectState>()(
  devtools(
    persist(
      (set, get) => ({
        projects: [],
        activeProjectId: null,
        worktrees: new Map(),
        projectSettings: new Map(),
        recentProjects: [],
        favorites: [],
        isLoading: false,
        error: null,

        setProjects: (projects) => set({ projects }),
        addProject: (project) => set((state) => ({
          projects: [...state.projects, project],
        })),
        updateProject: (projectId, updates) => set((state) => ({
          projects: state.projects.map((p) =>
            p.id === projectId ? { ...p, ...updates } : p
          ),
        })),
        deleteProject: (projectId) => set((state) => ({
          projects: state.projects.filter((p) => p.id !== projectId),
          activeProjectId: state.activeProjectId === projectId ? null : state.activeProjectId,
          recentProjects: state.recentProjects.filter((id) => id !== projectId),
          favorites: state.favorites.filter((id) => id !== projectId),
        })),
        setActiveProject: (projectId) => set({ activeProjectId: projectId }),

        setWorktrees: (projectId, worktrees) => set((state) => {
          const newWorktrees = new Map(state.worktrees);
          newWorktrees.set(projectId, worktrees);
          return { worktrees: newWorktrees };
        }),
        addWorktree: (projectId, worktree) => set((state) => {
          const newWorktrees = new Map(state.worktrees);
          const existing = newWorktrees.get(projectId) || [];
          newWorktrees.set(projectId, [...existing, worktree]);
          return { worktrees: newWorktrees };
        }),
        updateWorktree: (projectId, path, updates) => set((state) => {
          const newWorktrees = new Map(state.worktrees);
          const existing = newWorktrees.get(projectId) || [];
          newWorktrees.set(projectId, existing.map((w) =>
            w.path === path ? { ...w, ...updates } : w
          ));
          return { worktrees: newWorktrees };
        }),
        deleteWorktree: (projectId, path) => set((state) => {
          const newWorktrees = new Map(state.worktrees);
          const existing = newWorktrees.get(projectId) || [];
          newWorktrees.set(projectId, existing.filter((w) => w.path !== path));
          return { worktrees: newWorktrees };
        }),

        setProjectSettings: (projectId, settings) => set((state) => {
          const newSettings = new Map(state.projectSettings);
          newSettings.set(projectId, settings);
          return { projectSettings: newSettings };
        }),
        updateProjectSettings: (projectId, settings) => set((state) => {
          const newSettings = new Map(state.projectSettings);
          const existing = newSettings.get(projectId) || {};
          newSettings.set(projectId, { ...existing, ...settings });
          return { projectSettings: newSettings };
        }),

        addToFavorites: (projectId) => set((state) => ({
          favorites: state.favorites.includes(projectId)
            ? state.favorites
            : [...state.favorites, projectId],
        })),
        removeFromFavorites: (projectId) => set((state) => ({
          favorites: state.favorites.filter((id) => id !== projectId),
        })),
        addToRecents: (projectId) => set((state) => ({
          recentProjects: [
            projectId,
            ...state.recentProjects.filter((id) => id !== projectId),
          ].slice(0, 20),
        })),

        setIsLoading: (loading) => set({ isLoading: loading }),
        setError: (error) => set({ error }),

        getActiveProject: () => {
          const state = get();
          return state.activeProjectId
            ? state.projects.find((p) => p.id === state.activeProjectId) || null
            : null;
        },
        getActiveWorktrees: () => {
          const state = get();
          return state.activeProjectId
            ? state.worktrees.get(state.activeProjectId) || []
            : [];
        },
        getFavoriteProjects: () => {
          const state = get();
          return state.projects.filter((p) => state.favorites.includes(p.id));
        },
        getRecentProjects: () => {
          const state = get();
          return state.recentProjects
            .map((id) => state.projects.find((p) => p.id === id))
            .filter((p): p is Project => p !== undefined);
        },
      }),
      {
        name: 'codex-project-store',
        partialize: (state) => ({
          projects: state.projects.slice(-100),
          activeProjectId: state.activeProjectId,
          recentProjects: state.recentProjects.slice(0, 20),
          favorites: state.favorites,
        }),
      }
    ),
    { name: 'ProjectStore' }
  )
);
