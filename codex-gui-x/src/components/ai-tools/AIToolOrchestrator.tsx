'use client'

import { useState, useEffect } from 'react'
let taskCounter = 0;
let subtaskCounter = 0;
import { Card } from '../atoms/Card'
import { Badge } from '../atoms/Badge'
import { Progress } from '../atoms/Progress'
import type { AITool, AISession, DevelopmentTask, ExecutionResult } from '../../types/ai-tools'
import {
  Play,
  Square,
  Zap,
  CheckCircle,
  Clock,
  Cpu,
  Activity,
  Users,
  Target,
  TrendingUp
} from 'lucide-react'

interface AIToolOrchestratorProps {
  aiTools: AITool[]
  tasks: DevelopmentTask[]
  sessions: AISession[]
  onTaskExecute: (task: DevelopmentTask) => void
  onTaskComplete: (taskId: string, result: ExecutionResult) => void
  onSessionUpdate: (session: AISession) => void
}

export function AIToolOrchestrator({
  aiTools,
  tasks,
  sessions,
  onTaskExecute,
  onTaskComplete,
}: AIToolOrchestratorProps) {
  const [selectedTask, setSelectedTask] = useState<DevelopmentTask | null>(null)
  const [isExecuting, setIsExecuting] = useState(false)
  const [executionProgress, setExecutionProgress] = useState(0)
  const [naturalLanguagePrompt, setNaturalLanguagePrompt] = useState('')
  const [selectedTools, setSelectedTools] = useState<string[]>([])
  const [isExecutingNaturalLanguage, setIsExecutingNaturalLanguage] = useState(false)
  const [naturalLanguageProgress, setNaturalLanguageProgress] = useState(0)

  // Available AI tools for parallel execution
  const availableAITools = aiTools.filter(tool => 
    ['codex', 'opencode', 'claudecode', 'geminicli'].some(name => 
      tool.name.toLowerCase().includes(name)
    )
  )

  // Simulate parallel execution
  useEffect(() => {
    if (isExecuting && selectedTask) {
      const interval = setInterval(() => {
        setExecutionProgress(prev => {
          if (prev >= 100) {
            setIsExecuting(false)
            // Generate mock execution result
            const result: ExecutionResult = {
              taskId: selectedTask.id,
              success: true,
              integratedOutput: `Task "${selectedTask.title}" completed successfully using ${selectedTask.assignedTools.length} AI tools.`,
              subtaskResults: selectedTask.subtasks.map(subtask => ({
                subtaskId: subtask.id,
                toolId: subtask.assignedTool,
                success: true,
                output: `Completed: ${subtask.description}`,
                executionTime: Math.random() * 10 + 5,
                qualityScore: 0.8 + Math.random() * 0.2,
              })),
              errors: [],
              executionTime: Math.random() * 30 + 15,
              qualityScore: 0.85 + Math.random() * 0.15,
              recommendations: [
                'Review generated code for consistency',
                'Run automated tests',
                'Consider performance optimizations',
              ],
            }
            onTaskComplete(selectedTask.id, result)
            return 100
          }
          return prev + Math.random() * 15
        })
      }, 1000)

      return () => clearInterval(interval)
    }
  }, [isExecuting, selectedTask, onTaskComplete])

  const handleExecuteTask = (task: DevelopmentTask) => {
    setSelectedTask(task)
    setIsExecuting(true)
    setExecutionProgress(0)
    onTaskExecute(task)
  }

  const handleStopExecution = () => {
    setIsExecuting(false)
    setExecutionProgress(0)
    setSelectedTask(null)
  }

  const handleExecuteNaturalLanguagePrompt = async () => {
    if (!naturalLanguagePrompt.trim() || selectedTools.length === 0) {
      return
    }

    setIsExecutingNaturalLanguage(true)
    setNaturalLanguageProgress(0)

    try {
      // Use counters for unique IDs to avoid "impure function" lint errors
      taskCounter += 1;
      const taskId = `task_nl_${taskCounter}`
      // Create a development task from natural language prompt
      const task: DevelopmentTask = {
        id: taskId,
        title: naturalLanguagePrompt.substring(0, 50) + (naturalLanguagePrompt.length > 50 ? '...' : ''),
        description: naturalLanguagePrompt,
        assignedTools: selectedTools,
        subtasks: selectedTools.map((toolId) => {
          subtaskCounter += 1;
          return {
            id: `subtask_nl_${subtaskCounter}`,
            parentTaskId: taskId,
            description: `Execute with ${availableAITools.find(t => t.id === toolId)?.name || toolId}`,
            assignedTool: toolId,
            status: 'pending' as const,
          };
        }),
        status: 'running' as const,
        progress: 0,
        complexity: 'medium' as const,
        priority: 'high' as const,
        requirements: ['Parallel Execution'],
        createdAt: new Date(),
      }

      // Execute task
      onTaskExecute(task)

      // Simulate parallel execution progress
      const progressInterval = setInterval(() => {
        setNaturalLanguageProgress(prev => {
          if (prev >= 100) {
            clearInterval(progressInterval)
            setIsExecutingNaturalLanguage(false)
            
            // Generate execution result
            const result: ExecutionResult = {
              taskId: task.id,
              success: true,
              integratedOutput: `Natural language prompt executed successfully using ${selectedTools.length} AI tools in parallel.`,
              subtaskResults: task.subtasks.map(subtask => ({
                subtaskId: subtask.id,
                toolId: subtask.assignedTool,
                success: true,
                output: `Completed: ${subtask.description}`,
                executionTime: Math.random() * 10 + 5,
                qualityScore: 0.8 + Math.random() * 0.2,
              })),
              errors: [],
              executionTime: Math.random() * 30 + 15,
              qualityScore: 0.85 + Math.random() * 0.15,
              recommendations: [
                'Review results from all AI tools',
                'Compare outputs for consistency',
                'Integrate best solutions',
              ],
            }
            onTaskComplete(task.id, result)
            return 100
          }
          return prev + Math.random() * 10
        })
      }, 500)

      // Cleanup on unmount
      return () => clearInterval(progressInterval)
    } catch (error) {
      console.error('Failed to execute natural language prompt:', error)
      setIsExecutingNaturalLanguage(false)
      setNaturalLanguageProgress(0)
    }
  }

  const getTaskComplexityColor = (complexity: DevelopmentTask['complexity']) => {
    switch (complexity) {
      case 'simple': return 'bg-green-100 text-green-800'
      case 'medium': return 'bg-blue-100 text-blue-800'
      case 'complex': return 'bg-yellow-100 text-yellow-800'
      case 'critical': return 'bg-red-100 text-red-800'
    }
  }



  const getTaskStatusColor = (status: DevelopmentTask['status']) => {
    switch (status) {
      case 'pending': return 'bg-gray-100 text-gray-800'
      case 'running': return 'bg-blue-100 text-blue-800'
      case 'completed': return 'bg-green-100 text-green-800'
      case 'failed': return 'bg-red-100 text-red-800'
    }
  }

  const getToolStatusColor = (status: AITool['status']) => {
    switch (status) {
      case 'available': return 'bg-green-500/20 text-green-400 border-green-500/30'
      case 'running': return 'bg-blue-500/20 text-blue-400 border-blue-500/30'
      case 'busy': return 'bg-yellow-500/20 text-yellow-400 border-yellow-500/30'
      case 'error': return 'bg-red-500/20 text-red-400 border-red-500/30'
    }
  }

  const activeSessions = sessions.filter(s => s.status === 'running' || s.status === 'starting')
  const busyTools = aiTools.filter(t => t.status === 'busy' || t.status === 'running')
  const runningTasks = tasks.filter(t => t.status === 'running')

  return (
    <div className="h-full p-6 space-y-6 overflow-y-auto bg-background/50 backdrop-blur-sm">
      {/* Overview Stats */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        {[
          { label: 'Active Sessions', value: activeSessions.length, icon: Users, color: 'text-blue-400' },
          { label: 'Running Tasks', value: runningTasks.length, icon: Target, color: 'text-green-400' },
          { label: 'Tools Active', value: `${busyTools.length}/${aiTools.length}`, icon: Activity, color: 'text-purple-400' },
          { label: 'Avg Success Rate', value: `${aiTools.length > 0 ? Math.round(aiTools.reduce((sum, t) => sum + t.performance.successRate, 0) / aiTools.length) : 0}%`, icon: TrendingUp, color: 'text-orange-400' }
        ].map((stat, i) => (
          <Card key={i} className="p-4 border-white/5 bg-white/5 hover:bg-white/10 transition-colors">
            <div className="flex items-center gap-3">
              <stat.icon className={`w-8 h-8 ${stat.color}`} />
              <div>
                <div className="text-2xl font-bold tracking-tight">{stat.value}</div>
                <div className="text-xs font-medium text-muted-foreground uppercase tracking-wider">{stat.label}</div>
              </div>
            </div>
          </Card>
        ))}
      </div>

      {/* Execution Control */}
      {isExecuting && selectedTask && (
        <Card className="p-6 border-primary/20 bg-primary/5 backdrop-blur-md relative overflow-hidden group">
          <div className="absolute inset-0 bg-gradient-to-r from-primary/10 to-transparent opacity-0 group-hover:opacity-100 transition-opacity" />
          <div className="relative z-10 flex items-center justify-between mb-4">
            <div>
              <h3 className="text-lg font-semibold text-primary">
                Executing: {selectedTask.title}
              </h3>
              <p className="text-sm text-muted-foreground">
                Using {selectedTask.assignedTools.length} AI tools in parallel
              </p>
            </div>

            <div className="flex items-center gap-6">
              <div className="text-right">
                <div className="text-3xl font-bold text-primary tabular-nums">
                  {executionProgress.toFixed(0)}%
                </div>
                <div className="text-[10px] font-bold text-primary/60 uppercase tracking-tighter">Overall Progress</div>
              </div>
              <button 
                onClick={handleStopExecution}
                className="p-2 h-10 w-10 flex items-center justify-center rounded-xl bg-destructive/10 text-destructive border border-destructive/20 hover:bg-destructive hover:text-white transition-all"
              >
                <Square size={20} fill="currentColor" />
              </button>
            </div>
          </div>

          <Progress value={executionProgress} className="h-2 mb-6" />

          <div className="grid grid-cols-1 md:grid-cols-3 gap-6 text-xs font-medium relative z-10">
            <div className="flex items-center gap-3 text-blue-400">
              <Activity size={16} />
              <span>Initializing parallel execution...</span>
            </div>
            <div className="flex items-center gap-3 text-green-400">
              <Target size={16} />
              <span>{selectedTask.subtasks.length} subtasks distributed</span>
            </div>
            <div className="flex items-center gap-3 text-purple-400">
              <Clock size={16} />
              <span>Estimated finish: ~{Math.ceil((100 - executionProgress) / 10)}m</span>
            </div>
          </div>
        </Card>
      )}

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* AI Tools Status */}
        <Card className="p-6 border-white/5 bg-white/5">
          <div className="flex items-center justify-between mb-6">
            <h2 className="text-xl font-bold flex items-center gap-2">
              <Cpu size={20} className="text-primary" />
              AI Tools Cluster
            </h2>
            <Badge variant="outline" className="border-primary/20 text-primary">Live</Badge>
          </div>

          <div className="space-y-3">
            {aiTools.map((tool) => (
              <div key={tool.id} className="flex items-center justify-between p-4 border border-white/5 bg-white/2 hover:bg-white/5 rounded-xl transition-all group">
                <div className="flex items-center gap-4">
                  <div className={`w-2.5 h-2.5 rounded-full shadow-lg ${
                    tool.status === 'available' ? 'bg-green-500 shadow-green-500/20' :
                    tool.status === 'running' ? 'bg-blue-500 shadow-blue-500/20' :
                    tool.status === 'busy' ? 'bg-yellow-500 shadow-yellow-500/20' : 'bg-red-500 shadow-red-500/20'
                  }`} />

                  <div>
                    <h3 className="font-semibold text-sm group-hover:text-primary transition-colors">{tool.name}</h3>
                    <div className="flex gap-1.5 mt-1.5">
                      {tool.capabilities.slice(0, 2).map((cap, index) => (
                        <span key={index} className="text-[10px] px-1.5 py-0.5 rounded bg-white/5 border border-white/5 text-muted-foreground uppercase font-bold tracking-tighter">
                          {cap}
                        </span>
                      ))}
                    </div>
                  </div>
                </div>

                <div className="text-right">
                  <Badge variant="outline" className={getToolStatusColor(tool.status)}>
                    {tool.status.toUpperCase()}
                  </Badge>
                  <div className="text-[10px] font-bold text-muted-foreground mt-1.5 tabular-nums">
                    {tool.activeSessions}/{tool.maxSessions} LOAD
                  </div>
                </div>
              </div>
            ))}
          </div>
        </Card>

        {/* Task Queue */}
        <Card className="p-6 border-white/5 bg-white/5">
          <div className="flex items-center justify-between mb-6">
            <h2 className="text-xl font-bold flex items-center gap-2">
              <Zap size={20} className="text-yellow-400" />
              Task Queue
            </h2>
            <button className="text-xs font-bold text-primary hover:underline">View All History</button>
          </div>

          <div className="space-y-4 max-h-[480px] overflow-y-auto pr-2 custom-scrollbar">
            {tasks.map((task) => (
              <div key={task.id} className="p-4 border border-white/5 bg-white/2 rounded-xl border-l-4 border-l-primary/30">
                <div className="flex items-center justify-between mb-3">
                  <div className="space-y-1">
                    <div className="flex items-center gap-2">
                      <h3 className="font-bold text-sm">{task.title}</h3>
                      <span className={`text-[10px] px-1.5 py-0.5 rounded-full font-bold uppercase tracking-tighter ${getTaskComplexityColor(task.complexity)}`}>
                        {task.complexity}
                      </span>
                    </div>
                    <div className="text-[10px] text-muted-foreground font-mono">ID: {task.id.substring(0, 8)}</div>
                  </div>

                  <div className="flex items-center gap-2">
                    {task.status === 'pending' && (
                      <button
                        onClick={() => handleExecuteTask(task)}
                        disabled={isExecuting}
                        className="h-8 px-4 rounded-lg bg-primary text-primary-foreground text-xs font-bold hover:scale-105 active:scale-95 disabled:opacity-50 transition-all flex items-center gap-2"
                      >
                        <Play size={12} fill="currentColor" />
                        RUN
                      </button>
                    )}
                    {task.status !== 'pending' && (
                      <Badge variant="outline" className={getTaskStatusColor(task.status)}>
                        {task.status.toUpperCase()}
                      </Badge>
                    )}
                  </div>
                </div>

                <p className="text-xs text-muted-foreground line-clamp-2 mb-4 leading-relaxed">{task.description}</p>

                <div className="flex items-center justify-between pt-3 border-t border-white/5 text-[10px] font-bold">
                  <div className="flex items-center gap-4 text-muted-foreground uppercase tracking-widest">
                    <span className="flex items-center gap-1"><Cpu size={10} /> {task.assignedTools.length} TOOLS</span>
                    <span className="flex items-center gap-1"><Target size={10} /> {task.subtasks.length} SUBTASKS</span>
                  </div>

                  {task.status === 'running' && (
                    <div className="flex items-center gap-3">
                      <Progress value={task.progress} className="w-16 h-1.5" />
                      <span className="tabular-nums text-primary">{task.progress}%</span>
                    </div>
                  )}
                </div>
              </div>
            ))}

            {tasks.length === 0 && (
              <div className="text-center py-12 text-muted-foreground/30">
                <Target className="w-16 h-16 mx-auto mb-4 opacity-20" />
                <p className="text-sm font-bold uppercase tracking-widest">No Active Tasks</p>
                <p className="text-[10px] mt-1">Initiate work from the distribution hub</p>
              </div>
            )}
          </div>
        </Card>
      </div>

      {/* Natural Language Prompt Execution */}
      <Card className="p-8 border-primary/20 bg-gradient-to-br from-primary/10 via-purple-500/5 to-transparent relative overflow-hidden group">
        <div className="absolute top-0 right-0 p-8 opacity-10 group-hover:scale-110 transition-transform">
          <Zap size={120} className="text-primary fill-primary" />
        </div>
        
        <div className="relative z-10">
          <h2 className="text-2xl font-bold mb-2 flex items-center gap-3">
            <Zap className="w-7 h-7 text-primary fill-primary" />
            Parallel Intelligence Hub
          </h2>
          <p className="text-sm text-muted-foreground mb-8">Deploy multiple specialized agents across your codebase simultaneously.</p>

          <div className="space-y-6">
            <div className="relative">
              <label className="block text-[10px] font-bold text-muted-foreground uppercase tracking-widest mb-3 ml-1">
                Command Specification
              </label>
              <textarea
                id="natural-language-prompt"
                className="w-full h-32 p-6 bg-black/40 border border-white/5 rounded-2xl focus:ring-2 focus:ring-primary/50 focus:border-primary transition-all resize-none shadow-inner"
                placeholder="e.g. Implement a high-performance Rust backend for file indexing with concurrent lock-free data structures..."
                value={naturalLanguagePrompt}
                onChange={(e) => setNaturalLanguagePrompt(e.target.value)}
              />
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
              <div>
                <label className="block text-[10px] font-bold text-muted-foreground uppercase tracking-widest mb-4 ml-1">
                  Select Specialized Agents
                </label>
                <div className="flex flex-wrap gap-2">
                  {availableAITools.map((tool) => (
                    <label
                      key={tool.id}
                      className={`flex items-center gap-3 px-4 py-2.5 border rounded-xl cursor-pointer transition-all ${
                        selectedTools.includes(tool.id) 
                        ? 'bg-primary/20 border-primary text-primary shadow-lg shadow-primary/10' 
                        : 'bg-white/5 border-white/5 hover:bg-white/10 text-muted-foreground'
                      }`}
                    >
                      <input
                        type="checkbox"
                        checked={selectedTools.includes(tool.id)}
                        onChange={(e) => {
                          if (e.target.checked) setSelectedTools([...selectedTools, tool.id])
                          else setSelectedTools(selectedTools.filter(id => id !== tool.id))
                        }}
                        className="hidden"
                      />
                      <span className="text-xs font-bold tracking-tight">{tool.name}</span>
                      {selectedTools.includes(tool.id) && <CheckCircle size={14} fill="currentColor" className="text-primary-foreground" />}
                    </label>
                  ))}
                </div>
              </div>

              <div className="flex flex-col justify-end">
                <div className="flex items-center justify-between mb-4 px-2">
                  <div className="text-[10px] font-bold uppercase tracking-widest">
                    {selectedTools.length > 0 ? (
                      <span className="text-primary italic animate-pulse">
                        Ready: {selectedTools.length} Agents Assigned
                      </span>
                    ) : (
                      <span className="text-destructive">Minimum 1 Agent Required</span>
                    )}
                  </div>
                </div>
                <div className="flex gap-3">
                  <button
                    onClick={() => { setNaturalLanguagePrompt(''); setSelectedTools([]) }}
                    className="flex-1 h-12 rounded-xl bg-white/5 border border-white/5 font-bold text-xs hover:bg-white/10 transition-all uppercase tracking-widest"
                  >
                    Reset
                  </button>
                  <button
                    onClick={handleExecuteNaturalLanguagePrompt}
                    disabled={isExecuting || !naturalLanguagePrompt.trim() || selectedTools.length === 0}
                    className="flex-[2] h-12 rounded-xl bg-primary text-primary-foreground font-bold text-xs hover:scale-105 active:scale-95 shadow-xl shadow-primary/20 transition-all flex items-center justify-center gap-2 uppercase tracking-widest"
                  >
                    <Play size={16} fill="currentColor" />
                    Deploy Parallel Agents
                  </button>
                </div>
              </div>
            </div>

            {isExecutingNaturalLanguage && (
              <div className="mt-8 p-6 bg-primary/10 border border-primary/20 rounded-2xl backdrop-blur-md">
                <div className="flex items-center justify-between mb-3 text-xs font-bold uppercase tracking-widest">
                  <span className="text-primary flex items-center gap-2">
                    <Activity size={14} className="animate-spin" />
                    Processing with Intelligence Cluster...
                  </span>
                  <span className="tabular-nums text-primary">{naturalLanguageProgress}%</span>
                </div>
                <Progress value={naturalLanguageProgress} className="h-2" />
                <div className="mt-4 flex flex-wrap gap-2 italic text-[10px] text-primary/60">
                  Executing: {selectedTools.map(id => availableAITools.find(t => t.id === id)?.name).join(' • ')}
                </div>
              </div>
            )}
          </div>
        </div>
      </Card>

      {/* Quick Actions */}
      <Card className="p-6 border-white/5 bg-white/5">
        <h2 className="text-sm font-bold uppercase tracking-widest text-muted-foreground mb-6 ml-1">Orchestration Shortcuts</h2>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          {[
            { icon: Zap, label: 'Quick Task', desc: 'Auto-detect best agent' },
            { icon: Target, label: 'Complex Task', desc: 'Orchestrated execution' },
            { icon: Activity, label: 'Monitor cluster', desc: 'Real-time telemetry' },
            { icon: CheckCircle, label: 'View Results', desc: 'Audit generated code' }
          ].map((action, i) => (
            <button key={i} className="p-6 border border-white/5 bg-white/2 hover:bg-primary/5 hover:border-primary/20 transition-all rounded-2xl text-left group">
              <action.icon className="w-7 h-7 mb-4 text-muted-foreground group-hover:text-primary transition-colors" />
              <div className="font-bold text-xs uppercase tracking-tight group-hover:text-primary transition-colors">{action.label}</div>
              <div className="text-[10px] text-muted-foreground mt-1 group-hover:text-muted-foreground/80">{action.desc}</div>
            </button>
          ))}
        </div>
      </Card>
    </div>
  )
}
