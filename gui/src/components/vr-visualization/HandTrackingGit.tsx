// SPDX-License-Identifier: Apache-2.0

"use client";

export type Gesture =
  | "none"
  | "pinch"
  | "grab"
  | "point"
  | "thumbs_up"
  | "thumbs_down"
  | "open";

export interface HandGestureEvent {
  handedness: "left" | "right";
  gesture: Gesture;
  position: unknown;
  timestamp: number;
}

interface HandTrackingGitProps {
  onGesture?: (event: HandGestureEvent) => void;
  onPinch?: (position: unknown) => void;
  onThumbsUp?: () => void;
}

export function HandTrackingGit(_props: HandTrackingGitProps) {
  return null;
}

export function useHapticFeedback() {
  const triggerHaptic = async (
    _handedness: "left" | "right",
    _intensity: number = 0.5,
    _duration: number = 100,
  ) => {
    console.log("Haptic feedback triggered");
  };

  const triggerConflictPulse = async () => {
    await triggerHaptic("right", 0.8, 300);
  };

  const triggerMergePulse = async () => {
    await triggerHaptic("right", 1.0, 500);
  };

  const triggerSelectPulse = async () => {
    await triggerHaptic("right", 0.5, 30);
  };

  return {
    triggerHaptic,
    triggerConflictPulse,
    triggerMergePulse,
    triggerSelectPulse,
  };
}
