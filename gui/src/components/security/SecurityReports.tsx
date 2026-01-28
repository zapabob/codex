'use client'

import { useState } from 'react'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/atoms/Button'
import { ScanResult, SecurityAlert, SecurityMetrics } from '@/app/security/page'
import {
  FileText,
  Download,
  Calendar,
  TrendingUp,
  Shield,
  AlertTriangle,
  CheckCircle,
  BarChart3,
  PieChart,
  Clock
} from 'lucide-react'

interface SecurityReportsProps {
  scanResults: ScanResult[]
  alerts: SecurityAlert[]
  metrics: SecurityMetrics
}

export function SecurityReports({ scanResults, alerts, metrics }: SecurityReportsProps) {
  const [selectedTimeRange, setSelectedTimeRange] = useState<'24h' | '7d' | '30d' | '90d'>('7d')
  const [isGeneratingReport, setIsGeneratingReport] = useState(false)

  const generateReport = async () => {
    setIsGeneratingReport(true)

    // Simulate report generation
    await new Promise(resolve => setTimeout(resolve, 3000))

    const reportData = createReportData()
    const reportContent = formatReportAsMarkdown(reportData)

    // Create and download the report
    const blob = new Blob([reportContent], { type: 'text/markdown' })
    const url = URL.createObjectURL(blob)

    const a = document.createElement('a')
    a.href = url
    a.download = `security-report-${new Date().toISOString().split('T')[0]}.md`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)

    setIsGeneratingReport(false)
  }

  const createReportData = () => {
    const now = new Date()
    const timeRangeMs = {
      '24h': 24 * 60 * 60 * 1000,
      '7d': 7 * 24 * 60 * 60 * 1000,
      '30d': 30 * 24 * 60 * 60 * 1000,
      '90d': 90 * 24 * 60 * 60 * 1000,
    }[selectedTimeRange]

    const startDate = new Date(now.getTime() - timeRangeMs)

    // Filter data by time range
    const filteredScans = scanResults.filter(scan => scan.endTime && scan.endTime >= startDate)
    const filteredAlerts = alerts.filter(alert => alert.timestamp >= startDate)

    // Calculate statistics
    const totalScans = filteredScans.length
    const totalFilesScanned = filteredScans.reduce((sum, scan) => sum + scan.filesScanned, 0)
    const totalThreatsFound = filteredScans.reduce((sum, scan) => sum + scan.threatsFound, 0)
    const avgScanTime = filteredScans.length > 0
      ? filteredScans.reduce((sum, scan) => sum + scan.duration, 0) / filteredScans.length
      : 0

    // Alert statistics
    const criticalAlerts = filteredAlerts.filter(a => a.severity === 'critical').length
    const highAlerts = filteredAlerts.filter(a => a.severity === 'high').length
    const mediumAlerts = filteredAlerts.filter(a => a.severity === 'medium').length
    const lowAlerts = filteredAlerts.filter(a => a.severity === 'low').length
    const resolvedAlerts = filteredAlerts.filter(a => a.resolved).length

    return {
      reportPeriod: { start: startDate, end: now },
      summary: {
        totalScans,
        totalFilesScanned,
        totalThreatsFound,
        avgScanTime,
        scanSuccessRate: totalScans > 0 ? (filteredScans.filter(s => s.status === 'completed').length / totalScans) * 100 : 0,
      },
      alerts: {
        total: filteredAlerts.length,
        critical: criticalAlerts,
        high: highAlerts,
        medium: mediumAlerts,
        low: lowAlerts,
        resolved: resolvedAlerts,
        unresolved: filteredAlerts.length - resolvedAlerts,
      },
      systemHealth: {
        overall: metrics.systemHealth,
        lastScan: metrics.lastScan,
        totalScansAllTime: metrics.totalScans,
        threatsDetectedAllTime: metrics.threatsDetected,
      },
      recommendations: generateRecommendations({
        totalThreatsFound,
        criticalAlerts,
        systemHealth: metrics.systemHealth,
        lastScan: metrics.lastScan,
      }),
    }
  }

  const generateRecommendations = (data: {
    totalThreatsFound: number
    criticalAlerts: number
    systemHealth: number
    lastScan: Date | null
  }) => {
    const recommendations = []

    if (data.totalThreatsFound > 10) {
      recommendations.push('Consider running more frequent security scans due to high threat detection rate')
    }

    if (data.criticalAlerts > 0) {
      recommendations.push('URGENT: Address critical security alerts immediately')
    }

    if (data.systemHealth < 70) {
      recommendations.push('System health is below optimal levels - investigate underlying issues')
    }

    const daysSinceLastScan = data.lastScan
      ? (Date.now() - data.lastScan.getTime()) / (1000 * 60 * 60 * 24)
      : Infinity

    if (daysSinceLastScan > 7) {
      recommendations.push('Security scans have not been performed recently - schedule regular scans')
    }

    if (recommendations.length === 0) {
      recommendations.push('System security posture is good - continue regular monitoring and updates')
    }

    return recommendations
  }

  interface SecurityReportData {
    reportPeriod: {
      start: Date;
      end: Date;
    };
    summary: {
      totalScans: number;
      totalFilesScanned: number;
      totalThreatsFound: number;
      avgScanTime: number;
      [key: string]: unknown;
    };
    [key: string]: unknown;
  }

  const formatReportAsMarkdown = (data: SecurityReportData) => {
    return `# Security Assessment Report

**Report Period:** ${data.reportPeriod.start.toLocaleDateString()} - ${data.reportPeriod.end.toLocaleDateString()}
**Generated:** ${new Date().toLocaleString()}

## Executive Summary

This report provides a comprehensive overview of system security status over the selected time period.

### Key Metrics
- **Total Scans Performed:** ${data.summary.totalScans}
- **Files Scanned:** ${data.summary.totalFilesScanned.toLocaleString()}
- **Threats Detected:** ${data.summary.totalThreatsFound}
- **Average Scan Time:** ${Math.floor(data.summary.avgScanTime / 60)}m ${Math.floor(data.summary.avgScanTime % 60)}s
- **Scan Success Rate:** ${data.summary.scanSuccessRate.toFixed(1)}%

## Security Alerts

### Alert Distribution
- **Total Alerts:** ${data.alerts.total}
- **Critical:** ${data.alerts.critical}
- **High:** ${data.alerts.high}
- **Medium:** ${data.alerts.medium}
- **Low:** ${data.alerts.low}
- **Resolved:** ${data.alerts.resolved}
- **Unresolved:** ${data.alerts.unresolved}

## System Health

- **Current Health Score:** ${data.systemHealth.overall}%
- **Last Security Scan:** ${data.systemHealth.lastScan ? data.systemHealth.lastScan.toLocaleString() : 'Never'}
- **Total Scans (All Time):** ${data.systemHealth.totalScansAllTime}
- **Total Threats Detected (All Time):** ${data.systemHealth.threatsDetectedAllTime}

## Recent Scan Results

${scanResults.slice(0, 10).map(scan => `
### ${scan.scanType.toUpperCase()} Scan - ${scan.targetPath}
- **Status:** ${scan.status}
- **Files Scanned:** ${scan.filesScanned.toLocaleString()}
- **Threats Found:** ${scan.threatsFound}
- **Duration:** ${Math.floor(scan.duration / 60)}m ${Math.floor(scan.duration % 60)}s
- **Completed:** ${scan.endTime ? scan.endTime.toLocaleString() : 'In Progress'}
`).join('\n')}

## Recent Security Alerts

${alerts.slice(0, 10).map(alert => `
### ${alert.title}
- **Severity:** ${alert.severity.toUpperCase()}
- **Type:** ${alert.type}
- **Status:** ${alert.resolved ? 'Resolved' : 'Active'}
- **Time:** ${alert.timestamp.toLocaleString()}
- **Description:** ${alert.description}
${alert.affectedFiles.length > 0 ? `- **Affected Files:** ${alert.affectedFiles.join(', ')}` : ''}
`).join('\n')}

## Recommendations

${data.recommendations.map(rec => `- ${rec}`).join('\n')}

---

*Generated by Codex Security Center*
*Report covers the period from ${data.reportPeriod.start.toLocaleString()} to ${data.reportPeriod.end.toLocaleString()}*
`
  }

  const reportData = createReportData()

  return (
    <div className="h-full p-6 space-y-6 overflow-y-auto">
      {/* Report Generation Controls */}
      <Card className="p-6">
        <div className="flex items-center justify-between mb-6">
          <div>
            <h2 className="text-xl font-bold">Security Reports</h2>
            <p className="text-gray-600 mt-1">
              Generate comprehensive security assessment reports
            </p>
          </div>

          <div className="flex items-center gap-4">
            <select
              value={selectedTimeRange}
              onChange={(e) => setSelectedTimeRange(e.target.value as any)}
              className="px-3 py-2 border rounded"
            >
              <option value="24h">Last 24 Hours</option>
              <option value="7d">Last 7 Days</option>
              <option value="30d">Last 30 Days</option>
              <option value="90d">Last 90 Days</option>
            </select>

            <Button
              onClick={generateReport}
              disabled={isGeneratingReport}
              className="px-6"
            >
              {isGeneratingReport ? (
                <>
                  <div className="animate-spin w-4 h-4 border-2 border-white border-t-transparent rounded-full mr-2" />
                  Generating...
                </>
              ) : (
                <>
                  <Download className="w-4 h-4 mr-2" />
                  Generate Report
                </>
              )}
            </Button>
          </div>
        </div>

        {/* Report Preview */}
        <div className="bg-gray-50 rounded-lg p-4">
          <h3 className="font-semibold mb-3">Report Preview</h3>
          <div className="text-sm text-gray-600 space-y-2">
            <div><strong>Period:</strong> {reportData.reportPeriod.start.toLocaleDateString()} - {reportData.reportPeriod.end.toLocaleDateString()}</div>
            <div><strong>Scans:</strong> {reportData.summary.totalScans} ({reportData.summary.scanSuccessRate.toFixed(1)}% success rate)</div>
            <div><strong>Files Scanned:</strong> {reportData.summary.totalFilesScanned.toLocaleString()}</div>
            <div><strong>Threats Detected:</strong> {reportData.summary.totalThreatsFound}</div>
            <div><strong>Active Alerts:</strong> {reportData.alerts.unresolved}</div>
          </div>
        </div>
      </Card>

      {/* Report Statistics */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <Card className="p-4">
          <div className="flex items-center gap-3">
            <BarChart3 className="w-8 h-8 text-blue-500" />
            <div>
              <div className="text-2xl font-bold">{reportData.summary.totalScans}</div>
              <div className="text-sm text-gray-600">Scans Performed</div>
            </div>
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center gap-3">
            <Shield className="w-8 h-8 text-green-500" />
            <div>
              <div className="text-2xl font-bold">{reportData.summary.totalFilesScanned.toLocaleString()}</div>
              <div className="text-sm text-gray-600">Files Scanned</div>
            </div>
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center gap-3">
            <AlertTriangle className="w-8 h-8 text-red-500" />
            <div>
              <div className="text-2xl font-bold">{reportData.alerts.unresolved}</div>
              <div className="text-sm text-gray-600">Active Alerts</div>
            </div>
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center gap-3">
            <CheckCircle className="w-8 h-8 text-purple-500" />
            <div>
              <div className="text-2xl font-bold">{reportData.systemHealth.overall}%</div>
              <div className="text-sm text-gray-600">System Health</div>
            </div>
          </div>
        </Card>
      </div>

      {/* Scan Results Summary */}
      <Card className="p-6">
        <h2 className="text-xl font-bold mb-4">Scan Results Summary</h2>

        {scanResults.length === 0 ? (
          <div className="text-center py-8 text-gray-500">
            <FileText className="w-12 h-12 text-gray-400 mx-auto mb-4" />
            <p>No scan results available</p>
            <p className="text-sm">Run security scans to generate reports</p>
          </div>
        ) : (
          <div className="space-y-4">
            {scanResults.slice(0, 5).map((result) => (
              <div key={result.id} className="border rounded-lg p-4">
                <div className="flex items-center justify-between mb-3">
                  <div className="flex items-center gap-3">
                    <CheckCircle className={`w-5 h-5 ${result.threatsFound > 0 ? 'text-red-500' : 'text-green-500'}`} />
                    <div>
                      <h3 className="font-semibold">{result.scanType.toUpperCase()} Scan</h3>
                      <p className="text-sm text-gray-600">{result.targetPath}</p>
                    </div>
                  </div>

                  <div className="text-right">
                    <Badge variant={result.status === 'completed' ? 'secondary' : 'outline'}>
                      {result.status.toUpperCase()}
                    </Badge>
                    <div className="text-xs text-gray-500 mt-1">
                      {result.endTime ? result.endTime.toLocaleDateString() : 'In Progress'}
                    </div>
                  </div>
                </div>

                <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                  <div>
                    <div className="text-gray-600">Files Scanned</div>
                    <div className="font-semibold">{result.filesScanned.toLocaleString()}</div>
                  </div>
                  <div>
                    <div className="text-gray-600">Threats Found</div>
                    <div className={`font-semibold ${result.threatsFound > 0 ? 'text-red-600' : 'text-green-600'}`}>
                      {result.threatsFound}
                    </div>
                  </div>
                  <div>
                    <div className="text-gray-600">Duration</div>
                    <div className="font-semibold">{Math.floor(result.duration / 60)}m {result.duration % 60}s</div>
                  </div>
                  <div>
                    <div className="text-gray-600">Started</div>
                    <div className="font-semibold">{result.startTime.toLocaleTimeString()}</div>
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </Card>

      {/* Alert Summary */}
      <Card className="p-6">
        <h2 className="text-xl font-bold mb-4">Alert Summary</h2>

        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
          <div className="text-center">
            <div className="text-2xl font-bold text-red-600">{reportData.alerts.critical}</div>
            <div className="text-sm text-gray-600">Critical</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-orange-600">{reportData.alerts.high}</div>
            <div className="text-sm text-gray-600">High</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-yellow-600">{reportData.alerts.medium}</div>
            <div className="text-sm text-gray-600">Medium</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-blue-600">{reportData.alerts.low}</div>
            <div className="text-sm text-gray-600">Low</div>
          </div>
        </div>

        <div className="flex items-center justify-between text-sm">
          <span>Resolution Rate: {reportData.alerts.total > 0 ? ((reportData.alerts.resolved / reportData.alerts.total) * 100).toFixed(1) : 0}%</span>
          <span>Active Alerts: {reportData.alerts.unresolved}</span>
        </div>
      </Card>

      {/* Recommendations */}
      <Card className="p-6">
        <h2 className="text-xl font-bold mb-4">Security Recommendations</h2>

        {reportData.recommendations.length === 0 ? (
          <div className="text-center py-4 text-green-600">
            <CheckCircle className="w-8 h-8 mx-auto mb-2" />
            <p>No specific recommendations - system security is optimal</p>
          </div>
        ) : (
          <div className="space-y-3">
            {reportData.recommendations.map((recommendation, index) => (
              <div key={index} className="flex items-start gap-3 p-3 bg-blue-50 border border-blue-200 rounded">
                <div className="w-6 h-6 bg-blue-500 text-white rounded-full flex items-center justify-center text-xs font-bold mt-0.5">
                  {index + 1}
                </div>
                <p className="text-sm text-blue-800">{recommendation}</p>
              </div>
            ))}
          </div>
        )}

        <div className="mt-6 pt-4 border-t">
          <div className="flex items-center gap-2 text-sm text-gray-600">
            <Clock className="w-4 h-4" />
            <span>Report covers data from the last {selectedTimeRange}</span>
          </div>
        </div>
      </Card>
    </div>
  )
}
