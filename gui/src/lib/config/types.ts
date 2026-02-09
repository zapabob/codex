// SPDX-License-Identifier: Apache-2.0

export interface GitHubConfig {
  token?: string;
  owner: string;
  repo: string;
  webhook_secret?: string;
}

export interface WebSocketConfig {
  server_url: string;
  reconnect_interval: number;
  max_reconnect_attempts: number;
}

export interface I18nConfig {
  default_locale: string;
  supported_locales: string[];
}

export interface VRConfig {
  default_mode: "auto" | "vr" | "ar" | "desktop";
  haptic_enabled: boolean;
  hand_tracking: boolean;
  haptic_intensity: number;
}

export interface PerformanceConfig {
  target_fps: number;
  render_scale: number;
  lod_enabled: boolean;
  max_visible_commits: number;
  particles_enabled: boolean;
}

export interface CyberpunkConfig {
  bloom_intensity: number;
  chromatic_aberration: number;
  noise_opacity: number;
  vignette_darkness: number;
}

export interface Quest2Config {
  target_fps: number;
  refresh_rate: number;
  fov: number;
  render_scale: number;
  asw_enabled: boolean;
}

export interface SteamVRConfig {
  target_fps: number;
  refresh_rate: number;
  async_repro: boolean;
  motion_prediction: boolean;
}

export interface VirtualDesktopConfig {
  target_fps: number;
  render_scale: number;
  bitrate: number;
  codec: string;
}

export interface LoggingConfig {
  level: "debug" | "info" | "warn" | "error";
  console_enabled: boolean;
  file_enabled: boolean;
}

export interface Config {
  github: GitHubConfig;
  websocket: WebSocketConfig;
  i18n: I18nConfig;
  vr: VRConfig;
  performance: PerformanceConfig;
  cyberpunk: CyberpunkConfig;
  quest2: Quest2Config;
  steamvr: SteamVRConfig;
  virtual_desktop: VirtualDesktopConfig;
  logging: LoggingConfig;
}
