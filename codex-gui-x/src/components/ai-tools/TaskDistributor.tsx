'use client'

import { useState } from 'react'
let taskDistributionCounter = 0;
let subtaskDistributionCounter = 0;
import { Card } from '../atoms/Card'
import { Badge } from '../atoms/Badge'
import type { AITool, DevelopmentTask, SubTask } from '../../types/ai-tools'
import {
  Plus,
  Target,
  GitBranch,
  CheckCircle,
  Users,
  Zap,
  ArrowRight
} from 'lucide-react'

interface TaskDistributorProps {
  tasks: DevelopmentTask[]
  aiTools: AITool[]
  onTaskCreate: (task: DevelopmentTask) => void
  onTaskUpdate: (task: DevelopmentTask) => void
}

export function TaskDistributor({ tasks, aiTools, onTaskCreate }: TaskDistributorProps) {
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

    // Generate unique ID for the task using a counter to avoid lint errors
    taskDistributionCounter += 1;
    const taskId = `task-dist-${taskDistributionCounter}`
    
    const task: DevelopmentTask = {
      id: taskId,
      title: newTask.title,
      description: newTask.description,
      complexity: newTask.complexity,
      priority: newTask.priority,
      requirements: newTask.requirements,
      subtasks: generateSubtasks(newTask, taskId),
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

  const generateSubtasks = (taskData: typeof newTask, parentId: string): SubTask[] => {
    const subtasks: SubTask[] = []
    const subtaskCount = getSubtaskCount(taskData.complexity)
    const timestamp = new Date().getTime()

    for (let i = 0; i < subtaskCount; i++) {
      subtaskDistributionCounter += 1;
      subtasks.push({
        id: `subtask-dist-${subtaskDistributionCounter}`,
        parentTaskId: parentId,
        description: generateSubtaskDescription(taskData, i),
        assignedTool: assignToolForSubtask(taskData.requirements, i),
        status: 'pending',
      })
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

  const generateSubtaskDescription = (taskData: typeof newTask, index: number): string => {
    const baseDescription = taskData.description
    switch (taskData.complexity) {
      case 'simple': return baseDescription
      case 'medium': return index === 0 ? `Core: ${baseDescription}` : `QA: ${baseDescription}`
      case 'complex': return [`Design: ${baseDescription}`, `Build: ${baseDescription}`, `Test: ${baseDescription}`][index] || baseDescription
      case 'critical': return [`Research: ${baseDescription}`, `Arch: ${baseDescription}`, `Prod: ${baseDescription}`, `Verify: ${baseDescription}`][index] || baseDescription
      default: return baseDescription
    }
  }

  const assignToolForSubtask = (requirements: string[], index: number): string => {
    const availableTools = aiTools.map(t => t.id)
    if (requirements.includes('Code Generation') && index === 0) return availableTools.find(t => t.includes('codex')) || availableTools[0]
    return availableTools[index % (availableTools.length || 1)] || 'default-tool'
  }



  const handleToolToggle = (toolId: string) => {
    setNewTask(prev => ({
      ...prev,
      selectedTools: prev.selectedTools.includes(toolId)
        ? prev.selectedTools.filter(t => t !== toolId)
        : [...prev.selectedTools, toolId]
    }))
  }

  const getStatusBadgeColor = (status: string) => {
    switch (status) {
      case 'completed': return 'bg-green-500/20 text-green-400 border-green-500/30'
      case 'running': return 'bg-primary/20 text-primary border-primary/30'
      case 'failed': return 'bg-destructive/20 text-destructive border-destructive/30'
      default: return 'bg-white/5 text-muted-foreground border-white/10'
    }
  }

  const pendingTasks = tasks.filter(t => t.status === 'pending')
  const distributedTasks = tasks.filter(t => t.status === 'running' || t.status === 'completed')

  return (
    <div className="h-full p-6 space-y-6 overflow-y-auto custom-scrollbar">
      <div className="flex items-center justify-between">
        <div className="space-y-1">
          <h2 className="text-2xl font-bold tracking-tight">Intelligence Distribution</h2>
          <p className="text-sm text-muted-foreground">Manage parallel task allocation across the AI cluster</p>
        </div>
        <button 
          onClick={() => setShowCreateDialog(true)}
          className="h-10 px-5 rounded-xl bg-primary text-primary-foreground font-bold text-xs flex items-center gap-2 hover:scale-105 active:scale-95 transition-all shadow-lg shadow-primary/20"
        >
          <Plus size={16} />
          CREATE TASK
        </button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        {[
          { label: 'Pending', value: pendingTasks.length, icon: Target, color: 'text-blue-400' },
          { label: 'Distributed', value: distributedTasks.length, icon: GitBranch, color: 'text-green-400' },
          { label: 'Tools Available', value: aiTools.length, icon: Users, color: 'text-purple-400' },
          { label: 'Avg Subtasks', value: tasks.length > 0 ? Math.round(tasks.reduce((sum, t) => sum + t.subtasks.length, 0) / tasks.length) : 0, icon: Zap, color: 'text-amber-400' }
        ].map((stat, i) => (
          <Card key={i} className="p-4 bg-white/5 border-white/5">
            <div className="flex items-center gap-3">
              <stat.icon className={`w-8 h-8 ${stat.color}`} />
              <div>
                <div className="text-2xl font-bold tracking-tight">{stat.value}</div>
                <div className="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">{stat.label}</div>
              </div>
            </div>
          </Card>
        ))}
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <Card className="p-6 bg-white/5 border-white/5">
          <h2 className="text-lg font-bold mb-6 flex items-center gap-2">
            <Target size={18} className="text-primary" />
            Active Task Distribution
          </h2>
          <div className="space-y-4">
            {tasks.map((task) => (
              <div key={task.id} className="p-4 border border-white/5 bg-white/2 rounded-2xl group hover:border-primary/30 transition-all">
                <div className="flex items-center justify-between mb-4">
                  <div className="space-y-1">
                    <h3 className="font-bold text-sm group-hover:text-primary transition-colors">{task.title}</h3>
                    <div className="flex gap-2">
                      <span className="text-[9px] font-bold uppercase tracking-tight text-muted-foreground">{task.complexity} complexity</span>
                      <span className="text-[9px] font-bold uppercase tracking-tight text-muted-foreground">•</span>
                      <span className="text-[9px] font-bold uppercase tracking-tight text-muted-foreground">{task.priority} priority</span>
                    </div>
                  </div>
                  <Badge variant="outline" className={getStatusBadgeColor(task.status)}>
                    {task.status.toUpperCase()}
                  </Badge>
                </div>

                <div className="space-y-2 mb-4">
                  {task.subtasks.map((subtask) => (
                    <div key={subtask.id} className="flex items-center justify-between p-2.5 bg-black/20 rounded-xl border border-white/5 text-[11px]">
                      <div className="flex items-center gap-3">
                        <div className="text-primary font-bold"># {subtask.assignedTool}</div>
                        <div className="text-muted-foreground line-clamp-1">{subtask.description}</div>
                      </div>
                      <div className="text-[10px] font-mono opacity-60 italic">{subtask.status}</div>
                    </div>
                  ))}
                </div>

                <div className="flex items-center justify-between text-[10px] font-bold uppercase tracking-widest text-muted-foreground/60 pt-3 border-t border-white/5">
                  <span>Assigned: {task.assignedTools.length} Agents</span>
                  <span className="flex items-center gap-1.5 text-primary">
                    In Progress <ArrowRight size={12} />
                  </span>
                </div>
              </div>
            ))}
            {tasks.length === 0 && (
              <div className="text-center py-12 text-muted-foreground/20">
                <Target size={48} className="mx-auto mb-4 opacity-10" />
                <p className="text-xs font-bold uppercase tracking-widest">Pipeline Empty</p>
              </div>
            )}
          </div>
        </Card>

        <Card className="p-6 bg-white/5 border-white/5">
          <h2 className="text-lg font-bold mb-6 flex items-center gap-2">
            <Users size={18} className="text-purple-400" />
            Agent Matrix
          </h2>
          <div className="space-y-3">
            {aiTools.map((tool) => (
              <div key={tool.id} className="p-4 border border-white/5 bg-white/2 rounded-2xl flex items-center justify-between group hover:bg-white/5 transition-all">
                <div className="flex items-center gap-4">
                  <div className={`w-2.5 h-2.5 rounded-full ${
                    tool.status === 'available' ? 'bg-green-500 shadow-lg shadow-green-500/20' : 'bg-primary shadow-lg shadow-primary/20'
                  }`} />
                  <div>
                    <div className="font-bold text-sm tracking-tight">{tool.name}</div>
                    <div className="text-[10px] text-muted-foreground mt-0.5">{tool.activeSessions}/{tool.maxSessions} LOAD</div>
                  </div>
                </div>
                <div className="text-right space-y-1">
                  <div className="text-xs font-bold text-primary">{tool.performance.successRate}% <span className="text-muted-foreground/40 font-normal">SR</span></div>
                  <div className="text-[9px] text-muted-foreground/60 tabular-nums uppercase font-bold tracking-tighter">Lat: {tool.performance.avgResponseTime}s</div>
                </div>
              </div>
            ))}
          </div>
        </Card>
      </div>

      {showCreateDialog && (
        <div className="fixed inset-0 bg-black/80 backdrop-blur-xl flex items-center justify-center z-[100] p-6">
          <Card className="w-full max-w-2xl bg-card border-white/10 shadow-2xl overflow-hidden animate-in fade-in zoom-in duration-300">
            <div className="p-8 space-y-8">
              <div className="flex items-center justify-between">
                <h2 className="text-2xl font-bold tracking-tighter">New Intelligence Directive</h2>
                <button onClick={() => setShowCreateDialog(false)} className="text-muted-foreground hover:text-white">&times;</button>
              </div>

              <div className="space-y-6">
                <div className="space-y-2">
                  <label className="text-[10px] font-bold uppercase tracking-widest text-muted-foreground ml-1">Task Specification</label>
                  <input
                    type="text"
                    value={newTask.title}
                    onChange={(e) => setNewTask(prev => ({ ...prev, title: e.target.value }))}
                    className="w-full h-12 px-4 bg-white/5 border border-white/10 rounded-xl focus:ring-2 focus:ring-primary focus:border-transparent transition-all outline-none text-sm"
                    placeholder="Briefly name the directive..."
                  />
                  <textarea
                    value={newTask.description}
                    onChange={(e) => setNewTask(prev => ({ ...prev, description: e.target.value }))}
                    className="w-full h-32 p-4 bg-white/5 border border-white/10 rounded-xl focus:ring-2 focus:ring-primary focus:border-transparent transition-all outline-none resize-none text-sm"
                    placeholder="Provide detailed instructions for the agent cluster..."
                  />
                </div>

                <div className="grid grid-cols-2 gap-6">
                  <div className="space-y-2">
                    <label className="text-[10px] font-bold uppercase tracking-widest text-muted-foreground ml-1">Architectural Complexity</label>
                    <select
                      value={newTask.complexity}
                      onChange={(e) => setNewTask(prev => ({ ...prev, complexity: e.target.value as DevelopmentTask['complexity'] }))}
                      className="w-full h-12 px-4 bg-white/5 border border-white/10 rounded-xl outline-none text-sm"
                    >
                      <option value="simple">Simple Unit</option>
                      <option value="medium">Medium Subsystem</option>
                      <option value="complex">Complex Architecture</option>
                      <option value="critical">Critical Core Infrastructure</option>
                    </select>
                  </div>
                  <div className="space-y-2">
                    <label className="text-[10px] font-bold uppercase tracking-widest text-muted-foreground ml-1">Mission Priority</label>
                    <select
                      value={newTask.priority}
                      onChange={(e) => setNewTask(prev => ({ ...prev, priority: e.target.value as DevelopmentTask['priority'] }))}
                      className="w-full h-12 px-4 bg-white/5 border border-white/10 rounded-xl outline-none text-sm"
                    >
                      <option value="low">Standard</option>
                      <option value="medium">Elevated</option>
                      <option value="high">Mission Critical</option>
                      <option value="critical">Emergency / Hotfix</option>
                    </select>
                  </div>
                </div>

                <div className="space-y-3">
                  <label className="text-[10px] font-bold uppercase tracking-widest text-muted-foreground ml-1 text-primary">Assign Intelligence Units</label>
                  <div className="grid grid-cols-2 gap-3">
                    {aiTools.map((tool) => (
                      <button
                        key={tool.id}
                        onClick={() => handleToolToggle(tool.id)}
                        className={`flex items-center justify-between p-4 border rounded-2xl transition-all ${
                          newTask.selectedTools.includes(tool.id) 
                          ? 'bg-primary/20 border-primary text-primary' 
                          : 'bg-white/5 border-white/10 text-muted-foreground grayscale opacity-60'
                        }`}
                      >
                        <span className="text-xs font-bold tracking-tight">{tool.name}</span>
                        {newTask.selectedTools.includes(tool.id) && <CheckCircle size={14} fill="currentColor" />}
                      </button>
                    ))}
                  </div>
                </div>

                <div className="pt-6 flex gap-4">
                  <button 
                    onClick={() => setShowCreateDialog(false)}
                    className="flex-1 h-12 rounded-xl border border-white/10 font-bold text-xs uppercase tracking-widest hover:bg-white/5 transition-all"
                  >
                    ABORT
                  </button>
                  <button 
                    onClick={handleCreateTask}
                    disabled={!newTask.title.trim() || newTask.selectedTools.length === 0}
                    className="flex-[2] h-12 rounded-xl bg-primary text-primary-foreground font-bold text-xs uppercase tracking-widest hover:scale-105 active:scale-95 transition-all disabled:opacity-50"
                  >
                    EXECUTE DIRECTIVE
                  </button>
                </div>
              </div>
            </div>
          </Card>
        </div>
      )}
    </div>
  )
}
