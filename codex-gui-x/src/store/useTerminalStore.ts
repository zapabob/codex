import { create } from 'zustand';
import { devtools, persist } from 'zustand/middleware';
import type { TerminalSession, TerminalHistoryEntry } from '../types/mcp';

interface TerminalState {
  sessions: Map<string, TerminalSession>;
  activeSessionId: string | null;
  history: TerminalHistoryEntry[];
  maxHistoryLength: number;
  isAnyTerminalOpen: boolean;
  terminalTheme: 'dark' | 'light';
  fontSize: number;
  fontFamily: string;

  createSession: (session: TerminalSession) => void;
  updateSession: (sessionId: string, updates: Partial<TerminalSession>) => void;
  closeSession: (sessionId: string) => void;
  setActiveSession: (sessionId: string | null) => void;

  addToHistory: (entry: TerminalHistoryEntry) => void;
  clearHistory: () => void;
  pruneHistory: () => void;

  setTerminalTheme: (theme: 'dark' | 'light') => void;
  setFontSize: (size: number) => void;
  setFontFamily: (family: string) => void;

  getActiveSession: () => TerminalSession | null;
  getAllSessions: () => TerminalSession[];
}

export const useTerminalStore = create<TerminalState>()(
  devtools(
    persist(
      (set, get) => ({
        sessions: new Map(),
        activeSessionId: null,
        history: [],
        maxHistoryLength: 1000,
        isAnyTerminalOpen: false,
        terminalTheme: 'dark',
        fontSize: 14,
        fontFamily: "'JetBrains Mono', 'Fira Code', monospace",

        createSession: (session) => set((state) => {
          const newSessions = new Map(state.sessions);
          newSessions.set(session.id, session);
          return {
            sessions: newSessions,
            activeSessionId: session.id,
            isAnyTerminalOpen: true,
          };
        }),
        updateSession: (sessionId, updates) => set((state) => {
          const newSessions = new Map(state.sessions);
          const existing = newSessions.get(sessionId);
          if (existing) {
            newSessions.set(sessionId, { ...existing, ...updates });
          }
          return { sessions: newSessions };
        }),
        closeSession: (sessionId) => set((state) => {
          const newSessions = new Map(state.sessions);
          newSessions.delete(sessionId);
          return {
            sessions: newSessions,
            activeSessionId: state.activeSessionId === sessionId
              ? null
              : state.activeSessionId,
            isAnyTerminalOpen: newSessions.size > 0,
          };
        }),
        setActiveSession: (sessionId) => set({ activeSessionId: sessionId }),

        addToHistory: (entry) => set((state) => {
          const newHistory = [...state.history, entry];
          if (newHistory.length > state.maxHistoryLength) {
            newHistory.shift();
          }
          return { history: newHistory };
        }),
        clearHistory: () => set({ history: [] }),
        pruneHistory: () => set((state) => ({
          history: state.history.slice(-state.maxHistoryLength),
        })),

        setTerminalTheme: (theme) => set({ terminalTheme: theme }),
        setFontSize: (size) => set({ fontSize: Math.max(8, Math.min(24, size)) }),
        setFontFamily: (family) => set({ fontFamily: family }),

        getActiveSession: () => {
          const state = get();
          return state.activeSessionId
            ? state.sessions.get(state.activeSessionId) || null
            : null;
        },
        getAllSessions: () => {
          return Array.from(get().sessions.values());
        },
      }),
      {
        name: 'codex-terminal-store',
        partialize: (state) => ({
          history: state.history.slice(-100),
          maxHistoryLength: state.maxHistoryLength,
          terminalTheme: state.terminalTheme,
          fontSize: state.fontSize,
          fontFamily: state.fontFamily,
        }),
      }
    ),
    { name: 'TerminalStore' }
  )
);
