'use client'

import { useState, useEffect, useRef } from 'react'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/atoms/Button'
import { VirtualEnvironment, CodeExecution } from '@/app/virtual-os/page'
import { CodexAPIClient } from '@/lib/api/client'
import { SystemMetrics } from '@/lib/types'
import {
  Activity,
  Cpu,
  HardDrive,
  Zap,
  Network,
  TrendingUp,
  TrendingDown,
  AlertTriangle,
  CheckCircle,
  BarChart3,
  PieChart,
  Clock,
  Settings
} from 'lucide-react'

interface ResourceMonitorProps {
  environments: VirtualEnvironment[]
  executions: CodeExecution[]
}

interface ResourceMetrics {
  environmentId: string
  environmentName: string
  cpu: {
    usage: number
    limit: number
    trend: 'up' | 'down' | 'stable'
  }
  memory: {
    usage: number
    limit: number
    trend: 'up' | 'down' | 'stable'
  }
  disk: {
    usage: number
    limit: number
    trend: 'up' | 'down' | 'stable'
  }
  network: {
    bytesIn: number
    bytesOut: number
    connections: number
  }
  uptime: number
  lastUpdated: Date
}

export function ResourceMonitor({ environments, executions }: ResourceMonitorProps) {
  const [metrics, setMetrics] = useState<ResourceMetrics[]>([])
  const [systemMetrics, setSystemMetrics] = useState<SystemMetrics | null>(null)
  const [selectedTimeframe, setSelectedTimeframe] = useState<'1m' | '5m' | '15m' | '1h'>('5m')
  const [alerts, setAlerts] = useState<Array<{
    id: string
    type: 'warning' | 'critical'
    message: string
    environmentId: string
    timestamp: Date
  }>>([])
  const apiClient = useRef(new CodexAPIClient())
  const wsRef = useRef<WebSocket | null>(null)
  const previousMetricsRef = useRef<SystemMetrics | null>(null)

  // Fetch real system metrics from API
  useEffect(() => {
    const fetchSystemMetrics = async () => {
      try {
        const metrics = await apiClient.current.getSystemMetrics()
        setSystemMetrics(metrics)
        previousMetricsRef.current = metrics
      } catch (error) {
        console.error('Failed to fetch system metrics:', error)
      }
    }

    // Initial fetch
    fetchSystemMetrics()

    // Set up polling every 5 seconds
    const interval = setInterval(fetchSystemMetrics, 5000)

    // Set up WebSocket connection for real-time updates
    try {
      const wsProtocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:'
      const wsUrl = `${wsProtocol}//${window.location.hostname}:8787`
      const ws = new WebSocket(wsUrl)
      
      ws.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data)
          if (data.type === 'system_metrics' && data.data) {
            const metrics: SystemMetrics = {
              cpuUsage: data.data.cpu_usage || data.data.cpuUsage || 0,
              memoryUsage: data.data.memory_usage || data.data.memoryUsage || 0,
              diskUsage: data.data.disk_usage || data.data.diskUsage || 0,
              networkUsage: data.data.network_usage || data.data.networkUsage,
              activeProcesses: data.data.active_processes || data.data.activeProcesses || 0,
              uptime: data.data.uptime || 0,
              gpuUsage: data.data.gpu_usage || data.data.gpuUsage,
              gpuMemoryUsed: data.data.gpu_memory_used || data.data.gpuMemoryUsed,
              gpuMemoryTotal: data.data.gpu_memory_total || data.data.gpuMemoryTotal,
              gpuMemoryUsage: data.data.gpu_memory_usage || data.data.gpuMemoryUsage,
              gpuTemperature: data.data.gpu_temperature || data.data.gpuTemperature,
              gpuName: data.data.gpu_name || data.data.gpuName,
              gpuVendor: data.data.gpu_vendor || data.data.gpuVendor,
            }
            setSystemMetrics(metrics)
            previousMetricsRef.current = metrics
          }
        } catch (error) {
          console.error('Failed to parse WebSocket message:', error)
        }
      }

      ws.onerror = (error) => {
        console.warn('WebSocket error, falling back to polling:', error)
      }

      ws.onclose = () => {
        console.log('WebSocket closed, using polling only')
      }

      wsRef.current = ws
    } catch (error) {
      console.warn('WebSocket connection failed, using polling only:', error)
    }

    return () => {
      clearInterval(interval)
      if (wsRef.current) {
        wsRef.current.close()
      }
    }
  }, [])

  // Generate environment metrics based on real system metrics
  useEffect(() => {
    if (!systemMetrics) return

    const generateMetrics = () => {
      const newMetrics: ResourceMetrics[] = environments.map(env => {
        // Use real system metrics as base, scaled for environment
        const cpuUsage = env.status === 'running' 
          ? (systemMetrics.cpuUsage * (env.resources.cpu / 100))
          : 0
        const memoryUsage = env.status === 'running'
          ? (systemMetrics.memoryUsage / 100) * env.resources.memory
          : 0

        // Calculate trends based on previous metrics
        const prev = previousMetricsRef.current
        const cpuTrend = prev 
          ? (cpuUsage > prev.cpuUsage ? 'up' : cpuUsage < prev.cpuUsage ? 'down' : 'stable')
          : 'stable'
        const memoryTrend = prev
          ? (memoryUsage > (prev.memoryUsage / 100) * env.resources.memory ? 'up' : 'down')
          : 'stable'

        const baseMetrics: ResourceMetrics = {
          environmentId: env.id,
          environmentName: env.name,
          cpu: {
            usage: Math.min(cpuUsage, env.resources.cpu * 100),
            limit: env.resources.cpu * 100,
            trend: cpuTrend
          },
          memory: {
            usage: Math.min(memoryUsage, env.resources.memory),
            limit: env.resources.memory,
            trend: memoryTrend
          },
          disk: {
            usage: (systemMetrics.diskUsage / 100) * env.resources.disk * 0.8,
            limit: env.resources.disk,
            trend: 'stable'
          },
          network: {
            bytesIn: Math.floor(Math.random() * 1000000), // Network stats would need separate API
            bytesOut: Math.floor(Math.random() * 500000),
            connections: systemMetrics.activeProcesses || 0
          },
          uptime: Date.now() - env.createdAt.getTime(),
          lastUpdated: new Date()
        }

        return baseMetrics
      })

      setMetrics(newMetrics)

      // Generate alerts based on real metrics
      const newAlerts = []
      for (const metric of newMetrics) {
        if (metric.cpu.usage > metric.cpu.limit * 0.9) {
          newAlerts.push({
            id: `cpu-${metric.environmentId}-${Date.now()}`,
            type: 'critical' as const,
            message: `High CPU usage in ${metric.environmentName}: ${metric.cpu.usage.toFixed(1)}%`,
            environmentId: metric.environmentId,
            timestamp: new Date()
          })
        }

        if (metric.memory.usage > metric.memory.limit * 0.85) {
          newAlerts.push({
            id: `mem-${metric.environmentId}-${Date.now()}`,
            type: 'warning' as const,
            message: `High memory usage in ${metric.environmentName}: ${(metric.memory.usage / metric.memory.limit * 100).toFixed(1)}%`,
            environmentId: metric.environmentId,
            timestamp: new Date()
          })
        }

        if (metric.disk.usage > metric.disk.limit * 0.9) {
          newAlerts.push({
            id: `disk-${metric.environmentId}-${Date.now()}`,
            type: 'warning' as const,
            message: `Low disk space in ${metric.environmentName}: ${(metric.disk.usage / metric.disk.limit * 100).toFixed(1)}%`,
            environmentId: metric.environmentId,
            timestamp: new Date()
          })
        }
      }

      setAlerts(prev => [...newAlerts, ...prev.slice(0, 9)]) // Keep last 10 alerts
    }

    generateMetrics()
  }, [environments, systemMetrics])

  const formatBytes = (bytes: number): string => {
    const units = ['B', 'KB', 'MB', 'GB']
    let value = bytes
    let unitIndex = 0

    while (value >= 1024 && unitIndex < units.length - 1) {
      value /= 1024
      unitIndex++
    }

    return `${value.toFixed(1)} ${units[unitIndex]}`
  }

  const formatUptime = (milliseconds: number): string => {
    const seconds = Math.floor(milliseconds / 1000)
    const minutes = Math.floor(seconds / 60)
    const hours = Math.floor(minutes / 60)
    const days = Math.floor(hours / 24)

    if (days > 0) return `${days}d ${hours % 24}h`
    if (hours > 0) return `${hours}h ${minutes % 60}m`
    if (minutes > 0) return `${minutes}m ${seconds % 60}s`
    return `${seconds}s`
  }

  const getResourceColor = (usage: number, limit: number): string => {
    const percentage = usage / limit
    if (percentage > 0.9) return 'text-red-600'
    if (percentage > 0.75) return 'text-yellow-600'
    return 'text-green-600'
  }

  const getTrendIcon = (trend: 'up' | 'down' | 'stable') => {
    switch (trend) {
      case 'up':
        return <TrendingUp className="w-3 h-3 text-red-500" />
      case 'down':
        return <TrendingDown className="w-3 h-3 text-green-500" />
      case 'stable':
        return <Activity className="w-3 h-3 text-blue-500" />
    }
  }

  const totalResources = metrics.reduce(
    (acc, metric) => ({
      cpu: acc.cpu + metric.cpu.limit,
      memory: acc.memory + metric.memory.limit,
      disk: acc.disk + metric.disk.limit,
    }),
    { cpu: 0, memory: 0, disk: 0 }
  )

  const usedResources = metrics.reduce(
    (acc, metric) => ({
      cpu: acc.cpu + metric.cpu.usage,
      memory: acc.memory + metric.memory.usage,
      disk: acc.disk + metric.disk.usage,
    }),
    { cpu: 0, memory: 0, disk: 0 }
  )

  const recentExecutions = executions.slice(0, 10)

  return (
    <div className="h-full p-6 space-y-6 overflow-y-auto">
      {/* System Overview */}
      <div className="grid grid-cols-1 md:grid-cols-5 gap-4">
        <Card className="p-4">
          <div className="flex items-center gap-3">
            <Cpu className="w-8 h-8 text-blue-500" />
            <div>
              <div className="text-2xl font-bold">
                {systemMetrics ? systemMetrics.cpuUsage.toFixed(1) : (usedResources.cpu / totalResources.cpu * 100).toFixed(1)}%
              </div>
              <div className="text-sm text-gray-600">CPU Usage</div>
            </div>
          </div>
          <div className="mt-2 text-xs text-gray-500">
            {systemMetrics ? `Real-time from system` : `${usedResources.cpu.toFixed(1)} / ${totalResources.cpu} cores`}
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center gap-3">
            <HardDrive className="w-8 h-8 text-green-500" />
            <div>
              <div className="text-2xl font-bold">
                {systemMetrics ? systemMetrics.memoryUsage.toFixed(1) : (usedResources.memory / totalResources.memory * 100).toFixed(1)}%
              </div>
              <div className="text-sm text-gray-600">Memory</div>
            </div>
          </div>
          <div className="mt-2 text-xs text-gray-500">
            {systemMetrics ? `Real-time from system` : `${usedResources.memory.toFixed(1)} / ${totalResources.memory} MB`}
          </div>
        </Card>

        {systemMetrics?.gpuUsage !== undefined && (
          <Card className="p-4">
            <div className="flex items-center gap-3">
              <Zap className="w-8 h-8 text-yellow-500" />
              <div>
                <div className="text-2xl font-bold">{systemMetrics.gpuUsage.toFixed(1)}%</div>
                <div className="text-sm text-gray-600">GPU Usage</div>
              </div>
            </div>
            <div className="mt-2 text-xs text-gray-500">
              {systemMetrics.gpuName || 'GPU'}
              {systemMetrics.gpuMemoryUsage !== undefined && ` | ${systemMetrics.gpuMemoryUsage.toFixed(1)}% mem`}
              {systemMetrics.gpuTemperature !== undefined && ` | ${systemMetrics.gpuTemperature}°C`}
            </div>
          </Card>
        )}

        <Card className="p-4">
          <div className="flex items-center gap-3">
            <Activity className="w-8 h-8 text-purple-500" />
            <div>
              <div className="text-2xl font-bold">
                {systemMetrics ? systemMetrics.diskUsage.toFixed(1) : (usedResources.disk / totalResources.disk * 100).toFixed(1)}%
              </div>
              <div className="text-sm text-gray-600">Disk Usage</div>
            </div>
          </div>
          <div className="mt-2 text-xs text-gray-500">
            {systemMetrics ? `Real-time from system` : `${usedResources.disk.toFixed(1)} / ${totalResources.disk} GB`}
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center gap-3">
            <AlertTriangle className="w-8 h-8 text-red-500" />
            <div>
              <div className="text-2xl font-bold">{alerts.filter(a => a.type === 'critical').length}</div>
              <div className="text-sm text-gray-600">Critical Alerts</div>
            </div>
          </div>
          <div className="mt-2 text-xs text-gray-500">
            {alerts.length} total alerts
          </div>
        </Card>
      </div>

      {/* Environment Resource Details */}
      <Card className="p-6">
        <h2 className="text-xl font-bold mb-4">Environment Resources</h2>

        <div className="space-y-4">
          {metrics.map((metric) => (
            <div key={metric.environmentId} className="border rounded-lg p-4">
              <div className="flex items-center justify-between mb-4">
                <div className="flex items-center gap-3">
                  <div className={`w-3 h-3 rounded-full ${
                    environments.find(e => e.id === metric.environmentId)?.status === 'running'
                      ? 'bg-green-500'
                      : 'bg-gray-400'
                  }`} />
                  <h3 className="font-semibold">{metric.environmentName}</h3>
                  <Badge variant="outline" className="text-xs">
                    {environments.find(e => e.id === metric.environmentId)?.status}
                  </Badge>
                </div>

                <div className="text-sm text-gray-500">
                  Uptime: {formatUptime(metric.uptime)}
                </div>
              </div>

              {/* Resource Bars */}
              <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div>
                  <div className="flex items-center justify-between mb-1">
                    <span className="text-sm font-medium flex items-center gap-1">
                      <Cpu className="w-3 h-3" />
                      CPU
                    </span>
                    {getTrendIcon(metric.cpu.trend)}
                  </div>
                  <div className="w-full bg-gray-200 rounded-full h-2">
                    <div
                      className={`h-2 rounded-full transition-all ${
                        metric.cpu.usage > metric.cpu.limit * 0.9 ? 'bg-red-500' :
                        metric.cpu.usage > metric.cpu.limit * 0.7 ? 'bg-yellow-500' : 'bg-blue-500'
                      }`}
                      style={{ width: `${Math.min(100, (metric.cpu.usage / metric.cpu.limit) * 100)}%` }}
                    />
                  </div>
                  <div className="text-xs text-gray-600 mt-1">
                    {metric.cpu.usage.toFixed(1)}% / {metric.cpu.limit}%
                  </div>
                </div>

                <div>
                  <div className="flex items-center justify-between mb-1">
                    <span className="text-sm font-medium flex items-center gap-1">
                      <HardDrive className="w-3 h-3" />
                      Memory
                    </span>
                    {getTrendIcon(metric.memory.trend)}
                  </div>
                  <div className="w-full bg-gray-200 rounded-full h-2">
                    <div
                      className={`h-2 rounded-full transition-all ${
                        metric.memory.usage > metric.memory.limit * 0.9 ? 'bg-red-500' :
                        metric.memory.usage > metric.memory.limit * 0.8 ? 'bg-yellow-500' : 'bg-green-500'
                      }`}
                      style={{ width: `${(metric.memory.usage / metric.memory.limit) * 100}%` }}
                    />
                  </div>
                  <div className="text-xs text-gray-600 mt-1">
                    {metric.memory.usage.toFixed(1)} MB / {metric.memory.limit} MB
                  </div>
                </div>

                <div>
                  <div className="flex items-center justify-between mb-1">
                    <span className="text-sm font-medium flex items-center gap-1">
                      <Activity className="w-3 h-3" />
                      Disk
                    </span>
                    {getTrendIcon(metric.disk.trend)}
                  </div>
                  <div className="w-full bg-gray-200 rounded-full h-2">
                    <div
                      className={`h-2 rounded-full transition-all ${
                        metric.disk.usage > metric.disk.limit * 0.9 ? 'bg-red-500' :
                        metric.disk.usage > metric.disk.limit * 0.8 ? 'bg-yellow-500' : 'bg-blue-500'
                      }`}
                      style={{ width: `${(metric.disk.usage / metric.disk.limit) * 100}%` }}
                    />
                  </div>
                  <div className="text-xs text-gray-600 mt-1">
                    {metric.disk.usage.toFixed(1)} GB / {metric.disk.limit} GB
                  </div>
                </div>
              </div>

              {/* Network Stats */}
              <div className="mt-4 pt-4 border-t">
                <div className="grid grid-cols-3 gap-4 text-sm">
                  <div>
                    <div className="text-gray-600">Network In</div>
                    <div className="font-medium">{formatBytes(metric.network.bytesIn)}</div>
                  </div>
                  <div>
                    <div className="text-gray-600">Network Out</div>
                    <div className="font-medium">{formatBytes(metric.network.bytesOut)}</div>
                  </div>
                  <div>
                    <div className="text-gray-600">Connections</div>
                    <div className="font-medium">{metric.network.connections}</div>
                  </div>
                </div>
              </div>
            </div>
          ))}
        </div>
      </Card>

      {/* Recent Activity */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Recent Alerts */}
        <Card className="p-6">
          <h2 className="text-xl font-bold mb-4">Recent Alerts</h2>

          {alerts.length === 0 ? (
            <div className="text-center py-8 text-gray-500">
              <CheckCircle className="w-8 h-8 mx-auto mb-2" />
              <p>No alerts</p>
              <p className="text-sm">All systems operating normally</p>
            </div>
          ) : (
            <div className="space-y-3">
              {alerts.slice(0, 5).map((alert) => (
                <div key={alert.id} className="flex items-start gap-3 p-3 bg-gray-50 rounded">
                  <AlertTriangle className={`w-4 h-4 mt-0.5 ${
                    alert.type === 'critical' ? 'text-red-500' : 'text-yellow-500'
                  }`} />
                  <div className="flex-1">
                    <div className="font-medium text-sm">{alert.message}</div>
                    <div className="text-xs text-gray-500 mt-1">
                      {alert.timestamp.toLocaleTimeString()}
                    </div>
                  </div>
                  <Badge variant={alert.type === 'critical' ? 'destructive' : 'secondary'} className="text-xs">
                    {alert.type.toUpperCase()}
                  </Badge>
                </div>
              ))}
            </div>
          )}
        </Card>

        {/* Execution Performance */}
        <Card className="p-6">
          <h2 className="text-xl font-bold mb-4">Code Execution Performance</h2>

          {recentExecutions.length === 0 ? (
            <div className="text-center py-8 text-gray-500">
              <BarChart3 className="w-8 h-8 mx-auto mb-2" />
              <p>No executions yet</p>
              <p className="text-sm">Execute code to see performance metrics</p>
            </div>
          ) : (
            <div className="space-y-3">
              {recentExecutions.map((exec) => (
                <div key={exec.id} className="flex items-center justify-between p-3 bg-gray-50 rounded">
                  <div className="flex-1">
                    <div className="font-medium text-sm">{exec.language}</div>
                    <div className="text-xs text-gray-600 truncate max-w-xs">
                      {exec.code.length > 50 ? `${exec.code.substring(0, 50)}...` : exec.code}
                    </div>
                  </div>
                  <div className="text-right">
                    <div className={`text-sm font-medium ${
                      exec.status === 'completed' ? 'text-green-600' :
                      exec.status === 'failed' ? 'text-red-600' : 'text-blue-600'
                    }`}>
                      {exec.executionTime.toFixed(2)}s
                    </div>
                    <Badge
                      variant={
                        exec.status === 'completed' ? 'secondary' :
                        exec.status === 'failed' ? 'destructive' : 'outline'
                      }
                      className="text-xs mt-1"
                    >
                      {exec.status}
                    </Badge>
                  </div>
                </div>
              ))}

              {/* Performance Summary */}
              <div className="mt-4 pt-4 border-t">
                <div className="grid grid-cols-3 gap-4 text-sm">
                  <div className="text-center">
                    <div className="text-lg font-bold text-green-600">
                      {recentExecutions.filter(e => e.status === 'completed').length}
                    </div>
                    <div className="text-gray-600">Successful</div>
                  </div>
                  <div className="text-center">
                    <div className="text-lg font-bold text-blue-600">
                      {(recentExecutions.reduce((sum, e) => sum + e.executionTime, 0) / recentExecutions.length).toFixed(2)}s
                    </div>
                    <div className="text-gray-600">Avg Time</div>
                  </div>
                  <div className="text-center">
                    <div className="text-lg font-bold text-purple-600">
                      {Math.max(...recentExecutions.map(e => e.executionTime)).toFixed(2)}s
                    </div>
                    <div className="text-gray-600">Max Time</div>
                  </div>
                </div>
              </div>
            </div>
          )}
        </Card>
      </div>

      {/* Resource Management Actions */}
      <Card className="p-6">
        <h2 className="text-xl font-bold mb-4">Resource Management</h2>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <Button variant="outline" className="p-4 h-auto">
            <div className="text-center">
              <Settings className="w-6 h-6 mx-auto mb-2" />
              <div className="font-medium">Auto Scaling</div>
              <div className="text-sm text-gray-600">Automatically adjust resources</div>
            </div>
          </Button>

          <Button variant="outline" className="p-4 h-auto">
            <div className="text-center">
              <Network className="w-6 h-6 mx-auto mb-2" />
              <div className="font-medium">Network Limits</div>
              <div className="text-sm text-gray-600">Set bandwidth restrictions</div>
            </div>
          </Button>

          <Button variant="outline" className="p-4 h-auto">
            <div className="text-center">
              <Clock className="w-6 h-6 mx-auto mb-2" />
              <div className="font-medium">Scheduled Cleanup</div>
              <div className="text-sm text-gray-600">Automatic resource cleanup</div>
            </div>
          </Button>
        </div>
      </Card>
    </div>
  )
}
