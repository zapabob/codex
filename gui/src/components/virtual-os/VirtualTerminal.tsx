'use client';

import React, { useState, useEffect, useRef } from 'react';
import {
  Box,
  Typography,
  Paper,
  TextField,
  IconButton,
  Alert,
  Chip,
  List,
  ListItem,
  ListItemText,
  Divider,
  Tooltip,
  Autocomplete,
} from '@mui/material';
import {
  Terminal,
  Play,
  XCircle,
  CheckCircle,
  AlertTriangle,
  History,
  FolderOpen,
  Command,
} from 'lucide-react';
import { CodexAPIClient } from '@/lib/api/client';

interface TerminalHistoryEntry {
  command: string[];
  workingDirectory: string;
  timestamp: string;
  result?: {
    exitCode: number;
    stdout: string;
    stderr: string;
    isBlocked: boolean;
    blockReason?: string;
  };
}

export function VirtualTerminal() {
  const [sessionId, setSessionId] = useState<string | null>(null);
  const [command, setCommand] = useState('');
  const [commandHistory, setCommandHistory] = useState<string[]>([]);
  const [historyIndex, setHistoryIndex] = useState(-1);
  const [output, setOutput] = useState<Array<{
    type: 'command' | 'stdout' | 'stderr' | 'error' | 'blocked';
    content: string;
    timestamp: Date;
  }>>([]);
  const [availableCommands, setAvailableCommands] = useState<string[]>([]);
  const [workingDirectory, setWorkingDirectory] = useState('.');
  const [isExecuting, setIsExecuting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const outputEndRef = useRef<HTMLDivElement>(null);

  const apiClient = new CodexAPIClient();

  useEffect(() => {
    initializeTerminal();
  }, []);

  useEffect(() => {
    scrollToBottom();
  }, [output]);

  const initializeTerminal = async () => {
    try {
      const result = await apiClient.createTerminalSession({
        workingDirectory: '.',
      });
      setSessionId(result.sessionId);
      setWorkingDirectory(result.workingDirectory);

      // Load available commands
      await loadAvailableCommands(result.sessionId);
    } catch (err: any) {
      setError(err.message || 'ターミナルの初期化に失敗しました');
    }
  };

  const loadAvailableCommands = async (sid: string) => {
    try {
      const result = await apiClient.listTerminalCommands(sid);
      setAvailableCommands(result.commands);
    } catch (err) {
      console.error('Failed to load available commands:', err);
    }
  };

  const scrollToBottom = () => {
    outputEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  const handleCommandSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!command.trim() || !sessionId) return;

    const cmd = command.trim();
    setCommandHistory(prev => [...prev, cmd]);
    setHistoryIndex(-1);

    // Parse command into array
    const commandParts = parseCommand(cmd);

    // Add command to output
    setOutput(prev => [...prev, {
      type: 'command',
      content: cmd,
      timestamp: new Date(),
    }]);

    setIsExecuting(true);
    setError(null);

    try {
      const result = await apiClient.executeTerminalCommand(sessionId, commandParts);

      // Handle blocked commands
      if (result.isBlocked) {
        setOutput(prev => [...prev, {
          type: 'blocked',
          content: result.stderr || result.blockReason || 'コマンドがブロックされました',
          timestamp: new Date(),
        }]);
        return;
      }

      // Add stdout
      if (result.stdout) {
        setOutput(prev => [...prev, {
          type: 'stdout',
          content: result.stdout,
          timestamp: new Date(),
        }]);
      }

      // Add stderr
      if (result.stderr) {
        setOutput(prev => [...prev, {
          type: 'stderr',
          content: result.stderr,
          timestamp: new Date(),
        }]);
      }

      // Handle errors
      if (result.exitCode !== 0 && !result.stdout && !result.stderr) {
        setOutput(prev => [...prev, {
          type: 'error',
          content: `コマンドが終了コード ${result.exitCode} で終了しました`,
          timestamp: new Date(),
        }]);
      }
    } catch (err: any) {
      setError(err.message || 'コマンド実行に失敗しました');
      setOutput(prev => [...prev, {
        type: 'error',
        content: err.message || 'コマンド実行に失敗しました',
        timestamp: new Date(),
      }]);
    } finally {
      setIsExecuting(false);
      setCommand('');
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'ArrowUp') {
      e.preventDefault();
      if (commandHistory.length > 0 && historyIndex < commandHistory.length - 1) {
        const newIndex = historyIndex === -1 ? commandHistory.length - 1 : historyIndex + 1;
        setHistoryIndex(newIndex);
        setCommand(commandHistory[commandHistory.length - 1 - newIndex]);
      }
    } else if (e.key === 'ArrowDown') {
      e.preventDefault();
      if (historyIndex > 0) {
        const newIndex = historyIndex - 1;
        setHistoryIndex(newIndex);
        setCommand(commandHistory[commandHistory.length - 1 - newIndex]);
      } else if (historyIndex === 0) {
        setHistoryIndex(-1);
        setCommand('');
      }
    } else if (e.key === 'Tab') {
      e.preventDefault();
      // Auto-complete (simple implementation)
      const prefix = command.split(' ').pop() || '';
      if (prefix) {
        const matches = availableCommands.filter(cmd => cmd.startsWith(prefix));
        if (matches.length === 1) {
          const parts = command.split(' ');
          parts[parts.length - 1] = matches[0];
          setCommand(parts.join(' '));
        }
      }
    }
  };

  const parseCommand = (cmd: string): string[] => {
    // Simple command parsing (can be improved)
    const parts = cmd.trim().split(/\s+/);
    return parts.filter(p => p.length > 0);
  };

  const getOutputColor = (type: string) => {
    switch (type) {
      case 'command':
        return '#4CAF50';
      case 'stdout':
        return '#FFFFFF';
      case 'stderr':
        return '#FF9800';
      case 'error':
        return '#F44336';
      case 'blocked':
        return '#F44336';
      default:
        return '#FFFFFF';
    }
  };

  const clearOutput = () => {
    setOutput([]);
  };

  return (
    <Box sx={{ height: '100%', display: 'flex', flexDirection: 'column', bgcolor: '#1e1e1e' }}>
      {/* Terminal Header */}
      <Box
        sx={{
          p: 1,
          bgcolor: '#2d2d2d',
          display: 'flex',
          alignItems: 'center',
          gap: 1,
          borderBottom: '1px solid rgba(255, 255, 255, 0.1)',
        }}
      >
        <Terminal size={16} color="#4CAF50" />
        <Typography variant="caption" sx={{ color: 'white', flexGrow: 1 }}>
          仮想OSターミナル
        </Typography>
        <Chip
          label={workingDirectory}
          size="small"
          icon={<FolderOpen size={12} />}
          sx={{ bgcolor: '#3d3d3d', color: 'white', fontSize: '10px' }}
        />
        <Tooltip title="出力をクリア">
          <IconButton size="small" onClick={clearOutput} sx={{ color: 'white' }}>
            <XCircle size={14} />
          </IconButton>
        </Tooltip>
      </Box>

      {error && (
        <Alert severity="error" sx={{ m: 1 }} onClose={() => setError(null)}>
          {error}
        </Alert>
      )}

      {/* Output Area */}
      <Box
        sx={{
          flexGrow: 1,
          overflow: 'auto',
          p: 2,
          fontFamily: 'monospace',
          fontSize: '12px',
          color: '#FFFFFF',
        }}
      >
        {output.length === 0 && (
          <Typography variant="body2" color="text.secondary" sx={{ fontFamily: 'monospace' }}>
            {sessionId ? 'ターミナルが準備できました。コマンドを入力してください。' : 'ターミナルを初期化中...'}
          </Typography>
        )}
        {output.map((item, index) => (
          <Box
            key={index}
            sx={{
              mb: 1,
              color: getOutputColor(item.type),
              whiteSpace: 'pre-wrap',
              wordBreak: 'break-word',
            }}
          >
            {item.type === 'command' && (
              <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mb: 0.5 }}>
                <Typography variant="caption" sx={{ color: '#4CAF50', fontWeight: 600 }}>
                  $
                </Typography>
                <Typography variant="body2" sx={{ fontFamily: 'monospace' }}>
                  {item.content}
                </Typography>
              </Box>
            )}
            {item.type === 'blocked' && (
              <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mb: 0.5 }}>
                <AlertTriangle size={14} color="#F44336" />
                <Typography variant="body2" sx={{ fontFamily: 'monospace', color: '#F44336' }}>
                  {item.content}
                </Typography>
              </Box>
            )}
            {(item.type === 'stdout' || item.type === 'stderr' || item.type === 'error') && (
              <Typography variant="body2" sx={{ fontFamily: 'monospace' }}>
                {item.content}
              </Typography>
            )}
          </Box>
        ))}
        {isExecuting && (
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
            <Typography variant="caption" sx={{ color: '#FF9800' }}>
              実行中...
            </Typography>
          </Box>
        )}
        <div ref={outputEndRef} />
      </Box>

      {/* Command Input */}
      <Box
        component="form"
        onSubmit={handleCommandSubmit}
        sx={{
          p: 1,
          bgcolor: '#2d2d2d',
          borderTop: '1px solid rgba(255, 255, 255, 0.1)',
          display: 'flex',
          alignItems: 'center',
          gap: 1,
        }}
      >
        <Typography variant="caption" sx={{ color: '#4CAF50', fontWeight: 600 }}>
          $
        </Typography>
        <Autocomplete
          freeSolo
          options={availableCommands}
          value={command}
          onChange={(_, newValue) => setCommand(typeof newValue === 'string' ? newValue : newValue || '')}
          onInputChange={(_, newInputValue) => setCommand(newInputValue)}
          inputValue={command}
          renderInput={(params) => (
            <TextField
              {...params}
              fullWidth
              placeholder="コマンドを入力..."
              onKeyDown={handleKeyDown}
              disabled={!sessionId || isExecuting}
              sx={{
                '& .MuiOutlinedInput-root': {
                  color: 'white',
                  bgcolor: '#1e1e1e',
                  '& fieldset': {
                    borderColor: 'rgba(255, 255, 255, 0.2)',
                  },
                  '&:hover fieldset': {
                    borderColor: 'rgba(255, 255, 255, 0.3)',
                  },
                  '&.Mui-focused fieldset': {
                    borderColor: '#4CAF50',
                  },
                },
                '& .MuiInputBase-input': {
                  fontFamily: 'monospace',
                  fontSize: '12px',
                },
              }}
            />
          )}
        />
        <Tooltip title="実行 (Enter)">
          <IconButton
            type="submit"
            disabled={!sessionId || isExecuting || !command.trim()}
            sx={{ color: '#4CAF50' }}
          >
            <Play size={16} />
          </IconButton>
        </Tooltip>
      </Box>

      {/* Available Commands Hint */}
      {availableCommands.length > 0 && (
        <Box
          sx={{
            p: 1,
            bgcolor: '#2d2d2d',
            borderTop: '1px solid rgba(255, 255, 255, 0.1)',
            maxHeight: 100,
            overflow: 'auto',
          }}
        >
          <Typography variant="caption" sx={{ color: '#888', fontSize: '10px', mb: 0.5, display: 'block' }}>
            利用可能なコマンド:
          </Typography>
          <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 0.5 }}>
            {availableCommands.slice(0, 20).map(cmd => (
              <Chip
                key={cmd}
                label={cmd}
                size="small"
                sx={{
                  bgcolor: '#3d3d3d',
                  color: 'white',
                  fontSize: '10px',
                  height: 20,
                  '&:hover': {
                    bgcolor: '#4d4d4d',
                    cursor: 'pointer',
                  },
                }}
                onClick={() => setCommand(cmd)}
              />
            ))}
          </Box>
        </Box>
      )}
    </Box>
  );
}

