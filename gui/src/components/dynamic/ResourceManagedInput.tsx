import React, { useEffect, useState, useCallback } from 'react';
import { Input, InputProps } from '../atoms/Input';
import { Alert, AlertTitle, Box, Tooltip } from '@mui/material';
import { CodexAPIClient } from '../../lib/api/client';

export interface ResourceManagedInputProps extends Omit<InputProps, 'type'> {
  /** Input type - must be 'number' for resource validation */
  type?: 'number';
  /** Minimum value allowed */
  min?: number;
  /** Maximum value allowed based on available resources */
  maxBasedOnResources?: boolean;
  /** Maximum CPU usage percentage allowed (0-100) */
  maxCpuUsage?: number;
  /** Maximum memory usage percentage allowed (0-100) */
  maxMemoryUsage?: number;
  /** Show resource status tooltip */
  showResourceStatus?: boolean;
  /** Custom validation error message */
  validationErrorMessage?: string;
  /** Polling interval in milliseconds for resource status */
  pollInterval?: number;
  /** Callback when value exceeds resource limits */
  onResourceLimitExceeded?: (value: number, limit: number) => void;
}

/**
 * ResourceManagedInput - A number input component that validates values
 * against available system resources.
 * 
 * This component monitors CPU and memory usage, and validates input values
 * to ensure they don't exceed available resources.
 */
export const ResourceManagedInput: React.FC<ResourceManagedInputProps> = ({
  type = 'number',
  min = 0,
  maxBasedOnResources = true,
  maxCpuUsage = 90,
  maxMemoryUsage = 90,
  showResourceStatus = true,
  validationErrorMessage,
  pollInterval = 2000,
  onResourceLimitExceeded,
  value: externalValue,
  onChange,
  error: externalError,
  helperText: externalHelperText,
  ...inputProps
}) => {
  const [resourceStatus, setResourceStatus] = useState<{
    availableSlots: number;
    cpuUsage: number;
    memoryUsage: number;
    maxValue: number;
  } | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [internalValue, setInternalValue] = useState<string>(
    externalValue?.toString() ?? ''
  );
  const [validationError, setValidationError] = useState<string | null>(null);

  const apiClient = React.useMemo(() => new CodexAPIClient(), []);

  const checkResourceStatus = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);
      
      const status = await apiClient.getResourceStatus();
      
      const availableSlots = status?.capacity?.availableSlots ?? 0;
      const cpuUsage = status?.stats?.cpuUsagePercent ?? 0;
      const memoryUsage = status?.stats?.memoryUsagePercent ?? 0;
      
      // Calculate max value based on available resources
      const maxValue = maxBasedOnResources 
        ? Math.max(min, availableSlots)
        : Number.MAX_SAFE_INTEGER;

      setResourceStatus({
        availableSlots,
        cpuUsage,
        memoryUsage,
        maxValue,
      });
    } catch (err) {
      console.error('Failed to check resource status:', err);
      setError(err instanceof Error ? err.message : 'Failed to check resources');
      // On error, set conservative limits
      setResourceStatus({
        availableSlots: 1,
        cpuUsage: 0,
        memoryUsage: 0,
        maxValue: min,
      });
    } finally {
      setIsLoading(false);
    }
  }, [apiClient, min, maxBasedOnResources]);

  useEffect(() => {
    // Initial check
    checkResourceStatus();

    // Set up polling
    const interval = setInterval(checkResourceStatus, pollInterval);

    return () => clearInterval(interval);
  }, [checkResourceStatus, pollInterval]);

  useEffect(() => {
    // Sync external value
    if (externalValue !== undefined && externalValue !== null) {
      setInternalValue(externalValue.toString());
    }
  }, [externalValue]);

  const validateValue = useCallback((value: string): string | null => {
    if (!value || value.trim() === '') {
      return null; // Empty is valid (controlled by required prop)
    }

    const numValue = Number(value);
    
    if (isNaN(numValue)) {
      return 'Invalid number';
    }

    if (numValue < min) {
      return `Value must be at least ${min}`;
    }

    if (resourceStatus && maxBasedOnResources) {
      if (numValue > resourceStatus.maxValue) {
        const message = validationErrorMessage || 
          `Value exceeds available resources (max: ${resourceStatus.maxValue} slots)`;
        
        onResourceLimitExceeded?.(numValue, resourceStatus.maxValue);
        return message;
      }
    }

    return null;
  }, [min, resourceStatus, maxBasedOnResources, validationErrorMessage, onResourceLimitExceeded]);

  const handleChange = useCallback(
    (event: React.ChangeEvent<HTMLInputElement>) => {
      const newValue = event.target.value;
      setInternalValue(newValue);
      
      // Validate
      const validationErr = validateValue(newValue);
      setValidationError(validationErr);
      
      // Call external onChange if validation passes or value is empty
      if (!validationErr || newValue.trim() === '') {
        onChange?.(event);
      } else {
        // Create a synthetic event with error state
        const syntheticEvent = {
          ...event,
          target: {
            ...event.target,
            value: newValue,
          },
        } as React.ChangeEvent<HTMLInputElement>;
        onChange?.(syntheticEvent);
      }
    },
    [onChange, validateValue]
  );

  const hasError = externalError || validationError !== null || error !== null;
  const helperText = validationError || externalHelperText || error || 
    (showResourceStatus && resourceStatus
      ? `Available: ${resourceStatus.availableSlots} slots, CPU: ${resourceStatus.cpuUsage.toFixed(1)}%, Memory: ${resourceStatus.memoryUsage.toFixed(1)}%`
      : undefined);

  const getResourceTooltip = () => {
    if (!resourceStatus) return '';
    
    const { availableSlots, cpuUsage, memoryUsage, maxValue } = resourceStatus;
    
    return `Resources: ${availableSlots} slots available, CPU: ${cpuUsage.toFixed(1)}%, Memory: ${memoryUsage.toFixed(1)}%, Max value: ${maxValue}`;
  };

  return (
    <Box>
      {error && (
        <Alert severity="error" sx={{ mb: 1 }}>
          <AlertTitle>Resource Check Error</AlertTitle>
          {error}
        </Alert>
      )}
      
      <Tooltip title={showResourceStatus ? getResourceTooltip() : ''} arrow>
        <span>
          <Input
            {...inputProps}
            type={type}
            value={internalValue}
            onChange={handleChange}
            error={hasError}
            helperText={helperText}
            inputProps={{
              min,
              max: resourceStatus?.maxValue,
              ...inputProps.inputProps,
            }}
          />
        </span>
      </Tooltip>
    </Box>
  );
};

ResourceManagedInput.displayName = 'ResourceManagedInput';

export default ResourceManagedInput;

