import React, { useState, ReactNode } from "react";
import {
  Box,
  Tooltip,
  IconButton,
  Badge,
  Divider,
  Typography,
} from "@mui/material";
import {
  Folder as FolderIcon,
  Chat as ChatIcon,
  Play as PlayIcon,
  Shield as ShieldIcon,
  Notifications as NotificationsIcon,
  Settings as SettingsIcon,
  ChevronLeft,
  ChevronRight,
} from "@mui/icons-material";

interface SidebarTab {
  id: string;
  icon: ReactNode;
  label: string;
  panelId?: string;
  badge?: number;
  badgeColor?:
    | "default"
    | "primary"
    | "secondary"
    | "error"
    | "info"
    | "success"
    | "warning";
  onClick?: () => void;
}

interface SidebarTabsProps {
  tabs: SidebarTab[];
  activeTab: string;
  onTabChange: (tabId: string) => void;
  collapsed?: boolean;
  onToggleCollapse?: () => void;
  showBadgeCounts?: boolean;
}

export const SidebarTabs: React.FC<SidebarTabsProps> = ({
  tabs,
  activeTab,
  onTabChange,
  collapsed = false,
  onToggleCollapse,
  showBadgeCounts = true,
}) => {
  const [hoveredTab, setHoveredTab] = useState<string | null>(null);

  const handleTabClick = (tab: SidebarTab) => {
    if (tab.onClick) {
      tab.onClick();
    } else {
      onTabChange(tab.id);
    }
  };

  if (collapsed) {
    return (
      <Box
        className="sidebar-tabs-collapsed"
        sx={{
          width: 48,
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          py: 1,
          borderRight: 1,
          borderColor: "divider",
          bgcolor: "background.paper",
        }}
      >
        {tabs.map((tab) => (
          <Tooltip key={tab.id} title={tab.label} placement="right">
            <IconButton
              size="small"
              onClick={() => handleTabClick(tab)}
              sx={{
                "mb": 0.5,
                "color":
                  activeTab === tab.id ? "primary.main" : "text.secondary",
                "bgcolor":
                  activeTab === tab.id ? "action.selected" : "transparent",
                "&:hover": {
                  bgcolor: "action.hover",
                },
              }}
            >
              {showBadgeCounts && tab.badge ? (
                <Badge
                  badgeContent={tab.badge}
                  color={tab.badgeColor || "error"}
                  max={99}
                >
                  {tab.icon}
                </Badge>
              ) : (
                tab.icon
              )}
            </IconButton>
          </Tooltip>
        ))}

        <Box sx={{ flex: 1 }} />

        {onToggleCollapse && (
          <Tooltip title="Expand sidebar" placement="right">
            <IconButton size="small" onClick={onToggleCollapse}>
              <ChevronRight fontSize="small" />
            </IconButton>
          </Tooltip>
        )}
      </Box>
    );
  }

  return (
    <Box
      className="sidebar-tabs"
      sx={{
        width: 220,
        display: "flex",
        flexDirection: "column",
        borderRight: 1,
        borderColor: "divider",
        bgcolor: "background.paper",
      }}
    >
      {/* Tab List */}
      <Box
        sx={{
          display: "flex",
          flexDirection: "column",
          py: 1,
        }}
      >
        {tabs.map((tab, index) => (
          <React.Fragment key={tab.id}>
            <Tooltip title={tab.label} placement="right">
              <Box
                onClick={() => handleTabClick(tab)}
                onMouseEnter={() => setHoveredTab(tab.id)}
                onMouseLeave={() => setHoveredTab(null)}
                sx={{
                  "display": "flex",
                  "alignItems": "center",
                  "gap": 1.5,
                  "mx": 1,
                  "px": 1.5,
                  "py": 1,
                  "borderRadius": 1,
                  "cursor": "pointer",
                  "bgcolor":
                    activeTab === tab.id
                      ? "action.selected"
                      : hoveredTab === tab.id
                        ? "action.hover"
                        : "transparent",
                  "color":
                    activeTab === tab.id ? "primary.main" : "text.primary",
                  "transition": "all 0.15s ease",
                  "&:hover": {
                    bgcolor: "action.hover",
                  },
                }}
              >
                {showBadgeCounts && tab.badge ? (
                  <Badge
                    badgeContent={tab.badge}
                    color={tab.badgeColor || "error"}
                    max={99}
                  >
                    <Box
                      sx={{
                        color:
                          activeTab === tab.id
                            ? "primary.main"
                            : "text.secondary",
                        display: "flex",
                        alignItems: "center",
                      }}
                    >
                      {tab.icon}
                    </Box>
                  </Badge>
                ) : (
                  <Box
                    sx={{
                      color:
                        activeTab === tab.id
                          ? "primary.main"
                          : "text.secondary",
                      display: "flex",
                      alignItems: "center",
                    }}
                  >
                    {tab.icon}
                  </Box>
                )}

                <Typography
                  variant="body2"
                  sx={{
                    fontWeight: activeTab === tab.id ? 600 : 400,
                    flex: 1,
                    whiteSpace: "nowrap",
                    overflow: "hidden",
                    textOverflow: "ellipsis",
                  }}
                >
                  {tab.label}
                </Typography>
              </Box>
            </Tooltip>
            {index === 4 && <Divider sx={{ my: 1 }} />}
          </React.Fragment>
        ))}
      </Box>

      {/* Collapse Button */}
      <Box sx={{ flex: 1 }} />

      {onToggleCollapse && (
        <Box sx={{ p: 1, borderTop: 1, borderColor: "divider" }}>
          <Tooltip title="Collapse sidebar" placement="right">
            <IconButton
              fullWidth
              size="small"
              onClick={onToggleCollapse}
              sx={{
                "justifyContent": "flex-start",
                "color": "text.secondary",
                "&:hover": {
                  bgcolor: "action.hover",
                },
              }}
            >
              <ChevronLeft fontSize="small" />
              <Typography variant="caption" sx={{ ml: 1 }}>
                Collapse
              </Typography>
            </IconButton>
          </Tooltip>
        </Box>
      )}
    </Box>
  );
};

// Default tabs for Codex
export const defaultSidebarTabs: SidebarTab[] = [
  {
    id: "projects",
    icon: <FolderIcon />,
    label: "Projects",
    panelId: "projects-panel",
  },
  {
    id: "threads",
    icon: <ChatIcon />,
    label: "Threads",
    panelId: "threads-panel",
    badge: 3,
  },
  {
    id: "actions",
    icon: <PlayIcon />,
    label: "Actions",
    panelId: "actions-panel",
  },
  {
    id: "security",
    icon: <ShieldIcon />,
    label: "Security",
    panelId: "security-panel",
    badge: 1,
  },
  {
    id: "notifications",
    icon: <NotificationsIcon />,
    label: "Notifications",
    panelId: "notifications-panel",
    badge: 5,
  },
  {
    id: "settings",
    icon: <SettingsIcon />,
    label: "Settings",
    panelId: "settings-panel",
  },
];

export default SidebarTabs;
