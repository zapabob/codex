// SPDX-License-Identifier: Apache-2.0

"use client";

import { useState, useCallback } from "react";
import { Canvas } from "@react-three/fiber";
import { Environment } from "@react-three/drei";
import { loadConfig } from "@/lib/config";
import { useGitEventsWS } from "@/lib/websocket";
import { useTranslation } from "@/lib/i18n";
import { GitTimeline } from "./GitTimeline";
import { ConflictAlerts } from "./ConflictAlerts";
import { CommitParticles } from "./CommitParticles";
import { CyberpunkEffects } from "./CyberpunkEffects";

interface VRGitVisualizationProps {
  owner?: string;
  repo?: string;
  onCommitSelect?: (sha: string) => void;
}

export function VRGitVisualization({
  owner: propOwner,
  repo: propRepo,
  onCommitSelect,
}: VRGitVisualizationProps) {
  const config = loadConfig();
  const { t } = useTranslation();

  const owner = propOwner || config.github.owner;
  const repo = propRepo || config.github.repo;

  const { events, isConnected } = useGitEventsWS(owner, repo);
  const [selectedCommit, setSelectedCommit] = useState<string | null>(null);
  const [showEffects, setShowEffects] = useState(true);
  const [bloomIntensity, setBloomIntensity] = useState(
    config.cyberpunk.bloom_intensity,
  );

  const handleCommitSelect = useCallback(
    (sha: string) => {
      setSelectedCommit(sha);
      onCommitSelect?.(sha);
    },
    [onCommitSelect],
  );

  return (
    <div className="w-full h-screen relative">
      <div className="absolute top-4 right-4 z-10 flex gap-2">
        <div
          className={`px-3 py-1 rounded-full text-sm ${isConnected ? "bg-green-500" : "bg-red-500"}`}
        >
          {isConnected ? "Connected" : t("errors.network_error")}
        </div>
      </div>

      <div className="absolute bottom-4 left-4 z-10 flex gap-2">
        <button
          onClick={() => setShowEffects(!showEffects)}
          className="px-4 py-2 bg-purple-600 text-white rounded-lg"
        >
          {showEffects ? t("visualization.cyberpunk_mode") : "Effects Off"}
        </button>
        <input
          type="range"
          min="0.5"
          max="5"
          step="0.1"
          value={bloomIntensity}
          onChange={(e) => setBloomIntensity(parseFloat(e.target.value))}
          className="w-32"
        />
        <span className="text-white">{bloomIntensity.toFixed(1)}</span>
      </div>

      <Canvas
        gl={{ antialias: true, alpha: false }}
        camera={{ position: [0, 1.6, 3], fov: 75 }}
        dpr={[1, 2]}
      >
        <ambientLight intensity={0.3} />
        <pointLight position={[10, 10, 10]} intensity={0.8} />
        <pointLight position={[-10, 10, -10]} intensity={0.4} color="#00ffff" />

        <color attach="background" args={["#0a0a0f"]} />
        <Environment preset="night" />

        <GitTimeline
          events={events}
          onCommitSelect={handleCommitSelect}
          selectedCommit={selectedCommit}
        />

        <ConflictAlerts events={events} />

        <CommitParticles events={events} />

        {showEffects && <CyberpunkEffects bloomIntensity={bloomIntensity} />}
      </Canvas>
    </div>
  );
}

export default VRGitVisualization;
