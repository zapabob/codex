'use client'

import { useState } from 'react'
import { Card } from '../atoms/Card'
import { Badge } from '../atoms/Badge'
import type { ExecutionResult, DevelopmentTask } from '../../types/ai-tools'
import {
  CheckCircle,
  XCircle,
  AlertTriangle,
  FileText,
  Download,
  ThumbsUp,
  ThumbsDown,
  GitMerge,
  Target,
  TrendingUp,
  MessageSquare,
  Search
} from 'lucide-react'

interface ResultIntegratorProps {
  results: ExecutionResult[]
  tasks: DevelopmentTask[]
  onResultAccept: (result: ExecutionResult) => void
  onResultReject: (result: ExecutionResult) => void
}

export function ResultIntegrator({ results, tasks, onResultAccept, onResultReject }: ResultIntegratorProps) {
  const [selectedResult, setSelectedResult] = useState<ExecutionResult | null>(null)
  const [filter, setFilter] = useState<'all' | 'successful' | 'failed' | 'conflicts'>('all')

  const filteredResults = results.filter(result => {
    switch (filter) {
      case 'successful': return result.success
      case 'failed': return !result.success
      case 'conflicts': return result.errors.length > 1
      default: return true
    }
  })

  const handleResultAction = (result: ExecutionResult, action: 'accept' | 'reject') => {
    if (action === 'accept') onResultAccept(result)
    else onResultReject(result)
  }

  const getQualityScoreColor = (score: number) => {
    if (score >= 0.8) return 'text-green-400'
    if (score >= 0.6) return 'text-amber-400'
    return 'text-destructive'
  }

  const getTaskTitle = (taskId: string) => {
    return tasks.find(t => t.id === taskId)?.title || `Directive: ${taskId.substring(0,8)}`
  }

  const stats = {
    total: results.length,
    successful: results.filter(r => r.success).length,
    failed: results.filter(r => !r.success).length,
    avgQuality: results.length > 0 ? results.reduce((sum, r) => sum + r.qualityScore, 0) / results.length : 0,
    conflicts: results.filter(r => r.errors.length > 1).length,
  }

  return (
    <div className="h-full p-6 space-y-6 overflow-y-auto custom-scrollbar">
      <div className="space-y-1">
        <h2 className="text-2xl font-bold tracking-tight">Intelligence Convergence</h2>
        <p className="text-sm text-muted-foreground">Audit and integrate outputs from parallel agent clusters</p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-5 gap-4">
        {[
          { label: 'Total Audits', value: stats.total, icon: FileText, color: 'text-blue-400' },
          { label: 'Verified', value: stats.successful, icon: CheckCircle, color: 'text-green-400' },
          { label: 'Rejected', value: stats.failed, icon: XCircle, color: 'text-rose-400' },
          { label: 'Avg Quality', value: stats.avgQuality.toFixed(2), icon: TrendingUp, color: 'text-primary' },
          { label: 'Conflicts', value: stats.conflicts, icon: AlertTriangle, color: 'text-amber-400' }
        ].map((stat, i) => (
          <Card key={i} className="p-4 bg-white/5 border-white/5">
            <div className="flex items-center gap-3">
              <stat.icon className={`w-8 h-8 ${stat.color}`} />
              <div>
                <div className="text-2xl font-bold tracking-tight">{stat.value}</div>
                <div className="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">{stat.label}</div>
              </div>
            </div>
          </Card>
        ))}
      </div>

      <div className="flex items-center gap-2 p-1.5 bg-black/40 border border-white/5 rounded-2xl w-fit">
        {[
          { id: 'all', label: 'ALL UNITS', count: stats.total },
          { id: 'successful', label: 'VERIFIED', count: stats.successful },
          { id: 'failed', label: 'FLAGGED', count: stats.failed },
          { id: 'conflicts', label: 'CONFLICTS', count: stats.conflicts },
        ].map((opt) => (
          <button
            key={opt.id}
            onClick={() => setFilter(opt.id as typeof filter)}
            className={`px-4 py-2 rounded-xl text-[10px] font-bold tracking-widest transition-all ${
              filter === opt.id ? 'bg-primary text-primary-foreground shadow-lg shadow-primary/20' : 'text-muted-foreground hover:text-white'
            }`}
          >
            {opt.label} <span className="ml-1 opacity-60 tabular-nums">{opt.count}</span>
          </button>
        ))}
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 items-start">
        <Card className="p-6 bg-white/5 border-white/5">
          <div className="relative mb-6">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
            <input 
              type="text" 
              placeholder="Filter by directive ID or title..."
              className="w-full bg-black/20 border border-white/10 rounded-xl py-2.5 pl-10 pr-4 text-xs outline-none focus:ring-1 focus:ring-primary/40"
            />
          </div>

          <div className="space-y-3 max-h-[600px] overflow-y-auto pr-2 custom-scrollbar">
            {filteredResults.map((result) => (
              <div
                key={result.taskId}
                onClick={() => setSelectedResult(result)}
                className={`p-4 border rounded-2xl cursor-pointer transition-all ${
                  selectedResult?.taskId === result.taskId 
                  ? 'bg-primary/10 border-primary shadow-lg shadow-primary/5' 
                  : 'bg-white/2 border-white/5 hover:bg-white/5'
                }`}
              >
                <div className="flex items-center justify-between mb-3">
                  <div className="flex items-center gap-3">
                    {result.success ? <CheckCircle className="w-4 h-4 text-green-500" /> : <XCircle className="w-4 h-4 text-rose-500" />}
                    <h3 className="font-bold text-sm truncate max-w-[180px]">{getTaskTitle(result.taskId)}</h3>
                  </div>
                  <Badge variant="outline" className={`text-[9px] font-bold ${result.success ? 'text-green-400 border-green-500/20' : 'text-rose-400 border-rose-500/20'}`}>
                    {result.success ? 'VERIFIED' : 'FLAGGED'}
                  </Badge>
                </div>

                <div className="flex items-center justify-between text-[11px] tabular-nums">
                  <div className="flex gap-4">
                    <span className="text-muted-foreground font-medium">Quality Index: <span className={getQualityScoreColor(result.qualityScore)}>{(result.qualityScore * 100).toFixed(0)}%</span></span>
                    <span className="text-muted-foreground font-medium">Lat: <span className="text-white">{result.executionTime.toFixed(1)}s</span></span>
                  </div>
                  <div className="text-[10px] opacity-40 italic"># {result.taskId.substring(0,8)}</div>
                </div>
              </div>
            ))}
            {filteredResults.length === 0 && (
              <div className="text-center py-20 opacity-20">
                <GitMerge size={48} className="mx-auto mb-4" />
                <p className="text-xs font-bold uppercase tracking-widest">No Intelligence Logged</p>
              </div>
            )}
          </div>
        </Card>

        <Card className="p-6 bg-white/5 border-white/5 relative overflow-hidden min-h-[600px]">
          {selectedResult ? (
            <div className="space-y-8 animate-in fade-in slide-in-from-right-4 duration-500">
              <div className="flex items-center justify-between pb-6 border-b border-white/5">
                <div>
                  <h3 className="text-xl font-bold tracking-tight">{getTaskTitle(selectedResult.taskId)}</h3>
                  <div className="flex gap-3 mt-1.5 font-mono text-[10px] text-muted-foreground uppercase opacity-60">
                    <span>Cluster ID: {selectedResult.taskId}</span>
                    <span>•</span>
                    <span>Lat: {selectedResult.executionTime}s</span>
                  </div>
                </div>
                <button className="p-2.5 rounded-xl bg-white/5 border border-white/10 hover:bg-white/10 transition-colors">
                  <Download size={18} className="text-primary" />
                </button>
              </div>

              <div className="space-y-4">
                <label className="text-[10px] font-bold uppercase tracking-widest text-primary/80 flex items-center gap-2">
                  <GitMerge size={12} />
                  Integrated Intelligence Output
                </label>
                <div className="p-5 bg-black/40 border border-white/5 rounded-2xl font-mono text-[11px] leading-relaxed max-h-64 overflow-y-auto custom-scrollbar text-white/90">
                  <pre className="whitespace-pre-wrap">{selectedResult.integratedOutput}</pre>
                </div>
              </div>

              <div className="space-y-4">
                <label className="text-[10px] font-bold uppercase tracking-widest text-blue-400/80 flex items-center gap-2">
                  <Target size={12} />
                  Agent Unit Verification ({selectedResult.subtaskResults.length})
                </label>
                <div className="grid gap-3">
                  {selectedResult.subtaskResults.map((sub, i) => (
                    <div key={i} className="p-3.5 bg-white/2 border border-white/5 rounded-xl flex items-center justify-between group hover:bg-white/5 transition-all">
                      <div className="flex items-center gap-4">
                        <div className={`w-1.5 h-1.5 rounded-full ${sub.success ? 'bg-green-500' : 'bg-rose-500'}`} />
                        <div>
                          <div className="text-xs font-bold tracking-tight">{sub.toolId}</div>
                          <div className="text-[9px] text-muted-foreground mt-0.5 max-w-[200px] truncate">{sub.output}</div>
                        </div>
                      </div>
                      <div className="text-right">
                        <div className={`text-[10px] font-bold ${getQualityScoreColor(sub.qualityScore)}`}>{(sub.qualityScore*100).toFixed(0)}%</div>
                        <div className="text-[9px] text-muted-foreground tabular-nums opacity-60">{sub.executionTime}s</div>
                      </div>
                    </div>
                  ))}
                </div>
              </div>

              <div className="flex gap-4 pt-8">
                <button 
                  onClick={() => handleResultAction(selectedResult, 'accept')}
                  className="flex-[2] h-12 bg-primary text-primary-foreground rounded-xl font-bold text-xs uppercase tracking-widest flex items-center justify-center gap-3 hover:scale-105 active:scale-95 transition-all shadow-xl shadow-primary/20"
                >
                  <ThumbsUp size={16} fill="currentColor" />
                  INTEGRATE UNIT
                </button>
                <button 
                  onClick={() => handleResultAction(selectedResult, 'reject')}
                  className="flex-1 h-12 bg-destructive/10 text-destructive border border-destructive/20 rounded-xl font-bold text-xs uppercase tracking-widest flex items-center justify-center gap-3 hover:bg-destructive hover:text-white transition-all"
                >
                  <ThumbsDown size={16} />
                  REJECT
                </button>
              </div>
            </div>
          ) : (
            <div className="absolute inset-0 flex flex-col items-center justify-center text-center p-12 opacity-30">
              <div className="w-24 h-24 rounded-full border-2 border-dashed border-primary/40 flex items-center justify-center mb-6 animate-pulse">
                <Search size={32} className="text-primary" />
              </div>
              <h3 className="text-lg font-bold uppercase tracking-tighter">Awaiting Selection</h3>
              <p className="text-xs max-w-[200px] mt-2">Select an intelligence directive from the left cluster to begin the audit process.</p>
            </div>
          )}
        </Card>
      </div>

      <Card className="p-6 bg-white/5 border-white/5">
        <h2 className="text-sm font-bold uppercase tracking-widest text-muted-foreground mb-6">Integration Protocols</h2>
        <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
          {[
            { icon: GitMerge, label: 'UNIFIED MESH', desc: 'Seamlessly mesh all agent data streams', color: 'text-blue-400' },
            { icon: TrendingUp, label: 'QUAL-ELITE', desc: 'Prioritize highest index outputs', color: 'text-green-400' },
            { icon: Target, label: 'COMPLEMENT', desc: 'Fill gaps across specialized domains', color: 'text-purple-400' },
            { icon: MessageSquare, label: 'CONSENSUS', desc: 'Majority vote verification', color: 'text-amber-400' }
          ].map((prot, i) => (
            <div key={i} className="p-5 border border-white/5 bg-white/2 rounded-2xl relative overflow-hidden group">
              <div className="absolute top-0 right-0 p-4 opacity-10 -rotate-12 group-hover:rotate-0 transition-transform">
                <prot.icon size={32} />
              </div>
              <div className={`font-bold text-[11px] mb-1.5 tracking-widest uppercase ${prot.color}`}>{prot.label}</div>
              <p className="text-[10px] text-muted-foreground/80 leading-relaxed font-medium">{prot.desc}</p>
            </div>
          ))}
        </div>
      </Card>
    </div>
  )
}
