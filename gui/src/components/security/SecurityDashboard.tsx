'use client'

import { useMemo } from 'react'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Title,
  Tooltip,
  Legend,
  ArcElement,
} from 'chart.js'
import { Line, Bar, Doughnut } from 'react-chartjs-2'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { SecurityMetrics, SecurityAlert, SecurityStatus } from '@/app/security/page'
import {
  Shield,
  AlertTriangle,
  CheckCircle,
  XCircle,
  Activity,
  FileText,
  Clock,
  TrendingUp
} from 'lucide-react'

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Title,
  Tooltip,
  Legend,
  ArcElement
)

interface SecurityDashboardProps {
  metrics: SecurityMetrics
  alerts: SecurityAlert[]
  status: SecurityStatus
}

export function SecurityDashboard({ metrics, alerts, status }: SecurityDashboardProps) {
  // Calculate security score breakdown
  const securityBreakdown = useMemo(() => {
    const threatScore = Math.max(0, 100 - (metrics.threatsDetected * 10))
    const scanScore = metrics.lastScan ? Math.min(100, (Date.now() - metrics.lastScan.getTime()) / (24 * 60 * 60 * 1000) * -10 + 100) : 50
    const alertScore = Math.max(0, 100 - (alerts.filter(a => !a.resolved).length * 5))
    const quarantineScore = Math.max(0, 100 - (metrics.quarantinedFiles * 2))

    return {
      threatScore,
      scanScore,
      alertScore,
      quarantineScore,
      overall: (threatScore + scanScore + alertScore + quarantineScore) / 4
    }
  }, [metrics, alerts])

  // Security trends data (last 7 days)
  const trendsData = useMemo(() => {
    const last7Days = Array.from({ length: 7 }, (_, i) => {
      const date = new Date()
      date.setDate(date.getDate() - (6 - i))
      return date.toISOString().split('T')[0]
    })

    return {
      labels: last7Days,
      datasets: [
        {
          label: 'Threats Detected',
          data: [2, 1, 0, 3, 1, 0, metrics.threatsDetected],
          borderColor: 'rgb(239, 68, 68)',
          backgroundColor: 'rgba(239, 68, 68, 0.1)',
          tension: 0.4,
        },
        {
          label: 'Files Scanned',
          data: [800, 1200, 950, 1100, 1300, 1000, metrics.filesScanned],
          borderColor: 'rgb(59, 130, 246)',
          backgroundColor: 'rgba(59, 130, 246, 0.1)',
          tension: 0.4,
          yAxisID: 'y1',
        }
      ]
    }
  }, [metrics])

  // Alert distribution
  const alertDistribution = useMemo(() => {
    const distribution = alerts.reduce((acc, alert) => {
      if (!alert.resolved) {
        acc[alert.severity] = (acc[alert.severity] || 0) + 1
      }
      return acc
    }, {} as Record<string, number>)

    return {
      labels: ['Critical', 'High', 'Medium', 'Low'],
      datasets: [{
        data: [
          distribution.critical || 0,
          distribution.high || 0,
          distribution.medium || 0,
          distribution.low || 0
        ],
        backgroundColor: [
          'rgba(239, 68, 68, 0.8)',
          'rgba(245, 158, 11, 0.8)',
          'rgba(59, 130, 246, 0.8)',
          'rgba(34, 197, 94, 0.8)'
        ],
        borderColor: [
          'rgb(239, 68, 68)',
          'rgb(245, 158, 11)',
          'rgb(59, 130, 246)',
          'rgb(34, 197, 94)'
        ],
        borderWidth: 2,
      }]
    }
  }, [alerts])

  // Scan performance data
  const scanPerformance = useMemo(() => {
    const scanTypes = ['Quick Scan', 'Deep Scan', 'Custom Scan']
    const avgDurations = [300, 1800, 900] // seconds

    return {
      labels: scanTypes,
      datasets: [{
        label: 'Average Duration (seconds)',
        data: avgDurations,
        backgroundColor: [
          'rgba(34, 197, 94, 0.8)',
          'rgba(59, 130, 246, 0.8)',
          'rgba(168, 85, 247, 0.8)'
        ],
        borderColor: [
          'rgb(34, 197, 94)',
          'rgb(59, 130, 246)',
          'rgb(168, 85, 247)'
        ],
        borderWidth: 2,
      }]
    }
  }, [])

  const getStatusIcon = (status: SecurityStatus) => {
    switch (status) {
      case 'secure':
        return <CheckCircle className="w-6 h-6 text-green-500" />
      case 'warning':
        return <AlertTriangle className="w-6 h-6 text-yellow-500" />
      case 'threat':
        return <Shield className="w-6 h-6 text-orange-500" />
      case 'critical':
        return <XCircle className="w-6 h-6 text-red-500" />
    }
  }

  const getStatusColor = (status: SecurityStatus) => {
    switch (status) {
      case 'secure':
        return 'text-green-600 bg-green-50 border-green-200'
      case 'warning':
        return 'text-yellow-600 bg-yellow-50 border-yellow-200'
      case 'threat':
        return 'text-orange-600 bg-orange-50 border-orange-200'
      case 'critical':
        return 'text-red-600 bg-red-50 border-red-200'
    }
  }

  const chartOptions = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: {
        position: 'top' as const,
      },
    },
    scales: {
      y: {
        beginAtZero: true,
        title: {
          display: true,
          text: 'Threats'
        }
      },
      y1: {
        beginAtZero: true,
        position: 'right' as const,
        title: {
          display: true,
          text: 'Files Scanned'
        },
        grid: {
          drawOnChartArea: false,
        },
      }
    }
  }

  const distributionOptions = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: {
        position: 'bottom' as const,
      },
      title: {
        display: true,
        text: 'Active Alert Distribution',
        font: { size: 14, weight: 'bold' },
      },
    }
  }

  const performanceOptions = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: {
        display: false,
      },
      title: {
        display: true,
        text: 'Scan Performance',
        font: { size: 14, weight: 'bold' },
      },
    },
    scales: {
      y: {
        beginAtZero: true,
        title: {
          display: true,
          text: 'Duration (seconds)'
        }
      }
    }
  }

  return (
    <div className="h-full p-6 space-y-6 overflow-y-auto">
      {/* Security Status Overview */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <Card className="p-6">
          <div className="flex items-center justify-between mb-4">
            <div>
              <h3 className="text-lg font-semibold">System Health</h3>
              <p className="text-sm text-gray-600">Overall security score</p>
            </div>
            {getStatusIcon(status)}
          </div>
          <div className="text-3xl font-bold mb-2">{securityBreakdown.overall.toFixed(1)}%</div>
          <div className="w-full bg-gray-200 rounded-full h-2">
            <div
              className="bg-blue-600 h-2 rounded-full transition-all"
              style={{ width: `${securityBreakdown.overall}%` }}
            />
          </div>
        </Card>

        <Card className="p-6">
          <div className="flex items-center justify-between mb-4">
            <div>
              <h3 className="text-lg font-semibold">Active Threats</h3>
              <p className="text-sm text-gray-600">Detected & unresolved</p>
            </div>
            <XCircle className="w-6 h-6 text-red-500" />
          </div>
          <div className="text-3xl font-bold mb-2">{metrics.threatsDetected}</div>
          <div className="text-sm text-gray-600">Last scan: {metrics.lastScan ? metrics.lastScan.toLocaleDateString() : 'Never'}</div>
        </Card>

        <Card className="p-6">
          <div className="flex items-center justify-between mb-4">
            <div>
              <h3 className="text-lg font-semibold">Files Scanned</h3>
              <p className="text-sm text-gray-600">Total files analyzed</p>
            </div>
            <FileText className="w-6 h-6 text-blue-500" />
          </div>
          <div className="text-3xl font-bold mb-2">{metrics.filesScanned.toLocaleString()}</div>
          <div className="text-sm text-gray-600">{metrics.totalScans} total scans</div>
        </Card>

        <Card className="p-6">
          <div className="flex items-center justify-between mb-4">
            <div>
              <h3 className="text-lg font-semibold">Quarantined</h3>
              <p className="text-sm text-gray-600">Isolated files</p>
            </div>
            <Shield className="w-6 h-6 text-orange-500" />
          </div>
          <div className="text-3xl font-bold mb-2">{metrics.quarantinedFiles}</div>
          <div className="text-sm text-gray-600">Awaiting review</div>
        </Card>
      </div>

      {/* Security Breakdown */}
      <Card className="p-6">
        <h2 className="text-xl font-bold mb-4">Security Score Breakdown</h2>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div className="text-center">
            <div className="text-2xl font-bold text-red-600">{securityBreakdown.threatScore.toFixed(1)}%</div>
            <div className="text-sm text-gray-600">Threat Protection</div>
            <div className="w-full bg-gray-200 rounded-full h-2 mt-2">
              <div
                className="bg-red-500 h-2 rounded-full"
                style={{ width: `${securityBreakdown.threatScore}%` }}
              />
            </div>
          </div>

          <div className="text-center">
            <div className="text-2xl font-bold text-blue-600">{securityBreakdown.scanScore.toFixed(1)}%</div>
            <div className="text-sm text-gray-600">Scan Frequency</div>
            <div className="w-full bg-gray-200 rounded-full h-2 mt-2">
              <div
                className="bg-blue-500 h-2 rounded-full"
                style={{ width: `${securityBreakdown.scanScore}%` }}
              />
            </div>
          </div>

          <div className="text-center">
            <div className="text-2xl font-bold text-yellow-600">{securityBreakdown.alertScore.toFixed(1)}%</div>
            <div className="text-sm text-gray-600">Alert Management</div>
            <div className="w-full bg-gray-200 rounded-full h-2 mt-2">
              <div
                className="bg-yellow-500 h-2 rounded-full"
                style={{ width: `${securityBreakdown.alertScore}%` }}
              />
            </div>
          </div>

          <div className="text-center">
            <div className="text-2xl font-bold text-green-600">{securityBreakdown.quarantineScore.toFixed(1)}%</div>
            <div className="text-sm text-gray-600">Quarantine Management</div>
            <div className="w-full bg-gray-200 rounded-full h-2 mt-2">
              <div
                className="bg-green-500 h-2 rounded-full"
                style={{ width: `${securityBreakdown.quarantineScore}%` }}
              />
            </div>
          </div>
        </div>
      </Card>

      {/* Charts Grid */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Security Trends */}
        <Card className="p-6">
          <div className="h-80">
            <Line data={trendsData} options={chartOptions} />
          </div>
        </Card>

        {/* Alert Distribution */}
        <Card className="p-6">
          <div className="h-80">
            <Doughnut data={alertDistribution} options={distributionOptions} />
          </div>
        </Card>

        {/* Scan Performance */}
        <Card className="p-6">
          <div className="h-80">
            <Bar data={scanPerformance} options={performanceOptions} />
          </div>
        </Card>

        {/* Recent Alerts */}
        <Card className="p-6">
          <h3 className="text-lg font-bold mb-4">Recent Security Alerts</h3>
          <div className="space-y-3 max-h-72 overflow-y-auto">
            {alerts.slice(0, 5).map((alert) => (
              <div key={alert.id} className="flex items-start gap-3 p-3 bg-gray-50 rounded">
                <div className="mt-1">
                  {alert.type === 'malware' && <XCircle className="w-4 h-4 text-red-500" />}
                  {alert.type === 'suspicious' && <AlertTriangle className="w-4 h-4 text-yellow-500" />}
                  {alert.type === 'anomaly' && <Activity className="w-4 h-4 text-blue-500" />}
                  {alert.type === 'system' && <Shield className="w-4 h-4 text-green-500" />}
                </div>
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
                      <CheckCircle className="w-3 h-3 text-green-500" />
                    )}
                  </div>
                  <p className="text-xs text-gray-600 line-clamp-2">{alert.description}</p>
                  <div className="flex items-center gap-2 mt-1 text-xs text-gray-500">
                    <Clock className="w-3 h-3" />
                    {alert.timestamp.toLocaleString()}
                  </div>
                </div>
              </div>
            ))}
            {alerts.length === 0 && (
              <div className="text-center py-8 text-gray-500">
                <CheckCircle className="w-8 h-8 text-green-400 mx-auto mb-2" />
                <p>No security alerts</p>
              </div>
            )}
          </div>
        </Card>
      </div>

      {/* Quick Actions */}
      <Card className="p-6">
        <h2 className="text-xl font-bold mb-4">Quick Actions</h2>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div className="text-center">
            <Button variant="outline" className="w-full mb-2">
              <Shield className="w-4 h-4 mr-2" />
              Quick Scan
            </Button>
            <p className="text-xs text-gray-600">Scan common locations</p>
          </div>
          <div className="text-center">
            <Button variant="outline" className="w-full mb-2">
              <TrendingUp className="w-4 h-4 mr-2" />
              Deep Scan
            </Button>
            <p className="text-xs text-gray-600">Comprehensive analysis</p>
          </div>
          <div className="text-center">
            <Button variant="outline" className="w-full mb-2">
              <Activity className="w-4 h-4 mr-2" />
              Update Signatures
            </Button>
            <p className="text-xs text-gray-600">Refresh threat database</p>
          </div>
          <div className="text-center">
            <Button variant="outline" className="w-full mb-2">
              <FileText className="w-4 h-4 mr-2" />
              Generate Report
            </Button>
            <p className="text-xs text-gray-600">Security assessment</p>
          </div>
        </div>
      </Card>
    </div>
  )
}
