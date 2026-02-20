import React from "react";
import {
  Box,
  Typography,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  IconButton,
  
  Chip,
  Divider,
  Button,
} from "@mui/material";
import {
  Notifications as NotificationsIcon,
  Info as InfoIcon,
  CheckCircle as SuccessIcon,
  Warning as WarningIcon,
  Error as ErrorIcon,
  Security as SecurityIcon,
  DoneAll as ReadIcon,
  Delete as DeleteIcon,
} from "@mui/icons-material";
import { useNotificationStore } from "../../store/useNotificationStore";

interface Notification {
  id: string;
  type: "info" | "success" | "warning" | "error" | "approval";
  title: string;
  message: string;
  timestamp: Date;
  read: boolean;
}

const mockNotifications: Notification[] = [
  {
    id: "1",
    type: "success",
    title: "Build Completed",
    message: "pnpm build completed successfully",
    timestamp: new Date(Date.now() - 1000 * 60 * 5),
    read: false,
  },
  {
    id: "2",
    type: "approval",
    title: "Permission Required",
    message: "git push to main branch requires approval",
    timestamp: new Date(Date.now() - 1000 * 60 * 15),
    read: false,
  },
  {
    id: "3",
    type: "info",
    title: "Tests Passed",
    message: "All 42 tests passed in 12.3s",
    timestamp: new Date(Date.now() - 1000 * 60 * 30),
    read: true,
  },
  {
    id: "4",
    type: "warning",
    title: "Deprecation Warning",
    message: "pnpm install is using a deprecated flag",
    timestamp: new Date(Date.now() - 1000 * 60 * 60),
    read: true,
  },
  {
    id: "5",
    type: "security",
    title: "Security Update Available",
    message: "Update dependencies to fix vulnerabilities",
    timestamp: new Date(Date.now() - 1000 * 60 * 60 * 2),
    read: false,
  },
];

export const NotificationsPanel: React.FC = () => {
  const { notifications, markAsRead, markAllAsRead, dismissNotification } =
    useNotificationStore();
  const allNotifications = mockNotifications;

  const unreadCount = allNotifications.filter((n) => !n.read).length;

  const getIcon = (type: string) => {
    switch (type) {
      case "success":
        return <SuccessIcon sx={{ color: "success.main" }} />;
      case "warning":
        return <WarningIcon sx={{ color: "warning.main" }} />;
      case "error":
        return <ErrorIcon sx={{ color: "error.main" }} />;
      case "approval":
        return <SecurityIcon sx={{ color: "primary.main" }} />;
      default:
        return <InfoIcon sx={{ color: "info.main" }} />;
    }
  };

  const formatTime = (date: Date) => {
    const diff = Date.now() - date.getTime();
    const minutes = Math.floor(diff / 60000);
    const hours = Math.floor(diff / 3600000);
    const days = Math.floor(diff / 86400000);

    if (minutes < 1) return "Just now";
    if (minutes < 60) return `${minutes}m ago`;
    if (hours < 24) return `${hours}h ago`;
    return `${days}d ago`;
  };

  return (
    <Box sx={{ height: "100%", display: "flex", flexDirection: "column" }}>
      <Box
        sx={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          mb: 2,
        }}
      >
        <Box sx={{ display: "flex", alignItems: "center", gap: 1 }}>
          <Typography variant="h6" fontWeight={600}>
            Notifications
          </Typography>
          {unreadCount > 0 && (
            <Chip
              label={unreadCount}
              size="small"
              color="error"
              sx={{ height: 20, fontSize: "0.75rem" }}
            />
          )}
        </Box>
        <Button
          size="small"
          startIcon={<ReadIcon />}
          onClick={markAllAsRead}
          disabled={unreadCount === 0}
        >
          Mark all read
        </Button>
      </Box>

      <Box sx={{ flex: 1, overflow: "auto" }}>
        <List sx={{ p: 0 }}>
          {allNotifications.map((notification, index) => (
            <React.Fragment key={notification.id}>
              <ListItem
                sx={{
                  "py": 1.5,
                  "bgcolor": notification.read
                    ? "transparent"
                    : "action.selected",
                  "&:hover": {
                    bgcolor: "action.hover",
                  },
                  "borderRadius": 1,
                  "mb": 0.5,
                }}
                secondaryAction={
                  !notification.read && (
                    <IconButton
                      edge="end"
                      size="small"
                      onClick={() => markAsRead(notification.id)}
                    >
                      <ReadIcon fontSize="small" />
                    </IconButton>
                  )
                }
              >
                <ListItemIcon sx={{ minWidth: 40 }}>
                  {getIcon(notification.type)}
                </ListItemIcon>
                <ListItemText
                  primary={
                    <Box sx={{ display: "flex", alignItems: "center", gap: 1 }}>
                      <Typography
                        variant="subtitle2"
                        fontWeight={notification.read ? 400 : 600}
                      >
                        {notification.title}
                      </Typography>
                    </Box>
                  }
                  secondary={
                    <>
                      <Typography
                        variant="body2"
                        color="text.secondary"
                        sx={{ mt: 0.5 }}
                      >
                        {notification.message}
                      </Typography>
                      <Typography
                        variant="caption"
                        color="text.disabled"
                        sx={{ mt: 0.5, display: "block" }}
                      >
                        {formatTime(notification.timestamp)}
                      </Typography>
                    </>
                  }
                />
              </ListItem>
              {index < allNotifications.length - 1 && (
                <Divider sx={{ my: 0.5 }} />
              )}
            </React.Fragment>
          ))}
        </List>

        {allNotifications.length === 0 && (
          <Box
            sx={{
              display: "flex",
              flexDirection: "column",
              alignItems: "center",
              justifyContent: "center",
              py: 4,
            }}
          >
            <NotificationsIcon
              sx={{ fontSize: 48, color: "text.disabled", mb: 2 }}
            />
            <Typography variant="body2" color="text.secondary">
              No notifications
            </Typography>
          </Box>
        )}
      </Box>
    </Box>
  );
};

export default NotificationsPanel;
