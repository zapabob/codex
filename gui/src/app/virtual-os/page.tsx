'use client'

import { useState, useEffect } from 'react'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/atoms/Button'
import { VirtualEnvironmentManager } from '@/components/virtual-os/VirtualEnvironmentManager'
import { CodeEditor } from '@/components/virtual-os/CodeEditor'
import { BrowserInterface } from '@/components/virtual-os/BrowserInterface'
import { AIAssistant } from '@/components/virtual-os/AIAssistant'
import { ResourceMonitor } from '@/components/virtual-os/ResourceMonitor'
import { DashboardLayout } from '@/components/templates/DashboardLayout'

// Virtual environment types
export type VirtualEnvironment = {
  id: string
  name: string
  status: 'creating' | 'running' | 'stopped' | 'error'
  containerId: string
  image: string
  ports: Record<number, number>
  resources: {
    cpu: number
    memory: number
    disk: number
  }
  createdAt: Date
  lastAccessed: Date
}

// Code execution types
export type CodeExecution = {
  id: string
  environmentId: string
  code: string
  language: string
  status: 'pending' | 'running' | 'completed' | 'failed'
  output: string
  error: string
  executionTime: number
  timestamp: Date
}

// Browser session types
export type BrowserSession = {
  id: string
  environmentId: string
  url: string
  status: 'loading' | 'ready' | 'error'
  screenshot?: string
  console: string[]
  network: Array<{
    url: string
    status: number
    size: number
    time: number
  }>
}

// AI generation types
export type AIGeneration = {
  id: string
  prompt: string
  language: string
  code: string
  explanation: string
  confidence: number
  timestamp: Date
}

