// SPDX-License-Identifier: Apache-2.0

import fs from "fs";
import path from "path";
import toml from "toml";
import { type Config } from "./types";

let cachedConfig: Config | null = null;
let configPath: string = "";

function findConfigFile(): string {
  const possiblePaths = [
    path.join(process.cwd(), "config.toml"),
    path.join(process.cwd(), "gui", "config.toml"),
    path.join(__dirname, "..", "..", "config.toml"),
    path.join(__dirname, "..", "config.toml"),
  ];

  for (const p of possiblePaths) {
    if (fs.existsSync(p)) {
      return p;
    }
  }

  throw new Error(
    `config.toml not found in any of: ${possiblePaths.join(", ")}`,
  );
}

function substituteEnvVars(content: string): string {
  return content.replace(/\$\{(\w+)\}/g, (_, envVar) => {
    return process.env[envVar] || "";
  });
}

function validateConfig(config: Record<string, unknown>): Config {
  const requiredSections = ["github", "i18n", "performance", "vr"];

  for (const section of requiredSections) {
    if (!(section in config)) {
      throw new Error(`Missing required config section: ${section}`);
    }
  }

  return config as Config;
}

export function loadConfig(): Config {
  if (cachedConfig) {
    return cachedConfig;
  }

  try {
    configPath = findConfigFile();

    const configFile = fs.readFileSync(configPath, "utf-8");
    const configWithEnv = substituteEnvVars(configFile);
    const parsedConfig = toml.parse(configWithEnv);

    cachedConfig = validateConfig(parsedConfig);

    console.log(`[Config] Loaded from: ${configPath}`);

    return cachedConfig;
  } catch (error) {
    console.error("[Config] Failed to load config:", error);
    throw error;
  }
}

export function getConfigPath(): string {
  if (!configPath) {
    configPath = findConfigFile();
  }
  return configPath;
}

export function reloadConfig(): Config {
  cachedConfig = null;
  return loadConfig();
}

export function updateConfigSection<K extends keyof Config>(
  section: K,
  updates: Partial<Config[K]>,
): void {
  if (!cachedConfig) {
    loadConfig();
  }

  if (cachedConfig && section in cachedConfig) {
    (cachedConfig[section] as Record<string, unknown>) = {
      ...(cachedConfig[section] as Record<string, unknown>),
      ...updates,
    };
    console.log(`[Config] Updated section: ${section}`);
  }
}

export function getGitHubToken(): string | undefined {
  const config = loadConfig();
  return config.github.token;
}

export function getGitHubRepo(): { owner: string; repo: string } {
  const config = loadConfig();
  return {
    owner: config.github.owner,
    repo: config.github.repo,
  };
}

export function getVRSettings(): Config["vr"] {
  const config = loadConfig();
  return config.vr;
}

export function getPerformanceSettings(): Config["performance"] {
  const config = loadConfig();
  return config.performance;
}

export function getCyberpunkSettings(): Config["cyberpunk"] {
  const config = loadConfig();
  return config.cyberpunk;
}

export function getI18nSettings(): Config["i18n"] {
  const config = loadConfig();
  return config.i18n;
}
