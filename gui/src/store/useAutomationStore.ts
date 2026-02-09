import { create } from "zustand";
import { devtools, persist } from "zustand/middleware";
import type { Automation, InboxItem } from "../types/automation";

interface AutomationState {
  automations: Automation[];
  inbox: InboxItem[];

  // Actions
  addAutomation: (automation: Omit<Automation, "id" | "createdAt">) => void;
  updateAutomation: (id: string, updates: Partial<Automation>) => void;
  deleteAutomation: (id: string) => void;
  toggleAutomation: (id: string) => void;
  addToInbox: (item: Omit<InboxItem, "id" | "timestamp" | "read">) => void;
  markInboxAsRead: (id: string) => void;
  markAllInboxAsRead: () => void;
  clearInbox: () => void;
  getAutomation: (id: string) => Automation | undefined;
}

export const useAutomationStore = create<AutomationState>()(
  devtools(
    persist(
      (set, get) => ({
        automations: [],
        inbox: [],

        addAutomation: (automation) => {
          const newAutomation: Automation = {
            ...automation,
            id: `auto-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
            createdAt: new Date(),
          };

          set((state) => ({
            automations: [...state.automations, newAutomation],
          }));
        },

        updateAutomation: (id: string, updates: Partial<Automation>) => {
          set((state) => ({
            automations: state.automations.map((a) =>
              a.id === id ? { ...a, ...updates } : a,
            ),
          }));
        },

        deleteAutomation: (id: string) => {
          set((state) => ({
            automations: state.automations.filter((a) => a.id !== id),
          }));
        },

        toggleAutomation: (id: string) => {
          set((state) => ({
            automations: state.automations.map((a) =>
              a.id === id ? { ...a, enabled: !a.enabled } : a,
            ),
          }));
        },

        addToInbox: (item) => {
          const newItem: InboxItem = {
            ...item,
            id: `inbox-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
            timestamp: new Date(),
            read: false,
          };

          set((state) => ({
            inbox: [newItem, ...state.inbox].slice(0, 200),
          }));
        },

        markInboxAsRead: (id: string) => {
          set((state) => ({
            inbox: state.inbox.map((i) =>
              i.id === id ? { ...i, read: true } : i,
            ),
          }));
        },

        markAllInboxAsRead: () => {
          set((state) => ({
            inbox: state.inbox.map((i) => ({ ...i, read: true })),
          }));
        },

        clearInbox: () => {
          set({ inbox: [] });
        },

        getAutomation: (id: string) => {
          return get().automations.find((a) => a.id === id);
        },
      }),
      {
        name: "codex-automation-store",
        partialize: (state) => ({
          automations: state.automations,
          inbox: state.inbox.slice(0, 100),
        }),
      },
    ),
    { name: "AutomationStore" },
  ),
);
