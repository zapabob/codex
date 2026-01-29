import React from 'react';
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
  SxProps,
  Theme,
  Tooltip,
  Chip,
} from '@mui/material';
import { motion, AnimatePresence } from 'framer-motion';
import {
  Home,
  Code,
  Settings,
  FileText,
  BarChart3,
  Users,
  Shield,
  Zap,
  Search,
  Server,
  CheckSquare,
  TrendingUp,
  Monitor,
  Bot,
} from 'lucide-react';

export interface NavigationItem {
  id: string;
  label: string;
  icon: React.ComponentType<any>;
  path?: string;
  badge?: string | number;
  shortcut?: string;
  description?: string;
}

export interface SidebarProps {
  open: boolean;
  collapsed?: boolean;
  onClose: () => void;
  onNavigate: (item: NavigationItem) => void;
  onToggleCollapse?: () => void;
  activeItem?: string;
  width?: number;
  collapsedWidth?: number;
  sx?: SxProps<Theme>;
}

const navigationItems: NavigationItem[] = [
  { 
    id: 'dashboard', 
    label: 'ダッシュボード', 
    icon: Home, 
    shortcut: 'Ctrl+D',
    description: 'システム概要とメトリクス'
  },
  // ... other items remain the same
  { 
    id: 'code', 
    label: 'コード実行', 
    icon: Code,
    shortcut: 'Ctrl+C',
    description: 'コードを実行して結果を確認'
  },
  { 
    id: 'agents', 
    label: 'エージェント', 
    icon: Users,
    shortcut: 'Ctrl+A',
    description: 'AIエージェントの管理と実行'
  },
  { 
    id: 'tasks', 
    label: 'タスク管理', 
    icon: CheckSquare,
    shortcut: 'Ctrl+T',
    description: 'タスクの作成と管理'
  },
  { 
    id: 'qc', 
    label: 'QC管理', 
    icon: TrendingUp,
    shortcut: 'Ctrl+Q',
    description: '品質管理と分析'
  },
  { 
    id: 'security', 
    label: 'セキュリティ', 
    icon: Shield,
    shortcut: 'Ctrl+S',
    description: 'セキュリティ監視とスキャン'
  },
  { 
    id: 'virtual-os', 
    label: '仮想OS', 
    icon: Monitor,
    shortcut: 'Ctrl+V',
    description: '仮想OS環境とリソース監視'
  },
  { 
    id: 'ai-tools', 
    label: 'AIツール統合', 
    icon: Bot,
    shortcut: 'Ctrl+I',
    description: 'AIツールの統合と管理'
  },
  { 
    id: 'research', 
    label: 'Deep Research', 
    icon: Search,
    shortcut: 'Ctrl+R',
    description: '深い調査と研究'
  },
  {
    id: 'web-research',
    label: 'Web Research',
    icon: Search,
    shortcut: 'Ctrl+Shift+R',
    description: 'Official web_search tool for fast web queries'
  },
  { 
    id: 'mcp', 
    label: 'MCPサーバー', 
    icon: Server,
    shortcut: 'Ctrl+M',
    description: 'MCPサーバーの管理'
  },
  { 
    id: 'analytics', 
    label: '分析', 
    icon: BarChart3,
    shortcut: 'Ctrl+L',
    description: 'データ分析と可視化'
  },
  { 
    id: 'docs', 
    label: 'ドキュメント', 
    icon: FileText,
    shortcut: 'Ctrl+O',
    description: 'ドキュメントの閲覧'
  },
  { 
    id: 'performance', 
    label: 'パフォーマンス', 
    icon: Zap,
    shortcut: 'Ctrl+P',
    description: 'パフォーマンス監視'
  },
];

const settingsItems: NavigationItem[] = [
  { 
    id: 'settings', 
    label: '設定', 
    icon: Settings,
    shortcut: 'Ctrl+,',
    description: 'アプリケーション設定'
  },
];

import { IconButton } from '@mui/material';
import { ChevronLeft, ChevronRight } from 'lucide-react';

