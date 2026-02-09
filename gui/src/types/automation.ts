export interface Automation {
  id: string;
  name: string;
  description: string;
  schedule: string; // cron expression
  enabled: boolean;
  worktreeTemplate: string;
  steps: AutomationStep[];
  notifications: NotificationConfig[];
  createdAt: Date;
  lastRun?: Date;
  lastResult?: AutomationResult;
}

export interface AutomationStep {
  name: string;
  command: string;
  workingDirectory?: string;
  env?: Record<string, string>;
  continueOnError: boolean;
}

export interface AutomationResult {
  success: boolean;
  startTime: Date;
  endTime: Date;
  outputs: string[];
  errors: string[];
}

export interface NotificationConfig {
  type: "completion" | "failure" | "all";
  to: "inbox" | "email" | "slack";
  message?: string;
}

export interface InboxItem {
  id: string;
  automationId: string;
  automationName: string;
  type: "success" | "failure" | "warning" | "info";
  title: string;
  message: string;
  timestamp: Date;
  read: boolean;
  actionUrl?: string;
}
