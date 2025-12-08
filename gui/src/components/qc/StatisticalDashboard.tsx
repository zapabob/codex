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
import { QualityMetric, QCProcess, AnovaResult } from '@/app/qc/page'
import { TrendingUp, TrendingDown, Minus } from 'lucide-react'

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

interface StatisticalDashboardProps {
  metrics: QualityMetric[]
  processes: QCProcess[]
}

// ANOVA calculation utility
function calculateAnova(groups: number[][]): AnovaResult {
  // Calculate group statistics
  const groupStats = groups.map((group, index) => ({
    name: `Group ${index + 1}`,
    mean: group.reduce((sum, val) => sum + val, 0) / group.length,
    variance: group.reduce((sum, val) => sum + Math.pow(val - (group.reduce((s, v) => s + v, 0) / group.length), 2), 0) / (group.length - 1),
    count: group.length
  }))

  // Overall statistics
  const allValues = groups.flat()
  const grandMean = allValues.reduce((sum, val) => sum + val, 0) / allValues.length
  const totalSS = allValues.reduce((sum, val) => sum + Math.pow(val - grandMean, 2), 0)

  // Between groups SS
  const betweenSS = groupStats.reduce((sum, group) =>
    sum + group.count * Math.pow(group.mean - grandMean, 2), 0
  )

  // Within groups SS
  const withinSS = totalSS - betweenSS

  // Degrees of freedom
  const dfBetween = groups.length - 1
  const dfWithin = allValues.length - groups.length

  // F-statistic
  const fStatistic = (betweenSS / dfBetween) / (withinSS / dfWithin)

  // P-value approximation (simplified)
  const pValue = Math.exp(-fStatistic / 2)

  return {
    fStatistic,
    pValue,
    degreesOfFreedom: dfBetween + dfWithin,
    significance: pValue < 0.05,
    groups: groupStats
  }
}

