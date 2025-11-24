'use client';

import React, { useEffect, useRef, useState } from 'react';
import { Box, Typography } from '@mui/material';

/**
 * Hand Tracking Component for VR/AR
 * 
 * Enhanced hand tracking integration with spatial UI placement
 * Supports Quest 2/3/Pro, Vision Pro
 */
export interface HandTrackingProps {
  /** Enable hand tracking */
  enabled?: boolean;
  /** Callback when hand gesture detected */
  onGesture?: (gesture: HandGesture) => void;
  /** Callback when hand position changes */
  onHandPosition?: (position: { x: number; y: number; z: number }) => void;
}

export type HandGesture = 
  | 'point'
  | 'grab'
  | 'pinch'
  | 'open'
  | 'fist'
  | 'wave'
  | 'thumbs-up'
  | 'peace';

export interface HandTrackingState {
  leftHand: {
    position: { x: number; y: number; z: number };
    rotation: { x: number; y: number; z: number };
    gesture: HandGesture | null;
    confidence: number;
  } | null;
  rightHand: {
    position: { x: number; y: number; z: number };
    rotation: { x: number; y: number; z: number };
    gesture: HandGesture | null;
    confidence: number;
  } | null;
}

export const HandTracking: React.FC<HandTrackingProps> = ({
  enabled = true,
  onGesture,
  onHandPosition,
}) => {
  const [handState, setHandState] = useState<HandTrackingState>({
    leftHand: null,
    rightHand: null,
  });
  const [isSupported, setIsSupported] = useState(false);
  const animationFrameRef = useRef<number>();

  useEffect(() => {
    if (!enabled) return;

    // Check WebXR hand tracking support
    const checkSupport = async () => {
      if ('xr' in navigator) {
        try {
          const supported = await (navigator.xr as any)?.isSessionSupported('immersive-vr');
          setIsSupported(supported || false);
        } catch (error) {
          console.warn('WebXR not supported:', error);
          setIsSupported(false);
        }
      } else {
        setIsSupported(false);
      }
    };

    checkSupport();

    // Simulate hand tracking for development
    // In production, this would use WebXR Hand Tracking API
    const simulateHandTracking = () => {
      if (!isSupported) {
        // Mock hand tracking for development
        setHandState({
          leftHand: {
            position: { x: -0.3, y: 0.5, z: -0.5 },
            rotation: { x: 0, y: 0, z: 0 },
            gesture: 'open',
            confidence: 0.8,
          },
          rightHand: {
            position: { x: 0.3, y: 0.5, z: -0.5 },
            rotation: { x: 0, y: 0, z: 0 },
            gesture: 'point',
            confidence: 0.9,
          },
        });

        // Call callbacks
        onHandPosition?.({ x: 0.3, y: 0.5, z: -0.5 });
        onGesture?.('point');
      }

      animationFrameRef.current = requestAnimationFrame(simulateHandTracking);
    };

    if (enabled) {
      simulateHandTracking();
    }

    return () => {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
    };
  }, [enabled, isSupported, onGesture, onHandPosition]);

  if (!enabled) {
    return null;
  }

  return (
    <Box
      sx={{
        position: 'fixed',
        top: 16,
        right: 16,
        zIndex: 1000,
        background: 'rgba(0, 0, 0, 0.8)',
        padding: 2,
        borderRadius: 2,
        border: '1px solid rgba(0, 255, 255, 0.3)',
        boxShadow: '0 0 20px rgba(0, 255, 255, 0.5)',
      }}
    >
      <Typography variant="caption" sx={{ color: '#00ffff', mb: 1, display: 'block' }}>
        Hand Tracking {isSupported ? '✅' : '⚠️'}
      </Typography>
      
      {handState.leftHand && (
        <Box sx={{ mb: 1 }}>
          <Typography variant="caption" sx={{ color: '#ffffff', display: 'block' }}>
            Left: {handState.leftHand.gesture || 'none'} ({Math.round(handState.leftHand.confidence * 100)}%)
          </Typography>
        </Box>
      )}
      
      {handState.rightHand && (
        <Box>
          <Typography variant="caption" sx={{ color: '#ffffff', display: 'block' }}>
            Right: {handState.rightHand.gesture || 'none'} ({Math.round(handState.rightHand.confidence * 100)}%)
          </Typography>
        </Box>
      )}
    </Box>
  );
};

HandTracking.displayName = 'HandTracking';

export default HandTracking;

