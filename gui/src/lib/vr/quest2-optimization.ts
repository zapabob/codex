// SPDX-License-Identifier: Apache-2.0

import {
  loadConfig,
  Quest2Config,
  SteamVRConfig,
  VirtualDesktopConfig,
} from "@/lib/config";

export interface VROptimizationSettings {
  platform: "quest2" | "quest3" | "steamvr" | "virtual_desktop" | "unknown";
  targetFps: number;
  renderScale: number;
  fov: number;
  refreshRate: number;
  aswEnabled: boolean;
  asyncReprojection: boolean;
  motionPrediction: boolean;
  bitrate: number;
  codec: string;
}

export interface PerformanceProfile {
  low: VROptimizationSettings;
  medium: VROptimizationSettings;
  high: VROptimizationSettings;
}

function detectPlatform():
  | "quest2"
  | "quest3"
  | "steamvr"
  | "virtual_desktop"
  | "unknown" {
  const ua = navigator.userAgent;

  if (ua.includes("Quest 2")) return "quest2";
  if (ua.includes("Quest 3")) return "quest3";
  if (ua.includes("Quest") && !ua.includes("2") && !ua.includes("3"))
    return "quest2";
  if (ua.includes("SteamVR") || ua.includes("OpenXR")) return "steamvr";
  if (ua.includes("Virtual Desktop") || ua.includes("Parsec"))
    return "virtual_desktop";

  return "unknown";
}

export function getQuest2Optimizations(): Quest2Config {
  const config = loadConfig();
  return config.quest2;
}

export function getSteamVROptimizations(): SteamVRConfig {
  const config = loadConfig();
  return config.steamvr;
}

export function getVirtualDesktopOptimizations(): VirtualDesktopConfig {
  const config = loadConfig();
  return config.virtual_desktop;
}

export function getVROptimizations(): VROptimizationSettings {
  const platform = detectPlatform();
  const config = loadConfig();

  const baseSettings: VROptimizationSettings = {
    platform,
    targetFps: config.performance.target_fps,
    renderScale: config.performance.render_scale,
    fov: 90,
    refreshRate: 72,
    aswEnabled: false,
    asyncReprojection: false,
    motionPrediction: false,
    bitrate: 50,
    codec: "h264",
  };

  switch (platform) {
    case "quest2":
    case "quest3":
      return {
        ...baseSettings,
        targetFps: config.quest2.target_fps,
        renderScale: config.quest2.render_scale,
        fov: config.quest2.fov,
        refreshRate: config.quest2.refresh_rate,
        aswEnabled: config.quest2.asw_enabled,
      };

    case "steamvr":
      return {
        ...baseSettings,
        targetFps: config.steamvr.target_fps,
        refreshRate: config.steamvr.refresh_rate,
        asyncReprojection: config.steamvr.async_repro,
        motionPrediction: config.steamvr.motion_prediction,
      };

    case "virtual_desktop":
      return {
        ...baseSettings,
        targetFps: config.virtual_desktop.target_fps,
        renderScale: config.virtual_desktop.render_scale,
        bitrate: config.virtual_desktop.bitrate,
        codec: config.virtual_desktop.codec,
      };

    default:
      return baseSettings;
  }
}

export function getPerformanceProfiles(): PerformanceProfile {
  const settings = getVROptimizations();

  return {
    low: {
      ...settings,
      targetFps: 45,
      renderScale: 0.5,
      aswEnabled: true,
    },
    medium: {
      ...settings,
      targetFps: 60,
      renderScale: 0.75,
    },
    high: {
      ...settings,
      targetFps: settings.targetFps,
      renderScale: 1.0,
    },
  };
}

export function selectPerformanceProfile(
  targetFps: number,
  currentFps: number,
): "low" | "medium" | "high" {
  if (currentFps < targetFps * 0.7) return "low";
  if (currentFps < targetFps * 0.9) return "medium";
  return "high";
}

export function applyVROptimizations(
  canvas: HTMLCanvasElement,
  settings: VROptimizationSettings,
): void {
  canvas.width =
    canvas.clientWidth * settings.renderScale * window.devicePixelRatio;
  canvas.height =
    canvas.clientHeight * settings.renderScale * window.devicePixelRatio;

  console.log(`[VR Optimization] Applied settings for ${settings.platform}:`, {
    width: canvas.width,
    height: canvas.height,
    renderScale: settings.renderScale,
    targetFps: settings.targetFps,
  });
}

export function getFrameTimeBudget(targetFps: number): number {
  return 1000 / targetFps;
}

export function shouldSkipFrame(
  currentFps: number,
  targetFps: number,
  frameTime: number,
): boolean {
  const budget = getFrameTimeBudget(targetFps);
  return frameTime > budget * 1.5;
}
