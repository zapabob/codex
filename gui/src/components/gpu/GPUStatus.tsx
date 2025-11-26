'use client';

import React, { useEffect, useState } from 'react';
import {
  Box,
  Typography,
  Card as MuiCard,
  CardContent,
  LinearProgress,
  Chip,
  Tooltip,
  IconButton,
} from '@mui/material';
import Grid from '@/mui/Grid2';
import {
  Cpu,
  Zap,
  RefreshCw,
  TrendingUp,
  Activity,
} from 'lucide-react';
import { Card } from '../atoms/Card';
import { CodexAPIClient } from '../../lib/api/client';

export interface GPUStatusProps {
  /** Polling interval in milliseconds */
  pollInterval?: number;
  /** Show detailed information */
  showDetails?: boolean;
}

export interface GPUStats {
  name: string;
  vendor: 'nvidia' | 'amd' | 'intel' | 'unknown';
  usagePercent: number;
  memoryUsed: number;
  memoryTotal: number;
  memoryUsagePercent: number;
  temperature?: number;
  powerUsage?: number;
  clockSpeed?: number;
  computeCapability?: string;
  directMLVersion?: string;
  cudaVersion?: string;
}

export const GPUStatus: React.FC<GPUStatusProps> = ({
  pollInterval = 2000,
  showDetails = true,
}) => {
  const [gpuStats, setGpuStats] = useState<GPUStats[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [lastUpdate, setLastUpdate] = useState<Date>(new Date());

  const apiClient = React.useMemo(() => new CodexAPIClient(), []);

  const fetchGPUStats = async () => {
    try {
      setIsLoading(true);
      setError(null);

      const result = await apiClient.getGPUStatus();
      
      const stats: GPUStats[] = (result.gpus || []).map((gpu: any) => ({
        name: gpu.name || 'Unknown GPU',
        vendor: (gpu.vendor || 'unknown').toLowerCase() as GPUStats['vendor'],
        usagePercent: gpu.usagePercent || 0,
        memoryUsed: gpu.memoryUsed || 0,
        memoryTotal: gpu.memoryTotal || 0,
        memoryUsagePercent: gpu.memoryUsagePercent || 0,
        temperature: gpu.temperature,
        powerUsage: gpu.powerUsage,
        clockSpeed: gpu.clockSpeed,
        computeCapability: gpu.computeCapability,
        cudaVersion: gpu.cudaVersion,
        directMLVersion: gpu.directMLVersion,
      }));

      setGpuStats(stats);
      setLastUpdate(new Date());
    } catch (err) {
      console.error('Failed to fetch GPU stats:', err);
      setError(err instanceof Error ? err.message : 'Failed to fetch GPU stats');
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchGPUStats();

    const interval = setInterval(fetchGPUStats, pollInterval);

    return () => clearInterval(interval);
  }, [pollInterval]);

  const getVendorColor = (vendor: GPUStats['vendor']) => {
    switch (vendor) {
      case 'nvidia':
        return '#76b900'; // NVIDIA green
      case 'amd':
        return '#ed1c24'; // AMD red
      case 'intel':
        return '#0071c5'; // Intel blue
      default:
        return '#666666';
    }
  };

  const getUsageColor = (usage: number) => {
    if (usage >= 90) return 'error';
    if (usage >= 70) return 'warning';
    return 'success';
  };

  const formatBytes = (bytes: number) => {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
    return `${(bytes / (1024 * 1024 * 1024)).toFixed(1)} GB`;
  };

  if (error) {
    return (
      <Card>
        <Box sx={{ p: 2 }}>
          <Typography variant="body2" color="error">
            Error: {error}
          </Typography>
        </Box>
      </Card>
    );
  }

  return (
    <Box>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 2 }}>
        <Typography variant="h6" sx={{ fontWeight: 600 }}>
          GPU Status
        </Typography>
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <Typography variant="caption" color="text.secondary">
            Updated: {lastUpdate.toLocaleTimeString()}
          </Typography>
          <Tooltip title="Refresh">
            <IconButton size="small" onClick={fetchGPUStats} disabled={isLoading}>
              <RefreshCw size={16} />
            </IconButton>
          </Tooltip>
        </Box>
      </Box>

      {gpuStats.length === 0 ? (
        <Card>
          <Box sx={{ p: 3, textAlign: 'center' }}>
            <Activity size={48} style={{ marginBottom: 16, opacity: 0.5 }} />
            <Typography variant="body2" color="text.secondary">
              No GPU detected
            </Typography>
          </Box>
        </Card>
      ) : (
        <Grid container spacing={2}>
          {gpuStats.map((gpu, index) => (
            <Grid xs={12} md={6} key={index}>
              <MuiCard>
                <CardContent>
                  <Box sx={{ display: 'flex', alignItems: 'center', mb: 2 }}>
                    <Cpu size={24} style={{ color: getVendorColor(gpu.vendor), marginRight: 8 }} />
                    <Typography variant="h6" sx={{ flex: 1 }}>
                      {gpu.name}
                    </Typography>
                    <Chip
                      label={gpu.vendor.toUpperCase()}
                      size="small"
                      sx={{
                        bgcolor: getVendorColor(gpu.vendor),
                        color: 'white',
                        fontWeight: 600,
                      }}
                    />
                  </Box>

                  {/* GPU Usage */}
                  <Box sx={{ mb: 2 }}>
                    <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 1 }}>
                      <Typography variant="body2" color="text.secondary">
                        GPU Usage
                      </Typography>
                      <Typography variant="body2" fontWeight={600}>
                        {gpu.usagePercent.toFixed(1)}%
                      </Typography>
                    </Box>
                    <LinearProgress
                      variant="determinate"
                      value={gpu.usagePercent}
                      color={getUsageColor(gpu.usagePercent)}
                      sx={{ height: 8, borderRadius: 4 }}
                    />
                  </Box>

                  {/* Memory Usage */}
                  <Box sx={{ mb: 2 }}>
                    <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 1 }}>
                      <Typography variant="body2" color="text.secondary">
                        Memory Usage
                      </Typography>
                      <Typography variant="body2" fontWeight={600}>
                        {formatBytes(gpu.memoryUsed)} / {formatBytes(gpu.memoryTotal)} ({gpu.memoryUsagePercent.toFixed(1)}%)
                      </Typography>
                    </Box>
                    <LinearProgress
                      variant="determinate"
                      value={gpu.memoryUsagePercent}
                      color={getUsageColor(gpu.memoryUsagePercent)}
                      sx={{ height: 8, borderRadius: 4 }}
                    />
                  </Box>

                  {showDetails && (
                    <Grid container spacing={2} sx={{ mt: 1 }}>
                      {gpu.temperature !== undefined && (
                        <Grid xs={6}>
                          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                            <Zap size={16} />
                            <Typography variant="caption" color="text.secondary">
                              Temp: {gpu.temperature}°C
                            </Typography>
                          </Box>
                        </Grid>
                      )}
                      {gpu.powerUsage !== undefined && (
                        <Grid xs={6}>
                          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                            <TrendingUp size={16} />
                            <Typography variant="caption" color="text.secondary">
                              Power: {gpu.powerUsage}W
                            </Typography>
                          </Box>
                        </Grid>
                      )}
                      {gpu.clockSpeed !== undefined && (
                        <Grid xs={6}>
                          <Typography variant="caption" color="text.secondary">
                            Clock: {gpu.clockSpeed} MHz
                          </Typography>
                        </Grid>
                      )}
                      {gpu.computeCapability && (
                        <Grid xs={6}>
                          <Typography variant="caption" color="text.secondary">
                            Compute: {gpu.computeCapability}
                          </Typography>
                        </Grid>
                      )}
                      {gpu.directMLVersion && (
                        <Grid xs={6}>
                          <Typography variant="caption" color="text.secondary">
                            DirectML: {gpu.directMLVersion}
                          </Typography>
                        </Grid>
                      )}
                      {gpu.cudaVersion && (
                        <Grid xs={6}>
                          <Typography variant="caption" color="text.secondary">
                            CUDA: {gpu.cudaVersion}
                          </Typography>
                        </Grid>
                      )}
                    </Grid>
                  )}
                </CardContent>
              </MuiCard>
            </Grid>
          ))}
        </Grid>
      )}
    </Box>
  );
};

GPUStatus.displayName = 'GPUStatus';

export default GPUStatus;

