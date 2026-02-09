export interface AIToolPerformance {
  avgResponseTime: number
  successRate: number
  resourceUsage: number
}

export interface AITool {
  id: string
  name: string
  status: 'available' | 'running' | 'busy' | 'error'
  capabilities: string[]
  activeSessions: number
  maxSessions: number
  performance: AIToolPerformance
}

export interface AISession {
  id: string
  toolId: string
  taskId: string
  status: 'starting' | 'running' | 'completed' | 'failed' | 'cancelled'
  startTime: Date
  endTime?: Date
  progress: number
  output: string
  error?: string
}

export interface SubTask {
  id: string
  parentTaskId: string
  description: string
  assignedTool: string
  status: 'pending' | 'running' | 'completed' | 'failed'
  output?: string
  error?: string
}

export interface DevelopmentTask {
  id: string
  title: string
  description: string
  complexity: 'simple' | 'medium' | 'complex' | 'critical'
  priority: 'low' | 'medium' | 'high' | 'critical'
  requirements: string[]
  subtasks: SubTask[]
  status: 'pending' | 'running' | 'completed' | 'failed'
  createdAt: Date
  assignedTools: string[]
  progress: number
}

export interface SubTaskResult {
  subtaskId: string
  toolId: string
  success: boolean
  output: string
  error?: string
  executionTime: number
  qualityScore: number
}

export interface ExecutionResult {
  taskId: string
  success: boolean
  integratedOutput: string
  subtaskResults: SubTaskResult[]
  errors: string[]
  executionTime: number
  qualityScore: number
  recommendations: string[]
}