export const Sidebar: React.FC<SidebarProps> = ({
  open,
  collapsed = false,
  onClose,
  onNavigate,
  onToggleCollapse,
  activeItem,
  width = 280,
  collapsedWidth = 72,
  sx,
}) => {
  const currentWidth = collapsed ? collapsedWidth : width;

  const handleItemClick = (item: NavigationItem) => {
    onNavigate(item);
    // Mobileではクリック後に閉じる
    if (window.innerWidth < 768) {
      onClose();
    }
  };

  const sidebarContent = (
    <Box
      sx={{
        width: currentWidth,
        height: '100%',
        display: 'flex',
        flexDirection: 'column',
        background: (theme) =>
          theme.palette.mode === 'dark'
            ? 'linear-gradient(180deg, #0f1419 0%, #1d1b20 100%)'
            : 'linear-gradient(180deg, #fdfbff 0%, #e7e0ec 100%)',
        transition: 'width 0.2s cubic-bezier(0.4, 0, 0.2, 1)',
        overflowX: 'hidden', // Hide overflow during transition
        ...sx,
      }}
    >
      {/* Header */}
      <Box sx={{ 
        p: collapsed ? 2 : 3, 
        borderBottom: '1px solid', 
        borderColor: 'outline.variant',
        display: 'flex',
        alignItems: 'center',
        justifyContent: collapsed ? 'center' : 'space-between',
        height: 64, // Fixed height for header alignment
      }}>
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
                    background: 'linear-gradient(45deg, #0061a4, #565f71)',
                    backgroundClip: 'text',
                    WebkitBackgroundClip: 'text',
                    WebkitTextFillColor: 'transparent',
                    lineHeight: 1.2,
                    whiteSpace: 'nowrap',
                  }}
                >
                  Codex Control
                </Typography>
                <Typography variant="caption" sx={{ color: 'text.secondary', mt: 0, whiteSpace: 'nowrap' }}>
                  AI Assistant Platform
                </Typography>
              </Box>
            </motion.div>
          )}
        </AnimatePresence>
        
        {onToggleCollapse && !collapsed && (
           <IconButton onClick={onToggleCollapse} size="small" sx={{ ml: 1 }}>
             <ChevronLeft size={20} />
           </IconButton>
        )}
      </Box>

      {/* Navigation */}
      <Box sx={{ flex: 1, overflow: 'auto', py: 2 }}>
        <List sx={{ px: 1 }}>
          {navigationItems.map((item, index) => {
            const Icon = item.icon;
            const isActive = activeItem === item.id;

            return (
              <Tooltip
                key={item.id}
                title={
                  collapsed ? (
                    <Box>
                      <Typography variant="body2" sx={{ fontWeight: 600, mb: 0.5 }}>
                        {item.label}
                      </Typography>
                      {item.description && (
                        <Typography variant="caption" sx={{ display: 'block', opacity: 0.9 }}>
                          {item.description}
                        </Typography>
                      )}
                      {item.shortcut && (
                         <Chip label={item.shortcut} size="small" sx={{ mt:0.5, height:18, fontSize:'0.65rem', bgcolor:'rgba(255,255,255,0.15)' }} />
                      )}
                    </Box>
                  ) : '' // No tooltip if expanded (optional, or keep generic tooltip)
                }
                placement="right"
                arrow
              >
                <ListItem disablePadding sx={{ mb: 0.5 }}>
                  <ListItemButton
                    onClick={() => handleItemClick(item)}
                    sx={{
                      borderRadius: 2,
                      mx: collapsed ? 0 : 1,
                      justifyContent: collapsed ? 'center' : 'flex-start',
                      px: collapsed ? 1 : 2,
                      py: 1.5,
                      position: 'relative',
                      backgroundColor: isActive
                        ? 'primary.main'
                        : 'transparent',
                      color: isActive
                        ? 'primary.contrastText'
                        : 'text.primary',
                      '&:hover': {
                        backgroundColor: isActive
                          ? 'primary.dark'
                          : 'action.hover',
                        },
                      // Active indicator line
                      '&::before': {
                        content: '""',
                        position: 'absolute',
                        left: 0,
                        top: '50%',
                        transform: 'translateY(-50%)',
                        width: isActive ? 4 : 0,
                        height: '60%',
                        backgroundColor: 'primary.main',
                        borderRadius: '0 4px 4px 0',
                        transition: 'width 0.2s cubic-bezier(0.4, 0, 0.2, 1)',
                        display: collapsed ? 'none' : 'block',
                      },
                      transition: 'all 0.2s cubic-bezier(0.4, 0, 0.2, 1)',
                    }}
                  >
                    <ListItemIcon
                      sx={{
                        minWidth: collapsed ? 0 : 40,
                        color: 'inherit',
                        transition: 'transform 0.2s',
                        justifyContent: 'center',
                        mr: collapsed ? 0 : 0,
                        '&:hover': {
                          transform: 'scale(1.1)',
                        },
                      }}
                    >
                      <Icon size={20} />
                    </ListItemIcon>
                    
                    {!collapsed && (
                      <ListItemText
                        primary={item.label}
                        primaryTypographyProps={{
                          fontSize: '0.875rem',
                          fontWeight: isActive ? 600 : 500,
                          sx: { whiteSpace: 'nowrap' }
                        }}
                      />
                    )}

                    {!collapsed && (
                      <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, ml: 'auto' }}>
                        {item.shortcut && (
                          <Chip
                            label={item.shortcut}
                            size="small"
                            sx={{
                              height: 20,
                              fontSize: '0.7rem',
                              bgcolor: isActive
                                ? 'rgba(255, 255, 255, 0.2)'
                                : 'rgba(0, 0, 0, 0.05)',
                              color: 'inherit',
                              display: { xs: 'none', md: 'flex' },
                            }}
                          />
                        )}
                        {item.badge && (
                          <Box
                            sx={{
                              px: 1,
                              py: 0.25,
                              borderRadius: 1,
                              fontSize: '0.75rem',
                              fontWeight: 600,
                              backgroundColor: isActive
                                ? 'rgba(255, 255, 255, 0.2)'
                                : 'primary.main',
                              color: isActive
                                ? 'inherit'
                                : 'primary.contrastText',
                            }}
                          >
                            {item.badge}
                          </Box>
                        )}
                      </Box>
                    )}
                  </ListItemButton>
                </ListItem>
              </Tooltip>
            );
          })}
        </List>

        <Divider sx={{ my: 2, mx: 2 }} />

        <List sx={{ px: 1 }}>
            {/* Settings Item - reusing similar logic */}
            {settingsItems.map((item) => {
                 const Icon = item.icon;
                 const isActive = activeItem === item.id;
                 return (
                    <Tooltip key={item.id} title={collapsed ? item.label : ''} placement="right" arrow>
                         <ListItem disablePadding sx={{ mb: 0.5 }}>
                            <ListItemButton
                                onClick={() => handleItemClick(item)}
                                sx={{
                                    borderRadius: 2,
                                    mx: collapsed ? 0 : 1,
                                    justifyContent: collapsed ? 'center' : 'flex-start',
                                    px: collapsed ? 1 : 2,
                                    py: 1.5,
                                    backgroundColor: isActive ? 'primary.main' : 'transparent',
                                    color: isActive ? 'primary.contrastText' : 'text.secondary',
                                    '&:hover': { backgroundColor: isActive ? 'primary.dark' : 'action.hover' },
                                }}
                            >
                                <ListItemIcon sx={{ minWidth: collapsed ? 0 : 40, color: 'inherit', justifyContent: 'center' }}>
                                    <Icon size={20} />
                                </ListItemIcon>
                                {!collapsed && <ListItemText primary={item.label} primaryTypographyProps={{ fontSize: '0.875rem' }} />}
                            </ListItemButton>
                         </ListItem>
                    </Tooltip>
                 )
            })}
        </List>
        
        {/* Toggle Button at bottom if collapsed */}
        {collapsed && onToggleCollapse && (
            <Box sx={{ mt: 'auto', p: 2, display: 'flex', justifyContent: 'center' }}>
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
      <Drawer
        variant="temporary"
        open={open}
        onClose={onClose}
        ModalProps={{
          keepMounted: true, // Better mobile performance
        }}
        sx={{
          display: { xs: 'block', md: 'none' },
          '& .MuiDrawer-paper': {
            width,
            boxSizing: 'border-box',
          },
        }}
      >
        {sidebarContent}
      </Drawer>

      {/* Desktop Drawer */}
      <Drawer
        variant="permanent"
        sx={{
          display: { xs: 'none', md: 'block' },
          '& .MuiDrawer-paper': {
            width,
            boxSizing: 'border-box',
            borderRight: '1px solid',
            borderColor: 'outline.variant',
          },
        }}
        open
      >
        {sidebarContent}
      </Drawer>
    </>
  );
};

export default Sidebar;
