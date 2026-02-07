'use client';

import React, { useEffect, useRef, useState } from 'react';
import { motion } from 'framer-motion';
import { GitBranch, Play, Pause, RotateCcw } from 'lucide-react';
import {
  Box,
  Typography,
  Chip,
  IconButton,
  Switch,
  FormControlLabel,
  Slider,
  Paper,
} from '@mui/material';
import * as THREE from 'three';
import { useVirtualDesktopOptimizer } from '../../utils/virtualdesktop-optimizer';
import type { Git4DCommitData } from '../../lib/types/three';
import { apiClient } from '../../lib/api/client';
import type { Git4DSessionInfo } from '../../lib/types';

/**
 * Git4DVisualization Component Props
 */
export interface Git4DVisualizationProps {
  /** Visualization mode: 'desktop', 'vr', or 'ar' */
  mode?: 'desktop' | 'vr' | 'ar';
  /** Repository path (optional, defaults to current directory) */
  repositoryPath?: string;
  /** Session ID from API (optional) */
  sessionId?: string;
}

/**
 * Git4DVisualization Component
 *
 * 4D Git repository visualization with time axis
 * Supports Quest 2/3 VR integration and Windows 11 25H2 AI acceleration
 */
export const Git4DVisualization: React.FC<Git4DVisualizationProps> = ({ 
  mode = 'desktop',
  repositoryPath,
  sessionId 
}) => {
  const mountRef = useRef<HTMLDivElement>(null);
  const sceneRef = useRef<THREE.Scene>();
  const rendererRef = useRef<THREE.WebGLRenderer>();
  const cameraRef = useRef<THREE.PerspectiveCamera>();
  const animationFrameRef = useRef<number>();
  // const controlsRef = useRef<any>(null);

  const [isPlaying, setIsPlaying] = useState(false);
  const [timeScale, setTimeScale] = useState(1);
  const [showLabels, setShowLabels] = useState(true);
  const [vrMode, setVrMode] = useState(mode === 'vr');
  const [arMode, setArMode] = useState(mode === 'ar');
  const [windowsAiMode, setWindowsAiMode] = useState(false);
  const [handTrackingEnabled, setHandTrackingEnabled] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  // const [error, setError] = useState<string | null>(null);
  const [backendStatus, setBackendStatus] = useState<'ok' | 'offline' | 'loading'>('loading');
  const [backendMessage, setBackendMessage] = useState<string | null>(null);
  const [activeSessionId, setActiveSessionId] = useState<string | null>(sessionId ?? null);
  const [sessionPlatform, setSessionPlatform] = useState<string | null>(null);
  const [sessionDeviceName, setSessionDeviceName] = useState<string | null>(null);
  const [sessionList, setSessionList] = useState<Git4DSessionInfo[]>([]);
  const [eventStreamStatus, setEventStreamStatus] = useState<'idle' | 'connecting' | 'open' | 'error'>('idle');
  const [git4dEvents, setGit4dEvents] = useState<Array<{ timestamp: string; payload: string }>>([]);

  // VirtualDesktop optimization
  const { preset, isVD } = useVirtualDesktopOptimizer();

  // Mock git data - replace with real data from backend
  const [commits] = useState(() => generateMockCommits());

  useEffect(() => {
    let cancelled = false;

    const bootstrapBackend = async () => {
      setBackendStatus('loading');
      setBackendMessage(null);

      try {
        const health = await apiClient.getHealth();
        if (cancelled) return;
        setBackendStatus(health.status === 'ok' ? 'ok' : 'offline');
      } catch (err) {
        if (cancelled) return;
        setBackendStatus('offline');
        setBackendMessage(err instanceof Error ? err.message : 'Backend unreachable');
        return;
      }

      if (sessionId) {
        setActiveSessionId(sessionId);
      } else {
        setIsLoading(true);
        try {
          const response = await apiClient.launchGit4D({
            mode,
            repositoryPath: repositoryPath || '.',
            virtualDesktop: isVD && (mode === 'vr' || mode === 'ar'),
          });
          if (cancelled) return;
          setActiveSessionId(response.sessionId);
          setSessionPlatform(response.platform ?? null);
          setSessionDeviceName(response.deviceName ?? null);
        } catch (err) {
          if (cancelled) return;
          setError(err instanceof Error ? err.message : 'Failed to launch Git4D backend session');
        } finally {
          if (!cancelled) {
            setIsLoading(false);
          }
        }
      }

      try {
        const sessions = await apiClient.getGit4DSessions();
        if (!cancelled) {
          setSessionList(sessions);
        }
      } catch (err) {
        if (!cancelled) {
          setBackendMessage(err instanceof Error ? err.message : 'Failed to list sessions');
        }
      }
    };

    bootstrapBackend();

    return () => {
      cancelled = true;
    };
  }, [mode, repositoryPath, sessionId, isVD]);

  useEffect(() => {
    if (!activeSessionId) {
      setEventStreamStatus('idle');
      return;
    }

    const apiUrl = import.meta.env.VITE_API_URL || 'http://localhost:8787';
    const source = new EventSource(`${apiUrl}/api/visualization/git4d/${activeSessionId}/events`);
    setEventStreamStatus('connecting');

    source.onopen = () => {
      setEventStreamStatus('open');
    };

    source.onerror = () => {
      setEventStreamStatus('error');
    };

    source.onmessage = (event) => {
      const timestamp = new Date().toISOString();
      let payload = event.data;
      try {
        payload = JSON.stringify(JSON.parse(event.data));
      } catch {
        // keep raw payload
      }

      setGit4dEvents((prev) => {
        const next = [{ timestamp, payload }, ...prev];
        return next.slice(0, 5);
      });
    };

    return () => {
      source.close();
      setEventStreamStatus('idle');
    };
  }, [activeSessionId]);

  useEffect(() => {
    if (!mountRef.current) return;

    // Initialize Three.js scene
    const scene = new THREE.Scene();
    scene.background = new THREE.Color(0x0a0a0a);
    sceneRef.current = scene;

    const camera = new THREE.PerspectiveCamera(
      75,
      mountRef.current.clientWidth / mountRef.current.clientHeight,
      0.1,
      1000
    );
    camera.position.set(0, 0, 5);
    cameraRef.current = camera;

    const renderer = new THREE.WebGLRenderer({ antialias: true });
    
    // VirtualDesktop譛驕ｩ蛹・ 繝ｬ繝ｳ繝繝ｪ繝ｳ繧ｰ隗｣蜒丞ｺｦ繧定ｪｿ謨ｴ
    const renderScale = isVD && (vrMode || arMode) ? preset.renderScale : 1.0;
    const width = mountRef.current.clientWidth * renderScale;
    const height = mountRef.current.clientHeight * renderScale;
    renderer.setSize(width, height);
    renderer.setPixelRatio(Math.min(window.devicePixelRatio, preset.targetFps / 60)); // FPS蛻ｶ髯舌↓蜷医ｏ縺帙※隱ｿ謨ｴ
    renderer.shadowMap.enabled = true;
    renderer.shadowMap.type = THREE.PCFSoftShadowMap;
    rendererRef.current = renderer;

    mountRef.current.appendChild(renderer.domElement);
    
    // VirtualDesktop讀懷・譎ゅ・譛驕ｩ蛹夜←逕ｨ
    if (isVD && (vrMode || arMode)) {
      console.log('VirtualDesktop detected - applying optimizations', preset);
      // 繧ｹ繝医Μ繝ｼ繝溘Φ繧ｰ譛驕ｩ蛹悶ｒ驕ｩ逕ｨ
      if (renderer.domElement) {
        renderer.domElement.style.imageRendering = preset.renderScale < 1 ? 'pixelated' : 'auto';
      }
    }

    // Add lighting
    const ambientLight = new THREE.AmbientLight(0x404040, 0.4);
    scene.add(ambientLight);

    const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8);
    directionalLight.position.set(10, 10, 5);
    directionalLight.castShadow = true;
    scene.add(directionalLight);

    // Create commit visualization
    createCommitVisualization(scene, commits);

    // Animation loop with FPS limiting for VirtualDesktop
    let lastFrameTime = 0;
    const targetFrameTime = isVD && (vrMode || arMode) ? 1000 / preset.targetFps : 0; // 0 = no limit
    
    const animate = (currentTime: number) => {
      animationFrameRef.current = requestAnimationFrame(animate);

      // FPS limit for VirtualDesktop 
      if (targetFrameTime > 0) {
        const elapsed = currentTime - lastFrameTime;
        if (elapsed < targetFrameTime) {
          return; // Skip frame to maintain target FPS
        }
        lastFrameTime = currentTime;
      }

      if (isPlaying) {
        // Rotate camera for dynamic view
        camera.position.x = Math.cos(Date.now() * 0.001 * timeScale) * 8;
        camera.position.z = Math.sin(Date.now() * 0.001 * timeScale) * 8;
        camera.lookAt(0, 0, 0);
      }

      renderer.render(scene, camera);
    };
    animate(0);

    // Handle resize
    const handleResize = () => {
      if (!mountRef.current || !renderer) return;
      camera.aspect = mountRef.current.clientWidth / mountRef.current.clientHeight;
      camera.updateProjectionMatrix();
      
      // VirtualDesktop譛驕ｩ蛹・ 繝ｬ繝ｳ繝繝ｪ繝ｳ繧ｰ隗｣蜒丞ｺｦ繧定ｪｿ謨ｴ
      const renderScale = isVD && (vrMode || arMode) ? preset.renderScale : 1.0;
      const width = mountRef.current.clientWidth * renderScale;
      const height = mountRef.current.clientHeight * renderScale;
      renderer.setSize(width, height);
    };

    window.addEventListener('resize', handleResize);

    const currentMount = mountRef.current;
    return () => {
      window.removeEventListener('resize', handleResize);
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
      if (currentMount && renderer.domElement) {
        currentMount.removeChild(renderer.domElement);
      }
      renderer.dispose();
    };
  }, [commits, isPlaying, timeScale, isVD, vrMode, arMode, preset, createCommitVisualization]);

  const createCommitVisualization = React.useCallback((scene: THREE.Scene, commits: Git4DCommitData[]) => {
    commits.forEach((commit, _index) => {
      // Create commit node
      const geometry = new THREE.SphereGeometry(0.1, 16, 16);
      const material = new THREE.MeshPhongMaterial({
        color: new THREE.Color().setHSL((commit.branch || 0) / 10, 0.8, 0.6),
        emissive: new THREE.Color(0x111111),
      });

      const sphere = new THREE.Mesh(geometry, material);
      sphere.position.set(
        commit.x,
        commit.y,
        commit.z
      );
      sphere.userData = { commit, _index };
      scene.add(sphere);

      // Add connection lines to parent commits
      if (commit.parents && commit.parents.length > 0) {
        commit.parents.forEach((parentId: string) => {
          const parentCommit = commits.find(c => c.id === parentId);
          if (parentCommit) {
            const lineGeometry = new THREE.BufferGeometry().setFromPoints([
              new THREE.Vector3(commit.x, commit.y, commit.z),
              new THREE.Vector3(parentCommit.x, parentCommit.y, parentCommit.z),
            ]);

            const lineMaterial = new THREE.LineBasicMaterial({
              color: 0x666666,
              transparent: true,
              opacity: 0.6,
            });

            const line = new THREE.Line(lineGeometry, lineMaterial);
            scene.add(line);
          }
        });
      }

      // Add labels if enabled
      if (showLabels) {
        // Label implementation would go here
      }
    });
  }, [showLabels]);

  const generateMockCommits = () => {
    const commits = [];
    const branches = ['main', 'feature/auth', 'feature/ui', 'hotfix/security'];

    for (let i = 0; i < 100; i++) {
      commits.push({
        id: `commit-${i}`,
        message: `Commit ${i}: ${Math.random() > 0.5 ? 'Add' : 'Fix'} ${['feature', 'bug', 'docs', 'test'][Math.floor(Math.random() * 4)]}`,
        author: ['Alice', 'Bob', 'Charlie', 'Diana'][Math.floor(Math.random() * 4)],
        timestamp: Date.now() - (i * 24 * 60 * 60 * 1000), // One commit per day
        branch: Math.floor(Math.random() * branches.length),
        x: (Math.random() - 0.5) * 10,
        y: (Math.random() - 0.5) * 10,
        z: (Math.random() - 0.5) * 10,
        parents: i > 0 ? [`commit-${Math.max(0, i - Math.floor(Math.random() * 3) - 1)}`] : [],
        filesChanged: Math.floor(Math.random() * 20),
        insertions: Math.floor(Math.random() * 500),
        deletions: Math.floor(Math.random() * 300),
      });
    }

    return commits as Git4DCommitData[];
  };

  const resetView = () => {
    if (cameraRef.current) {
      cameraRef.current.position.set(0, 0, 5);
      cameraRef.current.lookAt(0, 0, 0);
    }
  };

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -20 }}
    >
      <Paper
        elevation={3}
        sx={{
          p: 2,
          m: 2,
          background: 'linear-gradient(135deg, #1a1a2e 0%, #16213e 100%)',
          color: 'white',
        }}
      >
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, mb: 1, flexWrap: 'wrap' }}>
          <GitBranch size={24} />
          <Typography variant="h6">Git4D Visualization</Typography>
          {isVD && (vrMode || arMode) && (
            <Chip 
              label={`VirtualDesktop: ${preset.name}`} 
              color="info" 
              size="small"
              sx={{ ml: 1 }}
            />
          )}
          <Chip
            label={`Backend: ${
              backendStatus === 'ok'
                ? 'OK'
                : backendStatus === 'loading'
                  ? 'Checking…'
                  : 'Offline'
            }`}
            color={backendStatus === 'ok' ? 'success' : backendStatus === 'loading' ? 'warning' : 'error'}
            size="small"
            sx={{ ml: 1 }}
          />
          {activeSessionId && (
            <Chip
              label={`Session: ${activeSessionId.slice(0, 8)}…`}
              color="primary"
              size="small"
              sx={{ ml: 1 }}
            />
          )}
          {sessionPlatform && (
            <Chip
              label={`Platform: ${sessionPlatform}${sessionDeviceName ? ` (${sessionDeviceName})` : ''}`}
              color="success"
              size="small"
              sx={{ ml: 1 }}
            />
          )}
          {sessionList.length > 0 && (
            <Chip
              label={`Sessions: ${sessionList.length}`}
              color="secondary"
              size="small"
              sx={{ ml: 1 }}
            />
          )}
          <Box sx={{ flex: 1 }} />
          {isLoading && (
            <Typography variant="caption" color="warning.main">
              Launching session…
            </Typography>
          )}
          <FormControlLabel
            control={
              <Switch
                checked={showLabels}
                onChange={(e) => setShowLabels(e.target.checked)}
                color="primary"
              />
            }
            label="Labels"
          />
          <FormControlLabel
            control={
              <Switch
                checked={vrMode}
                onChange={(e) => setVrMode(e.target.checked)}
                color="secondary"
              />
            }
            label="VR Mode (Quest 2/3)"
          />
          <FormControlLabel
            control={
              <Switch
                checked={arMode}
                onChange={(e) => setArMode(e.target.checked)}
                color="info"
              />
            }
            label="AR Mode"
          />
          <FormControlLabel
            control={
              <Switch
                checked={windowsAiMode}
                onChange={(e) => setWindowsAiMode(e.target.checked)}
                color="success"
              />
            }
            label="Windows AI (25H2)"
          />
          <FormControlLabel
            control={
              <Switch
                checked={handTrackingEnabled}
                onChange={(e) => setHandTrackingEnabled(e.target.checked)}
                color="warning"
              />
            }
            label="Hand Tracking"
          />
        </Box>
        {backendMessage && (
          <Typography variant="caption" color="error" sx={{ display: 'block', mb: 2 }}>
            Backend note: {backendMessage}
          </Typography>
        )}

        {/* Controls */}
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, mb: 2 }}>
          <IconButton
            onClick={() => setIsPlaying(!isPlaying)}
            color="primary"
            sx={{ bgcolor: 'rgba(255,255,255,0.1)' }}
          >
            {isPlaying ? <Pause size={20} /> : <Play size={20} />}
          </IconButton>

          <IconButton
            onClick={resetView}
            color="primary"
            sx={{ bgcolor: 'rgba(255,255,255,0.1)' }}
          >
            <RotateCcw size={20} />
          </IconButton>

          <Typography variant="body2" sx={{ minWidth: 60 }}>
            Speed:
          </Typography>
          <Slider
            value={timeScale}
            onChange={(_, value) => setTimeScale(value as number)}
            min={0.1}
            max={3}
            step={0.1}
            sx={{
              width: 100,
              color: 'primary.main',
              '& .MuiSlider-thumb': {
                bgcolor: 'primary.main',
              },
            }}
          />
          <Typography variant="body2">{timeScale.toFixed(1)}x</Typography>
        </Box>

        {/* 3D Visualization Canvas */}
        <Box
          ref={mountRef}
          sx={{
            width: '100%',
            height: 600,
            borderRadius: 2,
            overflow: 'hidden',
            bgcolor: '#000',
          }}
        />

        {/* Stats */}
        <Box sx={{ mt: 2, display: 'flex', gap: 4, flexWrap: 'wrap' }}>
          <Typography variant="body2">
            Commits: {commits.length}
          </Typography>
          <Typography variant="body2">
            Branches: {new Set(commits.map(c => c.branch)).size}
          </Typography>
          <Typography variant="body2">
            Contributors: {new Set(commits.map(c => c.author)).size}
          </Typography>
          {isVD && (vrMode || arMode) && (
            <>
              <Typography variant="body2">
                Quality: {preset.name}
              </Typography>
              <Typography variant="body2">
                Target FPS: {preset.targetFps}
              </Typography>
              <Typography variant="body2">
                Render Scale: {preset.renderScale.toFixed(2)}x
              </Typography>
            </>
          )}
        </Box>

        {/* Git4D SSE Events */}
        <Box sx={{ mt: 2 }}>
          <Typography variant="subtitle2" sx={{ mb: 1 }}>
            Git4D Events (SSE)
          </Typography>
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, mb: 1, flexWrap: 'wrap' }}>
            <Chip
              label={`SSE: ${
                eventStreamStatus === 'open'
                  ? 'Connected'
                  : eventStreamStatus === 'connecting'
                    ? 'Connecting'
                    : eventStreamStatus === 'error'
                      ? 'Error'
                      : 'Idle'
              }`}
              color={
                eventStreamStatus === 'open'
                  ? 'success'
                  : eventStreamStatus === 'connecting'
                    ? 'warning'
                    : eventStreamStatus === 'error'
                      ? 'error'
                      : 'default'
              }
              size="small"
            />
            {activeSessionId && (
              <Typography variant="caption" color="text.secondary">
                Session {activeSessionId.slice(0, 8)}…
              </Typography>
            )}
          </Box>
          {git4dEvents.length === 0 ? (
            <Typography variant="body2" color="text.secondary">
              No events received yet.
            </Typography>
          ) : (
            <Box sx={{ display: 'flex', flexDirection: 'column', gap: 0.5 }}>
              {git4dEvents.map((evt, idx) => (
                <Typography key={`${evt.timestamp}-${idx}`} variant="caption" color="text.secondary">
                  [{evt.timestamp}] {evt.payload}
                </Typography>
              ))}
            </Box>
          )}
        </Box>
      </Paper>
    </motion.div>
  );
};
