import React, { useEffect, useState, useCallback } from 'react';
import { Button, ButtonProps } from '../atoms/Button';
import { Alert, AlertTitle, Box, Tooltip } from '@mui/material';
import { CodexAPIClient } from '../../lib/api/client';

export interface ResourceManagedButtonProps extends ButtonProps {
  /** Minimum available slots required to enable the button */
  minAvailableSlots?: number;
  /** Maximum CPU usage percentage allowed (0-100) */
  maxCpuUsage?: number;
  /** Maximum memory usage percentage allowed (0-100) */
  maxMemoryUsage?: number;
  /** Show resource status tooltip */
  showResourceStatus?: boolean;
  /** Custom warning message when resources are insufficient */
  resourceWarningMessage?: string;
  /** Polling interval in milliseconds for resource status */
  pollInterval?: number;
}

/**
 * ResourceManagedButton - A button component that automatically enables/disables
 * based on system resource availability.
 * 
 * This component monitors CPU, memory, and available task slots, and disables
 * the button when resources are insufficient.
 */
export const ResourceManagedButton: React.FC<ResourceManagedButtonProps> = ({
  minAvailableSlots = 1,
  maxCpuUsage = 90,
  maxMemoryUsage = 90,
  showResourceStatus = true,
  resourceWarningMessage,
  pollInterval = 2000,
  disabled: externalDisabled,
  onClick,
  ...buttonProps
}) => {
  const [resourceStatus, setResourceStatus] = useState<{
    availableSlots: number;
    cpuUsage: number;
    memoryUsage: number;
    isAvailable: boolean;
  } | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const apiClient = React.useMemo(() => new CodexAPIClient(), []);

  const checkResourceStatus = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);
      
      const status = await apiClient.getResourceStatus();
      
      const availableSlots = status?.capacity?.availableSlots ?? 0;
      const cpuUsage = status?.stats?.cpuUsagePercent ?? 0;
      const memoryUsage = status?.stats?.memoryUsagePercent ?? 0;
      
      const isAvailable = 
        availableSlots >= minAvailableSlots &&
        cpuUsage <= maxCpuUsage &&
        memoryUsage <= maxMemoryUsage;

      setResourceStatus({
        availableSlots,
        cpuUsage,
        memoryUsage,
        isAvailable,
      });
    } catch (err) {
      console.error('Failed to check resource status:', err);
      setError(err instanceof Error ? err.message : 'Failed to check resources');
      // On error, assume resources are available to avoid blocking user
      setResourceStatus({
        availableSlots: minAvailableSlots,
        cpuUsage: 0,
        memoryUsage: 0,
        isAvailable: true,
      });
    } finally {
      setIsLoading(false);
    }
  }, [apiClient, minAvailableSlots, maxCpuUsage, maxMemoryUsage]);

  useEffect(() => {
    // Initial check
    checkResourceStatus();

    // Set up polling
    const interval = setInterval(checkResourceStatus, pollInterval);

    return () => clearInterval(interval);
  }, [checkResourceStatus, pollInterval]);

  const isResourceAvailable = resourceStatus?.isAvailable ?? true;
  const isDisabled = externalDisabled || !isResourceAvailable || isLoading;

  const handleClick = useCallback(
    (event: React.MouseEvent<HTMLButtonElement>) => {
      if (isDisabled) return;
      
      // Try to acquire resource slot before executing
      apiClient.acquireResource().catch((err) => {
        console.error('Failed to acquire resource slot:', err);
      });
      
      onClick?.(event);
    },
    [isDisabled, onClick, apiClient]
  );

  const getResourceTooltip = () => {
    if (!resourceStatus) return '';
    
    const { availableSlots, cpuUsage, memoryUsage, isAvailable } = resourceStatus;
    
    if (isAvailable) {
      return `Resources available: ${availableSlots} slots, CPU: ${cpuUsage.toFixed(1)}%, Memory: ${memoryUsage.toFixed(1)}%`;
    }
    
    const reasons: string[] = [];
    if (availableSlots < minAvailableSlots) {
      reasons.push(`Insufficient slots (${availableSlots}/${minAvailableSlots})`);
    }
    if (cpuUsage > maxCpuUsage) {
      reasons.push(`High CPU usage (${cpuUsage.toFixed(1)}%)`);
    }
    if (memoryUsage > maxMemoryUsage) {
      reasons.push(`High memory usage (${memoryUsage.toFixed(1)}%)`);
    }
    
    return `Resources unavailable: ${reasons.join(', ')}`;
  };

  const warningMessage = resourceWarningMessage || 
    (resourceStatus && !resourceStatus.isAvailable
      ? `Insufficient resources: ${resourceStatus.availableSlots} slots available, CPU: ${resourceStatus.cpuUsage.toFixed(1)}%, Memory: ${resourceStatus.memoryUsage.toFixed(1)}%`
      : undefined);

  return (
    <Box>
      {warningMessage && !isResourceAvailable && (
        <Alert severity="warning" sx={{ mb: 1 }}>
          <AlertTitle>Resource Warning</AlertTitle>
          {warningMessage}
        </Alert>
      )}
      
      <Tooltip title={showResourceStatus ? getResourceTooltip() : ''} arrow>
        <span>
          <Button
            {...buttonProps}
            disabled={isDisabled}
            onClick={handleClick}
            loading={isLoading}
          />
        </span>
      </Tooltip>
    </Box>
  );
};

ResourceManagedButton.displayName = 'ResourceManagedButton';

export default ResourceManagedButton;

