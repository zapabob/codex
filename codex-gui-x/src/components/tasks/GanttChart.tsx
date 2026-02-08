import { useMemo } from 'react'
import type {
  ChartData,
  ChartOptions
} from 'chart.js'
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
} from 'chart.js'
import { Bar } from 'react-chartjs-2'
import type { Task } from '../../types/tasks'
import { Card } from '../atoms/Card'
import { Badge } from '../atoms/Badge'
import { Briefcase, CheckCircle2, Clock, AlertCircle, BarChart3, TrendingUp } from 'lucide-react'

ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, BarElement, Title, Tooltip, Legend)

interface GanttChartProps {
  tasks: Task[]
}

export function GanttChart({ tasks }: GanttChartProps) {
  const chartData: ChartData<'bar'> = useMemo(() => ({
    labels: tasks.map(t => t.title),
    datasets: [{
      label: 'Phase Completion (%)',
      data: tasks.map(t => t.progress),
      backgroundColor: tasks.map(t => {
        if(t.status === 'done') return 'rgba(16, 185, 129, 0.7)'
        if(t.status === 'in-progress') return 'rgba(99, 102, 241, 0.7)'
        if(t.status === 'review') return 'rgba(245, 158, 11, 0.7)'
        return 'rgba(255, 255, 255, 0.1)'
      }),
      borderColor: 'rgba(255, 255, 255, 0.1)',
      borderWidth: 1,
      borderRadius: 12,
    }]
  }), [tasks])

  const chartOptions: ChartOptions<'bar'> = useMemo(() => ({
    indexAxis: 'y' as const,
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
        legend: { display: false },
        tooltip: {
            backgroundColor: 'rgba(15, 23, 42, 0.9)',
            titleFont: { size: 14, weight: 'bold' as const },
            bodyFont: { size: 12 },
            padding: 12,
            cornerRadius: 16,
            displayColors: false
        }
    },
    scales: {
        x: { grid: { color: 'rgba(255, 255, 255, 0.05)' }, ticks: { color: 'rgba(255, 255, 255, 0.5)', font: { size: 10 } } },
        y: { grid: { display: false }, ticks: { color: 'rgba(255, 255, 255, 0.7)', font: { size: 11, weight: 'bold' as const } } }
    }
  }), [])

  const stats = {
    total: tasks.length,
    done: tasks.filter(t => t.status === 'done').length,
    active: tasks.filter(t => t.status === 'in-progress').length,
    burnRate: '4.2h/d'
  }

  return (
    <div className="space-y-6">
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        {[
          { label: 'Total Initiatives', val: stats.total, icon: Briefcase, color: 'text-indigo-400' },
          { label: 'Finalized', val: stats.done, icon: CheckCircle2, color: 'text-emerald-400' },
          { label: 'In Execution', val: stats.active, icon: TrendingUp, color: 'text-sky-400' },
          { label: 'Burn Velocity', val: stats.burnRate, icon: Clock, color: 'text-amber-400' }
        ].map((stat, i) => (
          <Card key={i} animated>
            <div className="p-4 flex items-center gap-4">
                <div className={`p-3 rounded-2xl bg-muted/30 ${stat.color}`}>
                    <stat.icon size={20} />
                </div>
                <div>
                    <div className="text-xl font-black font-mono tracking-tighter">{stat.val}</div>
                    <div className="text-[10px] font-bold text-muted-foreground uppercase tracking-widest">{stat.label}</div>
                </div>
            </div>
          </Card>
        ))}
      </div>

      <Card animated sx={{ p: 6 }}>
        <div className="flex items-center justify-between mb-8">
            <h3 className="text-lg font-bold flex items-center gap-2 italic">
                <BarChart3 size={20} className="text-primary" />
                Strategic Timeline Analysis
            </h3>
            <Badge color="primary" variant="outline" size="sm" className="font-black">Q1 2026 ARCHITECTURE</Badge>
        </div>
        <div className="h-[400px]">
            <Bar data={chartData} options={chartOptions} />
        </div>
      </Card>

      <Card animated>
        <div className="p-6">
            <h3 className="text-lg font-bold mb-4 italic">Dependency Logic</h3>
            <div className="space-y-3">
                {tasks.slice(0, 3).map((_, i) => (
                    <div key={i} className="p-4 rounded-xl bg-muted/20 border border-border flex items-center gap-4 group hover:bg-muted/30 transition-all">
                        <AlertCircle size={16} className="text-indigo-400" />
                        <div className="flex-1">
                            <p className="text-xs font-bold leading-tight uppercase tracking-tight">
                                Cluster {i+1}: <span className="text-muted-foreground font-medium lowercase">Sequential dependency on</span> Architecture Alpha
                            </p>
                        </div>
                        <Badge size="sm" color="secondary">BLOCKER</Badge>
                    </div>
                ))}
            </div>
        </div>
      </Card>
    </div>
  )
}
