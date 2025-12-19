'use client'

import { useState, useEffect } from 'react'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/atoms/Button'
import { StatisticalDashboard } from '@/components/qc/StatisticalDashboard'
import { QCProcessAutomation } from '@/components/qc/QCProcessAutomation'
import { RealTimeMonitoring } from '@/components/qc/RealTimeMonitoring'
import { AlertSystem } from '@/components/qc/AlertSystem'
import { DashboardLayout } from '@/components/templates/DashboardLayout'

// Quality metrics interface
export interface QualityMetric {
  id: string
  name: string
  value: number
  unit: string
  target: number
  tolerance: number
  status: 'good' | 'warning' | 'critical'
  trend: 'up' | 'down' | 'stable'
  timestamp: Date
  category: string
}

// QC process interface
export interface QCProcess {
  id: string
  name: string
  description: string
  status: 'idle' | 'running' | 'completed' | 'failed'
  progress: number
  startTime?: Date
  endTime?: Date
  metrics: QualityMetric[]
  results?: QCResult
}

// QC result interface
export interface QCResult {
  id: string
  processId: string
  anovaResult?: AnovaResult
  overallScore: number
  passed: boolean
  recommendations: string[]
  timestamp: Date
}

// ANOVA result interface
export interface AnovaResult {
  fStatistic: number
  pValue: number
  degreesOfFreedom: number
  significance: boolean
  groups: Array<{
    name: string
    mean: number
    variance: number
    count: number
  }>
}

// Alert interface
export interface QCAlert {
  id: string
  type: 'warning' | 'critical' | 'info'
  title: string
  message: string
  metricId?: string
  threshold: number
  currentValue: number
  timestamp: Date
  acknowledged: boolean
}

