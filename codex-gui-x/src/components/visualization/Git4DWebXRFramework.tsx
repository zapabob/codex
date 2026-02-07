'use client';

import React, { useEffect, useRef, useState, useCallback } from 'react';
import { Box, Typography, Paper, Button, IconButton, Alert, Chip } from '@mui/material';
import { motion } from 'framer-motion';
import { Box as BoxIcon, Smartphone, Monitor, RotateCcw, Play, Pause } from 'lucide-react';
import { VRButton, XR, createXRStore, useXR, useXRControllerState } from '@react-three/xr';
import { Canvas, useFrame, useThree } from '@react-three/fiber';
import { OrbitControls, Text, PerspectiveCamera } from '@react-three/drei';
import * as THREE from 'three';

const xrStore = createXRStore();

import { WebXRManager, VRExperience, ARAnchor, HandTrackingData } from '../../lib/xr/webxr-manager';
import { useVirtualDesktopOptimizer } from '../../utils/virtualdesktop-optimizer';
import type { NavigatorXR } from '../../lib/types';
import type { XRErrorLike } from '../../lib/types/webxr';
import type { Git4DCommitData } from '../../lib/types/three';

/**
 * Git4D WebXR Framework Component
 *
 * Sprint 1 Story 2: WebXR基本フレームワークの実装
 * Three.js + WebXR統合と基本3Dシーンの実装
 */
