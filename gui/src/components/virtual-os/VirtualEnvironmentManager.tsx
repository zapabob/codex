'use client'

import { useState } from 'react'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/atoms/Button'
import { VirtualEnvironment } from '@/app/virtual-os/page'
import {
  Play,
  Square,
  Trash2,
  Plus,
  Settings,
  Monitor,
  HardDrive,
  Cpu,
  Zap,
  Calendar,
  Activity
} from 'lucide-react'

interface VirtualEnvironmentManagerProps {
  environments: VirtualEnvironment[]
  onEnvironmentSelect: (env: VirtualEnvironment) => void
  onEnvironmentCreate: (env: VirtualEnvironment) => void
  onEnvironmentDelete: (envId: string) => void
}

export function VirtualEnvironmentManager({
  environments,
  onEnvironmentSelect,
  onEnvironmentCreate,
  onEnvironmentDelete
}: VirtualEnvironmentManagerProps) {
  const [selectedEnvironment, setSelectedEnvironment] = useState<VirtualEnvironment | null>(null)
  const [showCreateDialog, setShowCreateDialog] = useState(false)
  const [newEnvironment, setNewEnvironment] = useState({
    name: '',
    image: 'ubuntu:20.04',
    cpu: 2,
    memory: 4096,
    disk: 50,
  })

  const handleCreateEnvironment = () => {
    const env: VirtualEnvironment = {
      id: `env-${Date.now()}`,
      name: newEnvironment.name,
      status: 'creating',
      containerId: `container-${Date.now()}`,
      image: newEnvironment.image,
      ports: {},
      resources: {
        cpu: newEnvironment.cpu,
        memory: newEnvironment.memory,
        disk: newEnvironment.disk,
      },
      createdAt: new Date(),
      lastAccessed: new Date(),
    }

    onEnvironmentCreate(env)
    setShowCreateDialog(false)
    setNewEnvironment({
      name: '',
      image: 'ubuntu:20.04',
      cpu: 2,
      memory: 4096,
      disk: 50,
    })
  }

  const handleEnvironmentAction = (env: VirtualEnvironment, action: 'start' | 'stop' | 'delete') => {
    switch (action) {
      case 'start':
        // Simulate starting environment
        console.log(`Starting environment: ${env.id}`)
        break
      case 'stop':
        // Simulate stopping environment
        console.log(`Stopping environment: ${env.id}`)
        break
      case 'delete':
        if (confirm(`Are you sure you want to delete environment "${env.name}"?`)) {
          onEnvironmentDelete(env.id)
        }
        break
    }
  }

  const getStatusIcon = (status: VirtualEnvironment['status']) => {
    switch (status) {
      case 'running':
        return <Activity className="w-4 h-4 text-green-500" />
      case 'stopped':
        return <Square className="w-4 h-4 text-gray-500" />
      case 'creating':
        return <Zap className="w-4 h-4 text-blue-500 animate-pulse" />
      case 'error':
        return <Activity className="w-4 h-4 text-red-500" />
    }
  }

  const getStatusColor = (status: VirtualEnvironment['status']) => {
    switch (status) {
      case 'running':
        return 'bg-green-100 text-green-800 border-green-200'
      case 'stopped':
        return 'bg-gray-100 text-gray-800 border-gray-200'
      case 'creating':
        return 'bg-blue-100 text-blue-800 border-blue-200'
      case 'error':
        return 'bg-red-100 text-red-800 border-red-200'
    }
  }

  const formatUptime = (createdAt: Date, lastAccessed: Date) => {
    const diff = Date.now() - createdAt.getTime()
    const hours = Math.floor(diff / (1000 * 60 * 60))
    const minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60))

    if (hours > 0) {
      return `${hours}h ${minutes}m`
    } else {
      return `${minutes}m`
    }
  }

  const predefinedTemplates = [
    {
      name: 'Node.js Development',
      image: 'node:18-alpine',
      description: 'Full-stack JavaScript development environment',
      cpu: 2,
      memory: 4096,
      disk: 50,
    },
    {
      name: 'Python Data Science',
      image: 'python:3.9-slim',
      description: 'Data science and machine learning environment',
      cpu: 4,
      memory: 8192,
      disk: 100,
    },
    {
      name: 'Rust Development',
      image: 'rust:1.70-slim',
      description: 'Systems programming with Rust',
      cpu: 2,
      memory: 2048,
      disk: 25,
    },
    {
      name: 'Full Stack Development',
      image: 'ubuntu:20.04',
      description: 'Complete development environment with multiple languages',
      cpu: 4,
      memory: 8192,
      disk: 100,
    }
  ]

  return (
    <div className="h-full p-6 space-y-6 overflow-y-auto">
      {/* Quick Actions */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-xl font-bold">Virtual Environments</h2>
          <p className="text-gray-600">Manage isolated development environments</p>
        </div>

        <Button onClick={() => setShowCreateDialog(true)}>
          <Plus className="w-4 h-4 mr-2" />
          Create Environment
        </Button>
      </div>

      {/* Environment Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {environments.map((env) => (
          <Card
            key={env.id}
            className={`p-6 cursor-pointer transition-all hover:shadow-lg ${
              selectedEnvironment?.id === env.id ? 'ring-2 ring-blue-500' : ''
            }`}
            onClick={() => {
              setSelectedEnvironment(env)
              onEnvironmentSelect(env)
            }}
          >
            <div className="flex items-center justify-between mb-4">
              <div className="flex items-center gap-3">
                {getStatusIcon(env.status)}
                <div>
                  <h3 className="font-semibold text-lg">{env.name}</h3>
                  <p className="text-sm text-gray-600">{env.image}</p>
                </div>
              </div>

              <Badge className={getStatusColor(env.status)}>
                {env.status.toUpperCase()}
              </Badge>
            </div>

            {/* Resources */}
            <div className="space-y-3 mb-4">
              <div className="flex items-center gap-2 text-sm">
                <Cpu className="w-4 h-4 text-blue-500" />
                <span>{env.resources.cpu} CPU cores</span>
              </div>
              <div className="flex items-center gap-2 text-sm">
                <Monitor className="w-4 h-4 text-green-500" />
                <span>{env.resources.memory} MB RAM</span>
              </div>
              <div className="flex items-center gap-2 text-sm">
                <HardDrive className="w-4 h-4 text-purple-500" />
                <span>{env.resources.disk} GB Disk</span>
              </div>
            </div>

            {/* Ports */}
            {Object.keys(env.ports).length > 0 && (
              <div className="mb-4">
                <div className="text-sm font-medium mb-2">Ports</div>
                <div className="flex gap-2">
                  {Object.entries(env.ports).map(([internal, external]) => (
                    <Badge key={internal} variant="outline" className="text-xs">
                      {external}:{internal}
                    </Badge>
                  ))}
                </div>
              </div>
            )}

            {/* Timestamps */}
            <div className="text-xs text-gray-500 space-y-1 mb-4">
              <div className="flex items-center gap-1">
                <Calendar className="w-3 h-3" />
                <span>Created: {env.createdAt.toLocaleDateString()}</span>
              </div>
              <div className="flex items-center gap-1">
                <Activity className="w-3 h-3" />
                <span>Uptime: {formatUptime(env.createdAt, env.lastAccessed)}</span>
              </div>
            </div>

            {/* Actions */}
            <div className="flex gap-2">
              {env.status === 'stopped' && (
                <Button
                  size="sm"
                  variant="outline"
                  onClick={(e) => {
                    e.stopPropagation()
                    handleEnvironmentAction(env, 'start')
                  }}
                >
                  <Play className="w-3 h-3 mr-1" />
                  Start
                </Button>
              )}

              {env.status === 'running' && (
                <Button
                  size="sm"
                  variant="outline"
                  onClick={(e) => {
                    e.stopPropagation()
                    handleEnvironmentAction(env, 'stop')
                  }}
                >
                  <Square className="w-3 h-3 mr-1" />
                  Stop
                </Button>
              )}

              <Button
                size="sm"
                variant="outline"
                onClick={(e) => {
                  e.stopPropagation()
                  handleEnvironmentAction(env, 'delete')
                }}
              >
                <Trash2 className="w-3 h-3 mr-1" />
                Delete
              </Button>

              <Button
                size="sm"
                variant="outline"
                onClick={(e) => {
                  e.stopPropagation()
                  // Open settings
                }}
              >
                <Settings className="w-3 h-3" />
              </Button>
            </div>
          </Card>
        ))}
      </div>

      {/* Empty State */}
      {environments.length === 0 && (
        <Card className="p-12">
          <div className="text-center">
            <Monitor className="w-16 h-16 text-gray-400 mx-auto mb-4" />
            <h3 className="text-xl font-bold text-gray-900 mb-2">No Environments</h3>
            <p className="text-gray-600 mb-6">
              Create your first virtual development environment to get started.
            </p>
            <Button onClick={() => setShowCreateDialog(true)}>
              <Plus className="w-4 h-4 mr-2" />
              Create Environment
            </Button>
          </div>
        </Card>
      )}

      {/* Create Environment Dialog */}
      {showCreateDialog && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <Card className="w-full max-w-2xl mx-4 max-h-[90vh] overflow-y-auto">
            <div className="p-6">
              <h2 className="text-xl font-bold mb-6">Create Virtual Environment</h2>

              {/* Templates */}
              <div className="mb-6">
                <h3 className="font-semibold mb-3">Quick Templates</h3>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
                  {predefinedTemplates.map((template) => (
                    <div
                      key={template.name}
                      className="p-3 border rounded-lg cursor-pointer hover:bg-gray-50"
                      onClick={() => setNewEnvironment({
                        name: template.name,
                        image: template.image,
                        cpu: template.cpu,
                        memory: template.memory,
                        disk: template.disk,
                      })}
                    >
                      <h4 className="font-medium">{template.name}</h4>
                      <p className="text-sm text-gray-600">{template.description}</p>
                      <div className="text-xs text-gray-500 mt-1">
                        {template.cpu} CPU, {template.memory}MB RAM, {template.disk}GB Disk
                      </div>
                    </div>
                  ))}
                </div>
              </div>

              {/* Custom Configuration */}
              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium mb-1">Environment Name</label>
                  <input
                    type="text"
                    value={newEnvironment.name}
                    onChange={(e) => setNewEnvironment(prev => ({ ...prev, name: e.target.value }))}
                    className="w-full px-3 py-2 border rounded"
                    placeholder="My Development Environment"
                  />
                </div>

                <div>
                  <label className="block text-sm font-medium mb-1">Base Image</label>
                  <select
                    value={newEnvironment.image}
                    onChange={(e) => setNewEnvironment(prev => ({ ...prev, image: e.target.value }))}
                    className="w-full px-3 py-2 border rounded"
                  >
                    <option value="ubuntu:20.04">Ubuntu 20.04</option>
                    <option value="ubuntu:22.04">Ubuntu 22.04</option>
                    <option value="node:18-alpine">Node.js 18 (Alpine)</option>
                    <option value="python:3.9-slim">Python 3.9 (Slim)</option>
                    <option value="rust:1.70-slim">Rust 1.70 (Slim)</option>
                    <option value="golang:1.19-alpine">Go 1.19 (Alpine)</option>
                  </select>
                </div>

                <div className="grid grid-cols-3 gap-4">
                  <div>
                    <label className="block text-sm font-medium mb-1">CPU Cores</label>
                    <input
                      type="number"
                      value={newEnvironment.cpu}
                      onChange={(e) => setNewEnvironment(prev => ({ ...prev, cpu: parseInt(e.target.value) || 1 }))}
                      className="w-full px-3 py-2 border rounded"
                      min="1"
                      max="8"
                    />
                  </div>

                  <div>
                    <label className="block text-sm font-medium mb-1">Memory (MB)</label>
                    <input
                      type="number"
                      value={newEnvironment.memory}
                      onChange={(e) => setNewEnvironment(prev => ({ ...prev, memory: parseInt(e.target.value) || 1024 }))}
                      className="w-full px-3 py-2 border rounded"
                      min="1024"
                      step="1024"
                    />
                  </div>

                  <div>
                    <label className="block text-sm font-medium mb-1">Disk (GB)</label>
                    <input
                      type="number"
                      value={newEnvironment.disk}
                      onChange={(e) => setNewEnvironment(prev => ({ ...prev, disk: parseInt(e.target.value) || 10 }))}
                      className="w-full px-3 py-2 border rounded"
                      min="10"
                      step="10"
                    />
                  </div>
                </div>
              </div>

              {/* Actions */}
              <div className="flex justify-end gap-3 mt-6 pt-4 border-t">
                <Button variant="outline" onClick={() => setShowCreateDialog(false)}>
                  Cancel
                </Button>
                <Button
                  onClick={handleCreateEnvironment}
                  disabled={!newEnvironment.name.trim()}
                >
                  Create Environment
                </Button>
              </div>
            </div>
          </Card>
        </div>
      )}
    </div>
  )
}
