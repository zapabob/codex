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
import { Card } from '../atoms/Card'
import { Badge } from '../atoms/Badge'
import type { QualityMetric, QCProcess, AnovaResult } from '../../types/qc'
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
}

// ANOVA calculation utility (same as legacy)
function calculateAnova(groups: number[][]): AnovaResult {
  const groupStats = groups.map((group, index) => ({
    name: `Group ${index + 1}`,
    mean: group.length > 0 ? group.reduce((sum, val) => sum + val, 0) / group.length : 0,
    variance: group.length > 1 ? group.reduce((sum, val) => sum + Math.pow(val - (group.reduce((s, v) => s + v, 0) / group.length), 2), 0) / (group.length - 1) : 0,
    count: group.length
  }))

  const allValues = groups.flat()
  if (allValues.length === 0) return { fStatistic: 0, pValue: 1, degreesOfFreedom: 0, significance: false, groups: groupStats }

  const grandMean = allValues.reduce((sum, val) => sum + val, 0) / allValues.length
  const totalSS = allValues.reduce((sum, val) => sum + Math.pow(val - grandMean, 2), 0)

  const betweenSS = groupStats.reduce((sum, group) =>
    sum + group.count * Math.pow(group.mean - grandMean, 2), 0
  )

  const withinSS = totalSS - betweenSS
  const dfBetween = Math.max(0, groups.length - 1)
  const dfWithin = Math.max(0, allValues.length - groups.length)

  const fStatistic = (dfWithin > 0 && dfBetween > 0) ? (betweenSS / dfBetween) / (withinSS / dfWithin) : 0
  const pValue = Math.exp(-fStatistic / 2)

  return {
    fStatistic,
    pValue,
    degreesOfFreedom: dfBetween + dfWithin,
    significance: pValue < 0.05,
    groups: groupStats
  }
}

