'use client'

import { useState, useEffect } from 'react'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/atoms/Button'
import { Progress } from '@/components/ui/progress'
import { AITool, AISession, DevelopmentTask, ExecutionResult } from '@/app/ai-tools/page'
import {
  Play,
  Square,
  Zap,
  CheckCircle,
  
  
  Clock,
  
  
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
      // Create a development task from natural language prompt
      const task: DevelopmentTask = {
        id: `task_${Date.now()}`,
        title: naturalLanguagePrompt.substring(0, 50) + (naturalLanguagePrompt.length > 50 ? '...' : ''),
        description: naturalLanguagePrompt,
        assignedTools: selectedTools,
        subtasks: selectedTools.map((toolId, index) => ({
          id: `subtask_${Date.now()}_${index}`,
          description: `Execute with ${availableAITools.find(t => t.id === toolId)?.name || toolId}`,
          assignedTool: toolId,
          status: 'pending' as const,
          progress: 0,
        })),
        status: 'running' as const,
        progress: 0,
        complexity: 'medium' as const,
        priority: 'high' as const,
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

  const getTaskPriorityColor = (priority: DevelopmentTask['priority']) => {
    switch (priority) {
      case 'low': return 'bg-gray-100 text-gray-800'
      case 'medium': return 'bg-blue-100 text-blue-800'
      case 'high': return 'bg-orange-100 text-orange-800'
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
      case 'available': return 'bg-green-100 text-green-800'
      case 'running': return 'bg-blue-100 text-blue-800'
      case 'busy': return 'bg-yellow-100 text-yellow-800'
      case 'error': return 'bg-red-100 text-red-800'
    }
  }

  const activeSessions = sessions.filter(s => s.status === 'running' || s.status === 'starting')
  const availableTools = aiTools.filter(t => t.status === 'available')
  const busyTools = aiTools.filter(t => t.status === 'busy' || t.status === 'running')
  const pendingTasks = tasks.filter(t => t.status === 'pending')
  const runningTasks = tasks.filter(t => t.status === 'running')

  return (
    <div className="h-full p-6 space-y-6 overflow-y-auto">
      {/* Overview Stats */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <Card className="p-4">
          <div className="flex items-center gap-3">
            <Users className="w-8 h-8 text-blue-500" />
            <div>
              <div className="text-2xl font-bold">{activeSessions.length}</div>
              <div className="text-sm text-gray-600">Active Sessions</div>
            </div>
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center gap-3">
            <Target className="w-8 h-8 text-green-500" />
            <div>
              <div className="text-2xl font-bold">{runningTasks.length}</div>
              <div className="text-sm text-gray-600">Running Tasks</div>
            </div>
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center gap-3">
            <Activity className="w-8 h-8 text-purple-500" />
            <div>
              <div className="text-2xl font-bold">{busyTools.length}/{aiTools.length}</div>
              <div className="text-sm text-gray-600">Tools Active</div>
            </div>
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center gap-3">
            <TrendingUp className="w-8 h-8 text-orange-500" />
            <div>
              <div className="text-2xl font-bold">
                {aiTools.length > 0 ? Math.round(aiTools.reduce((sum, t) => sum + t.performance.successRate, 0) / aiTools.length) : 0}%
              </div>
              <div className="text-sm text-gray-600">Avg Success Rate</div>
            </div>
          </div>
        </Card>
      </div>

      {/* Execution Control */}
      {isExecuting && selectedTask && (
        <Card className="p-6 border-blue-200 bg-blue-50">
          <div className="flex items-center justify-between mb-4">
            <div>
              <h3 className="text-lg font-semibold text-blue-900">
                Executing: {selectedTask.title}
              </h3>
              <p className="text-sm text-blue-700">
                Using {selectedTask.assignedTools.length} AI tools in parallel
              </p>
            </div>

            <div className="flex items-center gap-3">
              <div className="text-right">
                <div className="text-2xl font-bold text-blue-600">
                  {executionProgress.toFixed(0)}%
                </div>
                <div className="text-sm text-blue-600">Progress</div>
              </div>
              <Button onClick={handleStopExecution} variant="outline">
                <Square className="w-4 h-4 mr-1" />
                Stop
              </Button>
            </div>
          </div>

          <Progress value={executionProgress} className="h-3 mb-4" />

          <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
            <div className="flex items-center gap-2">
              <Activity className="w-4 h-4 text-blue-500" />
              <span>Initializing parallel execution...</span>
            </div>
            <div className="flex items-center gap-2">
              <Users className="w-4 h-4 text-green-500" />
              <span>{selectedTask.subtasks.length} subtasks distributed</span>
            </div>
            <div className="flex items-center gap-2">
              <Clock className="w-4 h-4 text-purple-500" />
              <span>Estimated completion: ~{Math.ceil((100 - executionProgress) / 10)} min</span>
            </div>
          </div>
        </Card>
      )}

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* AI Tools Status */}
        <Card className="p-6">
          <h2 className="text-xl font-bold mb-4">AI Tools Status</h2>

          <div className="space-y-4">
            {aiTools.map((tool) => (
              <div key={tool.id} className="flex items-center justify-between p-4 border rounded-lg">
                <div className="flex items-center gap-3">
                  <div className={`w-3 h-3 rounded-full ${
                    tool.status === 'available' ? 'bg-green-500' :
                    tool.status === 'running' ? 'bg-blue-500' :
                    tool.status === 'busy' ? 'bg-yellow-500' : 'bg-red-500'
                  }`} />

                  <div>
                    <h3 className="font-semibold">{tool.name}</h3>
                    <div className="flex gap-1 mt-1">
                      {tool.capabilities.slice(0, 3).map((cap, index) => (
                        <Badge key={index} variant="outline" className="text-xs">
                          {cap}
                        </Badge>
                      ))}
                      {tool.capabilities.length > 3 && (
                        <Badge variant="outline" className="text-xs">
                          +{tool.capabilities.length - 3}
                        </Badge>
                      )}
                    </div>
                  </div>
                </div>

                <div className="text-right">
                  <Badge className={getToolStatusColor(tool.status)}>
                    {tool.status.toUpperCase()}
                  </Badge>
                  <div className="text-xs text-gray-500 mt-1">
                    {tool.activeSessions}/{tool.maxSessions} sessions
                  </div>
                  <div className="text-xs text-gray-500">
                    {tool.performance.successRate}% success
                  </div>
                </div>
              </div>
            ))}
          </div>
        </Card>

        {/* Task Queue */}
        <Card className="p-6">
          <h2 className="text-xl font-bold mb-4">Task Queue</h2>

          <div className="space-y-4 max-h-96 overflow-y-auto">
            {tasks.map((task) => (
              <div key={task.id} className="p-4 border rounded-lg">
                <div className="flex items-center justify-between mb-3">
                  <div className="flex items-center gap-2">
                    <h3 className="font-semibold">{task.title}</h3>
                    <Badge className={getTaskComplexityColor(task.complexity)}>
                      {task.complexity}
                    </Badge>
                    <Badge className={getTaskPriorityColor(task.priority)}>
                      {task.priority}
                    </Badge>
                  </div>

                  <div className="flex items-center gap-2">
                    <Badge className={getTaskStatusColor(task.status)}>
                      {task.status}
                    </Badge>
                    {task.status === 'pending' && (
                      <Button
                        size="sm"
                        onClick={() => handleExecuteTask(task)}
                        disabled={isExecuting}
                      >
                        <Play className="w-3 h-3 mr-1" />
                        Execute
                      </Button>
                    )}
                  </div>
                </div>

                <p className="text-sm text-gray-600 mb-3">{task.description}</p>

                <div className="flex items-center justify-between text-sm">
                  <div className="flex items-center gap-4">
                    <span className="text-gray-500">
                      Tools: {task.assignedTools.join(', ')}
                    </span>
                    <span className="text-gray-500">
                      Subtasks: {task.subtasks.length}
                    </span>
                  </div>

                  {task.status === 'running' && (
                    <div className="flex items-center gap-2">
                      <Progress value={task.progress} className="w-20 h-2" />
                      <span className="text-xs">{task.progress}%</span>
                    </div>
                  )}
                </div>
              </div>
            ))}

            {tasks.length === 0 && (
              <div className="text-center py-8 text-gray-500">
                <Target className="w-12 h-12 text-gray-400 mx-auto mb-4" />
                <p>No tasks in queue</p>
                <p className="text-sm">Create a task to start parallel AI execution</p>
              </div>
            )}
          </div>
        </Card>
      </div>

      {/* Active Sessions */}
      <Card className="p-6">
        <h2 className="text-xl font-bold mb-4">Active Sessions</h2>

        {activeSessions.length === 0 ? (
          <div className="text-center py-8 text-gray-500">
            <Activity className="w-12 h-12 text-gray-400 mx-auto mb-4" />
            <p>No active sessions</p>
            <p className="text-sm">Start a task to see active AI tool sessions</p>
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {activeSessions.map((session) => (
              <div key={session.id} className="p-4 border rounded-lg">
                <div className="flex items-center justify-between mb-3">
                  <div className="flex items-center gap-2">
                    <Activity className="w-4 h-4 text-blue-500" />
                    <span className="font-medium">
                      {aiTools.find(t => t.id === session.toolId)?.name || session.toolId}
                    </span>
                  </div>

                  <Badge variant="secondary">
                    {session.status}
                  </Badge>
                </div>

                <div className="mb-3">
                  <div className="text-sm text-gray-600 mb-1">Progress</div>
                  <Progress value={session.progress} className="h-2" />
                </div>

                <div className="text-xs text-gray-500 space-y-1">
                  <div>Task: {session.taskId}</div>
                  <div>Started: {session.startTime.toLocaleTimeString()}</div>
                  <div className="truncate">Output: {session.output}</div>
                </div>
              </div>
            ))}
          </div>
        )}
      </Card>

      {/* Natural Language Prompt Input */}
      <Card className="p-6 border-blue-200 bg-gradient-to-br from-blue-50 to-purple-50">
        <h2 className="text-xl font-bold mb-4 flex items-center gap-2">
          <Zap className="w-6 h-6 text-blue-600" />
          Natural Language Prompt Execution
        </h2>

        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Enter your prompt (CodexCLI, OPENCODE, ClaudeCode, GeminiCLI will execute in parallel)
            </label>
            <textarea
              id="natural-language-prompt"
              className="w-full p-4 border-2 border-blue-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 resize-none"
              rows={4}
              placeholder="例: React 19のコンポーネントをTypeScriptで実装して、エラーハンドリングとローディング状態を追加してください..."
              value={naturalLanguagePrompt}
              onChange={(e) => setNaturalLanguagePrompt(e.target.value)}
            />
          </div>

          <div className="flex items-center gap-4">
            <div className="flex-1">
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Select AI Tools (Multiple selection)
              </label>
              <div className="flex flex-wrap gap-2">
                {availableAITools.map((tool) => (
                  <label
                    key={tool.id}
                    className="flex items-center gap-2 p-2 border rounded-lg cursor-pointer hover:bg-blue-50"
                  >
                    <input
                      type="checkbox"
                      checked={selectedTools.includes(tool.id)}
                      onChange={(e) => {
                        if (e.target.checked) {
                          setSelectedTools([...selectedTools, tool.id])
                        } else {
                          setSelectedTools(selectedTools.filter(id => id !== tool.id))
                        }
                      }}
                      className="w-4 h-4 text-blue-600"
                    />
                    <span className="text-sm font-medium">{tool.name}</span>
                    <Badge variant="outline" className="text-xs">
                      {tool.status}
                    </Badge>
                  </label>
                ))}
              </div>
            </div>
          </div>

          <div className="flex items-center justify-between">
            <div className="text-sm text-gray-600">
              {selectedTools.length > 0 ? (
                <span>
                  {selectedTools.length} tool(s) selected: {selectedTools.map(id => 
                    availableAITools.find(t => t.id === id)?.name
                  ).filter(Boolean).join(', ')}
                </span>
              ) : (
                <span className="text-orange-600">Please select at least one AI tool</span>
              )}
            </div>
            <div className="flex gap-2">
              <Button
                variant="outline"
                onClick={() => {
                  setNaturalLanguagePrompt('')
                  setSelectedTools([])
                }}
                disabled={isExecuting}
              >
                Clear
              </Button>
              <Button
                onClick={handleExecuteNaturalLanguagePrompt}
                disabled={isExecuting || !naturalLanguagePrompt.trim() || selectedTools.length === 0}
                className="bg-blue-600 hover:bg-blue-700 text-white"
              >
                <Play className="w-4 h-4 mr-2" />
                Execute in Parallel
              </Button>
            </div>
          </div>

          {isExecutingNaturalLanguage && (
            <div className="mt-4 p-4 bg-blue-100 rounded-lg">
              <div className="flex items-center justify-between mb-2">
                <span className="font-medium text-blue-900">Executing with {selectedTools.length} AI tools...</span>
                <span className="text-sm text-blue-700">{naturalLanguageProgress}%</span>
              </div>
              <Progress value={naturalLanguageProgress} className="h-2" />
              <div className="mt-2 text-sm text-blue-700">
                Tools running: {selectedTools.map(id => 
                  availableAITools.find(t => t.id === id)?.name
                ).filter(Boolean).join(', ')}
              </div>
            </div>
          )}
        </div>
      </Card>

      {/* Quick Actions */}
      <Card className="p-6">
        <h2 className="text-xl font-bold mb-4">Quick Actions</h2>

        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          <Button variant="outline" className="p-4 h-auto">
            <div className="text-center">
              <Zap className="w-6 h-6 mx-auto mb-2" />
              <div className="font-medium">Quick Task</div>
              <div className="text-sm text-gray-600">Execute simple task</div>
            </div>
          </Button>

          <Button variant="outline" className="p-4 h-auto">
            <div className="text-center">
              <Target className="w-6 h-6 mx-auto mb-2" />
              <div className="font-medium">Complex Task</div>
              <div className="text-sm text-gray-600">Multi-step execution</div>
            </div>
          </Button>

          <Button variant="outline" className="p-4 h-auto">
            <div className="text-center">
              <Activity className="w-6 h-6 mx-auto mb-2" />
              <div className="font-medium">Monitor Sessions</div>
              <div className="text-sm text-gray-600">View active sessions</div>
            </div>
          </Button>

          <Button variant="outline" className="p-4 h-auto">
            <div className="text-center">
              <CheckCircle className="w-6 h-6 mx-auto mb-2" />
              <div className="font-medium">View Results</div>
              <div className="text-sm text-gray-600">Check execution results</div>
            </div>
          </Button>
        </div>
      </Card>
    </div>
  )
}
