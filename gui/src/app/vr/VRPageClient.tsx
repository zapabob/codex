'use client'

import { useEffect, useState } from 'react'
import { useSearchParams } from 'next/navigation'
import { Git4DVisualization } from '@/components/visualization/Git4DVisualization'
import { apiClient } from '@/lib/api/client'

type VRMode = 'vr' | 'ar'

export default function VRPage() {
  const searchParams = useSearchParams()
  const [commits, setCommits] = useState<any[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [sessionId, setSessionId] = useState<string | null>(null)

  const mode = (searchParams?.get('mode') || 'vr') as VRMode
  const repositoryPath = searchParams?.get('repository_path') || '.'

  useEffect(() => {
    void launchVRSession(mode, repositoryPath)
  }, [mode, repositoryPath])

  const launchVRSession = async (
    nextMode: VRMode,
    nextRepositoryPath: string
  ) => {
    setLoading(true)
    setError(null)

    try {
      const response = await apiClient.launchGit4D({
        mode: nextMode,
        repositoryPath: nextRepositoryPath,
      })

      setSessionId(response.sessionId)

      // TODO: Load actual commit data from the backend when the VR session
      // exposes commit payloads for the overlay controls.
      setCommits([])
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to launch VR session')
    } finally {
      setLoading(false)
    }
  }

  if (loading) {
    return (
      <div
        style={{
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          height: '100vh',
          backgroundColor: '#000',
          color: '#fff',
        }}
      >
        <div className="text-center">
          <div className="animate-spin text-6xl mb-4">Loading</div>
          <h1 className="text-2xl mb-2">Loading Git {mode.toUpperCase()}...</h1>
          <p>Preparing immersive visualization</p>
        </div>
      </div>
    )
  }

  if (error) {
    return (
      <div
        style={{
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          height: '100vh',
          backgroundColor: '#000',
          color: '#fff',
        }}
      >
        <div className="text-center">
          <div className="text-6xl mb-4">!</div>
          <h1 className="text-2xl mb-2">Error</h1>
          <p className="text-red-400">{error}</p>
          <button
            onClick={() => void launchVRSession(mode, repositoryPath)}
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
      <div
        style={{
          position: 'absolute',
          top: 20,
          left: 20,
          color: '#fff',
          zIndex: 1000,
          backgroundColor: 'rgba(0,0,0,0.7)',
          padding: '10px 20px',
          borderRadius: '8px',
        }}
      >
        <h2>Codex Git {mode.toUpperCase()}</h2>
        <p>Put on your {mode.toUpperCase()} headset and click &quot;Enter {mode.toUpperCase()}&quot;</p>
        <p>Commits: {commits.length}</p>
      </div>

      <Git4DVisualization
        mode={mode}
        repositoryPath={repositoryPath}
        sessionId={sessionId || undefined}
      />
    </div>
  )
}