export const Git4DWebXRFramework: React.FC = () => {
  const mountRef = useRef<HTMLDivElement>(null);
  const [xrManager] = useState(() => new WebXRManager());
  const [xrSupported, setXrSupported] = useState<boolean | null>(null);
  const [vrSupported, setVrSupported] = useState(false);
  const [arSupported, setArSupported] = useState(false);
  const [currentMode, setCurrentMode] = useState<'desktop' | 'vr' | 'ar'>('desktop');
  const [isInitialized, setIsInitialized] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [isPlaying, setIsPlaying] = useState(false);
  const [handTrackingEnabled, setHandTrackingEnabled] = useState(false);

  // VirtualDesktop最適化
  const { preset, isVD, changePreset } = useVirtualDesktopOptimizer();

  // Initialize XR support check
  useEffect(() => {
    const checkXRSupport = async () => {
      try {
        const xr = (navigator as NavigatorXR).xr;
        if (!xr) {
          setXrSupported(false);
          setError('WebXR is not supported in this browser');
          return;
        }

        const vrSupport = await xr.isSessionSupported('immersive-vr');
        const arSupport = await xr.isSessionSupported('immersive-ar');

        setXrSupported(true);
        setVrSupported(vrSupport);
        setArSupported(arSupport);

        if (!vrSupport && !arSupport) {
          setError('Neither VR nor AR is supported on this device');
        }

        setIsInitialized(true);
      } catch (err) {
        setXrSupported(false);
        setError(`XR support check failed: ${err}`);
        setIsInitialized(true);
      }
    };

    checkXRSupport();
  }, []);

  // XR event handlers
  useEffect(() => {
    if (!xrManager) return;

    const handleVREntered = (experience: VRExperience) => {
      console.log('VR mode entered');
      setCurrentMode('vr');
      setError(null);
    };

    const handleAREntered = (experience: VRExperience) => {
      console.log('AR mode entered');
      setCurrentMode('ar');
      setError(null);
    };

    const handleXRError = (error: XRErrorLike) => {
      console.error('XR error:', error);
      const errorMessage = typeof error === 'string' 
        ? error 
        : error instanceof Error 
        ? error.message 
        : (error as { message?: string })?.message || String(error);
      setError(`XR Error: ${errorMessage}`);
    };

    const handleHandTracking = (handData: HandTrackingData) => {
      console.log('Hand tracking data:', handData);
      setHandTrackingEnabled(true);
      // Handle hand gestures for Git interaction
      if (handData.gestures.includes('pointing')) {
        // Implement commit selection by pointing
      }
    };

    const handleAnchorCreated = (anchor: ARAnchor) => {
      console.log('AR anchor created:', anchor);
      // Store anchor for Git commit placement
    };

    const handleCommitSelected = (commitData: Git4DCommitData) => {
      console.log('Commit selected:', commitData);
      // Handle commit selection in VR/AR
    };

    // Register event listeners
    xrManager.on('vrEntered', handleVREntered);
    xrManager.on('arEntered', handleAREntered);
    xrManager.on('vrError', handleXRError);
    xrManager.on('arError', handleXRError);
    xrManager.on('handTracking', handleHandTracking);
    xrManager.on('anchorCreated', handleAnchorCreated);
    xrManager.on('commitSelected', handleCommitSelected);

    return () => {
      xrManager.removeAllListeners();
    };
  }, [xrManager]);

  // VR mode entry
  const enterVRMode = useCallback(async () => {
    if (!xrManager || !vrSupported) return;

    try {
      setError(null);
      
      // VirtualDesktop検出時は最適化を適用
      if (isVD) {
        console.log('VirtualDesktop detected - applying streaming optimizations for VR');
        // ストリーミング最適化を適用
        document.documentElement.setAttribute('data-vd-mode', 'vr');
      }
      
      const experience = await xrManager.enterVR();

      if (experience) {
        console.log('VR experience initialized:', experience);
      } else {
        setError('Failed to initialize VR experience');
      }
    } catch (err) {
      setError(`VR mode entry failed: ${err}`);
    }
  }, [xrManager, vrSupported, isVD]);

  // AR mode entry
  const enterARMode = useCallback(async () => {
    if (!xrManager || !arSupported) return;

    try {
      setError(null);
      
      // VirtualDesktop検出時は最適化を適用
      if (isVD) {
        console.log('VirtualDesktop detected - applying streaming optimizations for AR');
        // ストリーミング最適化を適用
        document.documentElement.setAttribute('data-vd-mode', 'ar');
      }
      
      const experience = await xrManager.enterAR();

      if (experience) {
        console.log('AR experience initialized:', experience);
      } else {
        setError('Failed to initialize AR experience');
      }
    } catch (err) {
      setError(`AR mode entry failed: ${err}`);
    }
  }, [xrManager, arSupported, isVD]);

  // Exit XR mode
  const exitXRMode = useCallback(() => {
    if (xrManager) {
      xrManager.exitXR();
      setCurrentMode('desktop');
      setHandTrackingEnabled(false);
      // VirtualDesktopモード属性をクリア
      document.documentElement.removeAttribute('data-vd-mode');
    }
  }, [xrManager]);

  // Toggle animation
  const toggleAnimation = useCallback(() => {
    setIsPlaying(!isPlaying);
  }, [isPlaying]);

  // Reset view
  const resetView = useCallback(() => {
    // Reset camera and controls
    console.log('Reset view requested');
  }, []);

  if (!isInitialized) {
    return (
      <Box display="flex" justifyContent="center" alignItems="center" minHeight="400px">
        <Typography>Initializing WebXR support...</Typography>
      </Box>
    );
  }

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.5 }}
    >
      <Paper elevation={3} sx={{ p: 3, mb: 2 }}>
        <Box display="flex" alignItems="center" justifyContent="space-between" mb={2}>
          <Box display="flex" alignItems="center" gap={1}>
            <Typography variant="h6" component="h2">
              Git4D WebXR Framework
            </Typography>
            {isVD && (currentMode === 'vr' || currentMode === 'ar') && (
              <Chip 
                label={`VirtualDesktop: ${preset.name}`} 
                color="info" 
                size="small"
              />
            )}
          </Box>
          <Box display="flex" gap={1}>
            <IconButton onClick={resetView} title="Reset View">
              <RotateCcw size={20} />
            </IconButton>
            <IconButton onClick={toggleAnimation} title={isPlaying ? "Pause" : "Play"}>
              {isPlaying ? <Pause size={20} /> : <Play size={20} />}
            </IconButton>
          </Box>
        </Box>

        {/* Status and Controls */}
        <Box display="flex" gap={2} mb={2} flexWrap="wrap">
          <Button
            variant={currentMode === 'desktop' ? 'contained' : 'outlined'}
            startIcon={<Monitor size={16} />}
            onClick={exitXRMode}
            disabled={currentMode === 'desktop'}
          >
            Desktop
          </Button>

            {vrSupported && (
            <Button
              variant={currentMode === 'vr' ? 'contained' : 'outlined'}
              startIcon={<BoxIcon size={16} />}
              onClick={enterVRMode}
              color="primary"
            >
              VR Mode
            </Button>
          )}

          {arSupported && (
            <Button
              variant={currentMode === 'ar' ? 'contained' : 'outlined'}
              startIcon={<Smartphone size={16} />}
              onClick={enterARMode}
              color="secondary"
            >
              AR Mode
            </Button>
          )}
        </Box>

        {/* Status Information */}
        <Box mb={2}>
          <Typography variant="body2" color="text.secondary">
            Mode: {currentMode.toUpperCase()} |
            XR Supported: {xrSupported ? 'Yes' : 'No'} |
            VR: {vrSupported ? 'Supported' : 'Not Supported'} |
            AR: {arSupported ? 'Supported' : 'Not Supported'} |
            Hand Tracking: {handTrackingEnabled ? 'Enabled' : 'Disabled'}
            {isVD && (currentMode === 'vr' || currentMode === 'ar') && (
              <> | VirtualDesktop: {preset.name} ({preset.targetFps} FPS, {preset.renderScale.toFixed(2)}x scale)</>
            )}
          </Typography>
        </Box>

        {/* Error Display */}
        {error && (
          <Alert severity="error" sx={{ mb: 2 }}>
            {error}
          </Alert>
        )}

        {/* WebXR Info */}
        {!xrSupported && (
          <Alert severity="warning" sx={{ mb: 2 }}>
            WebXR is not supported in this browser. Please use a WebXR-compatible browser like Chrome or Edge with VR/AR hardware.
          </Alert>
        )}

        {/* 3D Canvas */}
        <Box
          ref={mountRef}
          sx={{
            width: '100%',
            height: '600px',
            border: '1px solid #ccc',
            borderRadius: 1,
            overflow: 'hidden'
          }}
        >
          <VRButton />
          <Canvas 
            // camera={{ position: [0, 0, 5], fov: 75 }} // Removed as PerspectiveCamera is now a child
            gl={{
              antialias: isVD && (currentMode === 'vr' || currentMode === 'ar') ? preset.enablePostProcessing : true,
              powerPreference: 'high-performance',
            }}
            dpr={isVD && (currentMode === 'vr' || currentMode === 'ar') ? Math.min(window.devicePixelRatio, preset.renderScale) : window.devicePixelRatio}
          >
            <XR store={xrStore}>
              <PerspectiveCamera makeDefault position={[0, 1.6, 5]} fov={75} />
              <Git4DScene
                mode={currentMode}
                isPlaying={isPlaying}
                handTrackingEnabled={handTrackingEnabled}
                isVD={isVD && (currentMode === 'vr' || currentMode === 'ar')}
                preset={isVD && (currentMode === 'vr' || currentMode === 'ar') ? preset : null}
              />
            </XR>
          </Canvas>
        </Box>

        {/* Performance Info */}
        <Box mt={2}>
          <Typography variant="caption" color="text.secondary">
            WebXR Framework initialized. Use VR/AR buttons to enter immersive modes.
            Hand tracking and gesture recognition available in supported environments.
          </Typography>
        </Box>
      </Paper>
    </motion.div>
  );
};

