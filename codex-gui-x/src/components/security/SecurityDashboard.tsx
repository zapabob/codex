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
import { Button } from '../atoms/Button'
import type { SecurityMetrics, SecurityAlert, SecurityStatus } from '../../types/security'
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
  const [now] = useState(() => Date.now())
  const securityBreakdown = useMemo(() => {
    const threatScore = Math.max(0, 100 - (metrics.threatsDetected * 10))
    const scanScore = metrics.lastScan ? Math.min(100, (now - metrics.lastScan.getTime()) / (24 * 60 * 60 * 1000) * -10 + 100) : 50
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

  const chartTheme = {
    textColor: 'rgba(255, 255, 255, 0.7)',
    gridColor: 'rgba(255, 255, 255, 0.1)',
  }

  const trendsData = useMemo(() => {
    const last7Days = Array.from({ length: 7 }, (_, i) => {
      const date = new Date()
      date.setDate(date.getDate() - (6 - i))
      return date.toLocaleDateString()
    })

    return {
      labels: last7Days,
      datasets: [
        {
          label: 'Threats Detected',
          data: [2, 1, 0, 3, 1, 0, metrics.threatsDetected],
          borderColor: '#f43f5e',
          backgroundColor: 'rgba(244, 63, 94, 0.1)',
          tension: 0.4,
          fill: true,
        },
        {
          label: 'Files Scanned',
          data: [800, 1200, 950, 1100, 1300, 1000, metrics.filesScanned],
          borderColor: '#6366f1',
          backgroundColor: 'rgba(99, 102, 241, 0.1)',
          tension: 0.4,
          fill: true,
          yAxisID: 'y1',
        }
      ]
    }
  }, [metrics])

  const alertDistribution = useMemo(() => {
    const distribution = alerts.reduce((acc, alert) => {
      if (!alert.resolved) acc[alert.severity] = (acc[alert.severity] || 0) + 1
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
          'rgba(244, 63, 94, 0.7)',
          'rgba(245, 158, 11, 0.7)',
          'rgba(99, 102, 241, 0.7)',
          'rgba(16, 185, 129, 0.7)'
        ],
        borderColor: 'rgba(255, 255, 255, 0.1)',
        borderWidth: 1,
      }]
    }
  }, [alerts])

  const commonOptions = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: { labels: { color: chartTheme.textColor, font: { family: 'Inter' } } }
    },
    scales: {
        y: { 
            grid: { color: chartTheme.gridColor },
            ticks: { color: chartTheme.textColor }
        },
        x: { 
            grid: { display: false },
            ticks: { color: chartTheme.textColor }
        }
    }
  }

  return (
    <div className="space-y-6">
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        {[
          { title: 'System Health', value: `${securityBreakdown.overall.toFixed(1)}%`, icon: Shield, color: 'text-indigo-400', sub: 'Overall protection level' },
          { title: 'Active Threats', value: metrics.threatsDetected, icon: XCircle, color: 'text-rose-400', sub: 'Critical alerts detected' },
          { title: 'Files Scanned', value: metrics.filesScanned.toLocaleString(), icon: FileText, color: 'text-sky-400', sub: `${metrics.totalScans} sessions` },
          { title: 'Quarantined', value: metrics.quarantinedFiles, icon: Activity, color: 'text-amber-400', sub: 'Awaiting clearance' }
        ].map((stat, i) => (
          <Card key={i} animated hover>
            <div className="p-5">
                <div className="flex items-center justify-between mb-4">
                    <span className="text-[10px] font-bold text-muted-foreground uppercase tracking-widest">{stat.title}</span>
                    <stat.icon size={20} className={stat.color} />
                </div>
                <div className="text-3xl font-mono font-black mb-1">{stat.value}</div>
                <div className="text-[10px] text-muted-foreground font-medium">{stat.sub}</div>
            </div>
          </Card>
        ))}
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <Card animated className="lg:col-span-2">
            <div className="p-6">
                <h3 className="text-lg font-bold mb-6">Threat Intelligence Trends</h3>
                <div className="h-80">
                    <Line 
                        data={trendsData} 
                        options={{
                            ...commonOptions,
                            scales: {
                                ...commonOptions.scales,
                                y1: { position: 'right', grid: { display: false }, ticks: { color: '#6366f1' } }
                            }
                        }} 
                    />
                </div>
            </div>
        </Card>

        <Card animated>
            <div className="p-6 h-full flex flex-col">
                <h3 className="text-lg font-bold mb-6">Alert Severity Matrix</h3>
                <div className="flex-1 min-h-[200px]">
                    <Doughnut data={alertDistribution} options={commonOptions} />
                </div>
            </div>
        </Card>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <Card animated>
            <div className="p-6">
                <h3 className="text-lg font-bold mb-6">Protection Integrity</h3>
                <div className="space-y-6">
                    {[
                        { label: 'Threat Protection', score: securityBreakdown.threatScore, color: 'bg-rose-500' },
                        { label: 'Scan Engine', score: securityBreakdown.scanScore, color: 'bg-indigo-500' },
                        { label: 'Alert Triage', score: securityBreakdown.alertScore, color: 'bg-amber-500' },
                        { label: 'Quarantine Protocol', score: securityBreakdown.quarantineScore, color: 'bg-emerald-500' }
                    ].map((item, i) => (
                        <div key={i} className="space-y-2">
                            <div className="flex justify-between text-xs font-bold uppercase tracking-wider">
                                <span>{item.label}</span>
                                <span>{item.score.toFixed(1)}%</span>
                            </div>
                            <div className="h-1.5 bg-muted rounded-full overflow-hidden">
                                <div className={`h-full ${item.color} rounded-full`} style={{ width: `${item.score}%` }} />
                            </div>
                        </div>
                    ))}
                </div>
            </div>
        </Card>

        <Card animated>
            <div className="p-6">
                <h3 className="text-lg font-bold mb-4">Active Lockdown Pulse</h3>
                <div className="space-y-3 max-h-[300px] overflow-y-auto pr-2">
                    {alerts.slice(0, 5).map(alert => (
                        <div key={alert.id} className="p-3 rounded-2xl bg-muted/20 border border-border group hover:bg-muted/40 transition-all">
                            <div className="flex items-center gap-3">
                                <div className={`h-8 w-8 rounded-xl flex items-center justify-center ${alert.severity === 'critical' ? 'bg-rose-500/20 text-rose-500' : 'bg-amber-500/20 text-amber-500'}`}>
                                    <AlertTriangle size={16} />
                                </div>
                                <div className="flex-1">
                                    <div className="flex items-center justify-between mb-0.5">
                                        <h4 className="text-sm font-bold">{alert.title}</h4>
                                        <Badge color={alert.severity === 'critical' ? 'error' : 'warning'} size="sm">{alert.severity.toUpperCase()}</Badge>
                                    </div>
                                    <p className="text-[10px] text-muted-foreground line-clamp-1">{alert.description}</p>
                                </div>
                            </div>
                        </div>
                    ))}
                </div>
                <Button fullWidth variant="outlined" size="small" className="mt-4">
                    View Comprehensive Logs
                </Button>
            </div>
        </Card>
      </div>
    </div>
  )
}
