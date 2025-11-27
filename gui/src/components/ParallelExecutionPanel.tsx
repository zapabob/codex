'use client';

import React, { useState, useEffect } from 'react';
import {
  Box,
  Paper,
  Typography,
  Button,
  Slider,
  Chip,
  Avatar,
  LinearProgress,
  Alert,
  Collapse,
  IconButton,
} from '@mui/material';
import Grid from '@/mui/Grid2';
import { motion, AnimatePresence } from 'framer-motion';
import {
  Play,
  Square,
  Settings,
  Cpu,
  HardDrive,
  Thermometer,
  Zap,
  Brain,
  Code2,
  Shield,
  ChevronDown,
  ChevronUp,
} from 'lucide-react';
import { useCodex } from '@/lib/context/CodexContext';

interface AgentConfig {
  id: string;
  name: string;
  icon: React.ComponentType<any>;
  color: string;
  description: string;
  maxConcurrent: number;
}

const AVAILABLE_AGENTS: AgentConfig[] = [
  {
    id: 'codex',
    name: 'Codex',
    icon: Brain,
    color: '#6366f1',
    description: 'AI-Native Development Assistant',
    maxConcurrent: 3,
  },
  {
    id: 'gemini-cli',
    name: 'Gemini CLI',
    icon: Code2,
    color: '#06b6d4',
    description: 'Google Gemini Command Line Interface',
    maxConcurrent: 2,
  },
  {
    id: 'claude-code',
    name: 'Claude Code',
    icon: Shield,
    color: '#f59e0b',
    description: 'Anthropic Claude Code Assistant',
    maxConcurrent: 4,
  },
];

interface ExecutionTask {
  id: string;
  agentId: string;
  status: 'pending' | 'running' | 'completed' | 'failed';
  progress: number;
  startTime?: Date;
  endTime?: Date;
  error?: string;
}

