'use client'

import { useState } from 'react'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/atoms/Button'
import { Progress } from '@/components/ui/progress'
import { QCProcess, QualityMetric, QCResult } from '@/app/qc/page'
import { Play, Square, FileText, Download, Settings } from 'lucide-react'

interface QCProcessAutomationProps {
  processes: QCProcess[]
  onProcessStart: (processId: string) => void
  onProcessStop: (processId: string) => void
}

export function QCProcessAutomation({ processes, onProcessStart, onProcessStop }: QCProcessAutomationProps) {
  const [selectedProcess, setSelectedProcess] = useState<QCProcess | null>(null)
  const [isGeneratingReport, setIsGeneratingReport] = useState(false)

  // Predefined QC processes
  const predefinedProcesses = [
    {
      id: 'code_quality_check',
      name: 'Code Quality Analysis',
      description: 'Automated code quality assessment using linting and static analysis',
      steps: [
        'Code parsing and AST analysis',
        'Linting rules application',
        'Complexity metrics calculation',
        'Best practices validation'
      ],
      estimatedDuration: 5
    },
    {
      id: 'performance_audit',
      name: 'Performance Audit',
      description: 'Comprehensive performance analysis and bottleneck identification',
      steps: [
        'Benchmark execution',
        'Memory usage analysis',
        'CPU profiling',
        'Optimization recommendations'
      ],
      estimatedDuration: 8
    },
    {
      id: 'security_scan',
      name: 'Security Vulnerability Scan',
      description: 'Automated security vulnerability detection and risk assessment',
      steps: [
        'Dependency vulnerability check',
        'Code security analysis',
        'Configuration review',
        'Risk assessment report'
      ],
      estimatedDuration: 10
    },
    {
      id: 'test_coverage_analysis',
      name: 'Test Coverage Analysis',
      description: 'Comprehensive test coverage evaluation and gap identification',
      steps: [
        'Coverage data collection',
        'Coverage gap analysis',
        'Test quality assessment',
        'Improvement recommendations'
      ],
      estimatedDuration: 6
    }
  ]

  const handleStartProcess = (processId: string) => {
    onProcessStart(processId)
    setSelectedProcess(processes.find(p => p.id === processId) || null)
  }

  const handleStopProcess = (processId: string) => {
    onProcessStop(processId)
    setSelectedProcess(null)
  }

  const handleGenerateReport = async (process: QCProcess) => {
    if (!process.results) return

    setIsGeneratingReport(true)

    // Simulate report generation
    await new Promise(resolve => setTimeout(resolve, 2000))

    const report = generateQCReport(process)
    const blob = new Blob([report], { type: 'text/markdown' })
    const url = URL.createObjectURL(blob)

    const a = document.createElement('a')
    a.href = url
    a.download = `qc-report-${process.id}-${new Date().toISOString().split('T')[0]}.md`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)

    setIsGeneratingReport(false)
  }

  const generateQCReport = (process: QCProcess): string => {
    const result = process.results!

    return `# Quality Control Report

**Process:** ${process.name}
**Date:** ${new Date().toISOString().split('T')[0]}
**Status:** ${result.passed ? 'PASSED' : 'FAILED'}
**Overall Score:** ${result.overallScore.toFixed(1)}%

## Executive Summary

${result.passed ? '✅ All quality criteria have been met.' : '❌ Quality issues detected that require attention.'}

## ANOVA Analysis

${result.anovaResult ? `
- **F-Statistic:** ${result.anovaResult.fStatistic.toFixed(2)}
- **P-Value:** ${result.anovaResult.pValue.toFixed(4)}
- **Significance:** ${result.anovaResult.significance ? 'Significant differences detected' : 'No significant differences'}
- **Degrees of Freedom:** ${result.anovaResult.degreesOfFreedom}

### Group Statistics
${result.anovaResult.groups.map(group => `- **${group.name}:** Mean: ${group.mean.toFixed(2)}, Variance: ${group.variance.toFixed(2)}, Count: ${group.count}`).join('\n')}
` : 'ANOVA analysis not performed.'}

## Quality Metrics

${process.metrics.map(metric => `
### ${metric.name}
- **Value:** ${metric.value.toFixed(1)}${metric.unit}
- **Target:** ${metric.target}${metric.unit}
- **Status:** ${metric.status.toUpperCase()}
- **Trend:** ${metric.trend === 'up' ? '↗️ Improving' : metric.trend === 'down' ? '↘️ Declining' : '➡️ Stable'}
`).join('\n')}

## Recommendations

${result.recommendations.map(rec => `- ${rec}`).join('\n')}

## Next Steps

${result.passed
  ? '- Continue monitoring quality metrics\n- Schedule next automated QC check\n- Review improvement opportunities'
  : '- Address critical quality issues immediately\n- Implement recommended improvements\n- Schedule follow-up QC assessment'
}

---
*Generated by Codex QC Automation System*
`
  }

  const getStatusColor = (status: QCProcess['status']) => {
    switch (status) {
      case 'idle': return 'bg-gray-100 text-gray-800'
      case 'running': return 'bg-blue-100 text-blue-800'
      case 'completed': return 'bg-green-100 text-green-800'
      case 'failed': return 'bg-red-100 text-red-800'
      default: return 'bg-gray-100 text-gray-800'
    }
  }

  const getStatusIcon = (status: QCProcess['status']) => {
    switch (status) {
      case 'idle': return '⏸️'
      case 'running': return '▶️'
      case 'completed': return '✅'
      case 'failed': return '❌'
      default: return '⏸️'
    }
  }

  return (
    <div className="h-full p-6 space-y-6 overflow-y-auto">
      {/* Process Controls */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        {predefinedProcesses.map((process) => (
          <Card key={process.id} className="p-4">
            <div className="flex items-center justify-between mb-3">
              <h3 className="font-semibold text-sm">{process.name}</h3>
              <Badge variant="outline" className="text-xs">
                ~{process.estimatedDuration}min
              </Badge>
            </div>

            <p className="text-xs text-gray-600 mb-3 line-clamp-2">
              {process.description}
            </p>

            <div className="space-y-1 mb-3">
              <div className="text-xs font-medium text-gray-700">Steps:</div>
              {process.steps.slice(0, 2).map((step, index) => (
                <div key={index} className="text-xs text-gray-600">
                  • {step}
                </div>
              ))}
              {process.steps.length > 2 && (
                <div className="text-xs text-gray-600">
                  • +{process.steps.length - 2} more steps
                </div>
              )}
            </div>

            <Button
              variant="outline"
              size="sm"
              className="w-full"
              onClick={() => handleStartProcess(process.id)}
              disabled={processes.some(p => p.id === process.id && p.status === 'running')}
            >
              <Play className="w-4 h-4 mr-1" />
              Start Process
            </Button>
          </Card>
        ))}
      </div>

      {/* Active Processes */}
      <Card className="p-6">
        <h2 className="text-xl font-bold mb-4">Active QC Processes</h2>

        {processes.length === 0 ? (
          <div className="text-center py-8 text-gray-500">
            No QC processes running. Start a process from above.
          </div>
        ) : (
          <div className="space-y-4">
            {processes.map((process) => (
              <div key={process.id} className="border rounded-lg p-4">
                <div className="flex items-center justify-between mb-3">
                  <div className="flex items-center gap-3">
                    <span className="text-lg">{getStatusIcon(process.status)}</span>
                    <div>
                      <h3 className="font-semibold">{process.name}</h3>
                      <p className="text-sm text-gray-600">{process.description}</p>
                    </div>
                  </div>

                  <div className="flex items-center gap-2">
                    <Badge className={getStatusColor(process.status)}>
                      {process.status.toUpperCase()}
                    </Badge>

                    {process.status === 'running' ? (
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => handleStopProcess(process.id)}
                      >
                        <Square className="w-4 h-4" />
                      </Button>
                    ) : process.status === 'completed' && process.results ? (
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => handleGenerateReport(process)}
                        disabled={isGeneratingReport}
                      >
                        {isGeneratingReport ? (
                          <div className="animate-spin w-4 h-4 border-2 border-blue-500 border-t-transparent rounded-full" />
                        ) : (
                          <Download className="w-4 h-4" />
                        )}
                      </Button>
                    ) : null}
                  </div>
                </div>

                {/* Progress */}
                <div className="mb-3">
                  <div className="flex justify-between text-sm mb-1">
                    <span>Progress</span>
                    <span>{process.progress}%</span>
                  </div>
                  <Progress value={process.progress} className="h-2" />
                </div>

                {/* Timing */}
                {process.startTime && (
                  <div className="flex justify-between text-xs text-gray-500 mb-3">
                    <span>Started: {process.startTime.toLocaleTimeString()}</span>
                    {process.endTime && (
                      <span>Ended: {process.endTime.toLocaleTimeString()}</span>
                    )}
                  </div>
                )}

                {/* Results Summary */}
                {process.results && (
                  <div className="bg-gray-50 rounded p-3">
                    <div className="flex justify-between items-center mb-2">
                      <span className="font-medium">QC Results</span>
                      <Badge variant={process.results.passed ? 'secondary' : 'destructive'}>
                        {process.results.passed ? 'PASSED' : 'FAILED'}
                      </Badge>
                    </div>
                    <div className="text-sm text-gray-600">
                      Overall Score: <span className="font-semibold">{process.results.overallScore.toFixed(1)}%</span>
                    </div>
                    {process.results.recommendations.length > 0 && (
                      <div className="mt-2">
                        <div className="text-xs font-medium text-gray-700 mb-1">Key Recommendations:</div>
                        <ul className="text-xs text-gray-600 space-y-1">
                          {process.results.recommendations.slice(0, 2).map((rec, index) => (
                            <li key={index}>• {rec}</li>
                          ))}
                        </ul>
                      </div>
                    )}
                  </div>
                )}
              </div>
            ))}
          </div>
        )}
      </Card>

      {/* Process Configuration */}
      <Card className="p-6">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-xl font-bold">QC Configuration</h2>
          <Button variant="outline" size="sm">
            <Settings className="w-4 h-4 mr-1" />
            Advanced Settings
          </Button>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div className="space-y-2">
            <label className="text-sm font-medium">Auto-run Schedule</label>
            <select className="w-full p-2 border rounded text-sm">
              <option>Manual only</option>
              <option>Daily at 9:00 AM</option>
              <option>Weekly on Monday</option>
              <option>Monthly on 1st</option>
            </select>
          </div>

          <div className="space-y-2">
            <label className="text-sm font-medium">Failure Threshold</label>
            <select className="w-full p-2 border rounded text-sm">
              <option>Any failure stops process</option>
              <option>Continue on warnings</option>
              <option>Ignore minor issues</option>
            </select>
          </div>

          <div className="space-y-2">
            <label className="text-sm font-medium">Report Format</label>
            <select className="w-full p-2 border rounded text-sm">
              <option>Markdown</option>
              <option>PDF</option>
              <option>JSON</option>
              <option>HTML</option>
            </select>
          </div>
        </div>
      </Card>
    </div>
  )
}