export function StatisticalDashboard({ metrics }: StatisticalDashboardProps) {
  const anovaResult = useMemo(() => {
    const categories = [...new Set(metrics.map(m => m.category))]
    const groups = categories.map(category =>
      metrics.filter(m => m.category === category).map(m => m.value)
    )
    return calculateAnova(groups)
  }, [metrics])

  const chartTheme = {
    textColor: 'rgba(255, 255, 255, 0.7)',
    gridColor: 'rgba(255, 255, 255, 0.1)',
  }

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
          fill: true,
        },
        {
          label: 'Test Coverage',
          data: [89, 90, 91, 92, 93, 92, 92.1],
          borderColor: 'rgb(34, 197, 94)',
          backgroundColor: 'rgba(34, 197, 94, 0.1)',
          tension: 0.4,
          fill: true,
        },
        {
          label: 'Performance',
          data: [85, 83, 81, 79, 78, 77, 78.5],
          borderColor: 'rgb(239, 68, 68)',
          backgroundColor: 'rgba(239, 68, 68, 0.1)',
          tension: 0.4,
          fill: true,
        }
      ]
    }
  }, [])

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
          'rgba(34, 197, 94, 0.7)',
          'rgba(245, 158, 11, 0.7)',
          'rgba(239, 68, 68, 0.7)'
        ],
        borderColor: 'rgba(255, 255, 255, 0.1)',
        borderWidth: 1,
      }]
    }
  }, [metrics])

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
        backgroundColor: 'rgba(99, 102, 241, 0.7)',
        borderColor: 'rgb(99, 102, 241)',
        borderWidth: 1,
        borderRadius: 8,
      }]
    }
  }, [metrics])

  const commonOptions = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: {
        labels: { color: chartTheme.textColor, font: { family: 'Inter' } }
      },
      title: {
        display: true,
        color: chartTheme.textColor,
        font: { size: 14, weight: 'bold' as const, family: 'Inter' }
      }
    }
  }

  return (
    <div className="space-y-6">
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <Card animated className="lg:col-span-2">
            <div className="p-6">
                <h3 className="text-lg font-bold mb-4">ANOVA Analysis</h3>
                <div className="grid grid-cols-2 sm:grid-cols-4 gap-4 mb-6">
                    <div className="p-3 rounded-2xl bg-muted/30 border border-border">
                        <div className="text-xl font-bold text-primary">{anovaResult.fStatistic.toFixed(2)}</div>
                        <div className="text-[10px] font-bold text-muted-foreground uppercase">F-Stat</div>
                    </div>
                    <div className="p-3 rounded-2xl bg-muted/30 border border-border">
                        <div className="text-xl font-bold text-emerald-400">{anovaResult.pValue.toFixed(4)}</div>
                        <div className="text-[10px] font-bold text-muted-foreground uppercase">P-Value</div>
                    </div>
                    <div className="p-3 rounded-2xl bg-muted/30 border border-border">
                        <div className="text-xl font-bold text-indigo-400">{anovaResult.degreesOfFreedom}</div>
                        <div className="text-[10px] font-bold text-muted-foreground uppercase">DoF</div>
                    </div>
                    <div className="flex items-center justify-center">
                        <Badge color={anovaResult.significance ? 'error' : 'secondary'} size="md">
                            {anovaResult.significance ? 'SIGNIFICANT' : 'NOT SIGNIFICANT'}
                        </Badge>
                    </div>
                </div>
                <div className="space-y-2 max-h-[160px] overflow-y-auto pr-2">
                    {anovaResult.groups.map((group, i) => (
                        <div key={i} className="flex items-center justify-between p-2 rounded-xl bg-card border border-border text-xs">
                            <span className="font-bold">{group.name}</span>
                            <div className="flex gap-3 text-muted-foreground">
                                <span>M: {group.mean.toFixed(1)}</span>
                                <span>V: {group.variance.toFixed(1)}</span>
                                <span className="text-primary">N: {group.count}</span>
                            </div>
                        </div>
                    ))}
                </div>
            </div>
        </Card>

        <Card animated>
            <div className="p-6 h-full flex flex-col">
                <h3 className="text-lg font-bold mb-4">Quality Distribution</h3>
                <div className="flex-1 relative min-h-[200px]">
                    <Doughnut 
                        data={distributionData} 
                        options={{
                            ...commonOptions,
                            plugins: { ...commonOptions.plugins, title: { ...commonOptions.plugins.title, display: false } }
                        }} 
                    />
                </div>
            </div>
        </Card>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        {metrics.map((metric) => (
          <Card key={metric.id} animated hover>
            <div className="p-4">
                <div className="flex items-center justify-between mb-3">
                    <span className="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">{metric.category}</span>
                    {metric.trend === 'up' && <TrendingUp size={14} className="text-emerald-400" />}
                    {metric.trend === 'down' && <TrendingDown size={14} className="text-red-400" />}
                    {metric.trend === 'stable' && <Minus size={14} className="text-muted-foreground" />}
                </div>
                <h4 className="font-bold text-sm truncate mb-1">{metric.name}</h4>
                <div className="text-2xl font-mono font-bold text-primary mb-3">
                    {metric.value.toFixed(1)}<span className="text-xs ml-1">{metric.unit}</span>
                </div>
                <div className="flex items-center justify-between mb-2">
                    <Badge variant="secondary" color={metric.status === 'good' ? 'success' : metric.status === 'warning' ? 'warning' : 'error'}>
                        {metric.status.toUpperCase()}
                    </Badge>
                    <span className="text-[10px] font-mono text-muted-foreground">T: {metric.target}{metric.unit}</span>
                </div>
                <div className="h-1 bg-muted rounded-full overflow-hidden">
                    <div 
                        className={`h-full rounded-full ${metric.status === 'good' ? 'bg-emerald-500' : metric.status === 'warning' ? 'bg-amber-500' : 'bg-red-500'}`}
                        style={{ width: `${Math.min(100, (metric.value / metric.target) * 100)}%` }}
                    />
                </div>
            </div>
          </Card>
        ))}
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <Card animated>
            <div className="p-6">
                <div className="h-80">
                    <Line 
                        data={trendData} 
                        options={{
                            ...commonOptions,
                            plugins: { ...commonOptions.plugins, title: { ...commonOptions.plugins.title, text: 'Quality Trends (7 Days)' } }
                        }} 
                    />
                </div>
            </div>
        </Card>

        <Card animated>
            <div className="p-6">
                <div className="h-80">
                    <Bar 
                        data={categoryData} 
                        options={{
                            ...commonOptions,
                            plugins: { ...commonOptions.plugins, title: { ...commonOptions.plugins.title, text: 'Category Performance' } }
                        }} 
                    />
                </div>
            </div>
        </Card>
      </div>
    </div>
  )
}
