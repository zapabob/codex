export interface Notification {
  id: string;
  type: "info" | "success" | "warning" | "error" | "approval" | "mention";
  title: string;
  message: string;
  timestamp: Date;
  read: boolean;
  source?: string;
  action?: NotificationAction;
  dismissible: boolean;
}

export interface NotificationAction {
  label: string;
  onClick: () => void;
  primary?: boolean;
}

export interface NotificationSettings {
  turnComplete: boolean;
  permissionRequests: boolean;
  buildComplete: boolean;
  mention: boolean;
  automationResults: boolean;
  inboxDigest: "realtime" | "hourly" | "daily";
  quietHours?: {
    enabled: boolean;
    start: number; // 0-23
    end: number; // 0-23
  };
}

export interface PermissionRequest {
  id: string;
  type: "network" | "execution" | "filesystem";
  command?: string;
  resource?: string;
  reason: string;
  timestamp: Date;
  status: "pending" | "approved" | "denied";
  decision?: boolean;
  decidedBy?: "user" | "rule";
}
