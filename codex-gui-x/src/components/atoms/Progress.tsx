import React, { forwardRef } from 'react';
import { LinearProgress, LinearProgressProps, Box, Typography, SxProps, Theme } from '@mui/material';
import { motion } from 'framer-motion';

export interface ProgressProps extends Omit<LinearProgressProps, 'sx'> {
  value: number;
  label?: string;
  showValue?: boolean;
  sx?: SxProps<Theme>;
}

export const Progress = forwardRef<HTMLDivElement, ProgressProps>(
  ({ value, label, showValue = false, sx, ...props }, ref) => {
    return (
      <Box sx={{ width: '100%', ...sx }} ref={ref}>
        {(label || showValue) && (
          <Box sx={{ display: 'flex', alignItems: 'center', mb: 1 }}>
            {label && (
              <Typography variant="caption" sx={{ flexGrow: 1, fontWeight: 600, color: 'text.secondary' }}>
                {label}
              </Typography>
            )}
            {showValue && (
              <Typography variant="caption" sx={{ fontWeight: 700, fontMono: true }}>
                {Math.round(value)}%
              </Typography>
            )}
          </Box>
        )}
        <LinearProgress
          variant="determinate"
          value={value}
          sx={{
            height: 8,
            borderRadius: 4,
            bgcolor: 'action.hover',
            '& .MuiLinearProgress-bar': {
              borderRadius: 4,
              transition: 'transform 0.4s cubic-bezier(0.4, 0, 0.2, 1)',
            },
          }}
          {...props}
        />
      </Box>
    );
  }
);

Progress.displayName = 'Progress';

export default Progress;
