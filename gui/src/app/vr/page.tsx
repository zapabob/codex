'use client'

import { useState, useEffect } from 'react'
import { useSearchParams } from 'next/navigation'
import dynamic from 'next/dynamic'
import { Git4DVisualization } from '@/components/visualization/Git4DVisualization'
import VRInterface from '@/components/vr/VRInterface'
import { apiClient } from '@/lib/api/client'

// Dynamically import VR components (client-side only)
const Scene3DVXR = dynamic(
  () => import('@/components/visualization/Scene3DVXR').catch(() => null),
  { ssr: false }
)

export default function VRPage() {
  const searchParams = useSearchParams()
  const [commits, setCommits] = useState<any[]>([])
  const [selectedCommit, setSelectedCommit] = useState<any | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [sessionId, setSessionId] = useState<string | null>(null)
  const [mode, setMode] = useState<'vr' | 'ar'>('vr')

  const urlMode = (searchParams?.get('mode') || 'vr') as 'vr' | 'ar'
  const repositoryPath = searchParams?.get('repository_path') || '.'

  useEffect(() => {
    setMode(urlMode)
    launchVRSession()
  }, [urlMode, repositoryPath])

  const launchVRSession = async () => {
    setLoading(true)
    setError(null)
    
    try {
      const response = await apiClient.launchGit4D({
        mode,
        repositoryPath,
      })
      
      setSessionId(response.sessionId)
      
      // Load commit data for VR interface
      // TODO: Load actual commit data from backend
      setCommits([])
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to launch VR session')
    } finally {
      setLoading(false)
    }
  }

  const handleCommitClick = (commit: any) => {
    setSelectedCommit(commit)
    console.log('Selected commit:', commit)
  }

  if (loading) {
    return (
      <div style={{ 
        display: 'flex', 
        alignItems: 'center', 
        justifyContent: 'center', 
        height: '100vh',
        backgroundColor: '#000',
        color: '#fff',
      }}>
        <div className="text-center">
          <div className="animate-spin text-6xl mb-4">⏳</div>
          <h1 className="text-2xl mb-2">Loading Git {mode.toUpperCase()}...</h1>
          <p>Preparing immersive visualization</p>
        </div>
      </div>
    )
  }

  if (error) {
    return (
      <div style={{ 
        display: 'flex', 
        alignItems: 'center', 
        justifyContent: 'center', 
        height: '100vh',
        backgroundColor: '#000',
        color: '#fff',
      }}>
        <div className="text-center">
          <div className="text-6xl mb-4">❌</div>
          <h1 className="text-2xl mb-2">Error</h1>
          <p className="text-red-400">{error}</p>
          <button
            onClick={launchVRSession}
            className="mt-4 px-6 py-3 bg-purple-500 hover:bg-purple-600 text-white rounded-lg font-semibold transition"
          >
            Retry
          </button>
        </div>
      </div>
    )
  }

  return (
    <div style={{ width: '100%', height: '100vh', backgroundColor: '#000' }}>
      {/* Desktop overlay UI */}
      <div style={{
        position: 'absolute',
        top: 20,
        left: 20,
        color: '#fff',
        zIndex: 1000,
        backgroundColor: 'rgba(0,0,0,0.7)',
        padding: '10px 20px',
        borderRadius: '8px',
      }}>
        <h2>Codex Git {mode.toUpperCase()}</h2>
        <p>Put on your {mode.toUpperCase()} headset and click "Enter {mode.toUpperCase()}"</p>
        <p>Commits: {commits.length}</p>
        {selectedCommit && (
          <div style={{ marginTop: '10px', fontSize: '12px' }}>
            <strong>Selected:</strong> {selectedCommit.sha?.substring(0, 8) || 'N/A'}
            <br />
            {selectedCommit.message || 'No message'}
          </div>
        )}
      </div>

      {/* Main VR Visualization */}
      <Git4DVisualization 
        mode={mode}
        repositoryPath={repositoryPath}
        sessionId={sessionId || undefined}
      />

      {/* VR Interface Overlay */}
      <VRInterface
        commits={commits}
        selectedCommit={selectedCommit}
        onTimelineChange={(time) => {
          // Handle timeline change
          console.log('Timeline changed:', time)
        }}
      />
    </div>
  )
}
