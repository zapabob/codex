'use client'

import { useState, useEffect } from 'react'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/atoms/Button'
import { AIToolOrchestrator } from '@/components/ai-tools/AIToolOrchestrator'
import { TaskDistributor } from '@/components/ai-tools/TaskDistributor'
import { ResultIntegrator } from '@/components/ai-tools/ResultIntegrator'
import { PerformanceMonitor } from '@/components/ai-tools/PerformanceMonitor'
import { DashboardLayout } from '@/components/templates/DashboardLayout'

// AI Tool types
export type AITool = {
  id: string
  name: string
  status: 'available' | 'running' | 'busy' | 'error'
  capabilities: string[]
  activeSessions: number
  maxSessions: number
  performance: {
    avgResponseTime: number
    successRate: number
    resourceUsage: number
  }
}

// Session types
export type AISession = {
  id: string
  toolId: string
  taskId: string
  status: 'starting' | 'running' | 'completed' | 'failed' | 'cancelled'
  startTime: Date
  endTime?: Date
  progress: number
  output: string
  error?: string
}

// Task types
export type DevelopmentTask = {
  id: string
  title: string
  description: string
  complexity: 'simple' | 'medium' | 'complex' | 'critical'
  priority: 'low' | 'medium' | 'high' | 'critical'
  requirements: string[]
  subtasks: SubTask[]
  status: 'pending' | 'running' | 'completed' | 'failed'
  createdAt: Date
  assignedTools: string[]
  progress: number
}

export type SubTask = {
  id: string
  parentTaskId: string
  description: string
  assignedTool: string
  status: 'pending' | 'running' | 'completed' | 'failed'
  output?: string
  error?: string
}

// Execution types
export type ExecutionResult = {
  taskId: string
  success: boolean
  integratedOutput: string
  subtaskResults: SubTaskResult[]
  errors: string[]
  executionTime: number
  qualityScore: number
  recommendations: string[]
}

export type SubTaskResult = {
  subtaskId: string
  toolId: string
  success: boolean
  output: string
  error?: string
  executionTime: number
  qualityScore: number
}