/**
 * Git4D 3D Scene Component
 */
interface Git4DSceneProps {
  mode: 'desktop' | 'vr' | 'ar';
  isPlaying: boolean;
  handTrackingEnabled: boolean;
  isVD?: boolean;
  preset?: { targetFps: number; renderScale: number; lodBias: number } | null;
}

const Git4DScene: React.FC<Git4DSceneProps> = ({ mode, isPlaying, handTrackingEnabled, isVD = false, preset = null }) => {
  const { scene, camera } = useThree();
  const groupRef = useRef<THREE.Group>(null);

  // Mock Git commit data - replace with real data from Rust backend
  const commits = React.useMemo(() => generateMockGit4DCommits(), []);

  // Initialize scene
  useEffect(() => {
    // Set scene background based on mode
    if (mode === 'vr' || mode === 'ar') {
      scene.background = new THREE.Color(0x000000);
    } else {
      scene.background = new THREE.Color(0x0a0a0a);
    }

    // Add lighting
    const ambientLight = new THREE.AmbientLight(0x404040, mode === 'ar' ? 0.8 : 0.4);
      scene.add(ambientLight as unknown as THREE.Object3D);

    const directionalLight = new THREE.DirectionalLight(0xffffff, mode === 'ar' ? 1.0 : 0.8);
    directionalLight.position.set(10, 10, 5);
    directionalLight.castShadow = mode === 'desktop';
      scene.add(directionalLight as unknown as THREE.Object3D);

    return () => {
      // Cleanup
      scene.remove(ambientLight);
      scene.remove(directionalLight);
    };
  }, [scene, mode]);

  // Animation frame with FPS limiting
  // Left and right controllers
  const leftController = useXRControllerState('left')
  const rightController = useXRControllerState('right');
  
  const frameTimeRef = useRef(0);
  const lastFrameTimeRef = useRef(0);
  
  useFrame((state, delta) => {
    // FPS制限（VirtualDesktop用）
    if (isVD && preset) {
      const targetFrameTime = 1000 / preset.targetFps;
      const currentTime = state.clock.elapsedTime * 1000;
      const elapsed = currentTime - lastFrameTimeRef.current;
      
      if (elapsed < targetFrameTime) {
        return; // Skip frame to maintain target FPS
      }
      lastFrameTimeRef.current = currentTime;
    }
    
    if (isPlaying && groupRef.current) {
      // Rotate the entire commit visualization
      // LOD調整: VirtualDesktop時は回転速度を調整
      const rotationSpeed = isVD && preset ? 0.005 * (1 + preset.lodBias * 0.1) : 0.005;
      groupRef.current.rotation.y += rotationSpeed;
    }

    if (mode === 'vr' || mode === 'ar') {
      // Adjust camera for XR modes
      camera.position.z = 3;
    }
  });

  return (
    <>
      {/* Camera controls for desktop mode */}
      {mode === 'desktop' && <OrbitControls enablePan={true} enableZoom={true} enableRotate={true} />}

      {/* Git commits visualization */}
      <group ref={groupRef}>
        {commits.map((commit, index) => (
          <CommitNode
            key={commit.id}
            commit={commit}
            index={index}
            mode={mode}
          />
        ))}

        {/* Branch connections */}
        <BranchConnections commits={commits} />
      </group>

      {/* Time axis */}
      <TimeAxis mode={mode} />

      {/* Hand tracking visualization */}
      {handTrackingEnabled && <HandTrackingVisualization />}

      {/* VR/AR UI elements */}
      {(mode === 'vr' || mode === 'ar') && <XRInterface mode={mode} />}
    </>
  );
};

