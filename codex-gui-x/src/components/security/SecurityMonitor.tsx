import { useState, useEffect } from 'react'
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
import { Badge } from '../atoms/Badge'
import { Button } from '../atoms/Button'
import { Activity, ShieldCheck, Zap, Globe, HardDrive, Cpu } from 'lucide-react'

ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Legend)

export function SecurityMonitor() {
  const [activeEvents, setActiveEvents] = useState<{ id: number; type: string; msg: string; time: string }[]>([])
  const [monitoring, setMonitoring] = useState(false)
  const [dataPoints, setDataPoints] = useState<number[]>([40, 45, 42, 48, 44, 46, 50])

  // monitoring logic doesn't strictly need a ref if we just use setInterval in effect

  useEffect(() => {
    if (!monitoring) return
    const interval = setInterval(() => {
        const val = 40 + Math.random() * 20
        setDataPoints(prev => [...prev.slice(-19), val])
        setActiveEvents(prev => [{
            id: Date.now(),
            type: ['Network', 'File System', 'Process'][Math.floor(Math.random()*3)],
            msg: `Auth request from cluster Node_${Math.floor(Math.random()*10)}`,
            time: new Date().toLocaleTimeString()
        }, ...prev.slice(0, 9)])
    }, 2000)
    return () => clearInterval(interval)
  }, [monitoring])

  const chartData = {
    labels: Array(dataPoints.length).fill(''),
    datasets: [{
      label: 'Security Pressure',
      data: dataPoints,
      borderColor: '#6366f1',
      backgroundColor: 'rgba(99, 102, 241, 0.1)',
      tension: 0.4,
      pointRadius: 0,
    }]
  }

  return (
    <div className="space-y-6">
      <Card animated sx={{ p: 6 }}>
        <div className="flex items-center justify-between mb-8">
            <div>
                <h2 className="text-xl font-bold">Sentinel Real-time Monitor</h2>
                <div className="flex items-center gap-2 mt-2">
                    <div className={`h-2 w-2 rounded-full ${monitoring ? 'bg-emerald-400 animate-pulse' : 'bg-muted'}`} />
                    <span className="text-[10px] font-bold text-muted-foreground uppercase tracking-widest">{monitoring ? 'Active Surveillance' : 'Monitor Inactive'}</span>
                </div>
            </div>
            <Button 
                onClick={() => setMonitoring(!monitoring)}
                color={monitoring ? 'error' : 'primary'}
                variant={monitoring ? 'outlined' : 'contained'}
            >
                {monitoring ? <Zap size={18} className="mr-2" /> : <Activity size={18} className="mr-2" />}
                {monitoring ? 'Deactivate Monitor' : 'Initiate Surveillance'}
            </Button>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div className="h-64">
                <Line 
                    data={chartData} 
                    options={{
                        responsive: true,
                        maintainAspectRatio: false,
                        scales: { x: { display: false }, y: { display: false } },
                        plugins: { legend: { display: false } }
                    }} 
                />
            </div>
            <div className="grid grid-cols-2 gap-4">
                {[
                    { label: 'Connections', val: monitoring ? '142' : '0', icon: Globe, color: 'text-sky-400' },
                    { label: 'File I/O', val: monitoring ? '8.4 GB/s' : '0', icon: HardDrive, color: 'text-amber-400' },
                    { label: 'CPU Load', val: monitoring ? '4.2%' : '0%', icon: Cpu, color: 'text-indigo-400' },
                    { label: 'Health', val: 'Optimum', icon: ShieldCheck, color: 'text-emerald-400' }
                ].map((stat, i) => (
                    <div key={i} className="p-4 rounded-2xl bg-muted/20 border border-border">
                        <stat.icon size={18} className={`${stat.color} mb-2`} />
                        <div className="text-xl font-black font-mono">{stat.val}</div>
                        <div className="text-[10px] font-bold text-muted-foreground uppercase tracking-widest">{stat.label}</div>
                    </div>
                ))}
            </div>
        </div>
      </Card>

      <Card animated className="flex-1">
        <div className="p-6">
            <h3 className="text-lg font-bold mb-4">Event Stream</h3>
            <div className="space-y-2">
                {activeEvents.map(ev => (
                    <div key={ev.id} className="p-3 rounded-xl bg-card border border-border flex items-center justify-between text-xs animate-in fade-in slide-in-from-left-2 transition-all">
                        <div className="flex items-center gap-3">
                            <Badge color="secondary" size="sm">{ev.type}</Badge>
                            <span className="font-medium">{ev.msg}</span>
                        </div>
                        <span className="font-mono text-muted-foreground">{ev.time}</span>
                    </div>
                ))}
                {activeEvents.length === 0 && (
                    <div className="text-center py-12 text-muted-foreground italic">
                        Initialize monitor to start event capture stream...
                    </div>
                )}
            </div>
        </div>
      </Card>
    </div>
  )
}
