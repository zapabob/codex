export type TaskStatus = 'todo' | 'in-progress' | 'review' | 'done'

export type TaskPriority = 'urgent' | 'high' | 'medium' | 'low'

export interface Task {
  id: string
  title: string
  description?: string
  status: TaskStatus
  priority: TaskPriority
  assignee?: string
  dueDate?: Date
  startDate?: Date
  estimatedHours?: number
  actualHours?: number
  progress: number // 0-100
  tags: string[]
  subtasks: { id: string, title: string, completed: boolean }[]
  parentTaskId?: string
}

export interface TaskColumn {
  id: TaskStatus
  title: string
  tasks: Task[]
}
