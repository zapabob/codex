import { useState, useEffect } from 'react'
import { Card } from '../atoms/Card'
import { Badge } from '../atoms/Badge'
import { Button } from '../atoms/Button'
import { QuarantineEntry } from '../../types/security'
import {
  Shield,
  AlertTriangle,
  XCircle,
  RotateCcw,
  Trash2,
  Eye,
  HardDrive
} from 'lucide-react'

interface QuarantineManagerProps {
  onFileRestored: () => void
  onFileDeleted: () => void
}

export function QuarantineManager({ onFileRestored, onFileDeleted }: QuarantineManagerProps) {
  const [quarantinedFiles, setQuarantinedFiles] = useState<QuarantineEntry[]>(() => [
    { id: 'q1', fileName: 'malicious_sig.exe', originalPath: 'C:\\Users\\Downloads', threatLevel: 'malicious', quarantineDate: new Date(), fileSize: 1024*240, canRestore: true },
    { id: 'q2', fileName: 'kernel_mod.dll', originalPath: 'C:\\Windows\\System32', threatLevel: 'critical', quarantineDate: new Date(), fileSize: 1024*180, canRestore: false }
  ])

  // onFileRestored and onFileDeleted can be used in handlers

  return (
    <div className="space-y-6">
      <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
        {[
          { label: 'Total Isolated', val: quarantinedFiles.length, icon: Shield, color: 'text-rose-400' },
          { label: 'Critical Heap', val: quarantinedFiles.filter(f=>f.threatLevel==='critical').length, icon: XCircle, color: 'text-red-500' },
          { label: 'Storage Impact', val: '420 KB', icon: HardDrive, color: 'text-sky-400' }
        ].map((stat, i) => (
          <Card key={i} animated>
            <div className="p-4 flex items-center gap-4">
                <div className={`p-3 rounded-2xl bg-muted/30 ${stat.color}`}>
                    <stat.icon size={24} />
                </div>
                <div>
                    <div className="text-2xl font-black font-mono">{stat.val}</div>
                    <div className="text-[10px] font-bold text-muted-foreground uppercase tracking-widest">{stat.label}</div>
                </div>
            </div>
          </Card>
        ))}
      </div>

      <Card animated className="flex-1">
        <div className="p-6">
            <h2 className="text-lg font-bold mb-6">Isolation Vault</h2>
            <div className="space-y-3">
                {quarantinedFiles.map(file => (
                    <div key={file.id} className="p-4 rounded-2xl bg-muted/20 border border-border group hover:bg-muted/30 transition-all flex flex-col sm:flex-row sm:items-center justify-between gap-4">
                        <div className="flex items-center gap-4">
                            <div className={`h-10 w-10 rounded-xl flex items-center justify-center ${file.threatLevel === 'critical' ? 'bg-rose-500/20 text-rose-500' : 'bg-amber-500/20 text-amber-500'}`}>
                                {file.threatLevel === 'critical' ? <XCircle size={20} /> : <AlertTriangle size={20} />}
                            </div>
                            <div>
                                <h4 className="text-sm font-bold">{file.fileName}</h4>
                                <p className="text-[10px] text-muted-foreground font-mono truncate max-w-[200px]">{file.originalPath}</p>
                            </div>
                        </div>

                        <div className="flex items-center gap-6">
                            <div className="hidden md:block text-right">
                                <div className="text-[10px] font-bold text-muted-foreground uppercase">Threat Level</div>
                                <Badge color={file.threatLevel === 'critical' ? 'error' : 'warning'} size="sm">{file.threatLevel.toUpperCase()}</Badge>
                            </div>
                            <div className="flex items-center gap-2">
                                <Button variant="outlined" size="small" className="min-w-0 px-2"><Eye size={16} /></Button>
                                <Button 
                                    variant="outlined" 
                                    size="small" 
                                    color="success" 
                                    disabled={!file.canRestore}
                                    className="min-w-0 px-2"
                                >
                                    <RotateCcw size={16} />
                                </Button>
                                <Button variant="outlined" size="small" color="error" className="min-w-0 px-2"><Trash2 size={16} /></Button>
                            </div>
                        </div>
                    </div>
                ))}
                {quarantinedFiles.length === 0 && (
                    <div className="text-center py-12 text-muted-foreground">
                        <Shield size={48} className="mx-auto mb-4 opacity-10" />
                        <p>Vault is empty. No threats isolated.</p>
                    </div>
                )}
            </div>
        </div>
      </Card>
      
      <Card animated sx={{ p: 4, bg: 'rgba(255, 69, 58, 0.05)', borderColor: 'rgba(255, 69, 58, 0.2)' }}>
        <div className="flex items-start gap-4">
            <AlertTriangle className="text-rose-500 mt-1" size={20} />
            <div>
                <h4 className="text-sm font-bold text-rose-500 uppercase tracking-widest mb-1">Security Advisory</h4>
                <p className="text-xs text-muted-foreground leading-relaxed">
                    Restoring system-level DLLs or executable binaries from quarantine may cause system instability if the threat persists. Perform deep-heuristic analysis before clearance.
                </p>
            </div>
        </div>
      </Card>
    </div>
  )
}
