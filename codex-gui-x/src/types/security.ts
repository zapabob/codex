export type SecurityStatus = 'secure' | 'warning' | 'threat' | 'critical'

export type ScanType = 'quick' | 'deep' | 'custom'

export interface SecurityMetrics {
  lastScan: Date | null
  totalScans: number
  threatsDetected: number
  filesScanned: number
  quarantinedFiles: number
  systemHealth: number // 0-100
  realTimeMonitoring: boolean
}

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

export interface QuarantineEntry {
  id: string
  fileName: string
  originalPath: string
  threatLevel: string
  quarantineDate: Date
  fileSize: number
  canRestore: boolean
}

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
