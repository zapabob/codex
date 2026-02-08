import { useState } from 'react'
import { Card } from '../atoms/Card'
import { Button } from '../atoms/Button'
import type { ScanResult, SecurityAlert, SecurityMetrics } from '../../types/security'
import {
  FileText,
  Download,
  Shield,
  BarChart3,
  Clock
} from 'lucide-react'

interface SecurityReportsProps {
  scanResults: ScanResult[]
  alerts: SecurityAlert[]
  metrics: SecurityMetrics
}

export function SecurityReports({ scanResults, alerts, metrics }: SecurityReportsProps) {
  const [selectedTimeRange, setSelectedTimeRange] = useState<'24h' | '7d' | '30d' | '90d'>('7d')
  const [isGenerating, setIsGenerating] = useState(false)

  // Summary data could be used here if needed
  const summaryInfo = {
    totalScans: metrics.totalScans,
    detected: alerts.length,
    resultsCount: scanResults.length
  };
  console.log('Report Summary:', summaryInfo);

  const handleGenerate = async () => {
    setIsGenerating(true)
    await new Promise(r => setTimeout(r, 2000))
    setIsGenerating(false)
    alert("Security Report Generated & Downloaded (simulated)")
  }

  return (
    <div className="space-y-6">
      <Card animated sx={{ p: 6 }}>
        <div className="flex flex-col md:flex-row md:items-center justify-between gap-6 mb-8">
            <div>
                <h2 className="text-xl font-bold">Threat Intelligence Reports</h2>
                <p className="text-xs text-muted-foreground mt-1 uppercase font-bold tracking-widest">Aggregate security data for compliance auditing</p>
            </div>
            
            <div className="flex items-center gap-3">
                <select 
                    value={selectedTimeRange}
                    onChange={(e) => setSelectedTimeRange(e.target.value as '24h' | '7d' | '30d' | '90d')}
                    className="bg-muted/30 border border-border rounded-xl px-4 py-2 text-sm outline-none focus:ring-1 focus:ring-primary/50"
                >
                    <option value="24h">L-24 Hours</option>
                    <option value="7d">L-7 Days</option>
                    <option value="30d">L-30 Days</option>
                    <option value="90d">L-90 Days</option>
                </select>
                <Button 
                    onClick={handleGenerate} 
                    loading={isGenerating}
                    className="min-w-[160px]"
                >
                    <Download size={18} className="mr-2" /> 
                    Generate PDF
                </Button>
            </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
            <div className="space-y-4">
                <h3 className="text-sm font-bold uppercase text-muted-foreground tracking-widest">Executive Summary Preview</h3>
                <div className="p-5 rounded-3xl bg-muted/20 border border-border space-y-4">
                    <div className="flex justify-between items-center text-sm font-medium">
                        <span className="text-muted-foreground">Assessment Period</span>
                        <span className="font-mono">{selectedTimeRange.toUpperCase()}</span>
                    </div>
                    <div className="flex justify-between items-center text-sm font-medium">
                        <span className="text-muted-foreground">Scans Conducted</span>
                        <span className="font-mono">14</span>
                    </div>
                    <div className="flex justify-between items-center text-sm font-medium">
                        <span className="text-muted-foreground">Vulnerabilities Locked</span>
                        <span className="font-mono text-emerald-400">2</span>
                    </div>
                    <div className="flex justify-between items-center text-sm font-medium">
                        <span className="text-muted-foreground">Avg Pulse Integrity</span>
                        <span className="font-mono text-indigo-400">98.4%</span>
                    </div>
                </div>
            </div>

            <div className="space-y-4">
                <h3 className="text-sm font-bold uppercase text-muted-foreground tracking-widest">Cluster Health Distribution</h3>
                <div className="grid grid-cols-2 gap-3">
                    {[
                        { label: 'Files', icon: FileText, color: 'text-sky-400' },
                        { label: 'Network', icon: BarChart3, color: 'text-indigo-400' },
                        { label: 'System', icon: Shield, color: 'text-emerald-400' },
                        { label: 'App Layer', icon: Clock, color: 'text-amber-400' }
                    ].map((item, i) => (
                        <div key={i} className="p-4 rounded-2xl bg-muted/20 border border-border flex items-center gap-3">
                            <item.icon size={16} className={item.color} />
                            <span className="text-xs font-bold uppercase tracking-tight">{item.label}</span>
                        </div>
                    ))}
                </div>
            </div>
        </div>
      </Card>

      <Card animated>
        <div className="p-6">
            <h3 className="text-lg font-bold mb-6">Strategic Security Hardening</h3>
            <div className="space-y-3">
                {[
                    { rec: 'Implement hardware-backed encryption for the User data cluster', sev: 'high' },
                    { rec: 'Rotate system-wide authentication tokens (Next scheduled: 48h)', sev: 'medium' },
                    { rec: 'Update virus signatures for the Malware Scanner engine', sev: 'low' }
                ].map((item, i) => (
                    <div key={i} className="flex items-start gap-4 p-4 rounded-2xl bg-muted/10 border border-border group hover:bg-muted/20 transition-all">
                        <div className={`mt-1 h-2 w-2 rounded-full shrink-0 ${item.sev === 'high' ? 'bg-rose-500 shadow-[0_0_8px_rgba(244,63,94,0.5)]' : item.sev === 'medium' ? 'bg-amber-500' : 'bg-emerald-500'}`} />
                        <div className="flex-1">
                            <p className="text-sm font-medium leading-relaxed">{item.rec}</p>
                            <span className="text-[10px] font-bold text-muted-foreground uppercase mt-1 block tracking-widest">{item.sev.toUpperCase()} PRIORITY</span>
                        </div>
                    </div>
                ))}
            </div>
        </div>
      </Card>
    </div>
  )
}