/**
 * Individual Git commit node
 */
interface CommitNodeProps {
  commit: Git4DCommitData;
  index: number;
  mode: 'desktop' | 'vr' | 'ar';
}

const CommitNode: React.FC<CommitNodeProps> = ({ commit, index, mode }) => {
  const meshRef = useRef<THREE.Mesh>(null);

  // Color based on commit properties
  const color = React.useMemo(() => {
    if (commit.filesChanged > 10) return '#ff4444'; // High impact
    if (commit.insertions > commit.deletions) return '#44ff44'; // Mostly additions
    if (commit.deletions > commit.insertions) return '#ff8844'; // Mostly deletions
    return '#4444ff'; // Balanced changes
  }, [commit]);

  // Size based on impact (LOD調整対応)
  const size = React.useMemo(() => {
    const baseSize = 0.05;
    const impact = Math.min(commit.filesChanged / 10, 3);
    // VirtualDesktop時はLOD調整を適用（lodBiasが大きいほどサイズを小さく）
    const lodAdjustment = mode === 'vr' || mode === 'ar' ? 0.95 : 1.0;
    return baseSize * (1 + impact) * lodAdjustment;
  }, [commit, mode]);

  return (
    <group position={[commit.x, commit.y, commit.z]}>
      <mesh ref={meshRef}>
        <sphereGeometry args={[size, 16, 16]} />
        <meshStandardMaterial color={color} />
      </mesh>

      {/* Commit info text */}
      {mode === 'desktop' && (
        <Text
          position={[0, size + 0.1, 0]}
          fontSize={0.03}
          color="white"
          anchorX="center"
          anchorY="middle"
        >
          {commit.message.substring(0, 20)}...
        </Text>
      )}
    </group>
  );
};

