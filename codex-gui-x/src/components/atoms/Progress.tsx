import { forwardRef } from 'react';
import { LinearProgress, Box, Typography } from '@mui/material';
import type { LinearProgressProps, SxProps, Theme } from '@mui/material';

export interface ProgressProps extends Partial<LinearProgressProps> {
  value: number;
  label?: string;
  showValue?: boolean;
  className?: string;
  sx?: SxProps<Theme>;
}

export const Progress = forwardRef<HTMLDivElement, ProgressProps>(
  ({ value, label, showValue = false, className, sx, ...props }, ref) => {
    return (
      <Box ref={ref} className={className} sx={{ width: '100%', ...sx }}>
        {(label || showValue) && (
          <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', mb: 1 }}>
            {label && (
              <Typography variant="caption" sx={{ fontWeight: 600, color: 'text.secondary' }}>
                {label}
              </Typography>
            )}
            {showValue && (
              <Typography variant="caption" sx={{ fontWeight: 700, fontFamily: 'monospace' }}>
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
