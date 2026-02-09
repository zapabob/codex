import React, { useState, useCallback, Suspense, lazy } from "react";
import { Box, CircularProgress } from "@mui/material";
import { ThemeProvider } from "@mui/material/styles";
import CssBaseline from "@mui/material/CssBaseline";
import { getTheme, type ThemeMode } from "@/theme/chatGPTTheme";
import { SidebarTabs, defaultSidebarTabs } from "@/components/ui/SidebarTabs";
import { ResizablePanel } from "@/components/ui/ResizablePanel";
import { ChatContainer } from "@/components/chat/ChatContainer";

// Lazy load dashboard components
const WorktreeDashboard = lazy(
  () => import("@/components/worktree/WorktreeDashboard"),
);
const SecurityDashboard = lazy(
  () => import("@/components/security/SecurityDashboard"),
);
const ActionsPanel = lazy(() => import("@/components/terminal/ActionsPanel"));
const NotificationsPanel = lazy(
  () => import("@/components/notifications/NotificationsPanel"),
);
const SettingsPanel = lazy(() => import("@/components/settings/SettingsPanel"));

interface LayoutProps {
  children?: React.ReactNode;
}

export const Layout: React.FC<LayoutProps> = () => {
  const [themeMode, setThemeMode] = useState<ThemeMode>("dark");
  const [sidebarCollapsed, setSidebarCollapsed] = useState(false);
  const [activeTab, setActiveTab] = useState("threads");
  const [panels, setPanels] = useState({
    threads: { width: 400, visible: true },
    projects: { width: 350, visible: true },
    actions: { width: 350, visible: true },
    security: { width: 450, visible: true },
    notifications: { width: 350, visible: true },
    settings: { width: 350, visible: false },
  });

  const theme = getTheme(themeMode);

  const toggleTheme = useCallback(() => {
    setThemeMode((prev) => (prev === "dark" ? "light" : "dark"));
  }, []);

  const handleTabChange = useCallback((tabId: string) => {
    setActiveTab(tabId);
  }, []);

  const renderPanel = useCallback(() => {
    const panelConfigs = {
      threads: {
        title: "Chat",
        icon: <ChatIcon />,
        component: <ChatContainer welcomeMode={!activeTab} />,
      },
      projects: {
        title: "Projects",
        icon: <FolderIcon />,
        component: (
          <Suspense fallback={<LoadingFallback />}>
            <WorktreeDashboard />
          </Suspense>
        ),
      },
      actions: {
        title: "Actions",
        icon: <PlayIcon />,
        component: (
          <Suspense fallback={<LoadingFallback />}>
            <ActionsPanel />
          </Suspense>
        ),
      },
      security: {
        title: "Security",
        icon: <ShieldIcon />,
        component: (
          <Suspense fallback={<LoadingFallback />}>
            <SecurityDashboard />
          </Suspense>
        ),
      },
      notifications: {
        title: "Notifications",
        icon: <NotificationsIcon />,
        component: (
          <Suspense fallback={<LoadingFallback />}>
            <NotificationsPanel />
          </Suspense>
        ),
      },
      settings: {
        title: "Settings",
        icon: <SettingsIcon />,
        component: (
          <Suspense fallback={<LoadingFallback />}>
            <SettingsPanel />
          </Suspense>
        ),
      },
    };

    const config = panelConfigs[activeTab as keyof typeof panelConfigs];
    if (!config) return null;

    return (
      <ResizablePanel
        id={`${activeTab}-panel`}
        title={config.title}
        icon={config.icon}
        defaultWidth={panels[activeTab as keyof typeof panels]?.width || 400}
        minWidth={250}
        maxWidth={600}
        collapsible
        onCollapse={() => {
          setPanels((prev) => ({
            ...prev,
            [activeTab]: {
              ...prev[activeTab as keyof typeof prev],
              visible: false,
            },
          }));
        }}
      >
        {config.component}
      </ResizablePanel>
    );
  }, [activeTab, panels]);

  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <Box
        sx={{
          display: "flex",
          height: "100vh",
          width: "100vw",
          overflow: "hidden",
          bgcolor: "background.default",
        }}
      >
        {/* Sidebar Tabs */}
        <SidebarTabs
          tabs={defaultSidebarTabs}
          activeTab={activeTab}
          onTabChange={handleTabChange}
          collapsed={sidebarCollapsed}
          onToggleCollapse={() => setSidebarCollapsed(!sidebarCollapsed)}
        />

        {/* Main Content Area */}
        <Box
          sx={{
            flex: 1,
            display: "flex",
            flexDirection: "column",
            overflow: "hidden",
          }}
        >
          {/* Panel Content */}
          <Box
            sx={{
              flex: 1,
              display: "flex",
              overflow: "hidden",
            }}
          >
            {renderPanel()}
          </Box>
        </Box>
      </Box>
    </ThemeProvider>
  );
};

const LoadingFallback: React.FC = () => (
  <Box
    sx={{
      display: "flex",
      alignItems: "center",
      justifyContent: "center",
      height: "100%",
    }}
  >
    <CircularProgress />
  </Box>
);

// Import icons for SidebarTabs
import FolderIcon from "@mui/icons-material/Folder";
import ChatIcon from "@mui/icons-material/Chat";
import PlayIcon from "@mui/icons-material/Play";
import ShieldIcon from "@mui/icons-material/Shield";
import NotificationsIcon from "@mui/icons-material/Notifications";
import SettingsIcon from "@mui/icons-material/Settings";

export default Layout;
