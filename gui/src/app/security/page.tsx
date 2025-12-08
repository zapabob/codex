'use client'

import { useState, useEffect } from 'react'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/atoms/Button'
import { SecurityDashboard } from '@/components/security/SecurityDashboard'
import { MalwareScanner } from '@/components/security/MalwareScanner'
import { QuarantineManager } from '@/components/security/QuarantineManager'
import { SecurityMonitor } from '@/components/security/SecurityMonitor'
import { SecurityReports } from '@/components/security/SecurityReports'
import { DashboardLayout } from '@/components/templates/DashboardLayout'

// Security status types
export type SecurityStatus = 'secure' | 'warning' | 'threat' | 'critical'

// Scan types
export type ScanType = 'quick' | 'deep' | 'custom'

// Security metrics interface
export interface SecurityMetrics {
  lastScan: Date | null
  totalScans: number
  threatsDetected: number
  filesScanned: number
  quarantinedFiles: number
  systemHealth: number // 0-100
  realTimeMonitoring: boolean
}

// Security alert interface
export interface SecurityAlert {
  id: string
  type: 'malware' | 'suspicious' | 'anomaly' | 'system'
  severity: 'low' | 'medium' | 'high' | 'critical'
  title: string
  description: string
  affectedFiles: string[]
  timestamp: Date
  resolved: boolean
}

// Quarantine entry interface
export interface QuarantineEntry {
  id: string
  fileName: string
  originalPath: string
  threatLevel: string
  quarantineDate: Date
  fileSize: number
  canRestore: boolean
}

// Security scan result interface
export interface ScanResult {
  id: string
  scanType: ScanType
  targetPath: string
  startTime: Date
  endTime: Date | null
  status: 'running' | 'completed' | 'failed'
  filesScanned: number
  threatsFound: number
  duration: number // in seconds
}

