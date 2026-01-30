'use client';

import React, { useState } from 'react';
import {
  Box,
  Typography,
  Card as MuiCard,
  CardContent,
  CardActions,
  Button,
  Chip,
  Avatar,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  Alert,
  LinearProgress,
  CircularProgress,
  Tooltip,
  IconButton,
} from '@mui/material';
import {
  Shield,
  Code,
  Search,
  Zap,
  Settings,
  Play,
  Info,
} from 'lucide-react';
import { DashboardLayout } from '@/components/templates/DashboardLayout';
import { useCodex } from '@/lib/context/CodexContext';

const AGENT_ICONS = {
  'code-reviewer': Code,
  'test-gen': Zap,
  'sec-audit': Shield,
  'researcher': Search,
  'performance': Zap,
  'debug': Settings,
  'docs': Info,
};

const AGENT_COLORS = {
  'code-reviewer': 'primary',
  'test-gen': 'secondary',
  'sec-audit': 'error',
  'researcher': 'info',
  'performance': 'warning',
  'debug': 'default',
  'docs': 'success',
} as const;

interface Agent {
  id: string;
  name: string;
  type: "code-reviewer" | "test-gen" | "sec-audit" | "researcher" | "performance" | "debug" | "docs";
  description?: string;
  capabilities?: string[];
}

interface AgentExecutionDialogProps {
  agent: Agent;
  open: boolean;
  onClose: () => void;
  onExecute: (context: { target?: string; query?: string; context?: string; code?: string; language?: string; path?: string }) => void;
}

function AgentExecutionDialog({ agent, open, onClose, onExecute }: AgentExecutionDialogProps) {
  const [context, setContext] = useState('');
  const [target, setTarget] = useState('');
  const [query, setQuery] = useState('');
  const [isExecuting, setIsExecuting] = useState(false);

  const handleExecute = async () => {
    setIsExecuting(true);
    try {
      let executionContext: { target?: string; query?: string; context?: string; code?: string; language?: string; path?: string } = {};

      switch (agent.type) {
        case 'code-reviewer':
          executionContext = { code: context, language: target };
          break;
        case 'test-gen':
          executionContext = { code: context, language: target };
          break;
        case 'sec-audit':
          executionContext = { path: target };
          break;
        case 'researcher':
          executionContext = { query };
          break;
        default:
          executionContext = { context, target, query };
      }

      await onExecute(executionContext);
      onClose();
    } catch (error) {
      console.error('Agent execution failed:', error);
    } finally {
      setIsExecuting(false);
    }
  };

  const renderForm = () => {
    switch (agent.type) {
      case 'code-reviewer':
        return (
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
            <TextField
              fullWidth
              multiline
              rows={6}
              label="レビューするコード"
              value={context}
              onChange={(e) => setContext(e.target.value)}
              placeholder="ここにレビューしたいコードを入力してください..."
            />
            <FormControl fullWidth>
              <InputLabel>言語</InputLabel>
              <Select
                value={target}
                label="言語"
                onChange={(e) => setTarget(e.target.value)}
              >
                <MenuItem value="javascript">JavaScript</MenuItem>
                <MenuItem value="typescript">TypeScript</MenuItem>
                <MenuItem value="python">Python</MenuItem>
                <MenuItem value="rust">Rust</MenuItem>
                <MenuItem value="go">Go</MenuItem>
              </Select>
            </FormControl>
          </Box>
        );

      case 'test-gen':
        return (
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
            <TextField
              fullWidth
              multiline
              rows={6}
              label="テストを生成するコード"
              value={context}
              onChange={(e) => setContext(e.target.value)}
              placeholder="ここにテストを生成したいコードを入力してください..."
            />
            <FormControl fullWidth>
              <InputLabel>言語</InputLabel>
              <Select
                value={target}
                label="言語"
                onChange={(e) => setTarget(e.target.value)}
              >
                <MenuItem value="javascript">JavaScript</MenuItem>
                <MenuItem value="typescript">TypeScript</MenuItem>
                <MenuItem value="python">Python</MenuItem>
                <MenuItem value="rust">Rust</MenuItem>
                <MenuItem value="go">Go</MenuItem>
              </Select>
            </FormControl>
          </Box>
        );

      case 'sec-audit':
        return (
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
            <TextField
              fullWidth
              label="スキャン対象のパス"
              value={target}
              onChange={(e) => setTarget(e.target.value)}
              placeholder="./src または特定のファイルパス"
            />
            <Alert severity="info">
              セキュリティスキャンでは、脆弱性、機密情報の漏洩、コード品質の問題を検出します。
            </Alert>
          </Box>
        );

      case 'researcher':
        return (
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
            <TextField
              fullWidth
              multiline
              rows={3}
              label="研究クエリ"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="研究したいトピックや質問を入力してください..."
            />
            <Alert severity="info">
              Deep Researchは複数のソースから情報を収集し、分析します。技術的な質問や調査に最適です。
            </Alert>
          </Box>
        );

      default:
        return (
          <TextField
            fullWidth
            multiline
            rows={4}
            label="実行コンテキスト"
            value={context}
            onChange={(e) => setContext(e.target.value)}
            placeholder="エージェントに渡すコンテキスト情報を入力してください..."
          />
        );
    }
  };

  return (
    <Dialog open={open} onClose={onClose} maxWidth="md" fullWidth>
      <DialogTitle sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
        <Avatar sx={{ bgcolor: `${AGENT_COLORS[agent.type as keyof typeof AGENT_COLORS]}.main` }}>
          {React.createElement(AGENT_ICONS[agent.type as keyof typeof AGENT_ICONS], { size: 20 })}
        </Avatar>
        {agent.name} の実行
      </DialogTitle>
      <DialogContent>
        <Typography variant="body2" color="text.secondary" sx={{ mb: 2 }}>
          {agent.description}
        </Typography>
        {renderForm()}
      </DialogContent>
      <DialogActions>
        <Button onClick={onClose}>キャンセル</Button>
        <Button
          onClick={handleExecute}
          variant="contained"
          disabled={isExecuting}
          startIcon={isExecuting ? <CircularProgress size={16} /> : <Play />}
        >
          {isExecuting ? '実行中...' : '実行'}
        </Button>
      </DialogActions>
    </Dialog>
  );
}