export function StatisticalDashboard({ metrics, processes }: StatisticalDashboardProps) {
  // Calculate ANOVA for quality metrics by category
  const anovaResult = useMemo(() => {
    const categories = [...new Set(metrics.map(m => m.category))]
    const groups = categories.map(category =>
      metrics.filter(m => m.category === category).map(m => m.value)
    )
    return calculateAnova(groups)
  }, [metrics])

  // Trend analysis data
  const trendData = useMemo(() => {
    const last7Days = Array.from({ length: 7 }, (_, i) => {
      const date = new Date()
      date.setDate(date.getDate() - (6 - i))
      return date.toISOString().split('T')[0]
    })

    return {
      labels: last7Days,
      datasets: [
        {
          label: 'Code Quality',
          data: [82, 84, 83, 85, 87, 86, 85.3],
          borderColor: 'rgb(59, 130, 246)',
          backgroundColor: 'rgba(59, 130, 246, 0.1)',
          tension: 0.4,
        },
        {
          label: 'Test Coverage',
          data: [89, 90, 91, 92, 93, 92, 92.1],
          borderColor: 'rgb(34, 197, 94)',
          backgroundColor: 'rgba(34, 197, 94, 0.1)',
          tension: 0.4,
        },
        {
          label: 'Performance',
          data: [85, 83, 81, 79, 78, 77, 78.5],
          borderColor: 'rgb(239, 68, 68)',
          backgroundColor: 'rgba(239, 68, 68, 0.1)',
          tension: 0.4,
        },
        {
          label: 'Security',
          data: [93, 93, 94, 94, 94, 94, 94.2],
          borderColor: 'rgb(168, 85, 247)',
          backgroundColor: 'rgba(168, 85, 247, 0.1)',
          tension: 0.4,
        }
      ]
    }
  }, [])

  // Quality distribution data
  const distributionData = useMemo(() => {
    const statusCounts = metrics.reduce((acc, metric) => {
      acc[metric.status] = (acc[metric.status] || 0) + 1
      return acc
    }, {} as Record<string, number>)

    return {
      labels: ['Good', 'Warning', 'Critical'],
      datasets: [{
        data: [
          statusCounts.good || 0,
          statusCounts.warning || 0,
          statusCounts.critical || 0
        ],
        backgroundColor: [
          'rgba(34, 197, 94, 0.8)',
          'rgba(245, 158, 11, 0.8)',
          'rgba(239, 68, 68, 0.8)'
        ],
        borderColor: [
          'rgb(34, 197, 94)',
          'rgb(245, 158, 11)',
          'rgb(239, 68, 68)'
        ],
        borderWidth: 2,
      }]
    }
  }, [metrics])

  // Category performance data
  const categoryData = useMemo(() => {
    const categories = [...new Set(metrics.map(m => m.category))]
    const categoryScores = categories.map(category => {
      const categoryMetrics = metrics.filter(m => m.category === category)
      return categoryMetrics.reduce((sum, m) => sum + m.value, 0) / categoryMetrics.length
    })

    return {
      labels: categories,
      datasets: [{
        label: 'Average Score',
        data: categoryScores,
        backgroundColor: [
          'rgba(59, 130, 246, 0.8)',
          'rgba(34, 197, 94, 0.8)',
          'rgba(239, 68, 68, 0.8)',
          'rgba(168, 85, 247, 0.8)'
        ],
        borderColor: [
          'rgb(59, 130, 246)',
          'rgb(34, 197, 94)',
          'rgb(239, 68, 68)',
          'rgb(168, 85, 247)'
        ],
        borderWidth: 2,
      }]
    }
  }, [metrics])

  const trendOptions = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: {
        position: 'top' as const,
      },
      title: {
        display: true,
        text: 'Quality Trends (7 Days)',
        font: { size: 16, weight: 'bold' },
      },
    },
    scales: {
      y: {
        beginAtZero: true,
        max: 100,
        title: {
          display: true,
          text: 'Score (%)'
        }
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
        text: 'Quality Distribution',
        font: { size: 16, weight: 'bold' },
      },
    }
  }

  const categoryOptions = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: {
        display: false,
      },
      title: {
        display: true,
        text: 'Category Performance',
        font: { size: 16, weight: 'bold' },
      },
    },
    scales: {
      y: {
        beginAtZero: true,
        max: 100,
      }
    }
  }

  return (
    <div className="h-full p-6 space-y-6 overflow-y-auto">
      {/* ANOVA Results */}
      <Card className="p-6">
        <h2 className="text-xl font-bold mb-4">ANOVA Analysis Results</h2>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-4">
          <div className="text-center">
            <div className="text-2xl font-bold text-blue-600">
              {anovaResult.fStatistic.toFixed(2)}
            </div>
            <div className="text-sm text-gray-600">F-Statistic</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-green-600">
              {anovaResult.pValue.toFixed(4)}
            </div>
            <div className="text-sm text-gray-600">P-Value</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-purple-600">
              {anovaResult.degreesOfFreedom}
            </div>
            <div className="text-sm text-gray-600">Degrees of Freedom</div>
          </div>
          <div className="text-center">
            <Badge variant={anovaResult.significance ? 'destructive' : 'secondary'}>
              {anovaResult.significance ? 'Significant' : 'Not Significant'}
            </Badge>
          </div>
        </div>

        <div className="space-y-2">
          <h3 className="font-semibold">Group Statistics</h3>
          {anovaResult.groups.map((group, index) => (
            <div key={index} className="flex justify-between items-center p-2 bg-gray-50 rounded">
              <span className="font-medium">{group.name}</span>
              <div className="flex gap-4 text-sm">
                <span>Mean: {group.mean.toFixed(2)}</span>
                <span>Variance: {group.variance.toFixed(2)}</span>
                <span>Count: {group.count}</span>
              </div>
            </div>
          ))}
        </div>
      </Card>

      {/* Quality Metrics Overview */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        {metrics.map((metric) => (
          <Card key={metric.id} className="p-4">
            <div className="flex items-center justify-between mb-2">
              <h3 className="font-semibold text-sm">{metric.name}</h3>
              <div className="flex items-center gap-1">
                {metric.trend === 'up' && <TrendingUp className="w-4 h-4 text-green-500" />}
                {metric.trend === 'down' && <TrendingDown className="w-4 h-4 text-red-500" />}
                {metric.trend === 'stable' && <Minus className="w-4 h-4 text-gray-500" />}
              </div>
            </div>

            <div className="text-2xl font-bold mb-1">
              {metric.value.toFixed(1)}{metric.unit}
            </div>

            <div className="flex items-center justify-between">
              <Badge
                variant={
                  metric.status === 'good' ? 'secondary' :
                  metric.status === 'warning' ? 'default' : 'destructive'
                }
                className="text-xs"
              >
                {metric.status.toUpperCase()}
              </Badge>
              <span className="text-xs text-gray-500">
                Target: {metric.target}{metric.unit}
              </span>
            </div>

            <div className="mt-2 bg-gray-200 rounded-full h-2">
              <div
                className={`h-2 rounded-full ${
                  metric.status === 'good' ? 'bg-green-500' :
                  metric.status === 'warning' ? 'bg-yellow-500' : 'bg-red-500'
                }`}
                style={{ width: `${Math.min(100, (metric.value / metric.target) * 100)}%` }}
              />
            </div>
          </Card>
        ))}
      </div>

      {/* Charts Grid */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Trend Chart */}
        <Card className="p-6">
          <div className="h-80">
            <Line data={trendData} options={trendOptions} />
          </div>
        </Card>

        {/* Quality Distribution */}
        <Card className="p-6">
          <div className="h-80">
            <Doughnut data={distributionData} options={distributionOptions} />
          </div>
        </Card>

        {/* Category Performance */}
        <Card className="p-6">
          <div className="h-80">
            <Bar data={categoryData} options={categoryOptions} />
          </div>
        </Card>

        {/* QC Process Summary */}
        <Card className="p-6">
          <h3 className="text-lg font-bold mb-4">QC Process Summary</h3>
          <div className="space-y-3">
            {processes.map((process) => (
              <div key={process.id} className="flex items-center justify-between p-3 bg-gray-50 rounded">
                <div>
                  <div className="font-medium">{process.name}</div>
                  <div className="text-sm text-gray-600">{process.status}</div>
                </div>
                <div className="text-right">
                  <div className="font-bold">{process.progress}%</div>
                  <div className="w-16 bg-gray-200 rounded-full h-2 mt-1">
                    <div
                      className="bg-blue-600 h-2 rounded-full transition-all"
                      style={{ width: `${process.progress}%` }}
                    />
                  </div>
                </div>
              </div>
            ))}
          </div>
        </Card>
      </div>
    </div>
  )
}
