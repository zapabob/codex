'use client'

import { useState, useEffect } from 'react'
import { useSearchParams } from 'next/navigation'
import { Scene3D } from '@/components/visualization/Scene3D'
import { Timeline } from '@/components/visualization/Timeline'
import { Git4DVisualization } from '@/components/visualization/Git4DVisualization'
import { apiClient } from '@/lib/api/client'

interface Commit3D {
  sha: string
  message: string
  author: string
  author_email: string
  timestamp: number
  branch: string
  parents: string[]
  x: number
  y: number
  z: number
  color: string
  filesChanged: number
  insertions: number
  deletions: number
}

export default function VisualizationPage() {
  const searchParams = useSearchParams()
  const [repoPath, setRepoPath] = useState('.')
  const [commits, setCommits] = useState<Commit3D[]>([])
  const [selectedCommitSha, setSelectedCommitSha] = useState<string | undefined>()
  const [currentIndex, setCurrentIndex] = useState(0)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [viewMode, setViewMode] = useState<'desktop' | 'vr' | 'ar'>('desktop')
  const [sessionId, setSessionId] = useState<string | null>(null)

  const mode = (searchParams?.get('mode') || 'desktop') as 'desktop' | 'vr' | 'ar'

  useEffect(() => {
    if (mode === 'vr' || mode === 'ar') {
      setViewMode(mode)
      launchVRSession()
    } else {
      loadData()
    }
  }, [repoPath, mode])

  const launchVRSession = async () => {
    setLoading(true)
    setError(null)
    
    try {
      const response = await apiClient.launchGit4D({
        mode: viewMode,
        repositoryPath: repoPath || '.',
      })
      
      setSessionId(response.sessionId)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to launch visualization')
    } finally {
      setLoading(false)
    }
  }

  const loadData = async () => {
    setLoading(true)
    setError(null)
    try {
      // Use Git4D API to get commit data
      // For now, use mock data structure
      // TODO: Implement actual Git data fetching via Rust backend
      const mockCommits: Commit3D[] = []
      setCommits(mockCommits)
      if (mockCommits.length > 0) {
        setCurrentIndex(0)
        setSelectedCommitSha(mockCommits[0].sha)
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load Git data')
    } finally {
      setLoading(false)
    }
  }

  const handleCommitClick = (commit: any) => {
    const index = commits.findIndex((c) => c.sha === commit.sha)
    if (index !== -1) {
      setCurrentIndex(index)
      setSelectedCommitSha(commit.sha)
    }
  }

  const handleSeek = (index: number) => {
    setCurrentIndex(index)
    if (commits[index]) {
      setSelectedCommitSha(commits[index].sha)
    }
  }

  // VR/AR mode
  if (viewMode === 'vr' || viewMode === 'ar') {
    return (
      <div className="min-h-screen bg-black">
        {loading && (
          <div className="absolute inset-0 flex items-center justify-center text-white z-50">
            <div className="text-center">
              <div className="animate-spin text-6xl mb-4">⏳</div>
              <p>Launching {viewMode.toUpperCase()} visualization...</p>
            </div>
          </div>
        )}
        {error && (
          <div className="absolute top-4 left-4 bg-red-500/20 border border-red-500 text-red-200 px-4 py-3 rounded-lg z-50">
            ❌ {error}
          </div>
        )}
        {!loading && !error && (
          <Git4DVisualization 
            mode={viewMode}
            repositoryPath={repoPath}
            sessionId={sessionId || undefined}
          />
        )}
      </div>
    )
  }

  // Desktop mode
  return (
    <div className="min-h-screen bg-gradient-to-br from-gray-900 via-purple-900 to-gray-900 p-8">
      <div className="max-w-7xl mx-auto">
        {/* Header */}
        <div className="flex items-center justify-between mb-8">
          <div>
            <h1 className="text-4xl font-bold text-white mb-2">Git Visualization</h1>
            <p className="text-gray-400">3D/4D repository visualization</p>
          </div>

          <div className="flex items-center gap-4">
            {/* Mode Selector */}
            <select
              value={viewMode}
              onChange={(e) => {
                const newMode = e.target.value as 'desktop' | 'vr' | 'ar'
                setViewMode(newMode)
                if (newMode === 'vr' || newMode === 'ar') {
                  launchVRSession()
                }
              }}
              className="px-4 py-2 bg-gray-800 text-white rounded-lg border border-gray-700 focus:outline-none focus:ring-2 focus:ring-purple-500"
            >
              <option value="desktop">🖥️ Desktop</option>
              <option value="vr">🥽 VR</option>
              <option value="ar">👓 AR</option>
            </select>

            {/* Repo Path Input */}
            <input
              type="text"
              value={repoPath}
              onChange={(e) => setRepoPath(e.target.value)}
              placeholder="Repository path (. for current)"
              className="px-4 py-2 bg-gray-800 text-white rounded-lg border border-gray-700 focus:outline-none focus:ring-2 focus:ring-purple-500 w-64"
            />

            <button
              onClick={loadData}
              className="px-6 py-2 bg-purple-500 hover:bg-purple-600 text-white rounded-lg font-semibold transition"
            >
              🔄 Reload
            </button>
          </div>
        </div>

        {/* Error Display */}
        {error && (
          <div className="bg-red-500/20 border border-red-500 text-red-200 px-4 py-3 rounded-lg mb-6">
            ❌ {error}
          </div>
        )}

        {/* Loading State */}
        {loading ? (
          <div className="text-center text-gray-400 py-20">
            <div className="animate-spin text-6xl mb-4">⏳</div>
            <p>Analyzing repository...</p>
          </div>
        ) : commits.length === 0 ? (
          <div className="text-center text-gray-400 py-20">
            <div className="text-6xl mb-4">📊</div>
            <p className="text-xl">No Git repository found</p>
            <p className="text-sm mt-2">Make sure you're in a Git repository directory</p>
          </div>
        ) : (
          <div className="space-y-6">
            {/* Stats Bar */}
            <div className="grid grid-cols-4 gap-4">
              <div className="bg-gray-800/50 backdrop-blur-lg border border-gray-700 rounded-lg p-4">
                <div className="text-sm text-gray-400 mb-1">Total Commits</div>
                <div className="text-3xl font-bold text-white">{commits.length}</div>
              </div>
              <div className="bg-gray-800/50 backdrop-blur-lg border border-gray-700 rounded-lg p-4">
                <div className="text-sm text-gray-400 mb-1">Unique Authors</div>
                <div className="text-3xl font-bold text-white">
                  {new Set(commits.map((c) => c.author)).size}
                </div>
              </div>
              <div className="bg-gray-800/50 backdrop-blur-lg border border-gray-700 rounded-lg p-4">
                <div className="text-sm text-gray-400 mb-1">Branches</div>
                <div className="text-3xl font-bold text-white">
                  {new Set(commits.map((c) => c.branch)).size}
                </div>
              </div>
              <div className="bg-gray-800/50 backdrop-blur-lg border border-gray-700 rounded-lg p-4">
                <div className="text-sm text-gray-400 mb-1">Current</div>
                <div className="text-3xl font-bold text-white">
                  {currentIndex + 1} / {commits.length}
                </div>
              </div>
            </div>

            {/* 3D Visualization */}
            <div className="bg-gray-800/50 backdrop-blur-lg border border-gray-700 rounded-xl overflow-hidden">
              <div className="p-4 border-b border-gray-700">
                <h2 className="text-xl font-bold text-white">3D Commit Graph</h2>
                <p className="text-sm text-gray-400">
                  Drag to rotate • Scroll to zoom • Click commits to select
                </p>
              </div>
              <div className="h-[600px]">
                <Scene3D
                  commits={commits.map(c => ({
                    sha: c.sha,
                    author: c.author,
                    message: c.message,
                    timestamp: c.timestamp,
                    filesChanged: c.filesChanged,
                    insertions: c.insertions,
                    deletions: c.deletions,
                  }))}
                  onCommitClick={handleCommitClick}
                  selectedCommitSha={selectedCommitSha}
                />
              </div>
            </div>

            {/* Timeline Control */}
            <div className="bg-gray-800/50 backdrop-blur-lg border border-gray-700 rounded-xl overflow-hidden">
              <Timeline
                commits={commits.map((c) => ({
                  sha: c.sha,
                  timestamp: c.timestamp,
                  message: c.message,
                  author: c.author,
                }))}
                onSeek={handleSeek}
                currentIndex={currentIndex}
              />
            </div>

            {/* Commit Details */}
            {selectedCommitSha && (
              <div className="bg-gray-800/50 backdrop-blur-lg border border-gray-700 rounded-xl p-6">
                <h2 className="text-xl font-bold text-white mb-4">Commit Details</h2>
                {(() => {
                  const commit = commits.find((c) => c.sha === selectedCommitSha)
                  if (!commit) return null

                  return (
                    <div className="space-y-4">
                      <div className="grid grid-cols-2 gap-4">
                        <div>
                          <div className="text-sm text-gray-400 mb-1">SHA</div>
                          <div className="text-white font-mono text-sm">
                            {commit.sha.substring(0, 12)}
                          </div>
                        </div>
                        <div>
                          <div className="text-sm text-gray-400 mb-1">Branch</div>
                          <div className="text-white">{commit.branch}</div>
                        </div>
                        <div>
                          <div className="text-sm text-gray-400 mb-1">Author</div>
                          <div className="text-white">{commit.author}</div>
                        </div>
                        <div>
                          <div className="text-sm text-gray-400 mb-1">Date</div>
                          <div className="text-white">
                            {new Date(commit.timestamp).toLocaleString()}
                          </div>
                        </div>
                      </div>

                      <div>
                        <div className="text-sm text-gray-400 mb-1">Message</div>
                        <div className="text-white bg-gray-700/50 p-3 rounded">
                          {commit.message}
                        </div>
                      </div>

                      {commit.parents.length > 0 && (
                        <div>
                          <div className="text-sm text-gray-400 mb-1">Parents</div>
                          <div className="flex gap-2 flex-wrap">
                            {commit.parents.map((parent) => (
                              <span
                                key={parent}
                                className="text-xs bg-gray-700 px-2 py-1 rounded font-mono"
                              >
                                {parent.substring(0, 8)}
                              </span>
                            ))}
                          </div>
                        </div>
                      )}

                      <div>
                        <div className="text-sm text-gray-400 mb-1">3D Coordinates</div>
                        <div className="text-sm text-gray-300 font-mono bg-gray-700/50 p-3 rounded">
                          X: {commit.x.toFixed(2)} | Y: {commit.y.toFixed(2)} | Z: {commit.z.toFixed(2)}
                        </div>
                      </div>
                    </div>
                  )
                })()}
              </div>
            )}

            {/* Author Legend */}
            <div className="bg-gray-800/50 backdrop-blur-lg border border-gray-700 rounded-xl p-6">
              <h2 className="text-xl font-bold text-white mb-4">Authors</h2>
              <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3">
                {Array.from(new Set(commits.map((c) => c.author_email))).map((email) => {
                  const commit = commits.find((c) => c.author_email === email)
                  if (!commit) return null

                  return (
                    <div
                      key={email}
                      className="flex items-center gap-2 bg-gray-700/50 p-2 rounded"
                    >
                      <div
                        className="w-4 h-4 rounded-full"
                        style={{ backgroundColor: commit.color }}
                      />
                      <div className="text-sm text-white truncate">{commit.author}</div>
                    </div>
                  )
                })}
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  )
}
