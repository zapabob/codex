'use client'

import { useState, useEffect } from 'react'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/atoms/Button'
import { QuarantineEntry } from '@/app/security/page'
import {
  Shield,
  AlertTriangle,
  CheckCircle,
  XCircle,
  RotateCcw,
  Trash2,
  Eye,
  Download,
  FileText,
  Calendar,
  HardDrive
} from 'lucide-react'

interface QuarantineManagerProps {
  onFileRestored: () => void
  onFileDeleted: () => void
}

export function QuarantineManager({ onFileRestored, onFileDeleted }: QuarantineManagerProps) {
  const [quarantinedFiles, setQuarantinedFiles] = useState<QuarantineEntry[]>([])
  const [selectedFiles, setSelectedFiles] = useState<Set<string>>(new Set())
  const [filter, setFilter] = useState<'all' | 'malicious' | 'suspicious' | 'critical'>('all')
  const [sortBy, setSortBy] = useState<'date' | 'size' | 'threat'>('date')

  // Initialize sample quarantined files
  useEffect(() => {
    const sampleFiles: QuarantineEntry[] = [
      {
        id: 'q1',
        fileName: 'suspicious_trojan.exe',
        originalPath: 'C:\\Users\\Downloads\\suspicious_trojan.exe',
        threatLevel: 'malicious',
        quarantineDate: new Date(Date.now() - 2 * 60 * 60 * 1000),
        fileSize: 245760, // 240KB
        canRestore: true,
      },
      {
        id: 'q2',
        fileName: 'keylogger.dll',
        originalPath: 'C:\\Windows\\System32\\keylogger.dll',
        threatLevel: 'critical',
        quarantineDate: new Date(Date.now() - 5 * 60 * 60 * 1000),
        fileSize: 184320, // 180KB
        canRestore: false, // System file - dangerous to restore
      },
      {
        id: 'q3',
        fileName: 'suspicious_script.js',
        originalPath: 'C:\\Users\\Documents\\suspicious_script.js',
        threatLevel: 'suspicious',
        quarantineDate: new Date(Date.now() - 1 * 60 * 60 * 1000),
        fileSize: 1536, // 1.5KB
        canRestore: true,
      },
      {
        id: 'q4',
        fileName: 'ransomware_encryptor.exe',
        originalPath: 'C:\\ProgramData\\ransomware_encryptor.exe',
        threatLevel: 'critical',
        quarantineDate: new Date(Date.now() - 12 * 60 * 60 * 1000),
        fileSize: 524288, // 512KB
        canRestore: false,
      },
      {
        id: 'q5',
        fileName: 'adware_installer.msi',
        originalPath: 'C:\\Users\\Downloads\\adware_installer.msi',
        threatLevel: 'malicious',
        quarantineDate: new Date(Date.now() - 30 * 60 * 1000),
        fileSize: 2097152, // 2MB
        canRestore: true,
      }
    ]

    setQuarantinedFiles(sampleFiles)
  }, [])

  // Filter and sort files
  const filteredFiles = quarantinedFiles
    .filter(file => {
      if (filter === 'all') return true
      return file.threatLevel === filter
    })
    .sort((a, b) => {
      switch (sortBy) {
        case 'date':
          return b.quarantineDate.getTime() - a.quarantineDate.getTime()
        case 'size':
          return b.fileSize - a.fileSize
        case 'threat':
          const threatOrder = { critical: 3, malicious: 2, suspicious: 1 }
          return threatOrder[b.threatLevel as keyof typeof threatOrder] - threatOrder[a.threatLevel as keyof typeof threatOrder]
        default:
          return 0
      }
    })

  const handleSelectFile = (fileId: string) => {
    const newSelected = new Set(selectedFiles)
    if (newSelected.has(fileId)) {
      newSelected.delete(fileId)
    } else {
      newSelected.add(fileId)
    }
    setSelectedFiles(newSelected)
  }

  const handleSelectAll = () => {
    if (selectedFiles.size === filteredFiles.length) {
      setSelectedFiles(new Set())
    } else {
      setSelectedFiles(new Set(filteredFiles.map(f => f.id)))
    }
  }

  const handleRestoreSelected = async () => {
    for (const fileId of selectedFiles) {
      const file = quarantinedFiles.find(f => f.id === fileId)
      if (file && file.canRestore) {
        // Simulate restore operation
        await new Promise(resolve => setTimeout(resolve, 1000))
        setQuarantinedFiles(prev => prev.filter(f => f.id !== fileId))
        onFileRestored()
      }
    }
    setSelectedFiles(new Set())
  }

  const handleDeleteSelected = async () => {
    if (!confirm(`Are you sure you want to permanently delete ${selectedFiles.size} quarantined files? This action cannot be undone.`)) {
      return
    }

    for (const fileId of selectedFiles) {
      // Simulate delete operation
      await new Promise(resolve => setTimeout(resolve, 500))
      setQuarantinedFiles(prev => prev.filter(f => f.id !== fileId))
      onFileDeleted()
    }
    setSelectedFiles(new Set())
  }

  const handleViewDetails = (file: QuarantineEntry) => {
    // In a real implementation, this would open a detailed view
    alert(`File Details:\nName: ${file.fileName}\nPath: ${file.originalPath}\nThreat: ${file.threatLevel}\nSize: ${formatFileSize(file.fileSize)}\nDate: ${file.quarantineDate.toLocaleString()}`)
  }

  const getThreatLevelColor = (level: string) => {
    switch (level) {
      case 'critical': return 'bg-red-100 text-red-800 border-red-200'
      case 'malicious': return 'bg-orange-100 text-orange-800 border-orange-200'
      case 'suspicious': return 'bg-yellow-100 text-yellow-800 border-yellow-200'
      default: return 'bg-gray-100 text-gray-800 border-gray-200'
    }
  }

  const getThreatIcon = (level: string) => {
    switch (level) {
      case 'critical': return <XCircle className="w-4 h-4 text-red-500" />
      case 'malicious': return <AlertTriangle className="w-4 h-4 text-orange-500" />
      case 'suspicious': return <Shield className="w-4 h-4 text-yellow-500" />
      default: return <FileText className="w-4 h-4 text-gray-500" />
    }
  }

  const formatFileSize = (bytes: number): string => {
    const units = ['B', 'KB', 'MB', 'GB']
    let size = bytes
    let unitIndex = 0

    while (size >= 1024 && unitIndex < units.length - 1) {
      size /= 1024
      unitIndex++
    }

    return `${size.toFixed(1)} ${units[unitIndex]}`
  }

  const stats = {
    total: quarantinedFiles.length,
    critical: quarantinedFiles.filter(f => f.threatLevel === 'critical').length,
    malicious: quarantinedFiles.filter(f => f.threatLevel === 'malicious').length,
    suspicious: quarantinedFiles.filter(f => f.threatLevel === 'suspicious').length,
    restorable: quarantinedFiles.filter(f => f.canRestore).length,
    totalSize: quarantinedFiles.reduce((sum, f) => sum + f.fileSize, 0),
  }

  return (
    <div className="h-full p-6 space-y-6 overflow-y-auto">
      {/* Quarantine Statistics */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        <Card className="p-4">
          <div className="flex items-center gap-3">
            <Shield className="w-8 h-8 text-red-500" />
            <div>
              <div className="text-2xl font-bold">{stats.total}</div>
              <div className="text-sm text-gray-600">Total Quarantined</div>
            </div>
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center gap-3">
            <XCircle className="w-8 h-8 text-red-500" />
            <div>
              <div className="text-2xl font-bold">{stats.critical}</div>
              <div className="text-sm text-gray-600">Critical Threats</div>
            </div>
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center gap-3">
            <HardDrive className="w-8 h-8 text-blue-500" />
            <div>
              <div className="text-2xl font-bold">{formatFileSize(stats.totalSize)}</div>
              <div className="text-sm text-gray-600">Space Used</div>
            </div>
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center gap-3">
            <RotateCcw className="w-8 h-8 text-green-500" />
            <div>
              <div className="text-2xl font-bold">{stats.restorable}</div>
              <div className="text-sm text-gray-600">Restorable</div>
            </div>
          </div>
        </Card>
      </div>

      {/* Controls */}
      <Card className="p-6">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-xl font-bold">Quarantined Files</h2>

          <div className="flex items-center gap-2">
            <select
              value={filter}
              onChange={(e) => setFilter(e.target.value as any)}
              className="px-3 py-1 border rounded text-sm"
            >
              <option value="all">All Threats</option>
              <option value="critical">Critical Only</option>
              <option value="malicious">Malicious Only</option>
              <option value="suspicious">Suspicious Only</option>
            </select>

            <select
              value={sortBy}
              onChange={(e) => setSortBy(e.target.value as any)}
              className="px-3 py-1 border rounded text-sm"
            >
              <option value="date">Sort by Date</option>
              <option value="size">Sort by Size</option>
              <option value="threat">Sort by Threat</option>
            </select>
          </div>
        </div>

        {/* Bulk Actions */}
        {selectedFiles.size > 0 && (
          <div className="flex items-center gap-2 mb-4 p-3 bg-blue-50 border border-blue-200 rounded">
            <span className="text-sm font-medium">{selectedFiles.size} files selected</span>
            <Button
              size="sm"
              onClick={handleRestoreSelected}
              disabled={!Array.from(selectedFiles).some(id => quarantinedFiles.find(f => f.id === id)?.canRestore)}
            >
              <RotateCcw className="w-4 h-4 mr-1" />
              Restore Selected
            </Button>
            <Button
              variant="destructive"
              size="sm"
              onClick={handleDeleteSelected}
            >
              <Trash2 className="w-4 h-4 mr-1" />
              Delete Selected
            </Button>
          </div>
        )}

        {/* File List */}
        <div className="space-y-2 max-h-96 overflow-y-auto">
          {/* Header */}
          <div className="flex items-center gap-2 p-3 bg-gray-50 rounded font-medium text-sm">
            <input
              type="checkbox"
              checked={selectedFiles.size === filteredFiles.length && filteredFiles.length > 0}
              onChange={handleSelectAll}
              className="rounded"
            />
            <div className="flex-1">File Name</div>
            <div className="w-24 text-center">Threat Level</div>
            <div className="w-20 text-center">Size</div>
            <div className="w-32 text-center">Date</div>
            <div className="w-24 text-center">Actions</div>
          </div>

          {filteredFiles.map((file) => (
            <div
              key={file.id}
              className={`flex items-center gap-2 p-3 border rounded hover:bg-gray-50 ${
                selectedFiles.has(file.id) ? 'bg-blue-50 border-blue-300' : ''
              }`}
            >
              <input
                type="checkbox"
                checked={selectedFiles.has(file.id)}
                onChange={() => handleSelectFile(file.id)}
                className="rounded"
              />

              <div className="flex items-center gap-3 flex-1">
                {getThreatIcon(file.threatLevel)}
                <div>
                  <div className="font-medium text-sm">{file.fileName}</div>
                  <div className="text-xs text-gray-500 truncate max-w-xs">{file.originalPath}</div>
                </div>
              </div>

              <div className="w-24 text-center">
                <Badge className={`text-xs ${getThreatLevelColor(file.threatLevel)}`}>
                  {file.threatLevel}
                </Badge>
              </div>

              <div className="w-20 text-center text-sm">
                {formatFileSize(file.fileSize)}
              </div>

              <div className="w-32 text-center text-xs text-gray-600">
                {file.quarantineDate.toLocaleDateString()}
              </div>

              <div className="w-24 flex gap-1 justify-center">
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={() => handleViewDetails(file)}
                  title="View Details"
                >
                  <Eye className="w-3 h-3" />
                </Button>

                {file.canRestore ? (
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => {
                      handleSelectFile(file.id)
                      handleRestoreSelected()
                    }}
                    title="Restore File"
                    className="text-green-600 hover:text-green-700"
                  >
                    <RotateCcw className="w-3 h-3" />
                  </Button>
                ) : (
                  <div className="w-6" /> // Spacer
                )}

                <Button
                  variant="ghost"
                  size="sm"
                  onClick={() => {
                    handleSelectFile(file.id)
                    handleDeleteSelected()
                  }}
                  title="Delete Permanently"
                  className="text-red-600 hover:text-red-700"
                >
                  <Trash2 className="w-3 h-3" />
                </Button>
              </div>
            </div>
          ))}

          {filteredFiles.length === 0 && (
            <div className="text-center py-8 text-gray-500">
              <CheckCircle className="w-12 h-12 text-green-400 mx-auto mb-4" />
              <p className="font-medium">No quarantined files</p>
              <p className="text-sm">All threats have been neutralized</p>
            </div>
          )}
        </div>
      </Card>

      {/* Quarantine Information */}
      <Card className="p-6">
        <h2 className="text-xl font-bold mb-4">Quarantine Information</h2>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <h3 className="font-semibold mb-3">What is Quarantine?</h3>
            <p className="text-sm text-gray-600 mb-4">
              Quarantine is a secure holding area for potentially dangerous files. These files are isolated
              from your system to prevent them from causing harm while you decide how to handle them.
            </p>

            <h3 className="font-semibold mb-3">Safety Guidelines</h3>
            <ul className="text-sm text-gray-600 space-y-1">
              <li>• Never restore files from unknown sources</li>
              <li>• System files should only be restored if you're certain they're safe</li>
              <li>• When in doubt, permanently delete suspicious files</li>
              <li>• Keep regular backups of important data</li>
            </ul>
          </div>

          <div>
            <h3 className="font-semibold mb-3">Threat Level Guide</h3>
            <div className="space-y-2">
              <div className="flex items-center gap-2">
                <XCircle className="w-4 h-4 text-red-500" />
                <div>
                  <div className="font-medium text-sm">Critical</div>
                  <div className="text-xs text-gray-600">Immediate threat - do not restore</div>
                </div>
              </div>

              <div className="flex items-center gap-2">
                <AlertTriangle className="w-4 h-4 text-orange-500" />
                <div>
                  <div className="font-medium text-sm">Malicious</div>
                  <div className="text-xs text-gray-600">Confirmed malware - high risk</div>
                </div>
              </div>

              <div className="flex items-center gap-2">
                <Shield className="w-4 h-4 text-yellow-500" />
                <div>
                  <div className="font-medium text-sm">Suspicious</div>
                  <div className="text-xs text-gray-600">Potentially unsafe - review carefully</div>
                </div>
              </div>
            </div>

            <div className="mt-4 p-3 bg-yellow-50 border border-yellow-200 rounded">
              <div className="flex items-center gap-2 text-yellow-800">
                <AlertTriangle className="w-4 h-4" />
                <span className="font-medium text-sm">Important</span>
              </div>
              <p className="text-xs text-yellow-700 mt-1">
                Permanently deleted files cannot be recovered. Always ensure you have backups
                before deleting important files.
              </p>
            </div>
          </div>
        </div>
      </Card>
    </div>
  )
}
