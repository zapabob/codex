import * as React from "react";
import { useTranslation } from "react-i18next";
import { useNavigate, useLocation } from "react-router-dom";
import {
  Drawer,
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Divider,
  Box,
  Typography,
  Tooltip,
  IconButton,
  styled,
} from "@mui/material";
import { motion, AnimatePresence } from "framer-motion";
import {
  Home,
  Code,
  Settings,
  Users,
  Shield,
  Search,
  Server,
  CheckSquare,
  TrendingUp,
  Monitor,
  Bot,
  ChevronLeft,
  ChevronRight,
} from "lucide-react";
import type { LucideIcon } from "lucide-react";

interface NavigationItem {
  id: string;
  label: string;
  path: string;
  icon: LucideIcon;
  shortcut?: string;
}

interface SidebarProps {
  open: boolean;
  collapsed?: boolean;
  onClose: () => void;
  onToggleCollapse?: () => void;
}

const StyledDrawer = styled(Drawer)(({ theme }) => ({
  "& .MuiDrawer-paper": {
    background:
      theme.palette.mode === "dark"
        ? "linear-gradient(180deg, rgba(15, 20, 25, 0.95) 0%, rgba(29, 27, 32, 0.95) 100%)"
        : "linear-gradient(180deg, rgba(253, 251, 255, 0.95) 0%, rgba(231, 224, 236, 0.95) 100%)",
    backdropFilter: "blur(20px)",
    borderRight: `1px solid ${
      theme.palette.mode === "dark"
        ? "rgba(255, 255, 255, 0.1)"
        : "rgba(0, 0, 0, 0.05)"
    }`,
  },
}));

const NavButton = styled(ListItemButton)<{ active?: boolean }>(
  ({ theme, active }) => ({
    "borderRadius": "12px",
    "margin": "4px 8px",
    "padding": "10px 16px",
    "position": "relative",
    "backgroundColor": active
      ? `${theme.palette.primary.main}20`
      : "transparent",
    "color": active ? theme.palette.primary.main : theme.palette.text.primary,
    "transition": "all 0.2s cubic-bezier(0.4, 0, 0.2, 1)",
    "&:hover": {
      backgroundColor: active
        ? `${theme.palette.primary.main}30`
        : theme.palette.action.hover,
    },
    "&::before": {
      content: '""',
      position: "absolute",
      left: 0,
      top: "50%",
      transform: "translateY(-50%)",
      width: active ? 3 : 0,
      height: "50%",
      backgroundColor: theme.palette.primary.main,
      borderRadius: "0 4px 4px 0",
      transition: "width 0.2s ease",
    },
  }),
);

