'use client'

import { useState, useEffect } from 'react'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/atoms/Button'
import { Progress } from '@/components/ui/progress'
import { AITool, AISession, DevelopmentTask, ExecutionResult } from '@/app/ai-tools/page'
import {
  Activity,
  TrendingUp,
  TrendingDown,
  Clock,
  Zap,
  BarChart3,
  PieChart,
  Cpu,
  HardDrive,
  Network,
  AlertTriangle,
  CheckCircle,
  XCircle,
  RefreshCw
} from 'lucide-react'

interface PerformanceMonitorProps {
  aiTools: AITool[]
  sessions: AISession[]
  tasks: DevelopmentTask[]
  results: ExecutionResult[]
}

export function PerformanceMonitor({ aiTools, sessions, tasks, results }: PerformanceMonitorProps) {
  const [timeRange, setTimeRange] = useState<'1h' | '6h' | '24h' | '7d'>('24h')
  const [realTimeData, setRealTimeData] = useState({
    activeSessions: 0,
    totalCpuUsage: 0,
    totalMemoryUsage: 0,
    networkThroughput: 0,
    tasksCompleted: 0,
    avgResponseTime: 0,
  })

  // Simulate real-time data updates
  useEffect(() => {
    const interval = setInterval(() => {
      setRealTimeData(prev => ({
        activeSessions: sessions.filter(s => s.status === 'running' || s.status === 'starting').length,
        totalCpuUsage: Math.max(0, Math.min(100, prev.totalCpuUsage + (Math.random() - 0.5) * 10)),
        totalMemoryUsage: Math.max(0, Math.min(100, prev.totalMemoryUsage + (Math.random() - 0.5) * 5)),
        networkThroughput: Math.max(0, prev.networkThroughput + Math.random() * 10),
        tasksCompleted: tasks.filter(t => t.status === 'completed').length,
        avgResponseTime: 2.1 + Math.random() * 0.8, // 2.1-2.9 seconds
      }))
    }, 3000)

    return () => clearInterval(interval)
  }, [sessions, tasks])

  const calculateToolPerformance = () => {
    return aiTools.map(tool => {
      const toolSessions = sessions.filter(s => s.toolId === tool.id)
      const toolResults = results.filter(r => r.subtaskResults.some(sr => sr.toolId === tool.id))

      const successRate = toolResults.length > 0
        ? (toolResults.filter(r => r.success).length / toolResults.length) * 100
        : tool.performance.successRate

      const avgExecutionTime = toolResults.length > 0
        ? toolResults.reduce((sum, r) => sum + r.executionTime, 0) / toolResults.length
        : 0

      const totalSessions = toolSessions.length
      const activeSessions = toolSessions.filter(s => s.status === 'running' || s.status === 'starting').length

      return {
        ...tool,
        calculatedSuccessRate: successRate,
        avgExecutionTime,
        totalSessions,
        activeSessions,
        efficiency: successRate * (1 / Math.max(avgExecutionTime, 1)), // Simple efficiency metric
      }
    })
  }

  const calculateTaskPerformance = () => {
    const completedTasks = tasks.filter(t => t.status === 'completed')
    const runningTasks = tasks.filter(t => t.status === 'running')

    const avgCompletionTime = completedTasks.length > 0
      ? completedTasks.reduce((sum, task) => {
          const result = results.find(r => r.taskId === task.id)
          return result ? sum + result.executionTime : sum
        }, 0) / completedTasks.length
      : 0

    const successRate = completedTasks.length > 0
      ? (completedTasks.filter(task => {
          const result = results.find(r => r.taskId === task.id)
          return result?.success
        }).length / completedTasks.length) * 100
      : 0

    return {
      totalTasks: tasks.length,
      completedTasks: completedTasks.length,
      runningTasks: runningTasks.length,
      pendingTasks: tasks.filter(t => t.status === 'pending').length,
      avgCompletionTime,
      successRate,
      avgSubtasksPerTask: tasks.length > 0
        ? tasks.reduce((sum, t) => sum + t.subtasks.length, 0) / tasks.length
        : 0,
    }
  }

  const calculateSystemPerformance = () => {
    const totalSessions = sessions.length
    const activeSessions = sessions.filter(s => s.status === 'running' || s.status === 'starting').length
    const completedSessions = sessions.filter(s => s.status === 'completed').length

    const avgSessionDuration = completedSessions > 0
      ? sessions.filter(s => s.status === 'completed').reduce((sum, s) => {
          const duration = s.endTime ? s.endTime.getTime() - s.startTime.getTime() : 0
          return sum + duration
        }, 0) / completedSessions / 1000 // Convert to seconds
      : 0

    const sessionSuccessRate = totalSessions > 0
      ? (sessions.filter(s => s.status === 'completed').length / totalSessions) * 100
      : 0

    return {
      totalSessions,
      activeSessions,
      completedSessions,
      avgSessionDuration,
      sessionSuccessRate,
      resourceUtilization: {
        cpu: aiTools.reduce((sum, t) => sum + t.performance.resourceUsage, 0) / Math.max(aiTools.length, 1),
        memory: realTimeData.totalMemoryUsage,
        network: realTimeData.networkThroughput,
      },
    }
  }

  const toolPerformance = calculateToolPerformance()
  const taskPerformance = calculateTaskPerformance()
  const systemPerformance = calculateSystemPerformance()

  const getPerformanceColor = (value: number, thresholds: { good: number, warning: number }) => {
    if (value >= thresholds.good) return 'text-green-600'
    if (value >= thresholds.warning) return 'text-yellow-600'
    return 'text-red-600'
  }

  const getEfficiencyColor = (efficiency: number) => {
    if (efficiency >= 30) return 'text-green-600'
    if (efficiency >= 15) return 'text-yellow-600'
    return 'text-red-600'
  }

  return (
    <div className="h-full p-6 space-y-6 overflow-y-auto">
      {/* Real-time Overview */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <Card className="p-4">
          <div className="flex items-center gap-3">
            <Activity className="w-8 h-8 text-blue-500" />
            <div>
              <div className="text-2xl font-bold">{realTimeData.activeSessions}</div>
              <div className="text-sm text-gray-600">Active Sessions</div>
            </div>
          </div>
          <div className="mt-2 text-xs text-gray-500">
            {systemPerformance.totalSessions} total today
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center gap-3">
            <Cpu className={`w-8 h-8 ${getPerformanceColor(realTimeData.totalCpuUsage, { good: 70, warning: 85 })}`} />
            <div>
              <div className={`text-2xl font-bold ${getPerformanceColor(realTimeData.totalCpuUsage, { good: 70, warning: 85 })}`}>
                {realTimeData.totalCpuUsage.toFixed(1)}%
              </div>
              <div className="text-sm text-gray-600">CPU Usage</div>
            </div>
          </div>
          <div className="mt-2 text-xs text-gray-500">
            System-wide utilization
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center gap-3">
            <CheckCircle className="w-8 h-8 text-green-500" />
            <div>
              <div className="text-2xl font-bold">{realTimeData.tasksCompleted}</div>
              <div className="text-sm text-gray-600">Tasks Completed</div>
            </div>
          </div>
          <div className="mt-2 text-xs text-gray-500">
            {taskPerformance.successRate.toFixed(1)}% success rate
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center gap-3">
            <Clock className="w-8 h-8 text-purple-500" />
            <div>
              <div className="text-2xl font-bold">{realTimeData.avgResponseTime.toFixed(1)}s</div>
              <div className="text-sm text-gray-600">Avg Response</div>
            </div>
          </div>
          <div className="mt-2 text-xs text-gray-500">
            Tool response time
          </div>
        </Card>
      </div>

      {/* Time Range Selector */}
      <Card className="p-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-4">
            <span className="font-medium">Time Range:</span>
            {[
              { id: '1h', label: 'Last Hour' },
              { id: '6h', label: 'Last 6 Hours' },
              { id: '24h', label: 'Last 24 Hours' },
              { id: '7d', label: 'Last 7 Days' },
            ].map((range) => (
              <Button
                key={range.id}
                variant={timeRange === range.id ? 'primary' : 'outline'}
                size="sm"
                onClick={() => setTimeRange(range.id as any)}
              >
                {range.label}
              </Button>
            ))}
          </div>

          <Button variant="outline" size="sm">
            <RefreshCw className="w-4 h-4 mr-1" />
            Refresh
          </Button>
        </div>
      </Card>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Tool Performance */}
        <Card className="p-6">
          <h2 className="text-xl font-bold mb-4">AI Tool Performance</h2>

          <div className="space-y-4">
            {toolPerformance.map((tool) => (
              <div key={tool.id} className="p-4 border rounded-lg">
                <div className="flex items-center justify-between mb-3">
                  <div className="flex items-center gap-3">
                    <div className={`w-3 h-3 rounded-full ${
                      tool.status === 'available' ? 'bg-green-500' :
                      tool.status === 'running' ? 'bg-blue-500' :
                      tool.status === 'busy' ? 'bg-yellow-500' : 'bg-red-500'
                    }`} />
                    <h3 className="font-semibold">{tool.name}</h3>
                  </div>

                  <Badge variant="outline">
                    {tool.activeSessions}/{tool.maxSessions} active
                  </Badge>
                </div>

                <div className="grid grid-cols-2 gap-4 text-sm">
                  <div>
                    <div className="text-gray-600">Success Rate</div>
                    <div className={`font-semibold ${getPerformanceColor(tool.calculatedSuccessRate, { good: 90, warning: 75 })}`}>
                      {tool.calculatedSuccessRate.toFixed(1)}%
                    </div>
                  </div>
                  <div>
                    <div className="text-gray-600">Avg Time</div>
                    <div className="font-semibold">{tool.avgExecutionTime.toFixed(1)}s</div>
                  </div>
                  <div>
                    <div className="text-gray-600">Efficiency</div>
                    <div className={`font-semibold ${getEfficiencyColor(tool.efficiency)}`}>
                      {tool.efficiency.toFixed(1)}
                    </div>
                  </div>
                  <div>
                    <div className="text-gray-600">Total Sessions</div>
                    <div className="font-semibold">{tool.totalSessions}</div>
                  </div>
                </div>

                <div className="mt-3">
                  <div className="text-xs text-gray-600 mb-1">Resource Usage</div>
                  <Progress value={tool.performance.resourceUsage} className="h-2" />
                  <div className="text-xs text-gray-500 mt-1">
                    {tool.performance.resourceUsage}% of allocated resources
                  </div>
                </div>
              </div>
            ))}
          </div>
        </Card>

        {/* Task Performance */}
        <Card className="p-6">
          <h2 className="text-xl font-bold mb-4">Task Performance Metrics</h2>

          <div className="space-y-6">
            {/* Task Status Breakdown */}
            <div>
              <h3 className="font-semibold mb-3">Task Status</h3>
              <div className="grid grid-cols-2 gap-4">
                <div className="text-center">
                  <div className="text-3xl font-bold text-blue-600">{taskPerformance.runningTasks}</div>
                  <div className="text-sm text-gray-600">Running</div>
                </div>
                <div className="text-center">
                  <div className="text-3xl font-bold text-green-600">{taskPerformance.completedTasks}</div>
                  <div className="text-sm text-gray-600">Completed</div>
                </div>
                <div className="text-center">
                  <div className="text-3xl font-bold text-gray-600">{taskPerformance.pendingTasks}</div>
                  <div className="text-sm text-gray-600">Pending</div>
                </div>
                <div className="text-center">
                  <div className="text-3xl font-bold text-red-600">{taskPerformance.totalTasks - taskPerformance.completedTasks - taskPerformance.runningTasks - taskPerformance.pendingTasks}</div>
                  <div className="text-sm text-gray-600">Failed</div>
                </div>
              </div>
            </div>

            {/* Performance Metrics */}
            <div>
              <h3 className="font-semibold mb-3">Performance Metrics</h3>
              <div className="space-y-3">
                <div className="flex justify-between items-center">
                  <span className="text-sm text-gray-600">Success Rate</span>
                  <span className={`font-semibold ${getPerformanceColor(taskPerformance.successRate, { good: 90, warning: 75 })}`}>
                    {taskPerformance.successRate.toFixed(1)}%
                  </span>
                </div>

                <div className="flex justify-between items-center">
                  <span className="text-sm text-gray-600">Avg Completion Time</span>
                  <span className="font-semibold">{taskPerformance.avgCompletionTime.toFixed(1)}s</span>
                </div>

                <div className="flex justify-between items-center">
                  <span className="text-sm text-gray-600">Avg Subtasks per Task</span>
                  <span className="font-semibold">{taskPerformance.avgSubtasksPerTask.toFixed(1)}</span>
                </div>

                <div className="flex justify-between items-center">
                  <span className="text-sm text-gray-600">Throughput</span>
                  <span className="font-semibold">
                    {taskPerformance.completedTasks > 0 ? (taskPerformance.completedTasks / Math.max((Date.now() - Math.min(...tasks.map(t => t.createdAt.getTime()))) / (1000 * 60 * 60), 1)).toFixed(1) : 0} tasks/hour
                  </span>
                </div>
              </div>
            </div>
          </div>
        </Card>
      </div>

      {/* System Performance */}
      <Card className="p-6">
        <h2 className="text-xl font-bold mb-4">System Performance</h2>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          {/* Session Performance */}
          <div>
            <h3 className="font-semibold mb-3">Session Performance</h3>
            <div className="space-y-3">
              <div className="flex justify-between">
                <span className="text-sm text-gray-600">Total Sessions</span>
                <span className="font-semibold">{systemPerformance.totalSessions}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-sm text-gray-600">Active Sessions</span>
                <span className="font-semibold text-blue-600">{systemPerformance.activeSessions}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-sm text-gray-600">Success Rate</span>
                <span className={`font-semibold ${getPerformanceColor(systemPerformance.sessionSuccessRate, { good: 90, warning: 75 })}`}>
                  {systemPerformance.sessionSuccessRate.toFixed(1)}%
                </span>
              </div>
              <div className="flex justify-between">
                <span className="text-sm text-gray-600">Avg Duration</span>
                <span className="font-semibold">{systemPerformance.avgSessionDuration.toFixed(1)}s</span>
              </div>
            </div>
          </div>

          {/* Resource Utilization */}
          <div>
            <h3 className="font-semibold mb-3">Resource Utilization</h3>
            <div className="space-y-3">
              <div>
                <div className="flex justify-between text-sm mb-1">
                  <span className="text-gray-600">CPU</span>
                  <span className={`font-semibold ${getPerformanceColor(systemPerformance.resourceUtilization.cpu, { good: 70, warning: 85 })}`}>
                    {systemPerformance.resourceUtilization.cpu.toFixed(1)}%
                  </span>
                </div>
                <Progress value={systemPerformance.resourceUtilization.cpu} className="h-2" />
              </div>

              <div>
                <div className="flex justify-between text-sm mb-1">
                  <span className="text-gray-600">Memory</span>
                  <span className={`font-semibold ${getPerformanceColor(systemPerformance.resourceUtilization.memory, { good: 75, warning: 90 })}`}>
                    {systemPerformance.resourceUtilization.memory.toFixed(1)}%
                  </span>
                </div>
                <Progress value={systemPerformance.resourceUtilization.memory} className="h-2" />
              </div>

              <div>
                <div className="flex justify-between text-sm mb-1">
                  <span className="text-gray-600">Network</span>
                  <span className="font-semibold">{systemPerformance.resourceUtilization.network.toFixed(1)} MB/s</span>
                </div>
                <Progress value={Math.min(100, systemPerformance.resourceUtilization.network * 10)} className="h-2" />
              </div>
            </div>
          </div>

          {/* Performance Insights */}
          <div>
            <h3 className="font-semibold mb-3">Performance Insights</h3>
            <div className="space-y-3">
              <div className="p-3 bg-blue-50 border border-blue-200 rounded">
                <div className="flex items-center gap-2 mb-1">
                  <TrendingUp className="w-4 h-4 text-blue-600" />
                  <span className="text-sm font-medium text-blue-800">Top Performer</span>
                </div>
                <p className="text-xs text-blue-700">
                  {toolPerformance.length > 0 ? toolPerformance.reduce((prev, current) =>
                    prev.efficiency > current.efficiency ? prev : current
                  ).name : 'No data'} has the highest efficiency score
                </p>
              </div>

              <div className="p-3 bg-yellow-50 border border-yellow-200 rounded">
                <div className="flex items-center gap-2 mb-1">
                  <AlertTriangle className="w-4 h-4 text-yellow-600" />
                  <span className="text-sm font-medium text-yellow-800">Bottleneck</span>
                </div>
                <p className="text-xs text-yellow-700">
                  CPU utilization is {systemPerformance.resourceUtilization.cpu > 80 ? 'high' : 'normal'}.
                  Consider scaling resources if tasks are queuing.
                </p>
              </div>

              <div className="p-3 bg-green-50 border border-green-200 rounded">
                <div className="flex items-center gap-2 mb-1">
                  <CheckCircle className="w-4 h-4 text-green-600" />
                  <span className="text-sm font-medium text-green-800">Optimization</span>
                </div>
                <p className="text-xs text-green-700">
                  System is operating efficiently with {systemPerformance.sessionSuccessRate.toFixed(1)}% session success rate
                </p>
              </div>
            </div>
          </div>
        </div>
      </Card>

      {/* Performance Trends */}
      <Card className="p-6">
        <h2 className="text-xl font-bold mb-4">Performance Trends</h2>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          <div className="text-center">
            <div className="text-2xl font-bold text-green-600 mb-1">
              {Math.max(0, (taskPerformance.successRate - 80)).toFixed(1)}%
            </div>
            <div className="text-sm text-gray-600">Improvement Margin</div>
            <div className="text-xs text-gray-500 mt-1">
              Potential for optimization
            </div>
          </div>

          <div className="text-center">
            <div className="text-2xl font-bold text-blue-600 mb-1">
              {((systemPerformance.completedSessions / Math.max(systemPerformance.totalSessions, 1)) * 100).toFixed(1)}%
            </div>
            <div className="text-sm text-gray-600">Completion Rate</div>
            <div className="text-xs text-gray-500 mt-1">
              Sessions finishing successfully
            </div>
          </div>

          <div className="text-center">
            <div className="text-2xl font-bold text-purple-600 mb-1">
              {toolPerformance.length > 0 ? (toolPerformance.reduce((sum, t) => sum + t.totalSessions, 0) / toolPerformance.length).toFixed(1) : '0'}
            </div>
            <div className="text-sm text-gray-600">Avg Load per Tool</div>
            <div className="text-xs text-gray-500 mt-1">
              Sessions distributed across tools
            </div>
          </div>

          <div className="text-center">
            <div className="text-2xl font-bold text-orange-600 mb-1">
              {Math.min(100, Math.max(0, 100 - systemPerformance.resourceUtilization.cpu)).toFixed(1)}%
            </div>
            <div className="text-sm text-gray-600">Resource Headroom</div>
            <div className="text-xs text-gray-500 mt-1">
              Available capacity remaining
            </div>
          </div>
        </div>
      </Card>
    </div>
  )
}