export default function AgentsPage() {
  const { state, runAgent, runSecurityScan, runResearch, runWebResearch } = useCodex();
  const [selectedAgent, setSelectedAgent] = useState<Agent | null>(null);
  const [dialogOpen, setDialogOpen] = useState(false);

  const handleAgentExecute = async (agent: Agent) => {
    setSelectedAgent(agent);
    setDialogOpen(true);
  };

  const handleExecuteConfirm = async (context: { target?: string; query?: string; context?: string; code?: string; language?: string; path?: string }) => {
    try {
      if (!selectedAgent) return;
      switch (selectedAgent.type) {
        case 'sec-audit':
          await runSecurityScan('code', context.path || '');
          break;
        case 'researcher':
          if (selectedAgent.id === 'web-research') {
            await runWebResearch(context.query || '');
          } else {
            await runResearch(context.query || '');
          }
          break;
        default:
          await runAgent(selectedAgent.id, context);
      }
    } catch (error) {
      console.error('Agent execution failed:', error);
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'idle': return 'default';
      case 'working': return 'warning';
      case 'completed': return 'success';
      case 'error': return 'error';
      default: return 'default';
    }
  };

  const getStatusText = (status: string) => {
    switch (status) {
      case 'idle': return '待機中';
      case 'working': return '実行中';
      case 'completed': return '完了';
      case 'error': return 'エラー';
      default: return status;
    }
  };

  return (
    <DashboardLayout title="エージェント管理">
      <Box sx={{ p: 3 }} data-testid="agents-page">
        <Typography variant="h4" sx={{ mb: 2, fontWeight: 700 }} data-testid="agents-title">
          AI エージェント
        </Typography>
        <Typography variant="body1" color="text.secondary" sx={{ mb: 4 }}>
          専門化されたAIエージェントで、コードレビュー、テスト生成、セキュリティスキャン、研究などを自動化します。
        </Typography>

        {state.error && (
          <Alert severity="error" sx={{ mb: 3 }}>
            {state.error}
          </Alert>
        )}

        <Box sx={{ display: 'grid', gridTemplateColumns: { xs: '1fr', md: 'repeat(2, 1fr)', lg: 'repeat(3, 1fr)' }, gap: 3 }} data-testid="agents-grid">
          {state.agents.map((agent) => {
            const IconComponent = AGENT_ICONS[agent.type as keyof typeof AGENT_ICONS];
            const color = AGENT_COLORS[agent.type as keyof typeof AGENT_COLORS];

            return (
              <Box key={agent.id} sx={{ height: '100%' }} data-testid={`agent-card-${agent.id}`}>
                <MuiCard
                  sx={{
                    height: '100%',
                    display: 'flex',
                    flexDirection: 'column',
                    transition: 'all 0.3s ease',
                    '&:hover': {
                      transform: 'translateY(-4px)',
                      boxShadow: 4,
                    },
                  }}
                >
                  <CardContent sx={{ flex: 1 }}>
                    <Box sx={{ display: 'flex', alignItems: 'center', mb: 2 }}>
                      <Avatar
                        sx={{
                          bgcolor: `${color}.main`,
                          mr: 2,
                          width: 48,
                          height: 48,
                        }}
                      >
                        <IconComponent size={24} />
                      </Avatar>
                      <Box sx={{ flex: 1 }}>
                        <Typography variant="h6" sx={{ fontWeight: 600 }}>
                          {agent.name}
                        </Typography>
                        <Chip
                          label={getStatusText(agent.status)}
                          size="small"
                          color={getStatusColor(agent.status)}
                          variant={agent.status === 'working' ? 'filled' : 'outlined'}
                        />
                      </Box>
                    </Box>

                    <Typography variant="body2" color="text.secondary" sx={{ mb: 2 }}>
                      {agent.description}
                    </Typography>

                    {agent.lastUsed && (
                      <Typography variant="caption" color="text.secondary">
                        最終使用: {new Date(agent.lastUsed).toLocaleString('ja-JP')}
                      </Typography>
                    )}

                    {agent.status === 'working' && (
                      <Box sx={{ mt: 2 }}>
                        <LinearProgress />
                        <Typography variant="caption" color="text.secondary" sx={{ mt: 1 }}>
                          処理中...
                        </Typography>
                      </Box>
                    )}
                  </CardContent>

                  <CardActions sx={{ justifyContent: 'space-between', px: 2, pb: 2 }}>
                    <Box sx={{ display: 'flex', gap: 1 }}>
                      <Tooltip title="実行">
                        <IconButton
                          size="small"
                          onClick={() => handleAgentExecute(agent)}
                          disabled={agent.status === 'working'}
                          sx={{
                            color: `${color}.main`,
                            '&:hover': {
                              backgroundColor: `${color}.light`,
                            },
                          }}
                        >
                          <Play size={16} />
                        </IconButton>
                      </Tooltip>

                      <Tooltip title="設定">
                        <IconButton size="small" sx={{ color: 'text.secondary' }}>
                          <Settings size={16} />
                        </IconButton>
                      </Tooltip>
                    </Box>

                    <Button
                      size="small"
                      variant="outlined"
                      onClick={() => handleAgentExecute(agent)}
                      disabled={agent.status === 'working'}
                      sx={{
                        borderColor: `${color}.main`,
                        color: `${color}.main`,
                        '&:hover': {
                          borderColor: `${color}.dark`,
                          backgroundColor: `${color}.light`,
                        },
                      }}
                    >
                      {agent.status === 'working' ? '実行中...' : '実行'}
                    </Button>
                  </CardActions>
                </MuiCard>
              </Box>
            );
          })}
        </Box>

        {/* Agent Execution Dialog */}
        {selectedAgent && (
          <AgentExecutionDialog
            agent={selectedAgent}
            open={dialogOpen}
            onClose={() => setDialogOpen(false)}
            onExecute={handleExecuteConfirm}
          />
        )}

        {/* Quick Actions */}
        <Box sx={{ mt: 4 }}>
          <Typography variant="h5" sx={{ mb: 2, fontWeight: 600 }}>
            クイックアクション
          </Typography>
          <Box sx={{ display: 'grid', gridTemplateColumns: { xs: '1fr', md: 'repeat(3, 1fr)' }, gap: 2 }}>
            <Box sx={{ width: '100%' }}>
              <Button
                fullWidth
                variant="contained"
                startIcon={<Shield />}
                onClick={() => {
                  const agent = state.agents.find(a => a.type === 'sec-audit');
                  if (agent) handleAgentExecute(agent);
                }}
                sx={{
                  py: 2,
                  background: 'linear-gradient(45deg, #d32f2f, #f44336)',
                  '&:hover': {
                    background: 'linear-gradient(45deg, #b71c1c, #d32f2f)',
                  },
                }}
              >
                セキュリティスキャン
              </Button>
            </Box>
            <Box sx={{ width: '100%' }}>
              <Button
                fullWidth
                variant="contained"
                startIcon={<Code />}
                onClick={() => {
                  const agent = state.agents.find(a => a.type === 'code-reviewer');
                  if (agent) handleAgentExecute(agent);
                }}
                sx={{
                  py: 2,
                  background: 'linear-gradient(45deg, #1976d2, #2196f3)',
                  '&:hover': {
                    background: 'linear-gradient(45deg, #0d47a1, #1976d2)',
                  },
                }}
              >
                コードレビュー
              </Button>
            </Box>
            <Box sx={{ width: '100%' }}>
              <Button
                fullWidth
                variant="contained"
                startIcon={<Search />}
                onClick={() => {
                  const agent = state.agents.find(a => a.type === 'researcher');
                  if (agent) handleAgentExecute(agent);
                }}
                sx={{
                  py: 2,
                  background: 'linear-gradient(45deg, #7b1fa2, #9c27b0)',
                  '&:hover': {
                    background: 'linear-gradient(45deg, #4a148c, #7b1fa2)',
                  },
                }}
              >
                Deep Research
              </Button>
            </Box>
          </Box>
        </Box>
      </Box>
    </DashboardLayout>
  );
}