export const Sidebar: React.FC<SidebarProps> = ({
  open,
  collapsed = false,
  onClose,
  onToggleCollapse,
}) => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const location = useLocation();
  const currentPath = location.pathname;

  const navigationItems: NavigationItem[] = [
    { id: "dashboard", label: t("nav.dashboard"), path: "/", icon: Home },
    { id: "agents", label: t("nav.agents"), path: "/agents", icon: Users },
    { id: "code", label: t("nav.code"), path: "/code", icon: Code },
    { id: "tasks", label: t("nav.tasks"), path: "/tasks", icon: CheckSquare },
    { id: "qc", label: t("nav.qc"), path: "/qc", icon: TrendingUp },
    {
      id: "security",
      label: t("nav.security"),
      path: "/security",
      icon: Shield,
    },
    {
      id: "virtual-os",
      label: t("nav.virtualOs"),
      path: "/virtual-os",
      icon: Monitor,
    },
    { id: "ai-tools", label: t("nav.aiTools"), path: "/ai-tools", icon: Bot },
    {
      id: "research",
      label: t("nav.research"),
      path: "/research",
      icon: Search,
    },
    { id: "mcp", label: t("nav.mcp"), path: "/mcp", icon: Server },
  ];

  const settingsItems: NavigationItem[] = [
    {
      id: "settings",
      label: t("nav.settings"),
      path: "/settings",
      icon: Settings,
    },
  ];

  const handleNavigate = (path: string) => {
    navigate(path);
    if (window.innerWidth < 768) {
      onClose();
    }
  };

  const isActive = (path: string) => {
    if (path === "/") {
      return currentPath === "/";
    }
    return currentPath.startsWith(path);
  };

  const drawerWidth = collapsed ? 72 : 280;

  const sidebarContent = (
    <Box
      sx={{
        width: drawerWidth,
        height: "100%",
        display: "flex",
        flexDirection: "column",
        transition: "width 0.3s cubic-bezier(0.4, 0, 0.2, 1)",
        overflowX: "hidden",
      }}
    >
      {/* Header */}
      <Box
        sx={{
          p: collapsed ? 2 : 3,
          borderBottom: "1px solid",
          borderColor: "divider",
          display: "flex",
          alignItems: "center",
          justifyContent: collapsed ? "center" : "space-between",
          height: 64,
        }}
      >
        <AnimatePresence mode="wait">
          {!collapsed && (
            <motion.div
              initial={{ opacity: 0, x: -10 }}
              animate={{ opacity: 1, x: 0 }}
              exit={{ opacity: 0, x: -10 }}
              transition={{ duration: 0.2 }}
            >
              <Box>
                <Typography
                  variant="h6"
                  sx={{
                    fontWeight: 700,
                    background: "linear-gradient(45deg, #0061a4, #565f71)",
                    backgroundClip: "text",
                    WebkitBackgroundClip: "text",
                    WebkitTextFillColor: "transparent",
                    lineHeight: 1.2,
                    whiteSpace: "nowrap",
                  }}
                >
                  {t("app.title")}
                </Typography>
                <Typography
                  variant="caption"
                  sx={{ color: "text.secondary", whiteSpace: "nowrap" }}
                >
                  {t("app.subtitle")}
                </Typography>
              </Box>
            </motion.div>
          )}
        </AnimatePresence>

        {onToggleCollapse && !collapsed && (
          <IconButton onClick={onToggleCollapse} size="small">
            <ChevronLeft size={20} />
          </IconButton>
        )}
      </Box>

      {/* Navigation */}
      <Box sx={{ flex: 1, overflow: "auto", py: 2 }}>
        <List sx={{ px: 1 }}>
          {navigationItems.map((item) => {
            const Icon = item.icon;
            const active = isActive(item.path);

            return (
              <Tooltip
                key={item.id}
                title={collapsed ? item.label : ""}
                placement="right"
                arrow
              >
                <ListItem disablePadding>
                  <NavButton
                    active={active}
                    onClick={() => handleNavigate(item.path)}
                    data-testid={`nav-${item.id}`}
                    sx={{
                      justifyContent: collapsed ? "center" : "flex-start",
                      px: collapsed ? 1.5 : 2,
                    }}
                  >
                    <ListItemIcon
                      sx={{
                        minWidth: collapsed ? 0 : 40,
                        color: "inherit",
                        justifyContent: "center",
                        mr: collapsed ? 0 : 1,
                      }}
                    >
                      <Icon size={20} />
                    </ListItemIcon>

                    {!collapsed && (
                      <ListItemText
                        primary={item.label}
                        primaryTypographyProps={{
                          sx: {
                            fontSize: "0.875rem",
                            fontWeight: active ? 600 : 500,
                            whiteSpace: "nowrap",
                          },
                        }}
                      />
                    )}
                  </NavButton>
                </ListItem>
              </Tooltip>
            );
          })}
        </List>

        <Divider sx={{ my: 2, mx: 2 }} />

        <List sx={{ px: 1 }}>
          {settingsItems.map((item) => {
            const Icon = item.icon;
            const active = isActive(item.path);

            return (
              <Tooltip
                key={item.id}
                title={collapsed ? item.label : ""}
                placement="right"
                arrow
              >
                <ListItem disablePadding>
                  <NavButton
                    active={active}
                    onClick={() => handleNavigate(item.path)}
                    sx={{
                      justifyContent: collapsed ? "center" : "flex-start",
                      px: collapsed ? 1.5 : 2,
                    }}
                  >
                    <ListItemIcon
                      sx={{
                        minWidth: collapsed ? 0 : 40,
                        color: "inherit",
                        justifyContent: "center",
                        mr: collapsed ? 0 : 1,
                      }}
                    >
                      <Icon size={20} />
                    </ListItemIcon>

                    {!collapsed && (
                      <ListItemText
                        primary={item.label}
                        primaryTypographyProps={{
                          sx: {
                            fontSize: "0.875rem",
                            fontWeight: active ? 600 : 500,
                            whiteSpace: "nowrap",
                          },
                        }}
                      />
                    )}
                  </NavButton>
                </ListItem>
              </Tooltip>
            );
          })}
        </List>

        {/* Toggle Button at bottom if collapsed */}
        {collapsed && onToggleCollapse && (
          <Box
            sx={{ mt: "auto", p: 2, display: "flex", justifyContent: "center" }}
          >
            <IconButton onClick={onToggleCollapse}>
              <ChevronRight size={20} />
            </IconButton>
          </Box>
        )}
      </Box>
    </Box>
  );

  return (
    <>
      {/* Mobile Drawer */}
      <StyledDrawer
        variant="temporary"
        open={open}
        onClose={onClose}
        ModalProps={{ keepMounted: true }}
        sx={{
          display: { xs: "block", md: "none" },
        }}
      >
        {sidebarContent}
      </StyledDrawer>

      {/* Desktop Drawer */}
      <StyledDrawer
        variant="permanent"
        open
        sx={{
          "display": { xs: "none", md: "block" },
          "width": drawerWidth,
          "flexShrink": 0,
          "& .MuiDrawer-paper": {
            width: drawerWidth,
            boxSizing: "border-box",
          },
        }}
      >
        {sidebarContent}
      </StyledDrawer>
    </>
  );
};

export default Sidebar;
