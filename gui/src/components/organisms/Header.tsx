import React from 'react';
import {
  AppBar,
  Toolbar,
  Typography,
  Box,
  SxProps,
  Theme,
} from '@mui/material';
import { motion } from 'framer-motion';
import { IconButton } from '@/components/atoms/IconButton';
import { useAppTheme } from '@/components/templates/ThemeProvider';
import { Sun, Moon, Monitor, Menu, Settings, User } from 'lucide-react';

export interface HeaderProps {
  title?: string;
  onMenuClick?: () => void;
  onSettingsClick?: () => void;
  onProfileClick?: () => void;
  showMenuButton?: boolean;
  sx?: SxProps<Theme>;
}

export const Header: React.FC<HeaderProps> = ({
  title = 'Codex GUI',
  onMenuClick,
  onSettingsClick,
  onProfileClick,
  showMenuButton = true,
  sx,
}) => {
  const { theme, toggleTheme } = useAppTheme();

  const getThemeIcon = () => {
    switch (theme) {
      case 'light':
        return Sun;
      case 'dark':
        return Moon;
      default:
        return Monitor;
    }
  };

  const ThemeIcon = getThemeIcon();

  return (
    <AppBar
      position="sticky"
      elevation={0}
      sx={{
        backgroundColor: 'background.paper',
        borderBottom: '1px solid',
        borderColor: 'outline.variant',
        backdropFilter: 'blur(12px)',
        ...sx,
      }}
    >
      <Toolbar sx={{ minHeight: 64 }}>
        {showMenuButton && (
          <motion.div
            whileHover={{ scale: 1.05 }}
            whileTap={{ scale: 0.95 }}
          >
            <IconButton
              icon={Menu}
              tooltip="メニューを開く"
              onClick={onMenuClick}
              sx={{ mr: 2 }}
            />
          </motion.div>
        )}

        <motion.div
          initial={{ opacity: 0, x: -20 }}
          animate={{ opacity: 1, x: 0 }}
          transition={{ duration: 0.5 }}
        >
          <Typography
            variant="h6"
            component="div"
            sx={{
              flexGrow: 1,
              fontWeight: 600,
              background: 'linear-gradient(45deg, #0061a4, #565f71)',
              backgroundClip: 'text',
              WebkitBackgroundClip: 'text',
              WebkitTextFillColor: 'transparent',
            }}
          >
            {title}
          </Typography>
        </motion.div>

        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <motion.div
            whileHover={{ scale: 1.05 }}
            whileTap={{ scale: 0.95 }}
          >
            <IconButton
              icon={ThemeIcon}
              tooltip={`テーマ切り替え (${theme})`}
              onClick={toggleTheme}
              variant="outlined"
              size="small"
            />
          </motion.div>

          <motion.div
            whileHover={{ scale: 1.05 }}
            whileTap={{ scale: 0.95 }}
          >
            <IconButton
              icon={Settings}
              tooltip="設定"
              onClick={onSettingsClick}
              variant="outlined"
              size="small"
            />
          </motion.div>

          <motion.div
            whileHover={{ scale: 1.05 }}
            whileTap={{ scale: 0.95 }}
          >
            <IconButton
              icon={User}
              tooltip="プロフィール"
              onClick={onProfileClick}
              variant="outlined"
              size="small"
            />
          </motion.div>
        </Box>
      </Toolbar>
    </AppBar>
  );
};

export default Header;
