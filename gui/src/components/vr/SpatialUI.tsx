'use client';

import React, { useEffect, useState } from 'react';
import { Box, Card as MuiCard } from '@mui/material';
import { motion, AnimatePresence } from 'framer-motion';

/**
 * Spatial UI Component for VR/AR
 * 
 * Places UI elements in 3D space with hand tracking interaction
 * Enhanced immersion with spatial placement
 */
export interface SpatialUIProps {
  /** UI elements to place in space */
  elements: Array<{
    id: string;
    content: React.ReactNode;
    position: { x: number; y: number; z: number };
    rotation?: { x: number; y: number; z: number };
    scale?: number;
  }>;
  /** Hand position for interaction */
  handPosition?: { x: number; y: number; z: number };
  /** Interaction distance threshold */
  interactionDistance?: number;
  /** Callback when element is interacted with */
  onElementInteract?: (elementId: string) => void;
}

export const SpatialUI: React.FC<SpatialUIProps> = ({
  elements,
  handPosition,
  interactionDistance = 0.5,
  onElementInteract,
}) => {
  const [interactingElements, setInteractingElements] = useState<Set<string>>(new Set());

  useEffect(() => {
    if (!handPosition) return;

    // Check which elements are within interaction distance
    const interacting = new Set<string>();
    
    elements.forEach((element) => {
      const distance = Math.sqrt(
        Math.pow(element.position.x - handPosition.x, 2) +
        Math.pow(element.position.y - handPosition.y, 2) +
        Math.pow(element.position.z - handPosition.z, 2)
      );

      if (distance < interactionDistance) {
        interacting.add(element.id);
        onElementInteract?.(element.id);
      }
    });

    setInteractingElements(interacting);
  }, [handPosition, elements, interactionDistance, onElementInteract]);

  return (
    <Box
      sx={{
        position: 'fixed',
        top: 0,
        left: 0,
        width: '100%',
        height: '100%',
        pointerEvents: 'none',
        zIndex: 100,
      }}
    >
      <AnimatePresence>
        {elements.map((element) => {
          const isInteracting = interactingElements.has(element.id);
          const scale = element.scale || 1.0;

          // Convert 3D position to 2D screen coordinates
          // This is a simplified projection - in VR, you'd use proper 3D rendering
          const screenX = (element.position.x + 1) * 50; // Normalize to 0-100%
          const screenY = (element.position.y + 1) * 50;

          return (
            <motion.div
              key={element.id}
              initial={{ opacity: 0, scale: 0.8 }}
              animate={{
                opacity: isInteracting ? 1 : 0.7,
                scale: isInteracting ? scale * 1.1 : scale,
                x: `${screenX}%`,
                y: `${screenY}%`,
              }}
              exit={{ opacity: 0, scale: 0.8 }}
              transition={{ duration: 0.2 }}
              style={{
                position: 'absolute',
                transform: `translate(-50%, -50%)`,
                pointerEvents: 'auto',
              }}
            >
              <MuiCard
                sx={{
                  background: isInteracting
                    ? 'rgba(0, 255, 255, 0.2)'
                    : 'rgba(0, 0, 0, 0.8)',
                  border: `1px solid ${isInteracting ? '#00ffff' : 'rgba(0, 255, 255, 0.3)'}`,
                  boxShadow: isInteracting
                    ? '0 0 30px rgba(0, 255, 255, 0.6)'
                    : '0 0 10px rgba(0, 255, 255, 0.3)',
                  padding: 2,
                  minWidth: 200,
                  transition: 'all 0.2s',
                }}
              >
                {element.content}
              </MuiCard>
            </motion.div>
          );
        })}
      </AnimatePresence>
    </Box>
  );
};

SpatialUI.displayName = 'SpatialUI';

export default SpatialUI;

