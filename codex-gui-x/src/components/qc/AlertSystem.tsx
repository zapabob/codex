import { useState } from 'react'
import { Card } from '../atoms/Card'
import { Badge } from '../atoms/Badge'
import { Button } from '../atoms/Button'
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

export interface QCAlert {
  id: string
  type: 'critical' | 'warning' | 'info'
  title: string
  message: string
  timestamp: Date
  acknowledged: boolean
  metricId?: string
  threshold: number
  currentValue: number
}

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
  const sortedAlerts = [...filteredAlerts].sort((a, b) => {
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

  const getAlertBadgeColor = (type: QCAlert['type']) => {
    switch (type) {
      case 'critical':
        return 'error'
      case 'warning':
        return 'warning'
      case 'info':
        return 'info'
      default:
        return 'primary'
    }
  }

  const handleBulkAcknowledge = (type: QCAlert['type']) => {
    const alertsToAcknowledge = alertGroups[type]?.filter(alert => !alert.acknowledged) || []
    alertsToAcknowledge.forEach(alert => onAcknowledge(alert.id))
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
    <div className="h-full space-y-6 overflow-y-auto">
      {/* Header with Controls */}
      <Card animated hover={false} sx={{ border: '1px solid', borderColor: 'divider', bgcolor: 'background.paper' }}>
         <div className="p-6">
            <div className="flex items-center justify-between mb-6">
            <div>
                <h2 className="text-xl font-bold">Quality Alert System</h2>
                <p className="text-muted-foreground mt-1">
                Automated quality monitoring and alert management
                </p>
            </div>

            <div className="flex items-center gap-4">
                <Button
                variant="outlined"
                size="small"
                onClick={() => setNotificationsEnabled(!notificationsEnabled)}
                >
                {notificationsEnabled ? (
                    <Bell className="w-4 h-4 mr-1" />
                ) : (
                    <BellOff className="w-4 h-4 mr-1" />
                )}
                {notificationsEnabled ? 'Notifications On' : 'Notifications Off'}
                </Button>

                <Button variant="outlined" size="small">
                <Settings className="w-4 h-4 mr-1" />
                Alert Settings
                </Button>
            </div>
            </div>

            {/* Statistics */}
            <div className="grid grid-cols-2 md:grid-cols-5 gap-4 mb-6">
            <div className="text-center">
                <div className="text-2xl font-bold">{stats.total}</div>
                <div className="text-sm text-muted-foreground">Total Alerts</div>
            </div>
            <div className="text-center">
                <div className="text-2xl font-bold text-red-500">{stats.critical}</div>
                <div className="text-sm text-muted-foreground">Critical</div>
            </div>
            <div className="text-center">
                <div className="text-2xl font-bold text-amber-500">{stats.warning}</div>
                <div className="text-sm text-muted-foreground">Warnings</div>
            </div>
            <div className="text-center">
                <div className="text-2xl font-bold text-blue-500">{stats.info}</div>
                <div className="text-sm text-muted-foreground">Info</div>
            </div>
            <div className="text-center">
                <div className="text-2xl font-bold text-emerald-500">{stats.acknowledged}</div>
                <div className="text-sm text-muted-foreground">Acknowledged</div>
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
                    variant={filter === key ? 'contained' : 'outlined'}
                    size="small"
                    onClick={() => setFilter(key as any)}
                >
                    {label}
                    {count > 0 && (
                    <Badge
                        variant="secondary"
                        color={key === 'critical' ? 'error' : key === 'warning' ? 'warning' : key === 'info' ? 'info' : 'primary'}
                        className="ml-2"
                    >
                        {count}
                    </Badge>
                    )}
                </Button>
                ))}
            </div>

            <div className="flex gap-2">
                <Button
                variant="outlined"
                size="small"
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
                variant="outlined"
                size="small"
                disabled={stats.acknowledged === 0}
                onClick={() => console.log('Clear acknowledged alerts')}
                >
                <Trash2 className="w-4 h-4 mr-1" />
                Clear Ack'd
                </Button>
            </div>
            </div>
         </div>
      </Card>

      {/* Alert Groups */}
      {Object.entries(alertGroups).map(([type, typeAlerts]) => (
        <Card key={type} animated sx={{ border: '1px solid', borderColor: 'divider', bgcolor: 'background.paper' }}>
          <div className="p-6">
            <div className="flex items-center justify-between mb-4">
                <div className="flex items-center gap-3">
                {getAlertIcon(type as QCAlert['type'])}
                <h3 className="text-lg font-bold capitalize">{type} Alerts</h3>
                <Badge color={getAlertBadgeColor(type as QCAlert['type'])}>
                    {typeAlerts.length}
                </Badge>
                </div>

                {typeAlerts.some(alert => !alert.acknowledged) && (
                <Button
                    variant="outlined"
                    size="small"
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
                    className={`border border-border rounded-xl p-4 transition-all ${
                    alert.acknowledged ? 'bg-muted/30 opacity-60' : 'bg-card'
                    }`}
                >
                    <div className="flex items-start justify-between">
                    <div className="flex-1">
                        <div className="flex items-center gap-3 mb-2">
                        <h4 className="font-semibold">{alert.title}</h4>
                        <Badge
                            variant={alert.acknowledged ? 'outline' : 'default'}
                            color={getAlertBadgeColor(alert.type)}
                        >
                            {alert.acknowledged ? 'Acknowledged' : 'Active'}
                        </Badge>
                        </div>

                        <p className="text-sm text-muted-foreground mb-3">{alert.message}</p>

                        <div className="flex flex-wrap items-center gap-4 text-[10px] text-muted-foreground uppercase tracking-wider font-bold">
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
                        variant="outlined"
                        size="small"
                        onClick={() => onAcknowledge(alert.id)}
                        >
                        <CheckCircle className="w-4 h-4 mr-1" />
                        Acknowledge
                        </Button>
                    )}
                    </div>

                    {/* Alert trend indicator */}
                    {alert.metricId && (
                    <div className="mt-4 flex items-center gap-3">
                        <div className="text-[10px] font-bold text-muted-foreground uppercase">Trend</div>
                        <div className="flex-1 max-w-xs flex items-center gap-2">
                        <div className="flex-1 h-1 bg-muted rounded-full overflow-hidden">
                            <div
                            className={`h-full rounded-full transition-all ${
                                (alert.currentValue / alert.threshold) > 1 ? 'bg-red-500' : 'bg-emerald-500'
                            }`}
                            style={{
                                width: `${Math.min(100, (alert.currentValue / alert.threshold) * 100)}%`
                            }}
                            />
                        </div>
                        <span className="text-[10px] font-mono font-bold">
                            {((alert.currentValue / alert.threshold - 1) * 100).toFixed(1)}%
                        </span>
                        </div>
                    </div>
                    )}
                </div>
                ))}
            </div>
          </div>
        </Card>
      ))}

      {/* Empty State */}
      {sortedAlerts.length === 0 && (
        <Card animated variant="outlined" sx={{ border: '1px dashed', borderColor: 'divider', py: 12 }}>
          <div className="text-center">
            <div className="h-16 w-16 bg-emerald-500/10 text-emerald-500 rounded-2xl flex items-center justify-center mx-auto mb-4 border border-emerald-500/20">
                <CheckCircle className="w-8 h-8" />
            </div>
            <h3 className="text-xl font-bold mb-2">All Clear!</h3>
            <p className="text-muted-foreground">
              {showAcknowledged
                ? 'No quality alerts detected.'
                : 'No active quality alerts. All systems are operating within normal parameters.'
              }
            </p>
          </div>
        </Card>
      )}

      {/* Alert Configuration */}
      <Card animated sx={{ border: '1px solid', borderColor: 'divider', bgcolor: 'background.paper', p: 6 }}>
        <h2 className="text-xl font-bold mb-6">Alert Configuration</h2>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
          <div>
            <h3 className="text-sm font-bold uppercase tracking-wider text-muted-foreground mb-4">Notification Channels</h3>
            <div className="space-y-3">
              {[
                { name: 'Email', enabled: true },
                { name: 'Slack', enabled: false },
                { name: 'Discord', enabled: true },
                { name: 'SMS', enabled: false },
                { name: 'In-app', enabled: true },
              ].map(({ name, enabled }) => (
                <div key={name} className="flex items-center justify-between p-2 rounded-lg hover:bg-muted/50 transition-colors">
                  <span className="text-sm font-medium">{name}</span>
                  <Badge variant={enabled ? 'secondary' : 'outline'} color={enabled ? 'success' : 'secondary'}>
                    {enabled ? 'Enabled' : 'Disabled'}
                  </Badge>
                </div>
              ))}
            </div>
          </div>

          <div>
            <h3 className="text-sm font-bold uppercase tracking-wider text-muted-foreground mb-4">Escalation Rules</h3>
            <div className="space-y-3">
              {[
                { label: 'Critical alerts', detail: 'Immediate notification' },
                { label: 'Warning alerts', detail: '5-minute delay' },
                { label: 'Info alerts', detail: 'Daily summary' },
                { label: 'Unacknowledged', detail: 'Auto-escalate after 1 hour' },
              ].map(({ label, detail }) => (
                <div key={label} className="flex items-center gap-3">
                  <div className="h-1.5 w-1.5 rounded-full bg-primary" />
                  <div className="flex-1 flex items-center justify-between">
                    <span className="text-sm font-medium">{label}</span>
                    <span className="text-xs text-muted-foreground">{detail}</span>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>
      </Card>
    </div>
  )
}
