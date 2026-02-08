import { useState, useEffect } from 'react'
import { SecurityDashboard } from './SecurityDashboard'
import { MalwareScanner } from './MalwareScanner'
import { QuarantineManager } from './QuarantineManager'
import { SecurityMonitor } from './SecurityMonitor'
import { Badge } from '../atoms/Badge'
import { SecurityMetrics, SecurityAlert, SecurityStatus } from '../../types/security'
import { Shield, Search, Lock, Activity, FileBarChart } from 'lucide-react'

export function SecurityPage() {
  const [activeTab, setActiveTab] = useState<'dashboard' | 'scanner' | 'quarantine' | 'monitor'>('dashboard')
  const [metrics, setMetrics] = useState<SecurityMetrics>({
    lastScan: new Date(),
    totalScans: 42,
    threatsDetected: 2,
    filesScanned: 15400,
    quarantinedFiles: 2,
    systemHealth: 94,
    realTimeMonitoring: true
  })
  const [alerts, setAlerts] = useState<SecurityAlert[]>([
    { id: 'a1', type: 'malware', severity: 'critical', title: 'Suspicious Execution', description: 'Unauthorized binary attempted memory injection', affectedFiles: ['sys_init.exe'], timestamp: new Date(), resolved: false }
  ])

  return (
    <div className="max-w-[1600px] mx-auto w-full p-8">
      <div className="flex flex-col lg:flex-row lg:items-center justify-between gap-8 mb-12">
        <div>
          <div className="flex items-center gap-3 mb-2">
            <div className="h-12 w-12 rounded-2xl bg-indigo-500/20 text-indigo-400 flex items-center justify-center shadow-lg shadow-indigo-500/10">
                <Shield size={32} />
            </div>
            <h1 className="text-4xl font-black tracking-tighter uppercase italic">Security Center</h1>
          </div>
          <p className="text-muted-foreground font-medium pl-1 tracking-tight">Enterprise-grade threat intelligence and system hardening protocols</p>
        </div>

        <div className="flex items-center gap-6 p-1 bg-muted/20 border border-border rounded-[2rem] backdrop-blur-3xl px-6 py-4">
            <div className="text-center">
                <div className="text-[10px] font-bold text-muted-foreground uppercase tracking-widest mb-1">Protection</div>
                <div className="text-2xl font-black font-mono text-emerald-400">ACTIVE</div>
            </div>
            <div className="h-8 w-[1px] bg-border" />
            <div className="text-center">
                <div className="text-[10px] font-bold text-muted-foreground uppercase tracking-widest mb-1">System Health</div>
                <div className="text-2xl font-black font-mono text-indigo-400">94%</div>
            </div>
        </div>
      </div>

      <nav className="flex flex-wrap gap-2 p-1.5 bg-muted/30 border border-border rounded-3xl w-fit mb-10 backdrop-blur-xl">
        {[
          { id: 'dashboard', label: 'Overview', icon: Shield },
          { id: 'scanner', label: 'Scanner', icon: Search },
          { id: 'quarantine', label: 'Isolation', icon: Lock },
          { id: 'monitor', label: 'Sentinel', icon: Activity },
        ].map(tab => (
          <button
            key={tab.id}
            onClick={() => setActiveTab(tab.id as any)}
            className={`flex items-center gap-2 px-6 py-2.5 rounded-2xl text-sm font-bold transition-all ${activeTab === tab.id ? 'bg-primary text-primary-foreground shadow-2xl shadow-primary/30 scale-105' : 'hover:bg-muted text-muted-foreground hover:text-foreground'}`}
          >
            <tab.icon size={16} />
            {tab.label}
          </button>
        ))}
      </nav>

      <div className="relative animate-in fade-in slide-in-from-bottom-4 duration-500">
        {activeTab === 'dashboard' && <SecurityDashboard metrics={metrics} alerts={alerts} status="secure" />}
        {activeTab === 'scanner' && <MalwareScanner onScanComplete={() => {}} />}
        {activeTab === 'quarantine' && <QuarantineManager onFileRestored={() => {}} onFileDeleted={() => {}} />}
        {activeTab === 'monitor' && <SecurityMonitor />}
      </div>
    </div>
  )
}