export default function SecurityPage() {
  const [activeTab, setActiveTab] = useState<'dashboard' | 'scanner' | 'quarantine' | 'monitor' | 'reports'>('dashboard')
  const [securityMetrics, setSecurityMetrics] = useState<SecurityMetrics>({
    lastScan: null,
    totalScans: 0,
    threatsDetected: 0,
    filesScanned: 0,
    quarantinedFiles: 0,
    systemHealth: 85,
    realTimeMonitoring: false,
  })
  const [alerts, setAlerts] = useState<SecurityAlert[]>([])
  const [scanResults, setScanResults] = useState<ScanResult[]>([])

  // Initialize sample data
  useEffect(() => {
    const sampleMetrics: SecurityMetrics = {
      lastScan: new Date(Date.now() - 2 * 60 * 60 * 1000), // 2 hours ago
      totalScans: 15,
      threatsDetected: 3,
      filesScanned: 1250,
      quarantinedFiles: 3,
      systemHealth: 92,
      realTimeMonitoring: true,
    }

    const sampleAlerts: SecurityAlert[] = [
      {
        id: '1',
        type: 'malware',
        severity: 'high',
        title: 'Trojan Malware Detected',
        description: 'Suspicious file with trojan signatures found in downloads folder',
        affectedFiles: ['/Users/Downloads/suspicious.exe'],
        timestamp: new Date(Date.now() - 30 * 60 * 1000),
        resolved: false,
      },
      {
        id: '2',
        type: 'anomaly',
        severity: 'medium',
        title: 'Unusual Network Activity',
        description: 'Detected unusual outbound connections to unknown IP addresses',
        affectedFiles: [],
        timestamp: new Date(Date.now() - 45 * 60 * 1000),
        resolved: true,
      },
      {
        id: '3',
        type: 'system',
        severity: 'low',
        title: 'Security Update Available',
        description: 'New security signatures are available for download',
        affectedFiles: [],
        timestamp: new Date(Date.now() - 2 * 60 * 60 * 1000),
        resolved: false,
      }
    ]

    const sampleScans: ScanResult[] = [
      {
        id: '1',
        scanType: 'quick',
        targetPath: '/Users',
        startTime: new Date(Date.now() - 3 * 60 * 60 * 1000),
        endTime: new Date(Date.now() - 2 * 60 * 60 * 1000),
        status: 'completed',
        filesScanned: 1250,
        threatsFound: 1,
        duration: 1800,
      },
      {
        id: '2',
        scanType: 'deep',
        targetPath: '/System',
        startTime: new Date(Date.now() - 6 * 60 * 60 * 1000),
        endTime: new Date(Date.now() - 5 * 60 * 60 * 1000),
        status: 'completed',
        filesScanned: 5000,
        threatsFound: 2,
        duration: 3600,
      }
    ]

    setSecurityMetrics(sampleMetrics)
    setAlerts(sampleAlerts)
    setScanResults(sampleScans)
  }, [])

  const getOverallStatus = (): SecurityStatus => {
    const unresolvedCritical = alerts.filter(a => !a.resolved && a.severity === 'critical').length
    const unresolvedHigh = alerts.filter(a => !a.resolved && a.severity === 'high').length

    if (unresolvedCritical > 0 || securityMetrics.systemHealth < 50) {
      return 'critical'
    } else if (unresolvedHigh > 0 || securityMetrics.systemHealth < 70) {
      return 'threat'
    } else if (alerts.filter(a => !a.resolved).length > 0 || securityMetrics.systemHealth < 90) {
      return 'warning'
    } else {
      return 'secure'
    }
  }

  const overallStatus = getOverallStatus()

  return (
    <DashboardLayout>
      <div className="h-full flex flex-col">
        {/* Header */}
        <div className="flex items-center justify-between p-6 border-b">
          <div>
            <h1 className="text-3xl font-bold text-gray-900">Security Center</h1>
            <p className="text-gray-600 mt-1">
              Advanced malware detection, isolation, and threat management
            </p>
          </div>

          {/* Overall Status */}
          <div className="flex items-center gap-4">
            <div className="text-right">
              <div className={`text-2xl font-bold ${
                overallStatus === 'secure' ? 'text-green-600' :
                overallStatus === 'warning' ? 'text-yellow-600' :
                overallStatus === 'threat' ? 'text-orange-600' : 'text-red-600'
              }`}>
                {overallStatus === 'secure' ? 'SECURE' :
                 overallStatus === 'warning' ? 'WARNING' :
                 overallStatus === 'threat' ? 'THREAT' : 'CRITICAL'}
              </div>
              <div className="text-sm text-gray-600">System Status</div>
            </div>

            {/* Quick Stats */}
            <div className="flex gap-4 text-sm">
              <div className="text-center">
                <div className="font-bold text-gray-900">{alerts.filter(a => !a.resolved).length}</div>
                <div className="text-gray-600">Active Alerts</div>
              </div>
              <div className="text-center">
                <div className="font-bold text-gray-900">{securityMetrics.quarantinedFiles}</div>
                <div className="text-gray-600">Quarantined</div>
              </div>
              <div className="text-center">
                <div className="font-bold text-gray-900">{securityMetrics.systemHealth}%</div>
                <div className="text-gray-600">Health</div>
              </div>
            </div>
          </div>
        </div>

        {/* Tab Navigation */}
        <div className="flex gap-1 p-4 bg-gray-50 border-b overflow-x-auto">
          {[
            { id: 'dashboard', label: 'Security Dashboard', icon: '📊' },
            { id: 'scanner', label: 'Malware Scanner', icon: '🔍' },
            { id: 'quarantine', label: 'Quarantine', icon: '🛡️' },
            { id: 'monitor', label: 'Real-time Monitor', icon: '📈' },
            { id: 'reports', label: 'Security Reports', icon: '📋' }
          ].map((tab) => (
            <Button
              key={tab.id}
              variant={activeTab === tab.id ? 'primary' : 'ghost'}
              onClick={() => setActiveTab(tab.id as any)}
              className="px-4 py-2 whitespace-nowrap"
            >
              <span className="mr-2">{tab.icon}</span>
              {tab.label}
            </Button>
          ))}
        </div>

        {/* Content */}
        <div className="flex-1 overflow-hidden">
          {activeTab === 'dashboard' && (
            <SecurityDashboard
              metrics={securityMetrics}
              alerts={alerts}
              status={overallStatus}
            />
          )}

          {activeTab === 'scanner' && (
            <MalwareScanner
              onScanComplete={(result) => {
                setScanResults(prev => [result, ...prev.slice(0, 9)])
                setSecurityMetrics(prev => ({
                  ...prev,
                  lastScan: new Date(),
                  totalScans: prev.totalScans + 1,
                  threatsDetected: prev.threatsDetected + result.threatsFound,
                  filesScanned: prev.filesScanned + result.filesScanned,
                }))
              }}
            />
          )}

          {activeTab === 'quarantine' && (
            <QuarantineManager
              onFileRestored={() => {
                setSecurityMetrics(prev => ({
                  ...prev,
                  quarantinedFiles: Math.max(0, prev.quarantinedFiles - 1),
                }))
              }}
              onFileDeleted={() => {
                setSecurityMetrics(prev => ({
                  ...prev,
                  quarantinedFiles: Math.max(0, prev.quarantinedFiles - 1),
                }))
              }}
            />
          )}

          {activeTab === 'monitor' && (
            <SecurityMonitor
              isMonitoring={securityMetrics.realTimeMonitoring}
              onToggleMonitoring={(enabled) => {
                setSecurityMetrics(prev => ({
                  ...prev,
                  realTimeMonitoring: enabled,
                }))
              }}
              onAlertDetected={(alert) => {
                setAlerts(prev => [alert, ...prev])
              }}
            />
          )}

          {activeTab === 'reports' && (
            <SecurityReports
              scanResults={scanResults}
              alerts={alerts}
              metrics={securityMetrics}
            />
          )}
        </div>
      </div>
    </DashboardLayout>
  )
}