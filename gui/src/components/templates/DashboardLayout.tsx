import React, { useState, useEffect } from 'react';
import { useRouter, usePathname } from 'next/navigation';
import {
  Box,
  Container,
  useMediaQuery,
  useTheme as useMuiTheme,
  SxProps,
  Theme,
} from '@mui/material';
import { Header } from '@/components/organisms/Header';
import { Sidebar } from '@/components/organisms/Sidebar';
import { NavigationItem } from '@/components/organisms/Sidebar';
import { AppThemeProvider } from './ThemeProvider';
import { useKeyboardShortcuts, ShortcutConfig } from '@/hooks/useKeyboardShortcuts';

export interface DashboardLayoutProps {
  children: React.ReactNode;
  title?: string;
  activeNavItem?: string;
  onNavigate?: (item: NavigationItem) => void;
  onSettingsClick?: () => void;
  onProfileClick?: () => void;
  sx?: SxProps<Theme>;
}

export const DashboardLayout: React.FC<DashboardLayoutProps> = ({
  children,
  title,
  activeNavItem = 'dashboard',
  onNavigate,
  onSettingsClick,
  onProfileClick,
  sx,
}) => {
  const [sidebarOpen, setSidebarOpen] = useState(false);
  const [sidebarCollapsed, setSidebarCollapsed] = useState(false);
  const router = useRouter();
  const pathname = usePathname();
  const muiTheme = useMuiTheme();
  const isMobile = useMediaQuery(muiTheme.breakpoints.down('md'));

  const handleMenuClick = () => {
    setSidebarOpen(!sidebarOpen);
  };

  const handleSidebarClose = () => {
    setSidebarOpen(false);
  };

  const handleToggleCollapse = () => {
    setSidebarCollapsed(!sidebarCollapsed);
  };

  const handleNavigate = (item: NavigationItem) => {
    // Navigate using Next.js router
    const pathMap: Record<string, string> = {
      'dashboard': '/',
      // ... mappings remain same
      'code': '/code',
      'agents': '/agents',
      'tasks': '/tasks',
      'qc': '/qc',
      'security': '/security',
      'virtual-os': '/virtual-os',
      'ai-tools': '/ai-tools',
      'research': '/research',
      'web-research': '/web-research',
      'mcp': '/mcp',
      'analytics': '/analytics',
      'docs': '/docs',
      'performance': '/performance',
      'settings': '/settings',
    };

    const path = pathMap[item.id] || '/';
    router.push(path);
    onNavigate?.(item);
  };

  const handleSettingsClick = () => {
    router.push('/settings');
    onSettingsClick?.();
  };

  const handleProfileClick = () => {
    onProfileClick?.();
  };

  // Keyboard shortcuts for navigation
  const navigationShortcuts: ShortcutConfig[] = [
    // ... existing shortcuts 
    {
        key: 'd',
        ctrl: true,
        description: 'ダッシュボードに移動',
        action: () => handleNavigate({ id: 'dashboard', label: 'ダッシュボード', icon: () => null }),
      },
      {
        key: 'c',
        ctrl: true,
        description: 'コード実行ページに移動',
        action: () => handleNavigate({ id: 'code', label: 'コード実行', icon: () => null }),
      },
      {
        key: 'a',
        ctrl: true,
        description: 'エージェントページに移動',
        action: () => handleNavigate({ id: 'agents', label: 'エージェント', icon: () => null }),
      },
      {
        key: 't',
        ctrl: true,
        description: 'タスク管理ページに移動',
        action: () => handleNavigate({ id: 'tasks', label: 'タスク管理', icon: () => null }),
      },
      {
        key: 'q',
        ctrl: true,
        description: 'QC管理ページに移動',
        action: () => handleNavigate({ id: 'qc', label: 'QC管理', icon: () => null }),
      },
      {
        key: 's',
        ctrl: true,
        description: 'セキュリティページに移動',
        action: () => handleNavigate({ id: 'security', label: 'セキュリティ', icon: () => null }),
      },
      {
        key: 'v',
        ctrl: true,
        description: '仮想OSページに移動',
        action: () => handleNavigate({ id: 'virtual-os', label: '仮想OS', icon: () => null }),
      },
      {
        key: 'i',
        ctrl: true,
        description: 'AIツール統合ページに移動',
        action: () => handleNavigate({ id: 'ai-tools', label: 'AIツール統合', icon: () => null }),
      },
      {
        key: 'r',
        ctrl: true,
        description: 'Deep Researchページに移動',
        action: () => handleNavigate({ id: 'research', label: 'Deep Research', icon: () => null }),
      },
      {
        key: 'r',
        ctrl: true,
        shift: true,
        description: 'Web Researchページに移動',
        action: () => handleNavigate({ id: 'web-research', label: 'Web Research', icon: () => null }),
      },
      {
        key: 'm',
        ctrl: true,
        description: 'MCPサーバーページに移動',
        action: () => handleNavigate({ id: 'mcp', label: 'MCPサーバー', icon: () => null }),
      },
      {
        key: ',',
        ctrl: true,
        description: '設定ページに移動',
        action: () => handleSettingsClick(),
      },
    {
      key: 'b',
      ctrl: true,
      description: 'サイドバーの表示/非表示（モバイル）',
      action: () => handleMenuClick(),
    },
    {
       key: '\\', // Backslash to toggle collapse
       ctrl: true,
       description: 'サイドバーの折りたたみ',
       action: () => handleToggleCollapse(),
    }
  ];

  useKeyboardShortcuts({
    shortcuts: navigationShortcuts,
    enabled: true,
    ignoreWhenInputFocused: true,
  });

  return (
    <AppThemeProvider>
      <Box
        sx={{
          display: 'flex',
          minHeight: '100vh',
          backgroundColor: 'background.default',
          ...sx,
        }}
      >
        <Sidebar
          open={sidebarOpen}
          collapsed={sidebarCollapsed}
          onClose={handleSidebarClose}
          onNavigate={handleNavigate}
          onToggleCollapse={handleToggleCollapse}
          activeItem={activeNavItem}
        />

        <Box
          component="main"
          sx={{
            flexGrow: 1,
            display: 'flex',
            flexDirection: 'column',
            minWidth: 0, // Prevents flex item from overflowing
          }}
        >
          <Header
            title={title}
            onMenuClick={handleMenuClick}
            onSettingsClick={handleSettingsClick}
            onProfileClick={handleProfileClick}
            showMenuButton={isMobile}
          />

          <Container
            maxWidth="xl"
            sx={{
              flex: 1,
              py: 3,
              px: { xs: 2, sm: 3, md: 4 },
              display: 'flex',
              flexDirection: 'column',
            }}
          >
            {children}
          </Container>
        </Box>
      </Box>
    </AppThemeProvider>
  );
};

export default DashboardLayout;