/**
 * Branch connection lines
 */
const BranchConnections: React.FC<{ commits: Git4DCommitData[] }> = ({ commits }) => {
  const lines = React.useMemo(() => {
    const connections: Array<{ start: THREE.Vector3; end: THREE.Vector3 }> = [];

    // Simple connection logic - connect commits in chronological order
    for (let i = 0; i < commits.length - 1; i++) {
      const current = commits[i];
      const next = commits[i + 1];

      connections.push({
        start: new THREE.Vector3(current.x, current.y, current.z),
        end: new THREE.Vector3(next.x, next.y, next.z)
      });
    }

    return connections;
  }, [commits]);

  return (
    <>
      {lines.map((line, index) => (
        <line key={index}>
          <bufferGeometry>
            <bufferAttribute
              attach="attributes-position"
              count={2}
              array={new Float32Array([
                line.start.x, line.start.y, line.start.z,
                line.end.x, line.end.y, line.end.z
              ])}
              itemSize={3}
            />
          </bufferGeometry>
          <lineBasicMaterial color="#666666" />
        </line>
      ))}
    </>
  );
};

/**
 * Time axis visualization
 */
const TimeAxis: React.FC<{ mode: 'desktop' | 'vr' | 'ar' }> = ({ mode }) => {
  return (
    <group>
      {/* Time axis line */}
      <line>
        <bufferGeometry>
          <bufferAttribute
            attach="attributes-position"
            count={2}
            array={new Float32Array([-5, 0, 0, 5, 0, 0])}
            itemSize={3}
          />
        </bufferGeometry>
        <lineBasicMaterial color="#888888" />
      </line>

      {/* Time labels */}
      {mode === 'desktop' && (
        <>
          <Text position={[-5, 0.2, 0]} fontSize={0.05} color="white">
            Past
          </Text>
          <Text position={[5, 0.2, 0]} fontSize={0.05} color="white">
            Present
          </Text>
        </>
      )}
    </group>
  );
};

/**
 * Hand tracking visualization
 */
const HandTrackingVisualization: React.FC = () => {
  // Placeholder for hand tracking visualization
  return (
    <group>
      {/* Hand tracking indicators would be rendered here */}
    </group>
  );
};

/**
 * XR interface elements
 */
const XRInterface: React.FC<{ mode: 'desktop' | 'vr' | 'ar' }> = ({ mode }) => {
  return (
    <group>
      {/* XR-specific UI elements */}
      <Text
        position={[0, 2, -3]}
        fontSize={0.1}
        color="white"
        anchorX="center"
        anchorY="middle"
      >
        {mode === 'vr' ? 'VR Mode Active' : 'AR Mode Active'}
      </Text>
    </group>
  );
};


/**
 * Generate mock Git4D commit data for demonstration
 */
function generateMockGit4DCommits(): Git4DCommitData[] {
  const commits: Git4DCommitData[] = [];
  const now = Date.now();

  for (let i = 0; i < 50; i++) {
    const timestamp = now - (i * 24 * 60 * 60 * 1000); // One commit per day
    const x = (Math.random() - 0.5) * 4; // Branch spread
    const y = (i / 49) * 8 - 4; // Time progression
    const z = Math.random() * 2 - 1; // Depth variation

    commits.push({
      id: `commit_${i}`,
      author: ['Alice', 'Bob', 'Charlie', 'Diana'][Math.floor(Math.random() * 4)],
      message: [
        'Add new feature',
        'Fix bug in authentication',
        'Update documentation',
        'Refactor code structure',
        'Add unit tests',
        'Optimize performance'
      ][Math.floor(Math.random() * 6)],
      timestamp,
      x,
      y,
      z,
      filesChanged: Math.floor(Math.random() * 20) + 1,
      insertions: Math.floor(Math.random() * 1000) + 10,
      deletions: Math.floor(Math.random() * 500) + 5
    });
  }

  return commits;
}

export default Git4DWebXRFramework;