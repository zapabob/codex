'use client'

import { useState, useEffect, useRef } from 'react'
import { Card } from '@//components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/atoms/Button'
import { SecurityAlert } from '@/app/security/page'
import {
  Activity,
  Eye,
  EyeOff,
  AlertTriangle,
  Shield,
  Wifi,
  HardDrive,
  Cpu,
  Zap,
  TrendingUp,
  TrendingDown,
  Minus
} from 'lucide-react'

interface SecurityMonitorProps {
  isMonitoring: boolean
  onToggleMonitoring: (enabled: boolean) => void
  onAlertDetected: (alert: SecurityAlert) => void
}

export function SecurityMonitor({ isMonitoring, onToggleMonitoring, onAlertDetected }: SecurityMonitorProps) {
  const [realtimeMetrics, setRealtimeMetrics] = useState({
    cpuUsage: 0,
    memoryUsage: 0,
    diskActivity: 0,
    networkActivity: 0,
    activeProcesses: 0,
    openConnections: 0,
  })

  const [systemEvents, setSystemEvents] = useState<Array<{
    id: string
    type: 'file' | 'process' | 'network' | 'system'
    description: string
    timestamp: Date
    severity: 'low' | 'medium' | 'high'
  }>>([])

  const [alerts, setAlerts] = useState<SecurityAlert[]>([])
  const intervalRef = useRef<NodeJS.Timeout | null>(null)

  // Start/stop real-time monitoring
  useEffect(() => {
    if (isMonitoring) {
      intervalRef.current = setInterval(() => {
        // Simulate real-time metrics updates
        setRealtimeMetrics(prev => ({
          cpuUsage: Math.max(0, Math.min(100, prev.cpuUsage + (Math.random() - 0.5) * 10)),
          memoryUsage: Math.max(0, Math.min(100, prev.memoryUsage + (Math.random() - 0.5) * 5)),
          diskActivity: Math.max(0, Math.min(100, prev.diskActivity + (Math.random() - 0.5) * 20)),
          networkActivity: Math.max(0, Math.min(100, prev.networkActivity + (Math.random() - 0.5) * 15)),
          activeProcesses: Math.max(50, Math.min(200, prev.activeProcesses + Math.floor((Math.random() - 0.5) * 10))),
          openConnections: Math.max(10, Math.min(100, prev.openConnections + Math.floor((Math.random() - 0.5) * 5))),
        }))

        // Simulate system events
        if (Math.random() < 0.3) { // 30% chance per interval
          const events = [
            {
              type: 'file' as const,
              description: 'File accessed: C:\\Windows\\System32\\kernel32.dll',
              severity: 'low' as const
            },
            {
              type: 'process' as const,
              description: 'New process started: chrome.exe',
              severity: 'low' as const
            },
            {
              type: 'network' as const,
              description: 'Outbound connection to 192.168.1.100:443',
              severity: 'medium' as const
            },
            {
              type: 'file' as const,
              description: 'File modified: C:\\Users\\Documents\\important.doc',
              severity: 'medium' as const
            },
          ]

          const randomEvent = events[Math.floor(Math.random() * events.length)]
          const newEvent = {
            id: Date.now().toString(),
            ...randomEvent,
            timestamp: new Date(),
          }

          setSystemEvents(prev => [newEvent, ...prev.slice(0, 49)]) // Keep last 50 events

          // Generate alerts for suspicious activity
          if (randomEvent.severity === 'high' || (randomEvent.severity === 'medium' && Math.random() < 0.2)) {
            const alert: SecurityAlert = {
              id: `alert_${Date.now()}`,
              type: randomEvent.type === 'network' ? 'anomaly' : 'suspicious',
              severity: randomEvent.severity === 'high' ? 'high' : 'medium',
              title: randomEvent.type === 'network' ? 'Suspicious Network Activity' :
                     randomEvent.type === 'file' ? 'File System Anomaly' :
                     'Process Anomaly',
              description: randomEvent.description,
              affectedFiles: randomEvent.type === 'file' ? [randomEvent.description.split(': ')[1]] : [],
              timestamp: new Date(),
              resolved: false,
            }

            setAlerts(prev => [alert, ...prev])
            onAlertDetected(alert)
          }
        }
      }, 2000)
    } else {
      if (intervalRef.current) {
        clearInterval(intervalRef.current)
        intervalRef.current = null
      }
    }

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current)
      }
    }
  }, [isMonitoring, onAlertDetected])

  const handleToggleMonitoring = () => {
    onToggleMonitoring(!isMonitoring)
  }

  const getMetricColor = (value: number, thresholds: { warning: number, critical: number }) => {
    if (value >= thresholds.critical) return 'text-red-600'
    if (value >= thresholds.warning) return 'text-yellow-600'
    return 'text-green-600'
  }

  const getEventIcon = (type: string) => {
    switch (type) {
      case 'file': return <HardDrive className="w-4 h-4" />
      case 'process': return <Activity className="w-4 h-4" />
      case 'network': return <Wifi className="w-4 h-4" />
      case 'system': return <Shield className="w-4 h-4" />
      default: return <Activity className="w-4 h-4" />
    }
  }

  const getEventColor = (severity: string) => {
    switch (severity) {
      case 'high': return 'text-red-600 bg-red-50 border-red-200'
      case 'medium': return 'text-yellow-600 bg-yellow-50 border-yellow-200'
      case 'low': return 'text-blue-600 bg-blue-50 border-blue-200'
      default: return 'text-gray-600 bg-gray-50 border-gray-200'
    }
  }

  const recentAlerts = alerts.slice(0, 5)

  return (
    <div className="h-full p-6 space-y-6 overflow-y-auto">
      {/* Monitor Controls */}
      <Card className="p-6">
        <div className="flex items-center justify-between mb-6">
          <div>
            <h2 className="text-xl font-bold">Real-time Security Monitor</h2>
            <p className="text-gray-600 mt-1">
              Continuous monitoring of system activity and threat detection
            </p>
          </div>

          <div className="flex items-center gap-4">
            <div className="flex items-center gap-2">
              <div className={`w-3 h-3 rounded-full ${isMonitoring ? 'bg-green-500 animate-pulse' : 'bg-gray-400'}`} />
              <span className="text-sm font-medium">
                {isMonitoring ? 'Active' : 'Inactive'}
              </span>
            </div>

            <Button
              onClick={handleToggleMonitoring}
              variant={isMonitoring ? 'destructive' : 'primary'}
            >
              {isMonitoring ? (
                <>
                  <EyeOff className="w-4 h-4 mr-1" />
                  Stop Monitoring
                </>
              ) : (
                <>
                  <Eye className="w-4 h-4 mr-1" />
                  Start Monitoring
                </>
              )}
            </Button>
          </div>
        </div>

        {/* Real-time Metrics */}
        <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
          <div className="text-center">
            <Cpu className={`w-8 h-8 mx-auto mb-2 ${getMetricColor(realtimeMetrics.cpuUsage, { warning: 70, critical: 90 })}`} />
            <div className={`text-2xl font-bold ${getMetricColor(realtimeMetrics.cpuUsage, { warning: 70, critical: 90 })}`}>
              {realtimeMetrics.cpuUsage.toFixed(1)}%
            </div>
            <div className="text-sm text-gray-600">CPU Usage</div>
          </div>

          <div className="text-center">
            <HardDrive className={`w-8 h-8 mx-auto mb-2 ${getMetricColor(realtimeMetrics.memoryUsage, { warning: 80, critical: 95 })}`} />
            <div className={`text-2xl font-bold ${getMetricColor(realtimeMetrics.memoryUsage, { warning: 80, critical: 95 })}`}>
              {realtimeMetrics.memoryUsage.toFixed(1)}%
            </div>
            <div className="text-sm text-gray-600">Memory</div>
          </div>

          <div className="text-center">
            <Activity className={`w-8 h-8 mx-auto mb-2 ${getMetricColor(realtimeMetrics.diskActivity, { warning: 60, critical: 80 })}`} />
            <div className={`text-2xl font-bold ${getMetricColor(realtimeMetrics.diskActivity, { warning: 60, critical: 80 })}`}>
              {realtimeMetrics.diskActivity.toFixed(1)}%
            </div>
            <div className="text-sm text-gray-600">Disk I/O</div>
          </div>

          <div className="text-center">
            <Wifi className={`w-8 h-8 mx-auto mb-2 ${getMetricColor(realtimeMetrics.networkActivity, { warning: 50, critical: 70 })}`} />
            <div className={`text-2xl font-bold ${getMetricColor(realtimeMetrics.networkActivity, { warning: 50, critical: 70 })}`}>
              {realtimeMetrics.networkActivity.toFixed(1)}%
            </div>
            <div className="text-sm text-gray-600">Network</div>
          </div>

          <div className="text-center">
            <Zap className="w-8 h-8 mx-auto mb-2 text-blue-500" />
            <div className="text-2xl font-bold text-blue-600">
              {realtimeMetrics.activeProcesses}
            </div>
            <div className="text-sm text-gray-600">Processes</div>
          </div>

          <div className="text-center">
            <Shield className="w-8 h-8 mx-auto mb-2 text-purple-500" />
            <div className="text-2xl font-bold text-purple-600">
              {realtimeMetrics.openConnections}
            </div>
            <div className="text-sm text-gray-600">Connections</div>
          </div>
        </div>
      </Card>

      {/* Recent Alerts */}
      <Card className="p-6">
        <h2 className="text-xl font-bold mb-4">Recent Security Alerts</h2>

        {recentAlerts.length === 0 ? (
          <div className="text-center py-8 text-gray-500">
            <Shield className="w-12 h-12 text-green-400 mx-auto mb-4" />
            <p>No alerts detected</p>
            <p className="text-sm">System is operating normally</p>
          </div>
        ) : (
          <div className="space-y-3">
            {recentAlerts.map((alert) => (
              <div key={alert.id} className="flex items-start gap-3 p-3 bg-gray-50 rounded">
                <AlertTriangle className={`w-5 h-5 mt-0.5 ${
                  alert.severity === 'high' ? 'text-red-500' :
                  alert.severity === 'medium' ? 'text-yellow-500' : 'text-blue-500'
                }`} />
                <div className="flex-1">
                  <div className="flex items-center gap-2 mb-1">
                    <span className="font-medium text-sm">{alert.title}</span>
                    <Badge
                      variant={
                        alert.severity === 'critical' ? 'destructive' :
                        alert.severity === 'high' ? 'default' : 'secondary'
                      }
                      className="text-xs"
                    >
                      {alert.severity.toUpperCase()}
                    </Badge>
                    {alert.resolved && (
                      <Badge variant="outline" className="text-xs">
                        Resolved
                      </Badge>
                    )}
                  </div>
                  <p className="text-sm text-gray-600">{alert.description}</p>
                  <div className="flex items-center gap-2 mt-1 text-xs text-gray-500">
                    <Activity className="w-3 h-3" />
                    {alert.timestamp.toLocaleTimeString()}
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </Card>

      {/* System Events Log */}
      <Card className="p-6">
        <h2 className="text-xl font-bold mb-4">System Events Log</h2>

        <div className="space-y-2 max-h-80 overflow-y-auto">
          {systemEvents.map((event) => (
            <div key={event.id} className={`flex items-center gap-3 p-2 rounded border ${getEventColor(event.severity)}`}>
              {getEventIcon(event.type)}
              <div className="flex-1">
                <div className="text-sm font-medium">{event.description}</div>
                <div className="text-xs text-gray-500">{event.timestamp.toLocaleTimeString()}</div>
              </div>
              <Badge
                variant={event.severity === 'high' ? 'destructive' : event.severity === 'medium' ? 'default' : 'secondary'}
                className="text-xs"
              >
                {event.severity}
              </Badge>
            </div>
          ))}

          {systemEvents.length === 0 && (
            <div className="text-center py-8 text-gray-500">
              <Activity className="w-8 h-8 text-gray-400 mx-auto mb-2" />
              <p>No system events yet</p>
              <p className="text-sm">Start monitoring to see real-time events</p>
            </div>
          )}
        </div>
      </Card>

      {/* Monitoring Configuration */}
      <Card className="p-6">
        <h2 className="text-xl font-bold mb-4">Monitoring Configuration</h2>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <h3 className="font-semibold mb-3">Alert Thresholds</h3>
            <div className="space-y-3">
              <div className="flex items-center justify-between">
                <span className="text-sm">CPU Usage Warning</span>
                <Badge variant="secondary">70%</Badge>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-sm">Memory Usage Critical</span>
                <Badge variant="destructive">95%</Badge>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-sm">Network Connections</span>
                <Badge variant="secondary">50+</Badge>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-sm">Active Processes</span>
                <Badge variant="secondary">200+</Badge>
              </div>
            </div>
          </div>

          <div>
            <h3 className="font-semibold mb-3">Monitoring Scope</h3>
            <div className="space-y-2">
              <label className="flex items-center gap-2">
                <input type="checkbox" defaultChecked className="rounded" />
                <span className="text-sm">File System Activity</span>
              </label>
              <label className="flex items-center gap-2">
                <input type="checkbox" defaultChecked className="rounded" />
                <span className="text-sm">Process Creation/Monitoring</span>
              </label>
              <label className="flex items-center gap-2">
                <input type="checkbox" defaultChecked className="rounded" />
                <span className="text-sm">Network Connections</span>
              </label>
              <label className="flex items-center gap-2">
                <input type="checkbox" defaultChecked className="rounded" />
                <span className="text-sm">Registry Changes</span>
              </label>
              <label className="flex items-center gap-2">
                <input type="checkbox" className="rounded" />
                <span className="text-sm">USB Device Detection</span>
              </label>
            </div>
          </div>
        </div>

        <div className="mt-6 pt-4 border-t">
          <div className="flex items-center justify-between">
            <div>
              <h3 className="font-semibold">Performance Impact</h3>
              <p className="text-sm text-gray-600">Real-time monitoring has minimal impact on system performance</p>
            </div>
            <div className="text-right">
              <div className="text-sm font-medium">CPU Overhead</div>
              <div className="text-lg font-bold text-green-600">~2-5%</div>
            </div>
          </div>
        </div>
      </Card>
    </div>
  )
}
