'use client'

import { useState, useEffect, useRef } from 'react'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
} from 'chart.js'
import { Line } from 'react-chartjs-2'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/atoms/Button'
import { QualityMetric, QCProcess } from '@/app/qc/page'
import { Activity, AlertTriangle, CheckCircle, XCircle } from 'lucide-react'

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
)

interface RealTimeMonitoringProps {
  metrics: QualityMetric[]
  processes: QCProcess[]
}

export function RealTimeMonitoring({ metrics, processes }: RealTimeMonitoringProps) {
  const [isMonitoring, setIsMonitoring] = useState(false)
  const [realtimeData, setRealtimeData] = useState<Array<{
    timestamp: Date
    metrics: Record<string, number>
  }>>([])
  const intervalRef = useRef<NodeJS.Timeout | null>(null)

  // Start/stop monitoring
  const toggleMonitoring = () => {
    if (isMonitoring) {
      if (intervalRef.current) {
        clearInterval(intervalRef.current)
        intervalRef.current = null
      }
      setIsMonitoring(false)
    } else {
      setIsMonitoring(true)
      // Generate sample real-time data every 2 seconds
      intervalRef.current = setInterval(() => {
        const newDataPoint = {
          timestamp: new Date(),
          metrics: metrics.reduce((acc, metric) => {
            // Simulate slight variations
            const variation = (Math.random() - 0.5) * 2 // -1 to +1
            const newValue = Math.max(0, Math.min(100, metric.value + variation))
            acc[metric.id] = newValue
            return acc
          }, {} as Record<string, number>)
        }

        setRealtimeData(prev => {
          const updated = [...prev, newDataPoint]
          // Keep only last 20 data points
          return updated.slice(-20)
        })
      }, 2000)
    }
  }

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current)
      }
    }
  }, [])

  // Prepare chart data
  const chartData = {
    labels: realtimeData.map(point =>
      point.timestamp.toLocaleTimeString('ja-JP', {
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit'
      })
    ),
    datasets: metrics.map((metric, index) => ({
      label: metric.name,
      data: realtimeData.map(point => point.metrics[metric.id] || metric.value),
      borderColor: [
        'rgb(59, 130, 246)', // blue
        'rgb(34, 197, 94)',  // green
        'rgb(239, 68, 68)',  // red
        'rgb(168, 85, 247)'  // purple
      ][index % 4],
      backgroundColor: [
        'rgba(59, 130, 246, 0.1)',
        'rgba(34, 197, 94, 0.1)',
        'rgba(239, 68, 68, 0.1)',
        'rgba(168, 85, 247, 0.1)'
      ][index % 4],
      tension: 0.4,
      pointRadius: 3,
      pointHoverRadius: 6,
    }))
  }

  const chartOptions = {
    responsive: true,
    maintainAspectRatio: false,
    animation: {
      duration: 1000,
      easing: 'easeInOutQuart' as const,
    },
    plugins: {
      legend: {
        position: 'top' as const,
      },
      title: {
        display: true,
        text: 'Real-time Quality Monitoring',
        font: { size: 16, weight: 'bold' },
      },
      tooltip: {
        mode: 'index' as const,
        intersect: false,
      },
    },
    scales: {
      y: {
        beginAtZero: true,
        max: 100,
        title: {
          display: true,
          text: 'Value (%)'
        }
      },
      x: {
        title: {
          display: true,
          text: 'Time'
        }
      }
    },
    interaction: {
      mode: 'nearest' as const,
      axis: 'x' as const,
      intersect: false,
    },
  }

  // Calculate real-time statistics
  const realtimeStats = realtimeData.length > 0
    ? realtimeData[realtimeData.length - 1].metrics
    : metrics.reduce((acc, metric) => {
        acc[metric.id] = metric.value
        return acc
      }, {} as Record<string, number>)

  // Calculate trends
  const trends = metrics.map(metric => {
    if (realtimeData.length < 2) return 'stable'

    const recent = realtimeData.slice(-5).map(point => point.metrics[metric.id] || metric.value)
    const avgRecent = recent.reduce((sum, val) => sum + val, 0) / recent.length
    const avgOlder = realtimeData.slice(-10, -5).map(point => point.metrics[metric.id] || metric.value)
    const avgOlderValue = avgOlder.length > 0 ? avgOlder.reduce((sum, val) => sum + val, 0) / avgOlder.length : avgRecent

    const diff = avgRecent - avgOlderValue
    if (Math.abs(diff) < 0.5) return 'stable'
    return diff > 0 ? 'up' : 'down'
  })

  return (
    <div className="h-full p-6 space-y-6 overflow-y-auto">
      {/* Monitoring Controls */}
      <Card className="p-6">
        <div className="flex items-center justify-between mb-4">
          <div>
            <h2 className="text-xl font-bold">Real-time Quality Monitoring</h2>
            <p className="text-gray-600 mt-1">
              Live quality metrics tracking with automated anomaly detection
            </p>
          </div>

          <div className="flex items-center gap-4">
            <div className="flex items-center gap-2">
              <div className={`w-3 h-3 rounded-full ${isMonitoring ? 'bg-green-500 animate-pulse' : 'bg-gray-400'}`} />
              <span className="text-sm font-medium">
                {isMonitoring ? 'Monitoring Active' : 'Monitoring Inactive'}
              </span>
            </div>

            <Button
              onClick={toggleMonitoring}
              variant={isMonitoring ? 'destructive' : 'primary'}
            >
              {isMonitoring ? (
                <>
                  <XCircle className="w-4 h-4 mr-1" />
                  Stop Monitoring
                </>
              ) : (
                <>
                  <Activity className="w-4 h-4 mr-1" />
                  Start Monitoring
                </>
              )}
            </Button>
          </div>
        </div>

        {/* Real-time Statistics */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          {metrics.map((metric, index) => {
            const currentValue = realtimeStats[metric.id] || metric.value
            const trend = trends[index]
            const isAnomaly = Math.abs(currentValue - metric.target) > metric.tolerance

            return (
              <Card key={metric.id} className={`p-4 ${isAnomaly ? 'border-red-300 bg-red-50' : ''}`}>
                <div className="flex items-center justify-between mb-2">
                  <h3 className="font-semibold text-sm">{metric.name}</h3>
                  {isAnomaly && <AlertTriangle className="w-4 h-4 text-red-500" />}
                </div>

                <div className="text-2xl font-bold mb-1">
                  {currentValue.toFixed(1)}{metric.unit}
                </div>

                <div className="flex items-center justify-between text-xs">
                  <span className="text-gray-500">
                    Target: {metric.target}{metric.unit}
                  </span>
                  <Badge
                    variant={
                      trend === 'up' ? 'secondary' :
                      trend === 'down' ? 'destructive' : 'outline'
                    }
                    className="text-xs"
                  >
                    {trend === 'up' ? '↗' : trend === 'down' ? '↘' : '→'} {trend}
                  </Badge>
                </div>

                {/* Mini trend indicator */}
                <div className="mt-2 flex items-end gap-1">
                  {realtimeData.slice(-5).map((point, idx) => {
                    const value = point.metrics[metric.id] || metric.value
                    const height = Math.max(4, Math.min(20, (value / 100) * 20))
                    return (
                      <div
                        key={idx}
                        className="bg-blue-500 rounded-sm flex-1"
                        style={{ height: `${height}px` }}
                      />
                    )
                  })}
                </div>
              </Card>
            )
          })}
        </div>
      </Card>

      {/* Real-time Chart */}
      <Card className="p-6">
        <div className="h-96">
          {realtimeData.length > 0 ? (
            <Line data={chartData} options={chartOptions} />
          ) : (
            <div className="flex items-center justify-center h-full">
              <div className="text-center">
                <Activity className="w-12 h-12 text-gray-400 mx-auto mb-4" />
                <h3 className="text-lg font-semibold text-gray-600 mb-2">
                  {isMonitoring ? 'Collecting Data...' : 'Monitoring Not Started'}
                </h3>
                <p className="text-gray-500">
                  {isMonitoring
                    ? 'Real-time data collection in progress. Chart will update automatically.'
                    : 'Click "Start Monitoring" to begin real-time data collection.'
                  }
                </p>
              </div>
            </div>
          )}
        </div>
      </Card>

      {/* Process Status */}
      <Card className="p-6">
        <h2 className="text-xl font-bold mb-4">Active QC Processes</h2>

        {processes.filter(p => p.status === 'running').length === 0 ? (
          <div className="text-center py-8 text-gray-500">
            <CheckCircle className="w-12 h-12 text-green-400 mx-auto mb-4" />
            <p>No active QC processes</p>
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {processes
              .filter(p => p.status === 'running')
              .map((process) => (
                <Card key={process.id} className="p-4 border-blue-200 bg-blue-50">
                  <div className="flex items-center justify-between mb-3">
                    <h3 className="font-semibold">{process.name}</h3>
                    <Badge className="bg-blue-100 text-blue-800">
                      Running
                    </Badge>
                  </div>

                  <div className="space-y-2">
                    <div className="flex justify-between text-sm">
                      <span>Progress</span>
                      <span>{process.progress}%</span>
                    </div>
                    <div className="w-full bg-blue-200 rounded-full h-2">
                      <div
                        className="bg-blue-600 h-2 rounded-full transition-all duration-300"
                        style={{ width: `${process.progress}%` }}
                      />
                    </div>

                    {process.startTime && (
                      <div className="text-xs text-gray-600">
                        Started: {process.startTime.toLocaleTimeString()}
                      </div>
                    )}
                  </div>
                </Card>
              ))}
          </div>
        )}
      </Card>

      {/* Alert Thresholds */}
      <Card className="p-6">
        <h2 className="text-xl font-bold mb-4">Alert Thresholds</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {metrics.map((metric) => (
            <div key={metric.id} className="flex items-center justify-between p-3 bg-gray-50 rounded">
              <div>
                <div className="font-medium text-sm">{metric.name}</div>
                <div className="text-xs text-gray-600">
                  Current: {realtimeStats[metric.id]?.toFixed(1) || metric.value.toFixed(1)}{metric.unit}
                </div>
              </div>

              <div className="text-right">
                <div className="text-sm font-medium">
                  Target: {metric.target}{metric.unit}
                </div>
                <div className="text-xs text-gray-600">
                  ±{metric.tolerance}{metric.unit}
                </div>
              </div>
            </div>
          ))}
        </div>
      </Card>
    </div>
  )
}
