import { useState } from 'react'
import { Badge } from '../atoms/Badge'
import { StatisticalDashboard } from './StatisticalDashboard'
import { QCProcessAutomation } from './QCProcessAutomation'
import { RealTimeMonitoring } from './RealTimeMonitoring'
import { AlertSystem } from './AlertSystem'
import type { QualityMetric, QCProcess, QCAlert } from '../../types/qc'
import { Layout, Zap as AlertIcon } from 'lucide-react'

const SAMPLE_METRICS: QualityMetric[] = [
  { id: 'code_quality', name: 'Code Quality Score', value: 85.3, unit: '%', target: 90, tolerance: 5, status: 'warning', trend: 'up', timestamp: new Date(), category: 'Development' },
  { id: 'test_coverage', name: 'Test Coverage', value: 92.1, unit: '%', target: 95, tolerance: 3, status: 'good', trend: 'stable', timestamp: new Date(), category: 'Testing' },
  { id: 'performance', name: 'Performance Score', value: 78.5, unit: '%', target: 85, tolerance: 7, status: 'critical', trend: 'down', timestamp: new Date(), category: 'Performance' },
  { id: 'security', name: 'Security Rating', value: 94.2, unit: '%', target: 95, tolerance: 2, status: 'good', trend: 'up', timestamp: new Date(), category: 'Security' }
]

const SAMPLE_PROCESSES: QCProcess[] = [
  { id: 'daily_qc', name: 'Daily Quality Check', description: 'Automated daily quality assessment', status: 'completed', progress: 100, startTime: new Date(Date.now() - 3600000), endTime: new Date(), metrics: SAMPLE_METRICS, results: { id: 'r1', processId: 'daily_qc', overallScore: 87.5, passed: true, recommendations: ['Optimize core loops', 'Update dev dependencies'], timestamp: new Date() } }
]

const SAMPLE_ALERTS: QCAlert[] = [
  { id: 'a1', type: 'critical', title: 'Performance Degradation', message: 'Core system latency exceeded threshold of 200ms', metricId: 'performance', threshold: 85, currentValue: 78.5, timestamp: new Date(), acknowledged: false }
]

export function QCPage() {
  const [activeTab, setActiveTab] = useState<'dashboard' | 'automation' | 'monitoring' | 'alerts'>('dashboard')
  const [qualityMetrics] = useState<QualityMetric[]>(SAMPLE_METRICS)
  const [qcProcesses] = useState<QCProcess[]>(SAMPLE_PROCESSES)
  const [alerts] = useState<QCAlert[]>(SAMPLE_ALERTS)

  const overallScore = qualityMetrics.length > 0 
    ? qualityMetrics.reduce((a: number, b: QualityMetric) => a + b.value, 0) / qualityMetrics.length 
    : 0

  return (
    <div className="max-w-[1600px] mx-auto w-full p-8 font-sans">
      <div className="flex flex-col lg:flex-row lg:items-center justify-between gap-6 mb-10">
        <div>
          <div className="flex items-center gap-2 mb-2">
            <div className="p-2 bg-amber-500/20 text-amber-500 rounded-lg">
                <AlertIcon size={24} />
            </div>
            <h1 className="text-4xl font-extrabold tracking-tighter uppercase transition-colors hover:text-primary">QC管理 System</h1>
          </div>
          <p className="text-muted-foreground font-medium pl-1">Advanced Statistical Process Control & Quality Assurance Engine</p>
        </div>

        <div className="flex items-center gap-4 bg-muted/30 p-4 rounded-3xl border border-border backdrop-blur-xl">
            <div className="text-right">
                <div className="text-xs font-bold text-muted-foreground uppercase tracking-widest mb-1">Overall Health</div>
                <div className="text-3xl font-mono font-black text-primary">{overallScore.toFixed(1)}%</div>
            </div>
            <div className="h-12 w-[1px] bg-border" />
            <div className="flex flex-col gap-2">
                {alerts.filter(a => !a.acknowledged).length > 0 && (
                    <Badge color="error" size="md" className="animate-pulse shadow-lg shadow-red-500/20">
                        {alerts.filter(a => !a.acknowledged).length} CRITICAL INCIDENTS
                    </Badge>
                )}
            </div>
        </div>
      </div>

      <nav className="flex gap-2 p-2 bg-muted/20 border border-border rounded-[2rem] w-fit mb-8 backdrop-blur-md">
        {[
          { id: 'dashboard', label: 'Dashboard', icon: AlertIcon },
          { id: 'automation', label: 'Automation', icon: Layout },
          { id: 'monitoring', label: 'Monitoring', icon: AlertIcon },
          { id: 'alerts', label: 'Alerts', icon: AlertIcon },
        ].map(tab => (
          <button
            key={tab.id}
            onClick={() => setActiveTab(tab.id as 'dashboard' | 'automation' | 'monitoring' | 'alerts')}
            className={`flex items-center gap-2 px-6 py-2.5 rounded-[1.5rem] text-sm font-bold transition-all ${activeTab === tab.id ? 'bg-primary text-primary-foreground shadow-xl shadow-primary/20 scale-105' : 'hover:bg-muted font-medium'}`}
          >
            <tab.icon size={16} />
            {tab.label}
          </button>
        ))}
      </nav>

      <div className="relative pt-4">
        {activeTab === 'dashboard' && <StatisticalDashboard metrics={qualityMetrics} />}
        {activeTab === 'automation' && <QCProcessAutomation processes={qcProcesses} onProcessStart={() => {}} onProcessStop={() => {}} />}
        {activeTab === 'monitoring' && <RealTimeMonitoring metrics={qualityMetrics} processes={qcProcesses} />}
        {activeTab === 'alerts' && <AlertSystem alerts={alerts} onAcknowledge={() => {}} />}
      </div>
    </div>
  )
}
