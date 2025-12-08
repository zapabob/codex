'use client'

import { useState } from 'react'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/atoms/Button'
import { ExecutionResult, DevelopmentTask } from '@/app/ai-tools/page'
import {
  CheckCircle,
  XCircle,
  AlertTriangle,
  FileText,
  Download,
  ThumbsUp,
  ThumbsDown,
  GitMerge,
  Target,
  TrendingUp,
  MessageSquare
} from 'lucide-react'

interface ResultIntegratorProps {
  results: ExecutionResult[]
  tasks: DevelopmentTask[]
  onResultAccept: (result: ExecutionResult) => void
  onResultReject: (result: ExecutionResult) => void
}

export function ResultIntegrator({ results, tasks, onResultAccept, onResultReject }: ResultIntegratorProps) {
  const [selectedResult, setSelectedResult] = useState<ExecutionResult | null>(null)
  const [filter, setFilter] = useState<'all' | 'successful' | 'failed' | 'conflicts'>('all')

  const filteredResults = results.filter(result => {
    switch (filter) {
      case 'successful':
        return result.success
      case 'failed':
        return !result.success
      case 'conflicts':
        return result.errors.length > 1 // Multiple errors indicate conflicts
      default:
        return true
    }
  })

  const handleResultAction = (result: ExecutionResult, action: 'accept' | 'reject') => {
    if (action === 'accept') {
      onResultAccept(result)
    } else {
      onResultReject(result)
    }
  }

  const handleExportResult = (result: ExecutionResult) => {
    const exportData = {
      taskId: result.taskId,
      success: result.success,
      integratedOutput: result.integratedOutput,
      executionTime: result.executionTime,
      qualityScore: result.qualityScore,
      recommendations: result.recommendations,
      subtaskResults: result.subtaskResults,
      errors: result.errors,
      exportedAt: new Date().toISOString(),
    }

    const blob = new Blob([JSON.stringify(exportData, null, 2)], { type: 'application/json' })
    const url = URL.createObjectURL(blob)

    const a = document.createElement('a')
    a.href = url
    a.download = `ai-result-${result.taskId}-${Date.now()}.json`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
  }

  const getResultStatusColor = (result: ExecutionResult) => {
    if (result.success) {
      return result.qualityScore > 0.8 ? 'bg-green-100 text-green-800 border-green-200' :
             result.qualityScore > 0.6 ? 'bg-blue-100 text-blue-800 border-blue-200' :
             'bg-yellow-100 text-yellow-800 border-yellow-200'
    } else {
      return 'bg-red-100 text-red-800 border-red-200'
    }
  }

  const getQualityScoreColor = (score: number) => {
    if (score >= 0.8) return 'text-green-600'
    if (score >= 0.6) return 'text-yellow-600'
    return 'text-red-600'
  }

  const getTaskTitle = (taskId: string) => {
    return tasks.find(t => t.id === taskId)?.title || `Task ${taskId}`
  }

  const stats = {
    total: results.length,
    successful: results.filter(r => r.success).length,
    failed: results.filter(r => !r.success).length,
    avgQuality: results.length > 0 ? results.reduce((sum, r) => sum + r.qualityScore, 0) / results.length : 0,
    avgExecutionTime: results.length > 0 ? results.reduce((sum, r) => sum + r.executionTime, 0) / results.length : 0,
    conflicts: results.filter(r => r.errors.length > 1).length,
  }

  return (
    <div className="h-full p-6 space-y-6 overflow-y-auto">
      {/* Integration Stats */}
      <div className="grid grid-cols-1 md:grid-cols-5 gap-4">
        <Card className="p-4">
          <div className="flex items-center gap-3">
            <FileText className="w-8 h-8 text-blue-500" />
            <div>
              <div className="text-2xl font-bold">{stats.total}</div>
              <div className="text-sm text-gray-600">Total Results</div>
            </div>
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center gap-3">
            <CheckCircle className="w-8 h-8 text-green-500" />
            <div>
              <div className="text-2xl font-bold">{stats.successful}</div>
              <div className="text-sm text-gray-600">Successful</div>
            </div>
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center gap-3">
            <XCircle className="w-8 h-8 text-red-500" />
            <div>
              <div className="text-2xl font-bold">{stats.failed}</div>
              <div className="text-sm text-gray-600">Failed</div>
            </div>
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center gap-3">
            <TrendingUp className="w-8 h-8 text-purple-500" />
            <div>
              <div className="text-2xl font-bold">{stats.avgQuality.toFixed(2)}</div>
              <div className="text-sm text-gray-600">Avg Quality</div>
            </div>
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center gap-3">
            <AlertTriangle className="w-8 h-8 text-orange-500" />
            <div>
              <div className="text-2xl font-bold">{stats.conflicts}</div>
              <div className="text-sm text-gray-600">Conflicts</div>
            </div>
          </div>
        </Card>
      </div>

      {/* Filter Controls */}
      <Card className="p-4">
        <div className="flex items-center gap-4">
          <span className="font-medium">Filter:</span>
          {[
            { id: 'all', label: 'All Results', count: stats.total },
            { id: 'successful', label: 'Successful', count: stats.successful },
            { id: 'failed', label: 'Failed', count: stats.failed },
            { id: 'conflicts', label: 'Conflicts', count: stats.conflicts },
          ].map((filterOption) => (
            <Button
              key={filterOption.id}
              variant={filter === filterOption.id ? 'primary' : 'outline'}
              size="sm"
              onClick={() => setFilter(filterOption.id as any)}
            >
              {filterOption.label} ({filterOption.count})
            </Button>
          ))}
        </div>
      </Card>

      {/* Results List */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Results Overview */}
        <Card className="p-6">
          <h2 className="text-xl font-bold mb-4">Integration Results</h2>

          <div className="space-y-4">
            {filteredResults.map((result) => (
              <div
                key={result.taskId}
                className={`p-4 border-2 rounded-lg cursor-pointer transition-all ${
                  selectedResult?.taskId === result.taskId ? 'border-blue-500 bg-blue-50' : 'border-gray-200 hover:border-gray-300'
                }`}
                onClick={() => setSelectedResult(result)}
              >
                <div className="flex items-center justify-between mb-3">
                  <div className="flex items-center gap-2">
                    {result.success ? (
                      <CheckCircle className="w-5 h-5 text-green-500" />
                    ) : (
                      <XCircle className="w-5 h-5 text-red-500" />
                    )}
                    <h3 className="font-semibold">{getTaskTitle(result.taskId)}</h3>
                  </div>

                  <Badge className={getResultStatusColor(result)}>
                    {result.success ? 'SUCCESS' : 'FAILED'}
                  </Badge>
                </div>

                <div className="grid grid-cols-2 gap-4 text-sm mb-3">
                  <div>
                    <div className="text-gray-600">Quality Score</div>
                    <div className={`font-semibold ${getQualityScoreColor(result.qualityScore)}`}>
                      {result.qualityScore.toFixed(2)}
                    </div>
                  </div>
                  <div>
                    <div className="text-gray-600">Execution Time</div>
                    <div className="font-semibold">{result.executionTime.toFixed(1)}s</div>
                  </div>
                </div>

                <div className="flex items-center justify-between">
                  <div className="text-sm text-gray-600">
                    {result.subtaskResults.length} subtasks integrated
                  </div>

                  <div className="flex gap-2">
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={(e) => {
                        e.stopPropagation()
                        handleResultAction(result, 'accept')
                      }}
                    >
                      <ThumbsUp className="w-3 h-3 mr-1" />
                      Accept
                    </Button>

                    <Button
                      variant="outline"
                      size="sm"
                      onClick={(e) => {
                        e.stopPropagation()
                        handleExportResult(result)
                      }}
                    >
                      <Download className="w-3 h-3 mr-1" />
                      Export
                    </Button>
                  </div>
                </div>
              </div>
            ))}

            {filteredResults.length === 0 && (
              <div className="text-center py-8 text-gray-500">
                <FileText className="w-12 h-12 text-gray-400 mx-auto mb-4" />
                <p>No results match the current filter</p>
                <p className="text-sm">Try changing the filter or run some tasks first</p>
              </div>
            )}
          </div>
        </Card>

        {/* Result Details */}
        <Card className="p-6">
          <h2 className="text-xl font-bold mb-4">Result Details</h2>

          {selectedResult ? (
            <div className="space-y-6">
              {/* Overview */}
              <div className="p-4 bg-gray-50 rounded">
                <h3 className="font-semibold mb-2">{getTaskTitle(selectedResult.taskId)}</h3>
                <div className="grid grid-cols-2 gap-4 text-sm">
                  <div>
                    <span className="text-gray-600">Status:</span>
                    <Badge className={`ml-2 ${getResultStatusColor(selectedResult)}`}>
                      {selectedResult.success ? 'SUCCESS' : 'FAILED'}
                    </Badge>
                  </div>
                  <div>
                    <span className="text-gray-600">Quality:</span>
                    <span className={`ml-2 font-semibold ${getQualityScoreColor(selectedResult.qualityScore)}`}>
                      {selectedResult.qualityScore.toFixed(2)}
                    </span>
                  </div>
                  <div>
                    <span className="text-gray-600">Execution Time:</span>
                    <span className="ml-2 font-semibold">{selectedResult.executionTime.toFixed(1)}s</span>
                  </div>
                  <div>
                    <span className="text-gray-600">Subtasks:</span>
                    <span className="ml-2 font-semibold">{selectedResult.subtaskResults.length}</span>
                  </div>
                </div>
              </div>

              {/* Integrated Output */}
              <div>
                <h4 className="font-semibold mb-2 flex items-center gap-2">
                  <GitMerge className="w-4 h-4" />
                  Integrated Output
                </h4>
                <div className="p-3 bg-gray-50 rounded max-h-48 overflow-y-auto">
                  <pre className="text-sm whitespace-pre-wrap">{selectedResult.integratedOutput}</pre>
                </div>
              </div>

              {/* Subtask Results */}
              <div>
                <h4 className="font-semibold mb-2 flex items-center gap-2">
                  <Target className="w-4 h-4" />
                  Subtask Results ({selectedResult.subtaskResults.length})
                </h4>

                <div className="space-y-2 max-h-64 overflow-y-auto">
                  {selectedResult.subtaskResults.map((subtask, index) => (
                    <div key={index} className="p-3 border rounded">
                      <div className="flex items-center justify-between mb-2">
                        <div className="flex items-center gap-2">
                          {subtask.success ? (
                            <CheckCircle className="w-4 h-4 text-green-500" />
                          ) : (
                            <XCircle className="w-4 h-4 text-red-500" />
                          )}
                          <span className="font-medium text-sm">{subtask.toolId}</span>
                        </div>
                        <Badge variant={subtask.success ? 'secondary' : 'destructive'} className="text-xs">
                          {subtask.success ? 'SUCCESS' : 'FAILED'}
                        </Badge>
                      </div>

                      <div className="text-sm text-gray-600 mb-1">
                        Quality: <span className={getQualityScoreColor(subtask.qualityScore)}>
                          {subtask.qualityScore.toFixed(2)}
                        </span>
                        | Time: {subtask.executionTime.toFixed(1)}s
                      </div>

                      <div className="text-sm bg-white p-2 rounded border max-h-24 overflow-y-auto">
                        {subtask.output || subtask.error || 'No output'}
                      </div>
                    </div>
                  ))}
                </div>
              </div>

              {/* Errors */}
              {selectedResult.errors.length > 0 && (
                <div>
                  <h4 className="font-semibold mb-2 flex items-center gap-2 text-red-600">
                    <AlertTriangle className="w-4 h-4" />
                    Errors ({selectedResult.errors.length})
                  </h4>

                  <div className="space-y-2">
                    {selectedResult.errors.map((error, index) => (
                      <div key={index} className="p-2 bg-red-50 border border-red-200 rounded text-sm text-red-800">
                        {error}
                      </div>
                    ))}
                  </div>
                </div>
              )}

              {/* Recommendations */}
              {selectedResult.recommendations.length > 0 && (
                <div>
                  <h4 className="font-semibold mb-2 flex items-center gap-2 text-blue-600">
                    <MessageSquare className="w-4 h-4" />
                    Recommendations
                  </h4>

                  <div className="space-y-2">
                    {selectedResult.recommendations.map((rec, index) => (
                      <div key={index} className="p-2 bg-blue-50 border border-blue-200 rounded text-sm text-blue-800">
                        {rec}
                      </div>
                    ))}
                  </div>
                </div>
              )}

              {/* Actions */}
              <div className="flex gap-3 pt-4 border-t">
                <Button onClick={() => handleResultAction(selectedResult, 'accept')}>
                  <ThumbsUp className="w-4 h-4 mr-2" />
                  Accept Result
                </Button>

                <Button variant="outline" onClick={() => handleResultAction(selectedResult, 'reject')}>
                  <ThumbsDown className="w-4 h-4 mr-2" />
                  Reject Result
                </Button>

                <Button variant="outline" onClick={() => handleExportResult(selectedResult)}>
                  <Download className="w-4 h-4 mr-2" />
                  Export JSON
                </Button>
              </div>
            </div>
          ) : (
            <div className="text-center py-12 text-gray-500">
              <FileText className="w-16 h-16 text-gray-400 mx-auto mb-4" />
              <p className="text-lg font-medium">Select a result to view details</p>
              <p className="text-sm">Click on any result from the list to see detailed information</p>
            </div>
          )}
        </Card>
      </div>

      {/* Integration Strategies */}
      <Card className="p-6">
        <h2 className="text-xl font-bold mb-4">Integration Strategies</h2>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          <div className="p-4 border rounded-lg">
            <div className="flex items-center gap-2 mb-2">
              <GitMerge className="w-5 h-5 text-blue-500" />
              <h3 className="font-semibold">Merge Strategy</h3>
            </div>
            <p className="text-sm text-gray-600">Combines all results into a unified output</p>
            <div className="text-xs text-gray-500 mt-2">Best for: Complementary results</div>
          </div>

          <div className="p-4 border rounded-lg">
            <div className="flex items-center gap-2 mb-2">
              <TrendingUp className="w-5 h-5 text-green-500" />
              <h3 className="font-semibold">Quality Selection</h3>
            </div>
            <p className="text-sm text-gray-600">Selects the highest quality result</p>
            <div className="text-xs text-gray-500 mt-2">Best for: Code generation tasks</div>
          </div>

          <div className="p-4 border rounded-lg">
            <div className="flex items-center gap-2 mb-2">
              <Target className="w-5 h-5 text-purple-500" />
              <h3 className="font-semibold">Combine Strategy</h3>
            </div>
            <p className="text-sm text-gray-600">Combines complementary results</p>
            <div className="text-xs text-gray-500 mt-2">Best for: Testing and validation</div>
          </div>

          <div className="p-4 border rounded-lg">
            <div className="flex items-center gap-2 mb-2">
              <MessageSquare className="w-5 h-5 text-orange-500" />
              <h3 className="font-semibold">Voting Strategy</h3>
            </div>
            <p className="text-sm text-gray-600">Uses consensus voting for final result</p>
            <div className="text-xs text-gray-500 mt-2">Best for: Review and analysis tasks</div>
          </div>
        </div>
      </Card>
    </div>
  )
}
