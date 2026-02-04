'use client';

import React, { useEffect, useState } from 'react';
import { useSearchParams } from 'next/navigation';
import { Box, Container, Typography, Paper, CircularProgress, Alert, Chip } from '@mui/material';
import { motion } from 'framer-motion';
import { GitBranch, Eye, Zap } from 'lucide-react';
import { Git4DWebXRFramework } from '../../components/visualization/Git4DWebXRFramework';
import { Git4DVisualization } from '../../components/visualization/Git4DVisualization';
import { useVirtualDesktopOptimizer } from '../../utils/virtualdesktop-optimizer';
import type { NavigatorXR, Git4DLaunchRequest, Git4DLaunchResponse } from '../../lib/types';

/**
 * Git4D VR/AR Visualization Page
 *
 * Sprint 1: Gitリポジトリ解析エンジン + WebXR基本フレームワーク
 * kamui4dを超える没入型4D Git可視化
 */
export default function Git4DPage() {
  const searchParams = useSearchParams();
  const [sessionId, setSessionId] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [detectedPlatform, setDetectedPlatform] = useState<string | null>(null);
  
  // VirtualDesktop検出
  const { preset, isVD } = useVirtualDesktopOptimizer();
  
  // Get mode from URL params or default to 'desktop'
  const mode = (searchParams?.get('mode') || 'desktop') as 'desktop' | 'vr' | 'ar';
  const repositoryPath = searchParams?.get('repository_path') || undefined;
  
  // Check device availability and launch visualization session
  useEffect(() => {
    const launchSession = async () => {
      if (mode && mode !== 'desktop' && !sessionId) {
        setIsLoading(true);
        setError(null);
        
        try {
          // Check device availability using WebXR Manager
          let deviceAvailable = false;
          let deviceWarning: string | null = null;
          
          if (mode === 'vr' || mode === 'ar') {
            if ('xr' in navigator) {
              const xr = (navigator as NavigatorXR).xr;
              if (xr) {
                try {
                  const sessionType = mode === 'vr' ? 'immersive-vr' : 'immersive-ar';
                  deviceAvailable = await xr.isSessionSupported(sessionType);
                  if (!deviceAvailable) {
                    deviceWarning = `${mode.toUpperCase()} device not available. Falling back to desktop mode.`;
                  }
                } catch (err) {
                  deviceWarning = `Failed to check ${mode.toUpperCase()} device availability: ${err instanceof Error ? err.message : 'Unknown error'}`;
                  deviceAvailable = false;
                }
              } else {
                deviceWarning = 'WebXR not available in this browser. Falling back to desktop mode.';
                deviceAvailable = false;
              }
            } else {
              deviceWarning = 'WebXR not supported in this browser. Falling back to desktop mode.';
              deviceAvailable = false;
            }
          } else {
            deviceAvailable = true; // Desktop mode always available
          }
          
          const apiUrl = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8787';
          
          // VirtualDesktop検出結果をAPIリクエストに含める
          const requestBody: Git4DLaunchRequest = {
            mode: deviceAvailable ? mode : 'desktop',
            repositoryPath: repositoryPath || '.',
          };
          
          // VirtualDesktop検出結果を追加（クライアント側検出）
          if (isVD && (mode === 'vr' || mode === 'ar')) {
            requestBody.virtualDesktop = true;
            console.log('VirtualDesktop detected - sending detection result to API');
          }
          
          const response = await fetch(`${apiUrl}/api/visualization/git4d`, {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
            },
            body: JSON.stringify(requestBody),
          });
          
          if (!response.ok) {
            const errorData = await response.json().catch(() => ({ message: 'Unknown error' }));
            throw new Error(errorData.message || `HTTP ${response.status}`);
          }
          
          const data = await response.json() as Git4DLaunchResponse;
          setSessionId(data.sessionId);
          
          // 検出されたプラットフォーム情報を保存
          if (data.platform) {
            setDetectedPlatform(data.platform);
            if (data.deviceName) {
              console.log(`Platform detected: ${data.platform} (${data.deviceName})`);
            } else {
              console.log(`Platform detected: ${data.platform}`);
            }
          }
          
          // VirtualDesktop検出時の通知
          if (isVD && (mode === 'vr' || mode === 'ar')) {
            console.log(`VirtualDesktop optimizations applied: ${preset.name} (${preset.targetFps} FPS)`);
          }
          
          // Show warning if device was not available
          if (deviceWarning) {
            console.warn(deviceWarning);
          }
        } catch (err) {
          setError(err instanceof Error ? err.message : 'Failed to launch visualization');
        } finally {
          setIsLoading(false);
        }
      }
    };
    
    launchSession();
  }, [mode, repositoryPath, sessionId]);
  
  return (
    <Container maxWidth="xl" sx={{ py: 4 }}>
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.6 }}
      >
        {/* Header */}
        <Box mb={4}>
          <Typography variant="h3" component="h1" gutterBottom sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
            <GitBranch size={40} color="#1976d2" />
            Git4D VR/AR Visualization
            {isVD && (mode === 'vr' || mode === 'ar') && (
              <Chip 
                label={`VirtualDesktop: ${preset.name}`} 
                color="info" 
                size="small"
                sx={{ ml: 1 }}
              />
            )}
            {detectedPlatform && detectedPlatform !== 'Desktop' && (
              <Chip 
                label={`Platform: ${detectedPlatform}`} 
                color="success" 
                size="small"
                sx={{ ml: 1 }}
              />
            )}
            {sessionId && (
              <Chip
                label={`Session: ${sessionId.slice(0, 8)}…`}
                color="primary"
                size="small"
                sx={{ ml: 1 }}
              />
            )}
          </Typography>
          <Typography variant="h6" color="text.secondary" paragraph>
            kamui4dを超える没入型4D Gitリポジトリ可視化システム
          </Typography>

          {/* Feature Highlights */}
          <Box display="flex" gap={3} mt={3} flexWrap="wrap">
            <Paper elevation={2} sx={{ p: 2, display: 'flex', alignItems: 'center', gap: 1, minWidth: 200 }}>
              <Eye size={24} color="#2e7d32" />
              <Box>
                <Typography variant="subtitle2">4D Visualization</Typography>
                <Typography variant="caption" color="text.secondary">
                  時間 + 空間 + 影響 + コラボレーション
                </Typography>
              </Box>
            </Paper>

            <Paper elevation={2} sx={{ p: 2, display: 'flex', alignItems: 'center', gap: 1, minWidth: 200 }}>
              <Zap size={24} color="#ed6c02" />
              <Box>
                <Typography variant="subtitle2">AI Enhanced</Typography>
                <Typography variant="caption" color="text.secondary">
                  LLM統合による知能的可視化
                </Typography>
              </Box>
            </Paper>

            <Paper elevation={2} sx={{ p: 2, display: 'flex', alignItems: 'center', gap: 1, minWidth: 200 }}>
              <GitBranch size={24} color="#9c27b0" />
              <Box>
                <Typography variant="subtitle2">VR/AR Ready</Typography>
                <Typography variant="caption" color="text.secondary">
                  没入型インタラクション
                </Typography>
              </Box>
            </Paper>
          </Box>
        </Box>

        {/* Sprint 1 Status */}
        <Paper elevation={3} sx={{ p: 3, mb: 4, background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)', color: 'white' }}>
          <Typography variant="h5" gutterBottom>
            🚀 Sprint 1: Foundation & Core Architecture
          </Typography>
          <Typography variant="body1" paragraph>
            Git履歴解析エンジンとWebXR基本フレームワークの実装完了
          </Typography>

          <Box display="flex" gap={2} mt={2} flexWrap="wrap">
            <Box sx={{ background: 'rgba(255,255,255,0.1)', px: 2, py: 1, borderRadius: 1 }}>
              ✅ Git Repository Parser (Rust)
            </Box>
            <Box sx={{ background: 'rgba(255,255,255,0.1)', px: 2, py: 1, borderRadius: 1 }}>
              ✅ WebXR Framework (React/Three.js)
            </Box>
            <Box sx={{ background: 'rgba(255,255,255,0.1)', px: 2, py: 1, borderRadius: 1 }}>
              ✅ 4D Data Transformation
            </Box>
            <Box sx={{ background: 'rgba(255,255,255,0.1)', px: 2, py: 1, borderRadius: 1 }}>
              ✅ Hand Tracking Integration
            </Box>
          </Box>
        </Paper>

        {/* Technical Specifications */}
        <Paper elevation={2} sx={{ p: 3, mb: 4 }}>
          <Typography variant="h6" gutterBottom>
            🔧 Technical Specifications
          </Typography>

          <Box display="grid" gridTemplateColumns="repeat(auto-fit, minmax(300px, 1fr))" gap={3}>
            <Box>
              <Typography variant="subtitle2" color="primary" gutterBottom>
                Backend (Rust)
              </Typography>
              <Typography variant="body2" component="div">
                • Git2-rs: 完全なGit履歴解析<br/>
                • Async processing: 非同期並列処理<br/>
                • Memory optimization: 大規模リポジトリ対応<br/>
                • 4D transformation: 座標系変換アルゴリズム
              </Typography>
            </Box>

            <Box>
              <Typography variant="subtitle2" color="primary" gutterBottom>
                Frontend (React/Three.js)
              </Typography>
              <Typography variant="body2" component="div">
                • WebXR: VR/AR統合<br/>
                • Three.js: 3Dレンダリング<br/>
                • React Three Fiber: React統合<br/>
                • Hand tracking: ジェスチャー認識
              </Typography>
            </Box>

            <Box>
              <Typography variant="subtitle2" color="primary" gutterBottom>
                AI Integration
              </Typography>
              <Typography variant="body2" component="div">
                • Sentiment analysis: コミット感情分析<br/>
                • Impact calculation: 変更影響度計算<br/>
                • Collaboration tracking: 共同作業分析<br/>
                • Context understanding: プロジェクト理解
              </Typography>
            </Box>
          </Box>
        </Paper>

        {/* Main Visualization */}
        {error && (
          <Alert severity="error" sx={{ mb: 2 }}>
            {error}
          </Alert>
        )}
        
        {/* VirtualDesktop検出通知 */}
        {isVD && (mode === 'vr' || mode === 'ar') && !error && (
          <Alert severity="info" sx={{ mb: 2 }}>
            VirtualDesktop detected - Streaming optimizations applied: {preset.name} quality ({preset.targetFps} FPS target, {preset.renderScale.toFixed(2)}x render scale)
          </Alert>
        )}
        
        {isLoading && (
          <Box display="flex" justifyContent="center" alignItems="center" minHeight="400px">
            <CircularProgress />
            <Typography variant="body1" sx={{ ml: 2 }}>
              Launching Git4D visualization in {mode} mode...
            </Typography>
          </Box>
        )}
        
        {!isLoading && !error && (
          <Git4DVisualization 
            mode={mode}
            repositoryPath={repositoryPath}
            sessionId={sessionId || undefined}
          />
        )}
        
        {/* Fallback to WebXR Framework if needed */}
        {mode === 'desktop' && !sessionId && (
          <Git4DWebXRFramework />
        )}

        {/* Performance Metrics */}
        <Paper elevation={2} sx={{ p: 3, mt: 4 }}>
          <Typography variant="h6" gutterBottom>
            📊 Performance & Compatibility
          </Typography>

          <Box display="grid" gridTemplateColumns="repeat(auto-fit, minmax(250px, 1fr))" gap={2}>
            <Box>
              <Typography variant="subtitle2">Processing Performance</Typography>
              <Typography variant="body2" color="text.secondary">
                • 10k commits: &lt; 30秒<br/>
                • Memory usage: &lt; 500MB<br/>
                • Parallel workers: 動的調整
              </Typography>
            </Box>

            <Box>
              <Typography variant="subtitle2">WebXR Compatibility</Typography>
              <Typography variant="body2" color="text.secondary">
                • VR: Quest 2/3, Windows Mixed Reality<br/>
                • AR: Windows 11 25H2<br/>
                • Hand Tracking: Windows 11, Quest Pro<br/>
                • VirtualDesktop: Streaming optimization enabled
              </Typography>
            </Box>

            <Box>
              <Typography variant="subtitle2">Browser Support</Typography>
              <Typography variant="body2" color="text.secondary">
                • Chrome 90+<br/>
                • Edge 90+<br/>
                • Firefox 91+<br/>
                • Safari 15+
              </Typography>
            </Box>
          </Box>
        </Paper>

        {/* Next Steps */}
        <Paper elevation={1} sx={{ p: 3, mt: 4, backgroundColor: '#f5f5f5' }}>
          <Typography variant="h6" gutterBottom>
            🎯 Next Steps (Sprint 2-3)
          </Typography>
          <Typography variant="body2" component="div">
            • <strong>Sprint 2:</strong> Advanced Analysis & Security - AI支援レビュー + セキュリティ脆弱性検出<br/>
            • <strong>Sprint 3:</strong> CI/CD Integration & Intelligence - 機械学習最適化 + ダッシュボード<br/>
            • <strong>Future:</strong> Real-time collaboration + Quantum optimization + Multi-user VR sessions
          </Typography>
        </Paper>
      </motion.div>
    </Container>
  );
}
