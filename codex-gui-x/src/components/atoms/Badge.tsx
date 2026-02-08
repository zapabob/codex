import { forwardRef } from 'react';
import { Box } from '@mui/material';
import type { SxProps, Theme } from '@mui/material';
import { motion } from 'framer-motion';

export interface BadgeProps {
  children: React.ReactNode;
  variant?: 'default' | 'outline' | 'secondary' | 'ghost';
  color?: 'primary' | 'secondary' | 'error' | 'warning' | 'info' | 'success';
  size?: 'sm' | 'md';
  className?: string;
  sx?: SxProps<Theme>;
}

const MotionBox = motion.create(Box);

export const Badge = forwardRef<HTMLDivElement, BadgeProps>(
  ({ children, variant = 'default', color = 'primary', size = 'sm', className, sx, ...props }, ref) => {
    const getStyles = (): SxProps<Theme> => {
      const styles: SxProps<Theme> = {
        display: 'inline-flex',
        alignItems: 'center',
        justifyContent: 'center',
        borderRadius: 'full',
        fontWeight: 600,
        whiteSpace: 'nowrap',
        transition: 'all 0.2s',
        px: size === 'sm' ? 1 : 1.5,
        py: size === 'sm' ? 0.25 : 0.5,
        fontSize: size === 'sm' ? '0.625rem' : '0.75rem',
        ...sx,
      };

      const colorMap: Record<string, string> = {
        primary: 'primary.main',
        secondary: 'secondary.main',
        error: 'error.main',
        warning: 'warning.main',
        info: 'info.main',
        success: 'success.main',
      };

      const colorValue = colorMap[color] || colorMap.primary;

      switch (variant) {
        case 'outline':
          return {
            ...styles,
            border: '1px solid',
            borderColor: colorValue,
            color: colorValue,
            bgcolor: 'transparent',
          };
        case 'secondary':
          return {
            ...styles,
            bgcolor: `${colorValue}22`,
            color: colorValue,
          };
        case 'ghost':
          return {
            ...styles,
            bgcolor: 'transparent',
            color: colorValue,
          };
        default:
          return {
            ...styles,
            bgcolor: colorValue,
            color: 'background.paper',
          };
      }
    };

    return (
      <MotionBox
        ref={ref}
        sx={getStyles()}
        className={className}
        initial={{ scale: 0.9, opacity: 0 }}
        animate={{ scale: 1, opacity: 1 }}
        whileHover={{ scale: 1.05 }}
        {...props}
      >
        {children}
      </MotionBox>
    );
  }
);

Badge.displayName = 'Badge';

export default Badge;