export default function QCPage() {
  const [activeTab, setActiveTab] = useState<'dashboard' | 'automation' | 'monitoring' | 'alerts'>('dashboard')
  const [qualityMetrics, setQualityMetrics] = useState<QualityMetric[]>([])
  const [qcProcesses, setQcProcesses] = useState<QCProcess[]>([])
  const [alerts, setAlerts] = useState<QCAlert[]>([])

  // Initialize sample data
  useEffect(() => {
    const sampleMetrics: QualityMetric[] = [
      {
        id: 'code_quality',
        name: 'Code Quality Score',
        value: 85.3,
        unit: '%',
        target: 90,
        tolerance: 5,
        status: 'warning',
        trend: 'up',
        timestamp: new Date(),
        category: 'Development'
      },
      {
        id: 'test_coverage',
        name: 'Test Coverage',
        value: 92.1,
        unit: '%',
        target: 95,
        tolerance: 3,
        status: 'good',
        trend: 'stable',
        timestamp: new Date(),
        category: 'Testing'
      },
      {
        id: 'performance',
        name: 'Performance Score',
        value: 78.5,
        unit: '%',
        target: 85,
        tolerance: 7,
        status: 'critical',
        trend: 'down',
        timestamp: new Date(),
        category: 'Performance'
      },
      {
        id: 'security',
        name: 'Security Rating',
        value: 94.2,
        unit: '%',
        target: 95,
        tolerance: 2,
        status: 'good',
        trend: 'up',
        timestamp: new Date(),
        category: 'Security'
      }
    ]

    const sampleProcesses: QCProcess[] = [
      {
        id: 'daily_qc',
        name: 'Daily Quality Check',
        description: 'Automated daily quality assessment',
        status: 'completed',
        progress: 100,
        startTime: new Date(Date.now() - 2 * 60 * 60 * 1000),
        endTime: new Date(Date.now() - 1 * 60 * 60 * 1000),
        metrics: sampleMetrics,
        results: {
          id: 'result_1',
          processId: 'daily_qc',
          overallScore: 87.5,
          passed: true,
          recommendations: [
            'Improve performance score by 6.5%',
            'Code quality is within acceptable range'
          ],
          timestamp: new Date(Date.now() - 1 * 60 * 60 * 1000)
        }
      },
      {
        id: 'weekly_analysis',
        name: 'Weekly ANOVA Analysis',
        description: 'Statistical analysis of quality trends',
        status: 'running',
        progress: 65,
        startTime: new Date(Date.now() - 30 * 60 * 1000),
        metrics: sampleMetrics,
      }
    ]

    const sampleAlerts: QCAlert[] = [
      {
        id: 'alert_1',
        type: 'critical',
        title: 'Performance Degradation',
        message: 'Performance score dropped below critical threshold',
        metricId: 'performance',
        threshold: 85,
        currentValue: 78.5,
        timestamp: new Date(Date.now() - 15 * 60 * 1000),
        acknowledged: false
      },
      {
        id: 'alert_2',
        type: 'warning',
        title: 'Code Quality Warning',
        message: 'Code quality score is below target',
        metricId: 'code_quality',
        threshold: 90,
        currentValue: 85.3,
        timestamp: new Date(Date.now() - 45 * 60 * 1000),
        acknowledged: true
      }
    ]

    setQualityMetrics(sampleMetrics)
    setQcProcesses(sampleProcesses)
    setAlerts(sampleAlerts)
  }, [])

  // Calculate overall quality score
  const overallQualityScore = qualityMetrics.reduce((sum, metric) => {
    const weight = metric.category === 'Security' ? 2 : 1
    return sum + (metric.value * weight)
  }, 0) / qualityMetrics.reduce((sum, metric) => {
    return sum + (metric.category === 'Security' ? 2 : 1)
  }, 0)

  const criticalAlerts = alerts.filter(alert => alert.type === 'critical' && !alert.acknowledged)

  return (
    <DashboardLayout>
      <div className="h-full flex flex-col">
        {/* Header */}
        <div className="flex items-center justify-between p-6 border-b">
          <div>
            <h1 className="text-3xl font-bold text-gray-900">QC管理</h1>
            <p className="text-gray-600 mt-1">
              ANOVA-based statistical analysis and automated quality management
            </p>
          </div>

          {/* Quality Score */}
          <div className="flex items-center gap-4">
            <div className="text-right">
              <div className="text-2xl font-bold text-blue-600">
                {overallQualityScore.toFixed(1)}%
              </div>
              <div className="text-sm text-gray-600">Overall Quality</div>
            </div>

            {/* Critical Alerts Badge */}
            {criticalAlerts.length > 0 && (
              <Badge variant="destructive" className="px-3 py-1">
                {criticalAlerts.length} Critical
              </Badge>
            )}
          </div>
        </div>

        {/* Tab Navigation */}
        <div className="flex gap-1 p-4 bg-gray-50 border-b">
          {[
            { id: 'dashboard', label: 'Statistical Dashboard', icon: '📊' },
            { id: 'automation', label: 'QC Automation', icon: '⚙️' },
            { id: 'monitoring', label: 'Real-time Monitoring', icon: '📈' },
            { id: 'alerts', label: 'Alert System', icon: '🚨' }
          ].map((tab) => (
            <Button
              key={tab.id}
              variant={activeTab === tab.id ? 'primary' : 'ghost'}
              onClick={() => setActiveTab(tab.id as any)}
              className="px-4 py-2"
            >
              <span className="mr-2">{tab.icon}</span>
              {tab.label}
            </Button>
          ))}
        </div>

        {/* Content */}
        <div className="flex-1 overflow-hidden">
          {activeTab === 'dashboard' && (
            <StatisticalDashboard
              metrics={qualityMetrics}
              processes={qcProcesses}
            />
          )}

          {activeTab === 'automation' && (
            <QCProcessAutomation
              processes={qcProcesses}
              onProcessStart={(processId) => {
                setQcProcesses(processes =>
                  processes.map(p =>
                    p.id === processId
                      ? { ...p, status: 'running' as const, startTime: new Date() }
                      : p
                  )
                )
              }}
              onProcessStop={(processId) => {
                setQcProcesses(processes =>
                  processes.map(p =>
                    p.id === processId
                      ? { ...p, status: 'completed' as const, endTime: new Date() }
                      : p
                  )
                )
              }}
            />
          )}

          {activeTab === 'monitoring' && (
            <RealTimeMonitoring
              metrics={qualityMetrics}
              processes={qcProcesses}
            />
          )}

          {activeTab === 'alerts' && (
            <AlertSystem
              alerts={alerts}
              onAcknowledge={(alertId) => {
                setAlerts(alerts =>
                  alerts.map(alert =>
                    alert.id === alertId
                      ? { ...alert, acknowledged: true }
                      : alert
                  )
                )
              }}
            />
          )}
        </div>
      </div>
    </DashboardLayout>
  )
}
