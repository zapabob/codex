'use client'

import { useState } from 'react'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/atoms/Button'
import { AITool, DevelopmentTask } from '@/app/ai-tools/page'
import {
  Plus,
  Target,
  GitBranch,
  Settings,
  CheckCircle,
  AlertTriangle,
  Users,
  Clock,
  Zap
} from 'lucide-react'

interface TaskDistributorProps {
  tasks: DevelopmentTask[]
  aiTools: AITool[]
  onTaskCreate: (task: DevelopmentTask) => void
  onTaskUpdate: (task: DevelopmentTask) => void
}

export function TaskDistributor({ tasks, aiTools, onTaskCreate, onTaskUpdate }: TaskDistributorProps) {
  const [showCreateDialog, setShowCreateDialog] = useState(false)
  const [newTask, setNewTask] = useState({
    title: '',
    description: '',
    complexity: 'medium' as DevelopmentTask['complexity'],
    priority: 'medium' as DevelopmentTask['priority'],
    requirements: [] as string[],
    selectedTools: [] as string[],
  })

  const handleCreateTask = () => {
    if (!newTask.title.trim() || !newTask.description.trim()) {
      alert('Please fill in title and description')
      return
    }

    // Generate subtasks based on complexity and requirements
    const subtasks = generateSubtasks(newTask)

    const task: DevelopmentTask = {
      id: `task-${Date.now()}`,
      title: newTask.title,
      description: newTask.description,
      complexity: newTask.complexity,
      priority: newTask.priority,
      requirements: newTask.requirements,
      subtasks,
      status: 'pending',
      createdAt: new Date(),
      assignedTools: newTask.selectedTools,
      progress: 0,
    }

    onTaskCreate(task)
    setShowCreateDialog(false)
    setNewTask({
      title: '',
      description: '',
      complexity: 'medium',
      priority: 'medium',
      requirements: [],
      selectedTools: [],
    })
  }

  const generateSubtasks = (taskData: typeof newTask) => {
    const subtasks = []
    const subtaskCount = getSubtaskCount(taskData.complexity)

    for (let i = 0; i < subtaskCount; i++) {
      const subtask = {
        id: `subtask-${Date.now()}-${i}`,
        parentTaskId: '', // Will be set when task is created
        description: generateSubtaskDescription(taskData, i, subtaskCount),
        assignedTool: assignToolForSubtask(taskData.requirements, i),
        status: 'pending' as const,
      }
      subtasks.push(subtask)
    }

    return subtasks
  }

  const getSubtaskCount = (complexity: DevelopmentTask['complexity']): number => {
    switch (complexity) {
      case 'simple': return 1
      case 'medium': return 2
      case 'complex': return 3
      case 'critical': return 4
      default: return 2
    }
  }

  const generateSubtaskDescription = (taskData: typeof newTask, index: number, total: number): string => {
    const baseDescription = taskData.description

    switch (taskData.complexity) {
      case 'simple':
        return baseDescription

      case 'medium':
        return index === 0
          ? `Implement core functionality: ${baseDescription}`
          : `Add testing and validation: ${baseDescription}`

      case 'complex':
        const complexTasks = [
          `Design and plan: ${baseDescription}`,
          `Implement core components: ${baseDescription}`,
          `Add comprehensive testing: ${baseDescription}`,
        ]
        return complexTasks[index] || baseDescription

      case 'critical':
        const criticalTasks = [
          `Analysis and requirements gathering: ${baseDescription}`,
          `Architecture design and prototyping: ${baseDescription}`,
          `Core implementation with quality checks: ${baseDescription}`,
          `Testing, documentation, and deployment preparation: ${baseDescription}`,
        ]
        return criticalTasks[index] || baseDescription

      default:
        return baseDescription
    }
  }

  const assignToolForSubtask = (requirements: string[], index: number): string => {
    // Simple tool assignment logic
    const availableTools = aiTools.map(t => t.id)

    if (requirements.includes('Code Generation') && index === 0) {
      return availableTools.find(t => t === 'codex') || availableTools[0]
    }

    if (requirements.includes('Testing') && (index === 1 || index === 2)) {
      return availableTools.find(t => t === 'claude-code') || availableTools[1] || availableTools[0]
    }

    if (requirements.includes('Analysis') && index === 0) {
      return availableTools.find(t => t === 'gemini-cli') || availableTools[2] || availableTools[0]
    }

    // Default assignment
    return availableTools[index % availableTools.length]
  }

  const handleRequirementToggle = (requirement: string) => {
    setNewTask(prev => ({
      ...prev,
      requirements: prev.requirements.includes(requirement)
        ? prev.requirements.filter(r => r !== requirement)
        : [...prev.requirements, requirement]
    }))
  }

  const handleToolToggle = (toolId: string) => {
    setNewTask(prev => ({
      ...prev,
      selectedTools: prev.selectedTools.includes(toolId)
        ? prev.selectedTools.filter(t => t !== toolId)
        : [...prev.selectedTools, toolId]
    }))
  }

  const getComplexityColor = (complexity: DevelopmentTask['complexity']) => {
    switch (complexity) {
      case 'simple': return 'bg-green-100 text-green-800'
      case 'medium': return 'bg-blue-100 text-blue-800'
      case 'complex': return 'bg-yellow-100 text-yellow-800'
      case 'critical': return 'bg-red-100 text-red-800'
    }
  }

  const getPriorityColor = (priority: DevelopmentTask['priority']) => {
    switch (priority) {
      case 'low': return 'bg-gray-100 text-gray-800'
      case 'medium': return 'bg-blue-100 text-blue-800'
      case 'high': return 'bg-orange-100 text-orange-800'
      case 'critical': return 'bg-red-100 text-red-800'
    }
  }

  const availableRequirements = [
    'Code Generation',
    'Code Review',
    'Testing',
    'Documentation',
    'Refactoring',
    'Analysis',
    'Chat',
    'Custom Logic'
  ]

  const pendingTasks = tasks.filter(t => t.status === 'pending')
  const distributedTasks = tasks.filter(t => t.status === 'running' || t.status === 'completed')

  return (
    <div className="h-full p-6 space-y-6 overflow-y-auto">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-xl font-bold">Task Distribution</h2>
          <p className="text-gray-600">Automatically distribute complex tasks across multiple AI tools</p>
        </div>

        <Button onClick={() => setShowCreateDialog(true)}>
          <Plus className="w-4 h-4 mr-2" />
          Create Task
        </Button>
      </div>

      {/* Distribution Stats */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <Card className="p-4">
          <div className="flex items-center gap-3">
            <Target className="w-8 h-8 text-blue-500" />
            <div>
              <div className="text-2xl font-bold">{pendingTasks.length}</div>
              <div className="text-sm text-gray-600">Pending Tasks</div>
            </div>
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center gap-3">
            <GitBranch className="w-8 h-8 text-green-500" />
            <div>
              <div className="text-2xl font-bold">{distributedTasks.length}</div>
              <div className="text-sm text-gray-600">Distributed</div>
            </div>
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center gap-3">
            <Users className="w-8 h-8 text-purple-500" />
            <div>
              <div className="text-2xl font-bold">{aiTools.length}</div>
              <div className="text-sm text-gray-600">Available Tools</div>
            </div>
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center gap-3">
            <CheckCircle className="w-8 h-8 text-orange-500" />
            <div>
              <div className="text-2xl font-bold">
                {Math.round(tasks.reduce((sum, t) => sum + t.subtasks.length, 0) / Math.max(tasks.length, 1))}
              </div>
              <div className="text-sm text-gray-600">Avg Subtasks</div>
            </div>
          </div>
        </Card>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Task Distribution Overview */}
        <Card className="p-6">
          <h2 className="text-xl font-bold mb-4">Task Distribution Overview</h2>

          <div className="space-y-4">
            {tasks.map((task) => (
              <div key={task.id} className="p-4 border rounded-lg">
                <div className="flex items-center justify-between mb-3">
                  <div className="flex items-center gap-2">
                    <h3 className="font-semibold">{task.title}</h3>
                    <Badge className={getComplexityColor(task.complexity)}>
                      {task.complexity}
                    </Badge>
                    <Badge className={getPriorityColor(task.priority)}>
                      {task.priority}
                    </Badge>
                  </div>

                  <Badge variant="outline">
                    {task.status}
                  </Badge>
                </div>

                <p className="text-sm text-gray-600 mb-3">{task.description}</p>

                <div className="grid grid-cols-2 gap-4 text-sm">
                  <div>
                    <div className="text-gray-600">Assigned Tools</div>
                    <div className="flex gap-1 mt-1">
                      {task.assignedTools.map((toolId) => (
                        <Badge key={toolId} variant="secondary" className="text-xs">
                          {aiTools.find(t => t.id === toolId)?.name || toolId}
                        </Badge>
                      ))}
                    </div>
                  </div>

                  <div>
                    <div className="text-gray-600">Subtasks</div>
                    <div className="text-lg font-semibold mt-1">{task.subtasks.length}</div>
                  </div>
                </div>

                {/* Subtask Details */}
                <div className="mt-4 space-y-2">
                  {task.subtasks.map((subtask) => (
                    <div key={subtask.id} className="flex items-center justify-between text-sm p-2 bg-gray-50 rounded">
                      <div className="flex-1">
                        <div className="font-medium">{subtask.description}</div>
                        <div className="text-gray-600 text-xs">
                          Tool: {aiTools.find(t => t.id === subtask.assignedTool)?.name || subtask.assignedTool}
                        </div>
                      </div>
                      <Badge
                        variant={
                          subtask.status === 'completed' ? 'secondary' :
                          subtask.status === 'running' ? 'default' : 'outline'
                        }
                        className="text-xs"
                      >
                        {subtask.status}
                      </Badge>
                    </div>
                  ))}
                </div>
              </div>
            ))}

            {tasks.length === 0 && (
              <div className="text-center py-8 text-gray-500">
                <Target className="w-12 h-12 text-gray-400 mx-auto mb-4" />
                <p>No tasks distributed yet</p>
                <p className="text-sm">Create a task to see distribution details</p>
              </div>
            )}
          </div>
        </Card>

        {/* Tool Capabilities Matrix */}
        <Card className="p-6">
          <h2 className="text-xl font-bold mb-4">Tool Capabilities Matrix</h2>

          <div className="space-y-4">
            {aiTools.map((tool) => (
              <div key={tool.id} className="border rounded-lg p-4">
                <div className="flex items-center justify-between mb-3">
                  <div className="flex items-center gap-3">
                    <div className={`w-3 h-3 rounded-full ${
                      tool.status === 'available' ? 'bg-green-500' :
                      tool.status === 'running' ? 'bg-blue-500' :
                      tool.status === 'busy' ? 'bg-yellow-500' : 'bg-red-500'
                    }`} />
                    <h3 className="font-semibold">{tool.name}</h3>
                  </div>

                  <div className="text-sm text-gray-600">
                    {tool.activeSessions}/{tool.maxSessions} active
                  </div>
                </div>

                <div className="mb-3">
                  <div className="text-sm text-gray-600 mb-2">Capabilities</div>
                  <div className="flex flex-wrap gap-1">
                    {tool.capabilities.map((cap, index) => (
                      <Badge key={index} variant="outline" className="text-xs">
                        {cap}
                      </Badge>
                    ))}
                  </div>
                </div>

                <div className="grid grid-cols-3 gap-4 text-sm">
                  <div>
                    <div className="text-gray-600">Response Time</div>
                    <div className="font-semibold">{tool.performance.avgResponseTime}s</div>
                  </div>
                  <div>
                    <div className="text-gray-600">Success Rate</div>
                    <div className="font-semibold">{tool.performance.successRate}%</div>
                  </div>
                  <div>
                    <div className="text-gray-600">Resource Usage</div>
                    <div className="font-semibold">{tool.performance.resourceUsage}%</div>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </Card>
      </div>

      {/* Create Task Dialog */}
      {showCreateDialog && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <Card className="w-full max-w-2xl mx-4 max-h-[90vh] overflow-y-auto">
            <div className="p-6">
              <h2 className="text-xl font-bold mb-6">Create Development Task</h2>

              <div className="space-y-6">
                {/* Basic Information */}
                <div>
                  <label className="block text-sm font-medium mb-2">Task Title</label>
                  <input
                    type="text"
                    value={newTask.title}
                    onChange={(e) => setNewTask(prev => ({ ...prev, title: e.target.value }))}
                    className="w-full px-3 py-2 border rounded"
                    placeholder="Enter task title..."
                  />
                </div>

                <div>
                  <label className="block text-sm font-medium mb-2">Description</label>
                  <textarea
                    value={newTask.description}
                    onChange={(e) => setNewTask(prev => ({ ...prev, description: e.target.value }))}
                    className="w-full px-3 py-2 border rounded h-24 resize-none"
                    placeholder="Describe what needs to be accomplished..."
                  />
                </div>

                {/* Complexity and Priority */}
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <label className="block text-sm font-medium mb-2">Complexity</label>
                    <select
                      value={newTask.complexity}
                      onChange={(e) => setNewTask(prev => ({ ...prev, complexity: e.target.value as DevelopmentTask['complexity'] }))}
                      className="w-full px-3 py-2 border rounded"
                    >
                      <option value="simple">Simple (1 subtask)</option>
                      <option value="medium">Medium (2 subtasks)</option>
                      <option value="complex">Complex (3 subtasks)</option>
                      <option value="critical">Critical (4 subtasks)</option>
                    </select>
                  </div>

                  <div>
                    <label className="block text-sm font-medium mb-2">Priority</label>
                    <select
                      value={newTask.priority}
                      onChange={(e) => setNewTask(prev => ({ ...prev, priority: e.target.value as DevelopmentTask['priority'] }))}
                      className="w-full px-3 py-2 border rounded"
                    >
                      <option value="low">Low</option>
                      <option value="medium">Medium</option>
                      <option value="high">High</option>
                      <option value="critical">Critical</option>
                    </select>
                  </div>
                </div>

                {/* Requirements */}
                <div>
                  <label className="block text-sm font-medium mb-3">Requirements</label>
                  <div className="grid grid-cols-2 md:grid-cols-4 gap-2">
                    {availableRequirements.map((req) => (
                      <label key={req} className="flex items-center gap-2 text-sm">
                        <input
                          type="checkbox"
                          checked={newTask.requirements.includes(req)}
                          onChange={() => handleRequirementToggle(req)}
                          className="rounded"
                        />
                        {req}
                      </label>
                    ))}
                  </div>
                </div>

                {/* Tool Selection */}
                <div>
                  <label className="block text-sm font-medium mb-3">AI Tools to Use</label>
                  <div className="space-y-2">
                    {aiTools.map((tool) => (
                      <label key={tool.id} className="flex items-center gap-3 p-3 border rounded hover:bg-gray-50">
                        <input
                          type="checkbox"
                          checked={newTask.selectedTools.includes(tool.id)}
                          onChange={() => handleToolToggle(tool.id)}
                          className="rounded"
                        />
                        <div className="flex items-center gap-3 flex-1">
                          <div className={`w-3 h-3 rounded-full ${
                            tool.status === 'available' ? 'bg-green-500' :
                            tool.status === 'running' ? 'bg-blue-500' :
                            tool.status === 'busy' ? 'bg-yellow-500' : 'bg-red-500'
                          }`} />
                          <div>
                            <div className="font-medium">{tool.name}</div>
                            <div className="text-xs text-gray-600">
                              {tool.capabilities.slice(0, 3).join(', ')}
                              {tool.capabilities.length > 3 && ` +${tool.capabilities.length - 3} more`}
                            </div>
                          </div>
                        </div>
                        <div className="text-xs text-gray-500">
                          {tool.performance.successRate}% success
                        </div>
                      </label>
                    ))}
                  </div>
                </div>

                {/* Preview */}
                {newTask.title && newTask.description && (
                  <div className="p-4 bg-gray-50 rounded">
                    <h4 className="font-medium mb-2">Task Preview</h4>
                    <div className="text-sm space-y-1">
                      <div><strong>Title:</strong> {newTask.title}</div>
                      <div><strong>Complexity:</strong> {newTask.complexity} ({getSubtaskCount(newTask.complexity)} subtasks)</div>
                      <div><strong>Tools:</strong> {newTask.selectedTools.length} selected</div>
                      <div><strong>Requirements:</strong> {newTask.requirements.join(', ')}</div>
                    </div>
                  </div>
                )}
              </div>

              {/* Actions */}
              <div className="flex justify-end gap-3 mt-6 pt-4 border-t">
                <Button variant="outline" onClick={() => setShowCreateDialog(false)}>
                  Cancel
                </Button>
                <Button
                  onClick={handleCreateTask}
                  disabled={!newTask.title.trim() || !newTask.description.trim() || newTask.selectedTools.length === 0}
                >
                  Create Task
                </Button>
              </div>
            </div>
          </Card>
        </div>
      )}
    </div>
  )
}