export const ParallelExecutionPanel: React.FC = () => {
  const { executeCommand, getResourceUsage } = useCodex();
  const [selectedAgents, setSelectedAgents] = useState<Set<string>>(new Set());
  const [concurrentCount, setConcurrentCount] = useState(2);
  const [isExecuting, setIsExecuting] = useState(false);
  const [executionTasks, setExecutionTasks] = useState<ExecutionTask[]>([]);
  const [resourceUsage, setResourceUsage] = useState({
    cpu: 0,
    memory: 0,
    gpu: 0,
    temperature: 0,
  });
  const [showAdvanced, setShowAdvanced] = useState(false);
  const [alerts, setAlerts] = useState<string[]>([]);

  // Resource monitoring
  useEffect(() => {
    const interval = setInterval(async () => {
      try {
        const usage = await getResourceUsage();
        setResourceUsage(usage);

        // Check limits
        const newAlerts: string[] = [];
        if (usage.memory > 85) {
          newAlerts.push('Memory usage above 85% - consider reducing concurrent tasks');
        }
        if (usage.temperature > 80) {
          newAlerts.push('High temperature detected - monitor cooling');
        }
        if (usage.cpu > 90) {
          newAlerts.push('High CPU usage - system may be overloaded');
        }

        setAlerts(newAlerts);
      } catch (error) {
        console.error('Failed to get resource usage:', error);
      }
    }, 2000);

    return () => clearInterval(interval);
  }, [getResourceUsage]);

  const handleAgentToggle = (agentId: string) => {
    const newSelected = new Set(selectedAgents);
    if (newSelected.has(agentId)) {
      newSelected.delete(agentId);
    } else {
      newSelected.add(agentId);
    }
    setSelectedAgents(newSelected);

    // Auto-adjust concurrent count based on selected agents
    const maxAllowed = Array.from(newSelected).reduce((max, id) => {
      const agent = AVAILABLE_AGENTS.find(a => a.id === id);
      return Math.min(max, agent?.maxConcurrent || 1);
    }, 5);

    if (concurrentCount > maxAllowed) {
      setConcurrentCount(maxAllowed);
    }
  };

  const handleExecute = async () => {
    if (selectedAgents.size === 0) return;

    setIsExecuting(true);
    const tasks: ExecutionTask[] = [];

    // Create tasks for each selected agent
    Array.from(selectedAgents).forEach((agentId, index) => {
      for (let i = 0; i < concurrentCount; i++) {
        tasks.push({
          id: `${agentId}-${index}-${i}`,
          agentId,
          status: 'pending',
          progress: 0,
        });
      }
    });

    setExecutionTasks(tasks);

    // Execute tasks in parallel with resource monitoring
    const executionPromises = tasks.map(async (task, index) => {
      // Check resource limits before starting
      const currentUsage = await getResourceUsage();
      if (currentUsage.memory > 85 || currentUsage.cpu > 90) {
        setExecutionTasks(prev =>
          prev.map(t =>
            t.id === task.id
              ? { ...t, status: 'failed', error: 'Resource limits exceeded' }
              : t
          )
        );
        return;
      }

      setExecutionTasks(prev =>
        prev.map(t =>
          t.id === task.id
            ? { ...t, status: 'running', startTime: new Date() }
            : t
        )
      );

      try {
        // Simulate execution with progress updates
        for (let progress = 0; progress <= 100; progress += 10) {
          await new Promise(resolve => setTimeout(resolve, 500));
          setExecutionTasks(prev =>
            prev.map(t =>
              t.id === task.id ? { ...t, progress } : t
            )
          );
        }

        const result = await executeCommand(
          `${task.agentId} --task "parallel-execution-${index}"`,
          { agentId: task.agentId, taskIndex: index }
        );

        setExecutionTasks(prev =>
          prev.map(t =>
            t.id === task.id
              ? { ...t, status: 'completed', endTime: new Date() }
              : t
          )
        );

      } catch (error) {
        setExecutionTasks(prev =>
          prev.map(t =>
            t.id === task.id
              ? { ...t, status: 'failed', error: error.message }
              : t
          )
        );
      }
    });

    await Promise.all(executionPromises);
    setIsExecuting(false);
  };

  const handleStop = () => {
    setIsExecuting(false);
    setExecutionTasks([]);
  };

  const getAgentById = (id: string) => AVAILABLE_AGENTS.find(a => a.id === id);

  const activeTasks = executionTasks.filter(t => t.status === 'running').length;
  const completedTasks = executionTasks.filter(t => t.status === 'completed').length;
  const failedTasks = executionTasks.filter(t => t.status === 'failed').length;
  const totalProgress = executionTasks.length > 0
    ? executionTasks.reduce((sum, t) => sum + t.progress, 0) / executionTasks.length
    : 0;

  return (
    <Paper
      elevation={2}
      sx={{
        p: 3,
        borderRadius: 2,
        background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
        color: 'white',
      }}
    >
      <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, mb: 3 }}>
        <Zap size={24} />
        <Typography variant="h5" fontWeight="bold">
          Parallel AI Execution
        </Typography>
      </Box>

      {/* Resource Usage Display */}
      <Grid container spacing={2} sx={{ mb: 3 }}>
        <Grid xs={12} sm={6} md={3}>
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
            <Cpu size={16} />
            <Typography variant="body2">CPU: {resourceUsage.cpu}%</Typography>
          </Box>
        </Grid>
        <Grid xs={12} sm={6} md={3}>
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
            <HardDrive size={16} />
            <Typography variant="body2">Memory: {resourceUsage.memory}%</Typography>
          </Box>
        </Grid>
        <Grid xs={12} sm={6} md={3}>
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
            <Thermometer size={16} />
            <Typography variant="body2">Temp: {resourceUsage.temperature}°C</Typography>
          </Box>
        </Grid>
        <Grid xs={12} sm={6} md={3}>
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
            <Brain size={16} />
            <Typography variant="body2">Active: {activeTasks}</Typography>
          </Box>
        </Grid>
      </Grid>

      {/* Alerts */}
      <AnimatePresence>
        {alerts.map((alert, index) => (
          <motion.div
            key={index}
            initial={{ opacity: 0, y: -20 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -20 }}
          >
            <Alert severity="warning" sx={{ mb: 2 }}>
              {alert}
            </Alert>
          </motion.div>
        ))}
      </AnimatePresence>

      {/* Agent Selection */}
      <Box sx={{ mb: 3 }}>
        <Typography variant="h6" gutterBottom>
          Select AI Agents
        </Typography>
        <Grid container spacing={2}>
          {AVAILABLE_AGENTS.map((agent) => {
            const isSelected = selectedAgents.has(agent.id);
            const AgentIcon = agent.icon;

            return (
              <Grid xs={12} sm={4} key={agent.id}>
                <motion.div
                  whileHover={{ scale: 1.02 }}
                  whileTap={{ scale: 0.98 }}
                >
                  <Paper
                    sx={{
                      p: 2,
                      cursor: 'pointer',
                      border: `2px solid ${isSelected ? agent.color : 'rgba(255,255,255,0.2)'}`,
                      backgroundColor: isSelected ? 'rgba(255,255,255,0.1)' : 'transparent',
                      transition: 'all 0.3s ease',
                      '&:hover': {
                        backgroundColor: 'rgba(255,255,255,0.05)',
                      },
                    }}
                    onClick={() => handleAgentToggle(agent.id)}
                  >
                    <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
                      <Avatar sx={{ bgcolor: agent.color, width: 40, height: 40 }}>
                        <AgentIcon size={20} />
                      </Avatar>
                      <Box>
                        <Typography variant="subtitle1" fontWeight="bold">
                          {agent.name}
                        </Typography>
                        <Typography variant="body2" sx={{ opacity: 0.8 }}>
                          {agent.description}
                        </Typography>
                      </Box>
                    </Box>
                  </Paper>
                </motion.div>
              </Grid>
            );
          })}
        </Grid>
      </Box>

      {/* Concurrent Count Slider */}
      {selectedAgents.size > 0 && (
        <Box sx={{ mb: 3 }}>
          <Typography variant="h6" gutterBottom>
            Concurrent Execution Count: {concurrentCount}
          </Typography>
          <Slider
            value={concurrentCount}
            onChange={(_, value) => setConcurrentCount(value as number)}
            min={1}
            max={Math.min(
              5,
              Array.from(selectedAgents).reduce((max, id) => {
                const agent = AVAILABLE_AGENTS.find(a => a.id === id);
                return Math.min(max, agent?.maxConcurrent || 1);
              }, 5)
            )}
            marks
            valueLabelDisplay="auto"
            sx={{
              color: 'white',
              '& .MuiSlider-thumb': {
                backgroundColor: 'white',
              },
              '& .MuiSlider-track': {
                backgroundColor: 'white',
              },
              '& .MuiSlider-rail': {
                backgroundColor: 'rgba(255,255,255,0.3)',
              },
            }}
          />
        </Box>
      )}

      {/* Advanced Settings */}
      <Box sx={{ mb: 3 }}>
        <Button
          onClick={() => setShowAdvanced(!showAdvanced)}
          startIcon={showAdvanced ? <ChevronUp /> : <ChevronDown />}
          sx={{ color: 'white' }}
        >
          Advanced Settings
        </Button>
        <Collapse in={showAdvanced}>
          <Box sx={{ mt: 2, p: 2, backgroundColor: 'rgba(255,255,255,0.1)', borderRadius: 1 }}>
            <Typography variant="body2" sx={{ mb: 2 }}>
              Advanced execution parameters will be configurable here.
            </Typography>
          </Box>
        </Collapse>
      </Box>

      {/* Execution Controls */}
      <Box sx={{ display: 'flex', gap: 2, mb: 3 }}>
        <Button
          variant="contained"
          startIcon={<Play />}
          onClick={handleExecute}
          disabled={selectedAgents.size === 0 || isExecuting}
          sx={{
            backgroundColor: 'rgba(255,255,255,0.2)',
            '&:hover': { backgroundColor: 'rgba(255,255,255,0.3)' },
            '&:disabled': { opacity: 0.5 },
          }}
        >
          Execute ({selectedAgents.size} agents × {concurrentCount})
        </Button>

        {isExecuting && (
          <Button
            variant="outlined"
            startIcon={<Square />}
            onClick={handleStop}
            sx={{
              borderColor: 'rgba(255,255,255,0.5)',
              color: 'white',
              '&:hover': { borderColor: 'white', backgroundColor: 'rgba(255,255,255,0.1)' },
            }}
          >
            Stop
          </Button>
        )}
      </Box>

      {/* Execution Progress */}
      {executionTasks.length > 0 && (
        <Box sx={{ mb: 3 }}>
          <Typography variant="h6" gutterBottom>
            Execution Progress
          </Typography>
          <LinearProgress
            variant="determinate"
            value={totalProgress}
            sx={{
              height: 8,
              borderRadius: 4,
              backgroundColor: 'rgba(255,255,255,0.2)',
              '& .MuiLinearProgress-bar': {
                backgroundColor: 'white',
              },
            }}
          />

          <Box sx={{ display: 'flex', gap: 2, mt: 2 }}>
            <Chip
              label={`Running: ${activeTasks}`}
              sx={{ backgroundColor: 'rgba(59, 130, 246, 0.2)', color: 'white' }}
            />
            <Chip
              label={`Completed: ${completedTasks}`}
              sx={{ backgroundColor: 'rgba(34, 197, 94, 0.2)', color: 'white' }}
            />
            <Chip
              label={`Failed: ${failedTasks}`}
              sx={{ backgroundColor: 'rgba(239, 68, 68, 0.2)', color: 'white' }}
            />
          </Box>
        </Box>
      )}

      {/* Task Details */}
      {executionTasks.length > 0 && (
        <Box>
          <Typography variant="h6" gutterBottom>
            Task Details
          </Typography>
          <Grid container spacing={2}>
            {executionTasks.slice(0, 6).map((task) => {
              const agent = getAgentById(task.agentId);
              return (
                <Grid xs={12} sm={6} md={4} key={task.id}>
                  <Paper sx={{ p: 2, backgroundColor: 'rgba(255,255,255,0.1)' }}>
                    <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mb: 1 }}>
                      {agent && <agent.icon size={16} />}
                      <Typography variant="body2" fontWeight="bold">
                        {agent?.name} #{task.id.split('-').pop()}
                      </Typography>
                    </Box>
                    <LinearProgress
                      variant="determinate"
                      value={task.progress}
                      sx={{ mb: 1, height: 4 }}
                    />
                    <Typography variant="caption" sx={{ opacity: 0.8 }}>
                      {task.status.toUpperCase()}
                      {task.error && ` - ${task.error}`}
                    </Typography>
                  </Paper>
                </Grid>
              );
            })}
          </Grid>
        </Box>
      )}
    </Paper>
  );
};
