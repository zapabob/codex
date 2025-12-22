'use client'

import { useState, useEffect } from 'react'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/atoms/Button'
import { AIToolOrchestrator } from '@/components/ai-tools/AIToolOrchestrator'
import { TaskDistributor } from '@/components/ai-tools/TaskDistributor'
import { ResultIntegrator } from '@/components/ai-tools/ResultIntegrator'
import { PerformanceMonitor } from '@/components/ai-tools/PerformanceMonitor'
import { DashboardLayout } from '@/components/templates/DashboardLayout'
import { useCodex } from '@/lib/context/CodexContext'
import { AITool, AISession, DevelopmentTask, ExecutionResult } from '@/lib/types/ai-tools'

type TabId = 'orchestrator' | 'distributor' | 'integrator' | 'monitor'

export default function AIToolsPage() {
  const [activeTab, setActiveTab] = useState<TabId>('orchestrator')
  const [aiTools, setAiTools] = useState<AITool[]>([])
  const [sessions, setSessions] = useState<AISession[]>([])
  const [tasks, setTasks] = useState<DevelopmentTask[]>([])
  const [results, setResults] = useState<ExecutionResult[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const { loadAITools, loadAISessions, loadDevelopmentTasks } = useCodex()

  useEffect(() => {
    const fetchLiveData = async () => {
      try {
        setLoading(true)
        setError(null)

        const [tools, sessionData, taskData] = await Promise.all([
          loadAITools(),
          loadAISessions(),
          loadDevelopmentTasks(),
        ])

        setAiTools(tools)
        setSessions(sessionData)
        setTasks(taskData)
      } catch (err) {
        setError(err instanceof Error ? err.message : 'AIツール情報の取得に失敗しました')
      } finally {
        setLoading(false)
      }
    }

    void fetchLiveData()
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

  const handleTaskExecution = (task: DevelopmentTask) => {
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
  const tabs: Array<{ id: TabId; label: string; icon: string }> = [
    { id: 'orchestrator', label: 'AI Orchestrator', icon: '🎯' },
    { id: 'distributor', label: 'Task Distributor', icon: '📋' },
    { id: 'integrator', label: 'Result Integrator', icon: '🔗' },
    { id: 'monitor', label: 'Performance Monitor', icon: '📊' }
  ]

  return (
    <DashboardLayout>
      <div className="h-full flex flex-col" data-testid="ai-tools-page">
        {/* Header */}
        <div className="flex items-center justify-between p-6 border-b">
          <div>
            <h1 className="text-3xl font-bold text-gray-900" data-testid="ai-tools-title">AIツール統合</h1>
            <p className="text-gray-600 mt-1">
              複数のAIツールを並行して使用した大規模プロジェクト開発
            </p>
            {loading && (
              <Badge variant="outline" className="mt-2">CLI/GUIブリッジ同期中...</Badge>
            )}
            {error && (
              <Badge variant="destructive" className="mt-2">{error}</Badge>
            )}
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
          {tabs.map((tab) => (
            <Button
              key={tab.id}
              variant={activeTab === tab.id ? 'contained' : 'outlined'}
              onClick={() => setActiveTab(tab.id)}
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
