import { useState } from 'react'
import { Card } from '../atoms/Card'
import { Badge } from '../atoms/Badge'
import { Button } from '../atoms/Button'
import { Progress } from '../atoms/Progress'
import type { QCProcess } from '../../types/qc'
import { Play, Square, Download, Settings } from 'lucide-react'

interface QCProcessAutomationProps {
  processes: QCProcess[]
  onProcessStart: (processId: string) => void
  onProcessStop: (processId: string) => void
}

export function QCProcessAutomation({ processes, onProcessStart, onProcessStop }: QCProcessAutomationProps) {
  // const [isGeneratingReport, setIsGeneratingReport] = useState(false)

  const predefinedProcesses = [
    {
      id: 'code_quality_check',
      name: 'Code Quality Analysis',
      description: 'Automated code quality assessment using linting and static analysis',
      steps: ['AST analysis', 'Linting rules', 'Complexity metrics', 'Best practices'],
      estimatedDuration: 5
    },
    {
      id: 'performance_audit',
      name: 'Performance Audit',
      description: 'Comprehensive performance analysis and bottleneck identification',
      steps: ['Benchmark', 'Memory analysis', 'CPU profiling', 'Optimization'],
      estimatedDuration: 8
    },
    {
      id: 'security_scan',
      name: 'Security Vulnerability Scan',
      description: 'Automated security vulnerability detection and risk assessment',
      steps: ['Dependency check', 'Security analysis', 'Config review', 'Risk assessment'],
      estimatedDuration: 10
    }
  ]

  const getStatusColor = (status: QCProcess['status']) => {
    switch (status) {
      case 'idle': return 'secondary'
      case 'running': return 'primary'
      case 'completed': return 'success'
      case 'failed': return 'error'
      default: return 'secondary'
    }
  }

  return (
    <div className="space-y-6">
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        {predefinedProcesses.map((process) => (
          <Card key={process.id} animated hover>
            <div className="p-4">
                <div className="flex items-center justify-between mb-3">
                <h3 className="font-bold text-sm">{process.name}</h3>
                <Badge variant="secondary" size="sm">~{process.estimatedDuration}m</Badge>
                </div>
                <p className="text-xs text-muted-foreground mb-4 line-clamp-2">{process.description}</p>
                <div className="flex flex-wrap gap-1 mb-4">
                    {process.steps.map(step => (
                        <span key={step} className="text-[10px] bg-muted/50 px-1.5 py-0.5 rounded text-muted-foreground">
                            {step}
                        </span>
                    ))}
                </div>
                <Button
                variant="outlined"
                size="small"
                fullWidth
                onClick={() => onProcessStart(process.id)}
                disabled={processes.some(p => p.id === process.id && p.status === 'running')}
                >
                <Play className="w-4 h-4 mr-2" /> Start Process
                </Button>
            </div>
          </Card>
        ))}
      </div>

      <Card animated>
        <div className="p-6">
            <h2 className="text-xl font-bold mb-6">Active QC Processes</h2>
            {processes.length === 0 ? (
            <div className="text-center py-12 text-muted-foreground border border-dashed border-border rounded-2xl">
                No QC processes running. Start a process from above.
            </div>
            ) : (
            <div className="space-y-4">
                {processes.map((process) => (
                <div key={process.id} className="border border-border rounded-2xl p-4 bg-card/50 backdrop-blur-sm">
                    <div className="flex items-center justify-between mb-4">
                    <div className="flex items-center gap-3">
                        <div className={`h-10 w-10 rounded-xl flex items-center justify-center transition-colors ${process.status === 'running' ? 'bg-primary/20 text-primary animate-pulse' : 'bg-muted text-muted-foreground'}`}>
                             {process.status === 'running' ? <Play size={20} /> : <Square size={20} />}
                        </div>
                        <div>
                            <h3 className="font-bold">{process.name}</h3>
                            <p className="text-xs text-muted-foreground">{process.description}</p>
                        </div>
                    </div>

                    <div className="flex items-center gap-3">
                        <Badge color={getStatusColor(process.status)}>
                        {process.status.toUpperCase()}
                        </Badge>
                        {process.status === 'running' ? (
                        <Button variant="outlined" size="small" onClick={() => onProcessStop(process.id)}>
                            <Square className="w-4 h-4" />
                        </Button>
                        ) : process.status === 'completed' && (
                        <Button variant="outlined" size="small" onClick={() => {}} disabled={isGeneratingReport}>
                            <Download className="w-4 h-4" />
                        </Button>
                        )}
                    </div>
                    </div>

                    <Progress value={process.progress} showValue label="Processing status..." />
                    
                    {process.results && (
                    <div className="mt-4 p-4 rounded-xl bg-muted/30 border border-border">
                        <div className="flex justify-between items-center mb-2">
                        <span className="text-sm font-bold uppercase tracking-wider">Results Overview</span>
                        <Badge color={process.results.passed ? 'success' : 'error'} size="md">
                            {process.results.passed ? 'PASSED' : 'FAILED'}
                        </Badge>
                        </div>
                        <div className="text-xl font-bold font-mono text-primary">{process.results.overallScore.toFixed(1)}%</div>
                        <div className="mt-2 space-y-1">
                            {process.results.recommendations.slice(0, 2).map((rec, i) => (
                                <div key={i} className="text-xs text-muted-foreground flex gap-2">
                                    <span className="text-primary">•</span> {rec}
                                </div>
                            ))}
                        </div>
                    </div>
                    )}
                </div>
                ))}
            </div>
            )}
        </div>
      </Card>

      <Card animated sx={{ p: 6 }}>
        <div className="flex items-center justify-between mb-6">
          <h2 className="text-xl font-bold text-primary">QC Configuration</h2>
          <Button variant="outlined" size="small">
            <Settings className="w-4 h-4 mr-2" /> Advanced
          </Button>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div className="space-y-2">
                <label className="text-[10px] font-bold text-muted-foreground uppercase tracking-widest pl-1">Schedule</label>
                <div className="p-3 bg-muted/30 border border-border rounded-xl text-sm font-medium">Daily at 09:00 AM</div>
            </div>
            <div className="space-y-2">
                <label className="text-[10px] font-bold text-muted-foreground uppercase tracking-widest pl-1">Threshold</label>
                <div className="p-3 bg-muted/30 border border-border rounded-xl text-sm font-medium">Stop on Errors Only</div>
            </div>
            <div className="space-y-2">
                <label className="text-[10px] font-bold text-muted-foreground uppercase tracking-widest pl-1">Output</label>
                <div className="p-3 bg-muted/30 border border-border rounded-xl text-sm font-medium">Bilingual Markdown (JP/EN)</div>
            </div>
        </div>
      </Card>
    </div>
  )
}
