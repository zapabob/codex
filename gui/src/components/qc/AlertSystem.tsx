'use client'

import { useState } from 'react'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/atoms/Button'
import { QCAlert } from '@/app/qc/page'
import {
  AlertTriangle,
  CheckCircle,
  XCircle,
  Info,
  Bell,
  BellOff,
  Settings,
  Trash2,
  Eye,
  EyeOff
} from 'lucide-react'

interface AlertSystemProps {
  alerts: QCAlert[]
  onAcknowledge: (alertId: string) => void
}

export function AlertSystem({ alerts, onAcknowledge }: AlertSystemProps) {
  const [filter, setFilter] = useState<'all' | 'critical' | 'warning' | 'info'>('all')
  const [showAcknowledged, setShowAcknowledged] = useState(false)
  const [notificationsEnabled, setNotificationsEnabled] = useState(true)

  // Filter alerts
  const filteredAlerts = alerts.filter(alert => {
    if (!showAcknowledged && alert.acknowledged) return false
    if (filter === 'all') return true
    return alert.type === filter
  })

  // Sort alerts by priority and timestamp
  const sortedAlerts = filteredAlerts.sort((a, b) => {
    const priorityOrder = { critical: 3, warning: 2, info: 1 }
    const priorityDiff = priorityOrder[b.type] - priorityOrder[a.type]
    if (priorityDiff !== 0) return priorityDiff
    return b.timestamp.getTime() - a.timestamp.getTime()
  })

  // Group alerts by type
  const alertGroups = sortedAlerts.reduce((acc, alert) => {
    if (!acc[alert.type]) acc[alert.type] = []
    acc[alert.type].push(alert)
    return acc
  }, {} as Record<string, QCAlert[]>)

  const getAlertIcon = (type: QCAlert['type']) => {
    switch (type) {
      case 'critical':
        return <XCircle className="w-5 h-5 text-red-500" />
      case 'warning':
        return <AlertTriangle className="w-5 h-5 text-yellow-500" />
      case 'info':
        return <Info className="w-5 h-5 text-blue-500" />
    }
  }

  const getAlertColor = (type: QCAlert['type']) => {
    switch (type) {
      case 'critical':
        return 'border-red-200 bg-red-50'
      case 'warning':
        return 'border-yellow-200 bg-yellow-50'
      case 'info':
        return 'border-blue-200 bg-blue-50'
    }
  }

  const getAlertBadgeColor = (type: QCAlert['type']) => {
    switch (type) {
      case 'critical':
        return 'bg-red-100 text-red-800'
      case 'warning':
        return 'bg-yellow-100 text-yellow-800'
      case 'info':
        return 'bg-blue-100 text-blue-800'
    }
  }

  const handleBulkAcknowledge = (type: QCAlert['type']) => {
    const alertsToAcknowledge = alertGroups[type]?.filter(alert => !alert.acknowledged) || []
    alertsToAcknowledge.forEach(alert => onAcknowledge(alert.id))
  }

  const handleClearAcknowledged = () => {
    // This would typically call a parent function to clear acknowledged alerts
    console.log('Clear acknowledged alerts')
  }

  const formatTimeAgo = (timestamp: Date) => {
    const now = new Date()
    const diffMs = now.getTime() - timestamp.getTime()
    const diffMins = Math.floor(diffMs / (1000 * 60))
    const diffHours = Math.floor(diffMs / (1000 * 60 * 60))
    const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24))

    if (diffMins < 1) return 'Just now'
    if (diffMins < 60) return `${diffMins}m ago`
    if (diffHours < 24) return `${diffHours}h ago`
    return `${diffDays}d ago`
  }

  // Calculate alert statistics
  const stats = {
    total: alerts.length,
    critical: alerts.filter(a => a.type === 'critical').length,
    warning: alerts.filter(a => a.type === 'warning').length,
    info: alerts.filter(a => a.type === 'info').length,
    acknowledged: alerts.filter(a => a.acknowledged).length,
    unacknowledged: alerts.filter(a => !a.acknowledged).length,
  }

  return (
    <div className="h-full p-6 space-y-6 overflow-y-auto">
      {/* Header with Controls */}
      <Card className="p-6">
        <div className="flex items-center justify-between mb-6">
          <div>
            <h2 className="text-xl font-bold">Quality Alert System</h2>
            <p className="text-gray-600 mt-1">
              Automated quality monitoring and alert management
            </p>
          </div>

          <div className="flex items-center gap-4">
            <Button
              variant="outline"
              size="sm"
              onClick={() => setNotificationsEnabled(!notificationsEnabled)}
            >
              {notificationsEnabled ? (
                <Bell className="w-4 h-4 mr-1" />
              ) : (
                <BellOff className="w-4 h-4 mr-1" />
              )}
              {notificationsEnabled ? 'Notifications On' : 'Notifications Off'}
            </Button>

            <Button variant="outline" size="sm">
              <Settings className="w-4 h-4 mr-1" />
              Alert Settings
            </Button>
          </div>
        </div>

        {/* Statistics */}
        <div className="grid grid-cols-2 md:grid-cols-5 gap-4 mb-6">
          <div className="text-center">
            <div className="text-2xl font-bold text-gray-900">{stats.total}</div>
            <div className="text-sm text-gray-600">Total Alerts</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-red-600">{stats.critical}</div>
            <div className="text-sm text-gray-600">Critical</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-yellow-600">{stats.warning}</div>
            <div className="text-sm text-gray-600">Warnings</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-blue-600">{stats.info}</div>
            <div className="text-sm text-gray-600">Info</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-green-600">{stats.acknowledged}</div>
            <div className="text-sm text-gray-600">Acknowledged</div>
          </div>
        </div>

        {/* Filters */}
        <div className="flex items-center justify-between">
          <div className="flex gap-2">
            {[
              { key: 'all', label: 'All', count: stats.total },
              { key: 'critical', label: 'Critical', count: stats.critical },
              { key: 'warning', label: 'Warnings', count: stats.warning },
              { key: 'info', label: 'Info', count: stats.info },
            ].map(({ key, label, count }) => (
              <Button
                key={key}
                variant={filter === key ? 'primary' : 'outline'}
                size="sm"
                onClick={() => setFilter(key as any)}
                className="relative"
              >
                {label}
                {count > 0 && (
                  <Badge
                    variant="secondary"
                    className="ml-2 h-5 w-5 p-0 flex items-center justify-center text-xs"
                  >
                    {count}
                  </Badge>
                )}
              </Button>
            ))}
          </div>

          <div className="flex gap-2">
            <Button
              variant="outline"
              size="sm"
              onClick={() => setShowAcknowledged(!showAcknowledged)}
            >
              {showAcknowledged ? (
                <EyeOff className="w-4 h-4 mr-1" />
              ) : (
                <Eye className="w-4 h-4 mr-1" />
              )}
              {showAcknowledged ? 'Hide Ack\'d' : 'Show Ack\'d'}
            </Button>

            <Button
              variant="outline"
              size="sm"
              onClick={handleClearAcknowledged}
              disabled={stats.acknowledged === 0}
            >
              <Trash2 className="w-4 h-4 mr-1" />
              Clear Ack'd
            </Button>
          </div>
        </div>
      </Card>

      {/* Alert Groups */}
      {Object.entries(alertGroups).map(([type, typeAlerts]) => (
        <Card key={type} className="p-6">
          <div className="flex items-center justify-between mb-4">
            <div className="flex items-center gap-3">
              {getAlertIcon(type as QCAlert['type'])}
              <h3 className="text-lg font-bold capitalize">{type} Alerts</h3>
              <Badge className={getAlertBadgeColor(type as QCAlert['type'])}>
                {typeAlerts.length}
              </Badge>
            </div>

            {typeAlerts.some(alert => !alert.acknowledged) && (
              <Button
                variant="outline"
                size="sm"
                onClick={() => handleBulkAcknowledge(type as QCAlert['type'])}
              >
                Acknowledge All
              </Button>
            )}
          </div>

          <div className="space-y-3">
            {typeAlerts.map((alert) => (
              <div
                key={alert.id}
                className={`border rounded-lg p-4 ${getAlertColor(alert.type)} ${
                  alert.acknowledged ? 'opacity-60' : ''
                }`}
              >
                <div className="flex items-start justify-between">
                  <div className="flex-1">
                    <div className="flex items-center gap-3 mb-2">
                      <h4 className="font-semibold">{alert.title}</h4>
                      <Badge
                        variant={alert.acknowledged ? 'outline' : 'default'}
                        className="text-xs"
                      >
                        {alert.acknowledged ? 'Acknowledged' : 'Active'}
                      </Badge>
                    </div>

                    <p className="text-gray-700 mb-3">{alert.message}</p>

                    <div className="flex items-center gap-4 text-sm text-gray-600">
                      <span>Threshold: {alert.threshold}</span>
                      <span>Current: {alert.currentValue}</span>
                      <span>Time: {formatTimeAgo(alert.timestamp)}</span>
                      {alert.metricId && (
                        <span>Metric: {alert.metricId}</span>
                      )}
                    </div>
                  </div>

                  {!alert.acknowledged && (
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => onAcknowledge(alert.id)}
                    >
                      <CheckCircle className="w-4 h-4 mr-1" />
                      Acknowledge
                    </Button>
                  )}
                </div>

                {/* Alert trend indicator */}
                {alert.metricId && (
                  <div className="mt-3 flex items-center gap-2">
                    <div className="text-xs text-gray-500">Trend:</div>
                    <div className="flex items-center gap-1">
                      <div className="w-16 h-2 bg-gray-200 rounded">
                        <div
                          className="h-2 bg-red-500 rounded transition-all"
                          style={{
                            width: `${Math.min(100, (alert.currentValue / alert.threshold) * 100)}%`
                          }}
                        />
                      </div>
                      <span className="text-xs text-gray-600">
                        {((alert.currentValue / alert.threshold - 1) * 100).toFixed(1)}%
                      </span>
                    </div>
                  </div>
                )}
              </div>
            ))}
          </div>
        </Card>
      ))}

      {/* Empty State */}
      {sortedAlerts.length === 0 && (
        <Card className="p-12">
          <div className="text-center">
            <CheckCircle className="w-16 h-16 text-green-400 mx-auto mb-4" />
            <h3 className="text-xl font-bold text-gray-900 mb-2">All Clear!</h3>
            <p className="text-gray-600">
              {showAcknowledged
                ? 'No quality alerts detected.'
                : 'No active quality alerts. All systems are operating within normal parameters.'
              }
            </p>
          </div>
        </Card>
      )}

      {/* Alert Configuration */}
      <Card className="p-6">
        <h2 className="text-xl font-bold mb-4">Alert Configuration</h2>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <h3 className="font-semibold mb-3">Notification Channels</h3>
            <div className="space-y-2">
              {[
                { name: 'Email', enabled: true },
                { name: 'Slack', enabled: false },
                { name: 'Discord', enabled: true },
                { name: 'SMS', enabled: false },
                { name: 'In-app', enabled: true },
              ].map(({ name, enabled }) => (
                <div key={name} className="flex items-center justify-between">
                  <span className="text-sm">{name}</span>
                  <Badge variant={enabled ? 'secondary' : 'outline'}>
                    {enabled ? 'Enabled' : 'Disabled'}
                  </Badge>
                </div>
              ))}
            </div>
          </div>

          <div>
            <h3 className="font-semibold mb-3">Escalation Rules</h3>
            <div className="space-y-2 text-sm">
              <div>• Critical alerts: Immediate notification</div>
              <div>• Warning alerts: 5-minute delay</div>
              <div>• Info alerts: Daily summary</div>
              <div>• Unacknowledged alerts: Auto-escalate after 1 hour</div>
            </div>
          </div>
        </div>
      </Card>
    </div>
  )
}
