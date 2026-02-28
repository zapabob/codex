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
import { Card } from '../atoms/Card'
import { Button } from '../atoms/Button'
import type { QCProcess, QualityMetric } from '../../types/qc'
import { Activity, XCircle } from 'lucide-react'

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
  processes?: QCProcess[]
}

export function RealTimeMonitoring({ metrics }: RealTimeMonitoringProps) {
  const [isMonitoring, setIsMonitoring] = useState(false)
  const [realtimeData, setRealtimeData] = useState<Array<{
    timestamp: Date
    metrics: Record<string, number>
  }>>([])
  const intervalRef = useRef<ReturnType<typeof setInterval> | null>(null)

  const toggleMonitoring = () => {
    if (isMonitoring) {
      if (intervalRef.current) clearInterval(intervalRef.current)
      setIsMonitoring(false)
    } else {
      setIsMonitoring(true)
      intervalRef.current = setInterval(() => {
        const newDataPoint = {
          timestamp: new Date(),
          metrics: metrics.reduce((acc, metric) => {
            const variation = (Math.random() - 0.5) * 2
            acc[metric.id] = Math.max(0, Math.min(100, metric.value + variation))
            return acc
          }, {} as Record<string, number>)
        }
        setRealtimeData(prev => [...prev.slice(-19), newDataPoint])
      }, 2000)
    }
  }

  useEffect(() => {
    return () => { if (intervalRef.current) clearInterval(intervalRef.current) }
  }, [])

  const chartTheme = {
    textColor: 'rgba(255, 255, 255, 0.7)',
    white: 'rgba(255, 255, 255, 0.1)',
  }

  const chartData = {
    labels: realtimeData.map(point => point.timestamp.toLocaleTimeString()),
    datasets: metrics.map((metric, index) => ({
      label: metric.name,
      data: realtimeData.map(p => p.metrics[metric.id] || metric.value),
      borderColor: ['#6366f1', '#10b981', '#f43f5e', '#f59e0b'][index % 4],
      backgroundColor: 'rgba(99, 102, 241, 0.1)',
      tension: 0.4,
      pointRadius: 0,
    }))
  }

  return (
    <div className="space-y-6">
      <Card animated sx={{ p: 6 }}>
        <div className="flex items-center justify-between mb-8">
          <div>
            <h2 className="text-xl font-bold">Real-time Quality Engine</h2>
            <div className="flex items-center gap-2 mt-2">
                <div className={`h-2 w-2 rounded-full ${isMonitoring ? 'bg-emerald-400 animate-pulse shadow-[0_0_8px_rgba(52,211,153,0.5)]' : 'bg-muted'}`} />
                <span className="text-[10px] font-bold text-muted-foreground uppercase tracking-widest">
                    {isMonitoring ? 'Streaming Live' : 'System Standby'}
                </span>
            </div>
          </div>

          <Button
            onClick={toggleMonitoring}
            color={isMonitoring ? 'error' : 'primary'}
            variant={isMonitoring ? 'outlined' : 'contained'}
          >
            {isMonitoring ? <XCircle size={18} className="mr-2" /> : <Activity size={18} className="mr-2" />}
            {isMonitoring ? 'Halt Capture' : 'Initiate Monitoring'}
          </Button>
        </div>

        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
          {metrics.map((metric) => {
            const currentVal = realtimeData.length > 0 ? realtimeData[realtimeData.length-1].metrics[metric.id] : metric.value
            return (
              <div key={metric.id} className="p-4 rounded-2xl bg-muted/20 border border-border group hover:bg-muted/30 transition-colors">
                <div className="flex items-center justify-between mb-2">
                    <span className="text-[10px] font-bold text-muted-foreground uppercase">{metric.name}</span>
                    <Activity size={12} className={isMonitoring ? 'text-primary' : 'text-muted-foreground'} />
                </div>
                <div className="text-xl font-mono font-bold">{currentVal.toFixed(1)}<span className="text-[10px] ml-1">{metric.unit}</span></div>
                <div className="mt-3 flex gap-0.5 items-end h-6">
                    {realtimeData.slice(-10).map((p, i) => (
                        <div 
                            key={i} 
                            className="flex-1 bg-primary/40 rounded-t-sm" 
                            style={{ height: `${(p.metrics[metric.id]/100)*100}%` }}
                        />
                    ))}
                </div>
              </div>
            )
          })}
        </div>
      </Card>

      <Card animated className="flex-1">
        <div className="p-6 h-[400px]">
          {realtimeData.length > 0 ? (
            <Line 
                data={chartData} 
                options={{
                    responsive: true,
                    maintainAspectRatio: false,
                    scales: {
                        y: { 
                            beginAtZero: true, 
                            max: 100, 
                            grid: { color: chartTheme.white },
                            ticks: { color: chartTheme.textColor, font: { family: 'Inter' } }
                        },
                        x: { 
                            grid: { display: false },
                            ticks: { color: chartTheme.textColor, font: { family: 'Inter' } }
                        }
                    },
                    plugins: {
                        legend: { labels: { color: chartTheme.textColor, font: { family: 'Inter' } } }
                    }
                }} 
            />
          ) : (
            <div className="h-full flex flex-col items-center justify-center text-muted-foreground">
                <Activity size={48} className="mb-4 opacity-20" />
                <p className="text-sm font-medium">Activate stream to visualize real-time quality matrices</p>
            </div>
          )}
        </div>
      </Card>
    </div>
  )
}
