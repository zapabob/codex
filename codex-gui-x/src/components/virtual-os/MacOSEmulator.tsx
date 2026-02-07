'use client';

import React, { useState, useEffect } from 'react';
import {
  Box,
  Typography,
  Paper,
  IconButton,
  Tooltip,
  Menu,
  MenuItem,
  TextField,
  InputAdornment,
  Drawer,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  Avatar,
  Chip,
} from '@mui/material';
import {
  Search,
  Apple,
  Folder,
  Terminal,
  Settings,
  X,
  Minimize2,
  Maximize2,
  Square,
  Menu as MenuIcon,
} from 'lucide-react';
import { CodexAPIClient } from '@/lib/api/client';

interface DockApp {
  id: string;
  name: string;
  icon?: string;
  isRunning: boolean;
}

interface Window {
  id: string;
  title: string;
  x: number;
  y: number;
  width: number;
  height: number;
  isMinimized: boolean;
  isMaximized: boolean;
  isVisible: boolean;
}

export function MacOSEmulator() {
  const [dockApps, setDockApps] = useState<DockApp[]>([
    { id: 'finder', name: 'Finder', isRunning: false },
    { id: 'terminal', name: 'Terminal', isRunning: false },
    { id: 'settings', name: 'Settings', isRunning: false },
  ]);
  const [windows, setWindows] = useState<Window[]>([]);
  const [spotlightOpen, setSpotlightOpen] = useState(false);
  const [spotlightQuery, setSpotlightQuery] = useState('');
  const [menuAnchor, setMenuAnchor] = useState<null | HTMLElement>(null);
  const [selectedApp, setSelectedApp] = useState<string | null>(null);

  const apiClient = new CodexAPIClient();

  useEffect(() => {
    // Load virtual OS state
    loadVirtualOSState();
  }, []);

  const loadVirtualOSState = async () => {
    // TODO: Load state from API
  };

  const handleDockAppClick = (appId: string) => {
    setDockApps(prev => prev.map(app =>
      app.id === appId ? { ...app, isRunning: !app.isRunning } : app
    ));

    // Create window for the app
    const newWindow: Window = {
      id: `window-${Date.now()}`,
      title: dockApps.find(a => a.id === appId)?.name || appId,
      x: 100 + windows.length * 30,
      y: 100 + windows.length * 30,
      width: 800,
      height: 600,
      isMinimized: false,
      isMaximized: false,
      isVisible: true,
    };
    setWindows(prev => [...prev, newWindow]);
    setSelectedApp(appId);
  };

  const handleWindowClose = (windowId: string) => {
    setWindows(prev => prev.filter(w => w.id !== windowId));
  };

  const handleWindowMinimize = (windowId: string) => {
    setWindows(prev => prev.map(w =>
      w.id === windowId ? { ...w, isMinimized: true, isVisible: false } : w
    ));
  };

  const handleWindowMaximize = (windowId: string) => {
    setWindows(prev => prev.map(w =>
      w.id === windowId ? { ...w, isMaximized: !w.isMaximized } : w
    ));
  };

  const handleSpotlightKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Escape') {
      setSpotlightOpen(false);
      setSpotlightQuery('');
    } else if (e.key === 'Enter') {
      // Execute spotlight search
      handleSpotlightSearch();
    }
  };

  const handleSpotlightSearch = () => {
    // TODO: Implement spotlight search
    console.log('Spotlight search:', spotlightQuery);
    setSpotlightOpen(false);
    setSpotlightQuery('');
  };

  const handleMenuClick = (event: React.MouseEvent<HTMLElement>) => {
    setMenuAnchor(event.currentTarget);
  };

  const handleMenuClose = () => {
    setMenuAnchor(null);
  };

  return (
    <Box
      sx={{
        width: '100%',
        height: '100vh',
        bgcolor: '#1e1e1e',
        display: 'flex',
        flexDirection: 'column',
        position: 'relative',
        overflow: 'hidden',
      }}
    >
      {/* Menu Bar */}
      <Box
        sx={{
          height: 24,
          bgcolor: 'rgba(0, 0, 0, 0.8)',
          display: 'flex',
          alignItems: 'center',
          px: 1,
          color: 'white',
          fontSize: '12px',
          backdropFilter: 'blur(10px)',
          zIndex: 1000,
        }}
      >
        <IconButton
          size="small"
          onClick={handleMenuClick}
          sx={{ color: 'white', p: 0.5 }}
        >
          <Apple size={16} />
        </IconButton>
        <Typography variant="caption" sx={{ ml: 1, fontWeight: 600 }}>
          {selectedApp || 'Codex'}
        </Typography>
        <Box sx={{ flexGrow: 1 }} />
        <Typography variant="caption" sx={{ fontSize: '11px' }}>
          {new Date().toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit' })}
        </Typography>
      </Box>

      {/* Apple Menu */}
      <Menu
        anchorEl={menuAnchor}
        open={Boolean(menuAnchor)}
        onClose={handleMenuClose}
        PaperProps={{
          sx: {
            bgcolor: 'rgba(0, 0, 0, 0.9)',
            color: 'white',
            minWidth: 200,
            backdropFilter: 'blur(20px)',
          },
        }}
      >
        <MenuItem onClick={handleMenuClose}>
          <Typography variant="body2">About This Mac</Typography>
        </MenuItem>
        <MenuItem onClick={handleMenuClose}>
          <Typography variant="body2">System Preferences...</Typography>
        </MenuItem>
        <MenuItem onClick={handleMenuClose}>
          <Typography variant="body2">App Store...</Typography>
        </MenuItem>
        <MenuItem onClick={handleMenuClose}>
          <Typography variant="body2" sx={{ borderTop: '1px solid rgba(255,255,255,0.1)', pt: 1, mt: 1 }}>
            Quit Codex
          </Typography>
        </MenuItem>
      </Menu>

      {/* Desktop Area */}
      <Box
        sx={{
          flexGrow: 1,
          position: 'relative',
          overflow: 'hidden',
        }}
      >
        {/* Windows */}
        {windows.map(window => (
          window.isVisible && (
            <Paper
              key={window.id}
              sx={{
                position: 'absolute',
                left: window.x,
                top: window.y,
                width: window.isMaximized ? '100%' : window.width,
                height: window.isMaximized ? 'calc(100% - 24px)' : window.height,
                bgcolor: '#2d2d2d',
                display: 'flex',
                flexDirection: 'column',
                boxShadow: '0 8px 32px rgba(0, 0, 0, 0.3)',
                zIndex: 100,
              }}
            >
              {/* Window Title Bar */}
              <Box
                sx={{
                  height: 32,
                  bgcolor: '#3d3d3d',
                  display: 'flex',
                  alignItems: 'center',
                  px: 1,
                  cursor: 'move',
                }}
              >
                <Box sx={{ display: 'flex', gap: 0.5 }}>
                  <IconButton
                    size="small"
                    onClick={() => handleWindowClose(window.id)}
                    sx={{
                      width: 12,
                      height: 12,
                      bgcolor: '#ff5f57',
                      '&:hover': { bgcolor: '#ff3b30' },
                    }}
                  >
                    <X size={8} />
                  </IconButton>
                  <IconButton
                    size="small"
                    onClick={() => handleWindowMinimize(window.id)}
                    sx={{
                      width: 12,
                      height: 12,
                      bgcolor: '#ffbd2e',
                      '&:hover': { bgcolor: '#ff9500' },
                    }}
                  >
                    <Minimize2 size={8} />
                  </IconButton>
                  <IconButton
                    size="small"
                    onClick={() => handleWindowMaximize(window.id)}
                    sx={{
                      width: 12,
                      height: 12,
                      bgcolor: '#28c840',
                      '&:hover': { bgcolor: '#1fb231' },
                    }}
                  >
                    <Maximize2 size={8} />
                  </IconButton>
                </Box>
                <Typography
                  variant="caption"
                  sx={{
                    ml: 2,
                    color: 'white',
                    fontSize: '12px',
                    flexGrow: 1,
                    textAlign: 'center',
                  }}
                >
                  {window.title}
                </Typography>
              </Box>

              {/* Window Content */}
              <Box
                sx={{
                  flexGrow: 1,
                  p: 2,
                  color: 'white',
                  overflow: 'auto',
                }}
              >
                <Typography variant="h6" sx={{ mb: 2 }}>
                  {window.title}
                </Typography>
                <Typography variant="body2" color="text.secondary">
                  {window.title === 'Finder' && 'Finder content goes here'}
                  {window.title === 'Terminal' && 'Terminal content goes here'}
                  {window.title === 'Settings' && 'Settings content goes here'}
                </Typography>
              </Box>
            </Paper>
          )
        ))}

        {/* Spotlight Search Overlay */}
        {spotlightOpen && (
          <Box
            sx={{
              position: 'absolute',
              top: '50%',
              left: '50%',
              transform: 'translate(-50%, -50%)',
              width: 600,
              bgcolor: 'rgba(0, 0, 0, 0.9)',
              borderRadius: 2,
              p: 2,
              boxShadow: '0 8px 32px rgba(0, 0, 0, 0.5)',
              zIndex: 2000,
              backdropFilter: 'blur(20px)',
            }}
          >
            <TextField
              fullWidth
              placeholder="Spotlight Search"
              value={spotlightQuery}
              onChange={(e) => setSpotlightQuery(e.target.value)}
              onKeyDown={handleSpotlightKeyPress}
              autoFocus
              InputProps={{
                startAdornment: (
                  <InputAdornment position="start">
                    <Search size={20} color="white" />
                  </InputAdornment>
                ),
              }}
              sx={{
                '& .MuiOutlinedInput-root': {
                  color: 'white',
                  '& fieldset': {
                    borderColor: 'rgba(255, 255, 255, 0.3)',
                  },
                  '&:hover fieldset': {
                    borderColor: 'rgba(255, 255, 255, 0.5)',
                  },
                  '&.Mui-focused fieldset': {
                    borderColor: 'rgba(255, 255, 255, 0.7)',
                  },
                },
              }}
            />
          </Box>
        )}
      </Box>

      {/* Dock */}
      <Box
        sx={{
          height: 60,
          bgcolor: 'rgba(0, 0, 0, 0.6)',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          gap: 1,
          px: 2,
          backdropFilter: 'blur(20px)',
          borderTop: '1px solid rgba(255, 255, 255, 0.1)',
        }}
      >
        {dockApps.map(app => (
          <Tooltip key={app.id} title={app.name}>
            <Box
              onClick={() => handleDockAppClick(app.id)}
              sx={{
                width: 48,
                height: 48,
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                borderRadius: 1,
                cursor: 'pointer',
                bgcolor: app.isRunning ? 'rgba(255, 255, 255, 0.2)' : 'transparent',
                '&:hover': {
                  bgcolor: 'rgba(255, 255, 255, 0.1)',
                  transform: 'scale(1.1)',
                },
                transition: 'all 0.2s',
              }}
            >
              <Avatar
                sx={{
                  width: 40,
                  height: 40,
                  bgcolor: app.isRunning ? '#007AFF' : '#555',
                }}
              >
                {app.name.charAt(0)}
              </Avatar>
              {app.isRunning && (
                <Chip
                  size="small"
                  sx={{
                    position: 'absolute',
                    bottom: 0,
                    width: 4,
                    height: 4,
                    bgcolor: '#007AFF',
                    borderRadius: '50%',
                  }}
                />
              )}
            </Box>
          </Tooltip>
        ))}
      </Box>

      {/* Spotlight Shortcut Hint */}
      <Box
        sx={{
          position: 'absolute',
          bottom: 80,
          left: '50%',
          transform: 'translateX(-50%)',
          bgcolor: 'rgba(0, 0, 0, 0.7)',
          px: 2,
          py: 1,
          borderRadius: 1,
          color: 'white',
          fontSize: '12px',
        }}
      >
        Press Cmd+Space to open Spotlight
      </Box>
    </Box>
  );
}

