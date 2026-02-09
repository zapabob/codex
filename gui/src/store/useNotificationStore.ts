import { create } from "zustand";
import { devtools, persist } from "zustand/middleware";
import type {
  Notification,
  NotificationSettings,
  PermissionRequest,
} from "../types/notification";

interface NotificationState {
  notifications: Notification[];
  permissionRequests: PermissionRequest[];
  settings: NotificationSettings;
  unreadCount: number;

  // Actions
  addNotification: (
    notification: Omit<Notification, "id" | "timestamp" | "read">,
  ) => void;
  markAsRead: (id: string) => void;
  markAllAsRead: () => void;
  dismissNotification: (id: string) => void;
  clearAll: () => void;
  addPermissionRequest: (
    request: Omit<PermissionRequest, "id" | "timestamp" | "status">,
  ) => void;
  respondToPermission: (id: string, approved: boolean) => void;
  updateSettings: (settings: Partial<NotificationSettings>) => void;
}

const defaultSettings: NotificationSettings = {
  turnComplete: true,
  permissionRequests: true,
  buildComplete: true,
  mention: true,
  automationResults: true,
  inboxDigest: "realtime",
};

export const useNotificationStore = create<NotificationState>()(
  devtools(
    persist(
      (set, get) => ({
        notifications: [],
        permissionRequests: [],
        settings: defaultSettings,
        unreadCount: 0,

        addNotification: (notification) => {
          const newNotification: Notification = {
            ...notification,
            id: `notif-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
            timestamp: new Date(),
            read: false,
          };

          set((state) => {
            const newNotifications = [
              newNotification,
              ...state.notifications,
            ].slice(0, 100);
            return {
              notifications: newNotifications,
              unreadCount: state.unreadCount + 1,
            };
          });
        },

        markAsRead: (id: string) => {
          set((state) => {
            const notifications = state.notifications.map((n) =>
              n.id === id ? { ...n, read: true } : n,
            );
            const unreadCount = notifications.filter((n) => !n.read).length;
            return { notifications, unreadCount };
          });
        },

        markAllAsRead: () => {
          set((state) => ({
            notifications: state.notifications.map((n) => ({
              ...n,
              read: true,
            })),
            unreadCount: 0,
          }));
        },

        dismissNotification: (id: string) => {
          set((state) => {
            const notifications = state.notifications.filter(
              (n) => n.id !== id,
            );
            const unreadCount = notifications.filter((n) => !n.read).length;
            return { notifications, unreadCount };
          });
        },

        clearAll: () => {
          set({ notifications: [], unreadCount: 0 });
        },

        addPermissionRequest: (request) => {
          const newRequest: PermissionRequest = {
            ...request,
            id: `perm-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
            timestamp: new Date(),
            status: "pending",
          };

          set((state) => ({
            permissionRequests: [...state.permissionRequests, newRequest],
          }));
        },

        respondToPermission: (id: string, approved: boolean) => {
          set((state) => ({
            permissionRequests: state.permissionRequests.map((r) =>
              r.id === id
                ? {
                    ...r,
                    status: approved ? "approved" : "denied",
                    decision: approved,
                    decidedBy: "user",
                  }
                : r,
            ),
          }));
        },

        updateSettings: (settings: Partial<NotificationSettings>) => {
          set((state) => ({
            settings: { ...state.settings, ...settings },
          }));
        },
      }),
      {
        name: "codex-notification-store",
        partialize: (state) => ({
          notifications: state.notifications.slice(0, 50),
          settings: state.settings,
        }),
      },
    ),
    { name: "NotificationStore" },
  ),
);