export default function AIToolsPage() {
  const [activeTab, setActiveTab] = useState<'orchestrator' | 'distributor' | 'integrator' | 'monitor'>('orchestrator')
  const [aiTools, setAiTools] = useState<AITool[]>([])
  const [sessions, setSessions] = useState<AISession[]>([])
  const [tasks, setTasks] = useState<DevelopmentTask[]>([])
  const [results, setResults] = useState<ExecutionResult[]>([])

  // Initialize sample data
  useEffect(() => {
    const sampleTools: AITool[] = [
      {
        id: 'codex',
        name: 'Codex',
        status: 'available',
        capabilities: ['Code Generation', 'Code Review', 'Testing', 'Refactoring', 'Documentation'],
        activeSessions: 1,
        maxSessions: 3,
        performance: {
          avgResponseTime: 2.3,
          successRate: 94,
          resourceUsage: 65,
        },
      },
      {
        id: 'gemini-cli',
        name: 'Gemini CLI',
        status: 'running',
        capabilities: ['Code Generation', 'Analysis', 'Chat', 'Documentation'],
        activeSessions: 2,
        maxSessions: 5,
        performance: {
          avgResponseTime: 1.8,
          successRate: 89,
          resourceUsage: 45,
        },
      },
      {
        id: 'claude-code',
        name: 'Claude Code',
        status: 'busy',
        capabilities: ['Code Generation', 'Code Review', 'Refactoring', 'Testing', 'Analysis'],
        activeSessions: 2,
        maxSessions: 2,
        performance: {
          avgResponseTime: 3.1,
          successRate: 96,
          resourceUsage: 78,
        },
      },
    ]

    const sampleSessions: AISession[] = [
      {
        id: 'session-1',
        toolId: 'codex',
        taskId: 'task-1',
        status: 'running',
        startTime: new Date(Date.now() - 2 * 60 * 1000),
        progress: 65,
        output: 'Analyzing code structure...',
      },
      {
        id: 'session-2',
        toolId: 'gemini-cli',
        taskId: 'task-1',
        status: 'running',
        startTime: new Date(Date.now() - 1.5 * 60 * 1000),
        progress: 45,
        output: 'Generating test cases...',
      },
      {
        id: 'session-3',
        toolId: 'claude-code',
        taskId: 'task-2',
        status: 'completed',
        startTime: new Date(Date.now() - 5 * 60 * 1000),
        endTime: new Date(Date.now() - 2 * 60 * 1000),
        progress: 100,
        output: 'Refactoring completed successfully',
      },
    ]

    const sampleTasks: DevelopmentTask[] = [
      {
        id: 'task-1',
        title: 'Implement User Authentication System',
        description: 'Create a complete authentication system with JWT tokens, password hashing, and session management',
        complexity: 'complex',
        priority: 'high',
        requirements: ['Code Generation', 'Testing', 'Documentation'],
        subtasks: [
          {
            id: 'subtask-1-1',
            parentTaskId: 'task-1',
            description: 'Implement JWT token generation and validation',
            assignedTool: 'codex',
            status: 'running',
          },
          {
            id: 'subtask-1-2',
            parentTaskId: 'task-1',
            description: 'Add password hashing and verification',
            assignedTool: 'gemini-cli',
            status: 'running',
          },
          {
            id: 'subtask-1-3',
            parentTaskId: 'task-1',
            description: 'Create comprehensive test suite',
            assignedTool: 'claude-code',
            status: 'pending',
          },
        ],
        status: 'running',
        createdAt: new Date(Date.now() - 10 * 60 * 1000),
        assignedTools: ['codex', 'gemini-cli', 'claude-code'],
        progress: 55,
      },
      {
        id: 'task-2',
        title: 'Database Schema Optimization',
        description: 'Optimize database schema for better performance and add proper indexing',
        complexity: 'medium',
        priority: 'medium',
        requirements: ['Analysis', 'Code Generation', 'Testing'],
        subtasks: [
          {
            id: 'subtask-2-1',
            parentTaskId: 'task-2',
            description: 'Analyze current schema performance',
            assignedTool: 'claude-code',
            status: 'completed',
          },
          {
            id: 'subtask-2-2',
            parentTaskId: 'task-2',
            description: 'Implement optimized indexes',
            assignedTool: 'codex',
            status: 'pending',
          },
        ],
        status: 'completed',
        createdAt: new Date(Date.now() - 30 * 60 * 1000),
        assignedTools: ['claude-code', 'codex'],
        progress: 100,
      },
    ]

    setAiTools(sampleTools)
    setSessions(sampleSessions)
    setTasks(sampleTasks)
  }, [])

  const handleTaskExecution = (task: DevelopmentTask) => {
    // Simulate task execution
    setTasks(prev => prev.map(t =>
      t.id === task.id
        ? { ...t, status: 'running' as const, progress: 10 }
        : t
    ))
  }

  const handleTaskComplete = (taskId: string, result: ExecutionResult) => {
    setResults(prev => [result, ...prev])
    setTasks(prev => prev.map(t =>
      t.id === taskId
        ? { ...t, status: 'completed' as const, progress: 100 }
        : t
    ))
  }

  const handleSessionUpdate = (session: AISession) => {
    setSessions(prev => prev.map(s =>
      s.id === session.id ? session : s
    ))
  }

  const activeSessions = sessions.filter(s => s.status === 'running' || s.status === 'starting')
  const completedTasks = tasks.filter(t => t.status === 'completed')
  const runningTasks = tasks.filter(t => t.status === 'running')
  const totalTools = aiTools.length
  const activeTools = aiTools.filter(t => t.status === 'running' || t.status === 'busy').length

  return (
    <DashboardLayout>
      <div className="h-full flex flex-col">
        {/* Header */}
        <div className="flex items-center justify-between p-6 border-b">
          <div>
            <h1 className="text-3xl font-bold text-gray-900">AI Tools Orchestration</h1>
            <p className="text-gray-600 mt-1">
              Parallel development with multiple AI tools for large-scale projects
            </p>
          </div>

          {/* Stats */}
          <div className="flex items-center gap-6">
            <div className="text-center">
              <div className="text-2xl font-bold text-blue-600">{activeSessions.length}</div>
              <div className="text-sm text-gray-600">Active Sessions</div>
            </div>
            <div className="text-center">
              <div className="text-2xl font-bold text-green-600">{runningTasks.length}</div>
              <div className="text-sm text-gray-600">Running Tasks</div>
            </div>
            <div className="text-center">
              <div className="text-2xl font-bold text-purple-600">{activeTools}/{totalTools}</div>
              <div className="text-sm text-gray-600">Active Tools</div>
            </div>
            <div className="text-center">
              <div className="text-2xl font-bold text-orange-600">{completedTasks.length}</div>
              <div className="text-sm text-gray-600">Completed</div>
            </div>
          </div>
        </div>

        {/* Tab Navigation */}
        <div className="flex gap-1 p-4 bg-gray-50 border-b overflow-x-auto">
          {[
            { id: 'orchestrator', label: 'AI Orchestrator', icon: '🎯' },
            { id: 'distributor', label: 'Task Distributor', icon: '📋' },
            { id: 'integrator', label: 'Result Integrator', icon: '🔗' },
            { id: 'monitor', label: 'Performance Monitor', icon: '📊' }
          ].map((tab) => (
            <Button
              key={tab.id}
              variant={activeTab === tab.id ? 'primary' : 'ghost'}
              onClick={() => setActiveTab(tab.id as any)}
              className="px-4 py-2 whitespace-nowrap"
            >
              <span className="mr-2">{tab.icon}</span>
              {tab.label}
            </Button>
          ))}
        </div>

        {/* Content */}
        <div className="flex-1 overflow-hidden">
          {activeTab === 'orchestrator' && (
            <AIToolOrchestrator
              aiTools={aiTools}
              tasks={tasks}
              sessions={sessions}
              onTaskExecute={handleTaskExecution}
              onTaskComplete={handleTaskComplete}
              onSessionUpdate={handleSessionUpdate}
            />
          )}

          {activeTab === 'distributor' && (
            <TaskDistributor
              tasks={tasks}
              aiTools={aiTools}
              onTaskCreate={(task) => setTasks(prev => [...prev, task])}
              onTaskUpdate={(task) => setTasks(prev => prev.map(t => t.id === task.id ? task : t))}
            />
          )}

          {activeTab === 'integrator' && (
            <ResultIntegrator
              results={results}
              tasks={tasks}
              onResultAccept={(result) => console.log('Result accepted:', result)}
              onResultReject={(result) => console.log('Result rejected:', result)}
            />
          )}

          {activeTab === 'monitor' && (
            <PerformanceMonitor
              aiTools={aiTools}
              sessions={sessions}
              tasks={tasks}
              results={results}
            />
          )}
        </div>
      </div>
    </DashboardLayout>
  )
}