export default function VirtualOSPage() {
  const [activeTab, setActiveTab] = useState<'environments' | 'editor' | 'browser' | 'ai' | 'monitor'>('environments')
  const [environments, setEnvironments] = useState<VirtualEnvironment[]>([])
  const [selectedEnvironment, setSelectedEnvironment] = useState<VirtualEnvironment | null>(null)
  const [executions, setExecutions] = useState<CodeExecution[]>([])
  const [browserSession, setBrowserSession] = useState<BrowserSession | null>(null)
  const [aiGenerations, setAiGenerations] = useState<AIGeneration[]>([])

  // Initialize sample data
  useEffect(() => {
    const sampleEnvironments: VirtualEnvironment[] = [
      {
        id: 'env-1',
        name: 'Node.js Development',
        status: 'running',
        containerId: 'abc123',
        image: 'node:18-alpine',
        ports: { 3000: 3000, 8080: 8080 },
        resources: { cpu: 2, memory: 4096, disk: 50 },
        createdAt: new Date(Date.now() - 2 * 60 * 60 * 1000),
        lastAccessed: new Date(Date.now() - 30 * 60 * 1000),
      },
      {
        id: 'env-2',
        name: 'Python Data Science',
        status: 'stopped',
        containerId: 'def456',
        image: 'python:3.9-slim',
        ports: { 8888: 8888 },
        resources: { cpu: 4, memory: 8192, disk: 100 },
        createdAt: new Date(Date.now() - 5 * 60 * 60 * 1000),
        lastAccessed: new Date(Date.now() - 2 * 60 * 60 * 1000),
      },
      {
        id: 'env-3',
        name: 'Rust Development',
        status: 'creating',
        containerId: 'ghi789',
        image: 'rust:1.70-slim',
        ports: {},
        resources: { cpu: 2, memory: 2048, disk: 25 },
        createdAt: new Date(),
        lastAccessed: new Date(),
      }
    ]

    const sampleExecutions: CodeExecution[] = [
      {
        id: 'exec-1',
        environmentId: 'env-1',
        code: 'console.log("Hello, World!");',
        language: 'javascript',
        status: 'completed',
        output: 'Hello, World!',
        error: '',
        executionTime: 0.05,
        timestamp: new Date(Date.now() - 10 * 60 * 1000),
      },
      {
        id: 'exec-2',
        environmentId: 'env-2',
        code: 'print("Python execution")',
        language: 'python',
        status: 'completed',
        output: 'Python execution',
        error: '',
        executionTime: 0.03,
        timestamp: new Date(Date.now() - 5 * 60 * 1000),
      }
    ]

    setEnvironments(sampleEnvironments)
    setExecutions(sampleExecutions)
  }, [])

  const handleEnvironmentSelect = (env: VirtualEnvironment) => {
    setSelectedEnvironment(env)
  }

  const handleCodeExecution = (execution: CodeExecution) => {
    setExecutions(prev => [execution, ...prev.slice(0, 49)]) // Keep last 50 executions
  }

  const handleBrowserLaunch = (session: BrowserSession) => {
    setBrowserSession(session)
  }

  const handleAIGeneration = (generation: AIGeneration) => {
    setAiGenerations(prev => [generation, ...prev.slice(0, 19)]) // Keep last 20 generations
  }

  const getStatusColor = (status: VirtualEnvironment['status']) => {
    switch (status) {
      case 'running': return 'bg-green-100 text-green-800'
      case 'stopped': return 'bg-gray-100 text-gray-800'
      case 'creating': return 'bg-blue-100 text-blue-800'
      case 'error': return 'bg-red-100 text-red-800'
      default: return 'bg-gray-100 text-gray-800'
    }
  }

  return (
    <DashboardLayout>
      <div className="h-full flex flex-col">
        {/* Header */}
        <div className="flex items-center justify-between p-6 border-b">
          <div>
            <h1 className="text-3xl font-bold text-gray-900">Virtual OS Environments</h1>
            <p className="text-gray-600 mt-1">
              Container-based development environments with browser integration and AI assistance
            </p>
          </div>

          {/* Stats */}
          <div className="flex items-center gap-6">
            <div className="text-center">
              <div className="text-2xl font-bold text-blue-600">{environments.length}</div>
              <div className="text-sm text-gray-600">Environments</div>
            </div>
            <div className="text-center">
              <div className="text-2xl font-bold text-green-600">
                {environments.filter(e => e.status === 'running').length}
              </div>
              <div className="text-sm text-gray-600">Running</div>
            </div>
            <div className="text-center">
              <div className="text-2xl font-bold text-purple-600">{executions.length}</div>
              <div className="text-sm text-gray-600">Executions</div>
            </div>
          </div>
        </div>

        {/* Tab Navigation */}
        <div className="flex gap-1 p-4 bg-gray-50 border-b overflow-x-auto">
          {[
            { id: 'environments', label: 'Environments', icon: '🖥️' },
            { id: 'editor', label: 'Code Editor', icon: '📝' },
            { id: 'browser', label: 'Browser', icon: '🌐' },
            { id: 'ai', label: 'AI Assistant', icon: '🤖' },
            { id: 'monitor', label: 'Resource Monitor', icon: '📊' }
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
          {activeTab === 'environments' && (
            <VirtualEnvironmentManager
              environments={environments}
              onEnvironmentSelect={handleEnvironmentSelect}
              onEnvironmentCreate={(env) => setEnvironments(prev => [...prev, env])}
              onEnvironmentDelete={(envId) => setEnvironments(prev => prev.filter(e => e.id !== envId))}
            />
          )}

          {activeTab === 'editor' && (
            <CodeEditor
              selectedEnvironment={selectedEnvironment}
              onCodeExecute={handleCodeExecution}
              executions={executions}
            />
          )}

          {activeTab === 'browser' && (
            <BrowserInterface
              selectedEnvironment={selectedEnvironment}
              browserSession={browserSession}
              onBrowserLaunch={handleBrowserLaunch}
            />
          )}

          {activeTab === 'ai' && (
            <AIAssistant
              selectedEnvironment={selectedEnvironment}
              onCodeGenerate={handleAIGeneration}
              generations={aiGenerations}
            />
          )}

          {activeTab === 'monitor' && (
            <ResourceMonitor
              environments={environments}
              executions={executions}
            />
          )}
        </div>
      </div>
    </DashboardLayout>
  )
}
