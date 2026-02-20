'use client'

import { useState, useEffect } from 'react'
import {
  DndContext,
  DragEndEvent,
  DragOverEvent,
  DragOverlay,
  DragStartEvent,
  PointerSensor,
  useSensor,
  useSensors,
} from '@dnd-kit/core'
import {
  
  arrayMove,
  
} from '@dnd-kit/sortable'
import { createPortal } from 'react-dom'

import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/atoms/Button'
import { KanbanBoard } from '@/components/kanban/KanbanBoard'
import { GanttChart } from '@/components/gantt/GanttChart'
import { MermaidDiagram } from '@/components/mermaid/MermaidDiagram'
import { DashboardLayout } from '@/components/templates/DashboardLayout'

// Task status types
export type TaskStatus = 'todo' | 'in-progress' | 'review' | 'done'

// Task priority types
export type TaskPriority = 'low' | 'medium' | 'high' | 'urgent'

// Task interface
export interface Task {
  id: string
  title: string
  description?: string
  status: TaskStatus
  priority: TaskPriority
  assignee?: string
  tags: string[]
  createdAt: Date
  updatedAt: Date
  dueDate?: Date
  estimatedHours?: number
  actualHours?: number
  dependencies: string[] // Task IDs this task depends on
  subtasks: string[] // Subtask IDs
}

// Task column interface
export interface TaskColumn {
  id: TaskStatus
  title: string
  tasks: Task[]
}

export default function TasksPage() {
  const [activeView, setActiveView] = useState<'kanban' | 'gantt' | 'mermaid'>('kanban')
  const [tasks, setTasks] = useState<Task[]>([])
  const [activeTask, setActiveTask] = useState<Task | null>(null)
  const [activeId, setActiveId] = useState<string | null>(null)

  // DnD sensors
  const sensors = useSensors(
    useSensor(PointerSensor, {
      activationConstraint: {
        distance: 3,
      },
    })
  )

  // Initialize sample tasks
  useEffect(() => {
    const sampleTasks: Task[] = [
      {
        id: '1',
        title: 'Implement Git Lock Manager',
        description: 'Create fine-grained locking system for parallel development',
        status: 'done',
        priority: 'high',
        assignee: 'developer',
        tags: ['rust', 'concurrency', 'git'],
        createdAt: new Date('2025-12-01'),
        updatedAt: new Date('2025-12-08'),
        dueDate: new Date('2025-12-10'),
        estimatedHours: 16,
        actualHours: 18,
        dependencies: [],
        subtasks: ['1.1', '1.2', '1.3'],
      },
      {
        id: '2',
        title: 'Add Conflict Detection',
        description: 'Implement AST-based conflict detection and resolution',
        status: 'in-progress',
        priority: 'high',
        assignee: 'developer',
        tags: ['rust', 'ast', 'ml'],
        createdAt: new Date('2025-12-02'),
        updatedAt: new Date('2025-12-08'),
        dueDate: new Date('2025-12-15'),
        estimatedHours: 12,
        actualHours: 8,
        dependencies: ['1'],
        subtasks: [],
      },
      {
        id: '3',
        title: 'Create Kanban Board UI',
        description: 'Build drag-and-drop kanban board component',
        status: 'todo',
        priority: 'medium',
        assignee: 'frontend',
        tags: ['react', 'dnd', 'ui'],
        createdAt: new Date('2025-12-08'),
        updatedAt: new Date('2025-12-08'),
        dueDate: new Date('2025-12-20'),
        estimatedHours: 8,
        actualHours: 0,
        dependencies: [],
        subtasks: [],
      },
      {
        id: '4',
        title: 'Implement Gantt Chart',
        description: 'Create timeline visualization with dependencies',
        status: 'todo',
        priority: 'medium',
        assignee: 'frontend',
        tags: ['react', 'chart', 'timeline'],
        createdAt: new Date('2025-12-08'),
        updatedAt: new Date('2025-12-08'),
        dueDate: new Date('2025-12-25'),
        estimatedHours: 10,
        actualHours: 0,
        dependencies: ['3'],
        subtasks: [],
      },
      {
        id: '5',
        title: 'Integrate Mermaid Diagrams',
        description: 'Add flowchart and diagram generation capabilities',
        status: 'todo',
        priority: 'low',
        assignee: 'frontend',
        tags: ['mermaid', 'diagrams', 'visualization'],
        createdAt: new Date('2025-12-08'),
        updatedAt: new Date('2025-12-08'),
        dueDate: new Date('2025-12-30'),
        estimatedHours: 6,
        actualHours: 0,
        dependencies: ['3'],
        subtasks: [],
      },
    ]
    setTasks(sampleTasks)
  }, [])

  // Handle drag start
  const handleDragStart = (event: DragStartEvent) => {
    const { active } = event
    setActiveId(active.id as string)

    const task = tasks.find(t => t.id === active.id)
    setActiveTask(task || null)
  }

  // Handle drag over
  const handleDragOver = (event: DragOverEvent) => {
    const { active, over } = event

    if (!over) return

    const activeId = active.id as string
    const overId = over.id as string

    // Find the containers
    const activeTask = tasks.find(t => t.id === activeId)
    const overTask = tasks.find(t => t.id === overId)

    if (!activeTask) return

    // If dropping on a task in a different column
    if (overTask && activeTask.status !== overTask.status) {
      setTasks(tasks => {
        const activeIndex = tasks.findIndex(t => t.id === activeId)
        const overIndex = tasks.findIndex(t => t.id === overId)

        // Move to new status
        tasks[activeIndex].status = overTask.status as TaskStatus
        tasks[activeIndex].updatedAt = new Date()

        return arrayMove(tasks, activeIndex, overIndex)
      })
    }
  }

  // Handle drag end
  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over } = event
    setActiveId(null)
    setActiveTask(null)

    if (!over) return

    const activeId = active.id as string
    const overId = over.id as string

    // Find tasks
    const activeTask = tasks.find(t => t.id === activeId)
    const overTask = tasks.find(t => t.id === overId)

    if (!activeTask) return

    // If dropping on a column
    if (!overTask && overId.includes('column-')) {
      const newStatus = overId.replace('column-', '') as TaskStatus
      if (activeTask.status !== newStatus) {
        setTasks(tasks =>
          tasks.map(task =>
            task.id === activeId
              ? { ...task, status: newStatus, updatedAt: new Date() }
              : task
          )
        )
      }
    }
  }

  // Create columns from tasks
  const columns: TaskColumn[] = [
    {
      id: 'todo',
      title: 'To Do',
      tasks: tasks.filter(task => task.status === 'todo'),
    },
    {
      id: 'in-progress',
      title: 'In Progress',
      tasks: tasks.filter(task => task.status === 'in-progress'),
    },
    {
      id: 'review',
      title: 'Review',
      tasks: tasks.filter(task => task.status === 'review'),
    },
    {
      id: 'done',
      title: 'Done',
      tasks: tasks.filter(task => task.status === 'done'),
    },
  ]

  return (
    <DashboardLayout>
      <div className="h-full flex flex-col">
        {/* Header */}
        <div className="flex items-center justify-between p-6 border-b">
          <div>
            <h1 className="text-3xl font-bold text-gray-900">Task Management</h1>
            <p className="text-gray-600 mt-1">
              Kanban board, Gantt charts, and Mermaid diagrams for project management
            </p>
          </div>

          {/* View Toggle */}
          <div className="flex gap-2">
            <Button
              variant={activeView === 'kanban' ? 'primary' : 'secondary'}
              onClick={() => setActiveView('kanban')}
            >
              Kanban
            </Button>
            <Button
              variant={activeView === 'gantt' ? 'primary' : 'secondary'}
              onClick={() => setActiveView('gantt')}
            >
              Gantt Chart
            </Button>
            <Button
              variant={activeView === 'mermaid' ? 'primary' : 'secondary'}
              onClick={() => setActiveView('mermaid')}
            >
              Mermaid
            </Button>
          </div>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-hidden">
          <DndContext
            sensors={sensors}
            onDragStart={handleDragStart}
            onDragOver={handleDragOver}
            onDragEnd={handleDragEnd}
          >
            {activeView === 'kanban' && (
              <KanbanBoard columns={columns} />
            )}

            {activeView === 'gantt' && (
              <GanttChart tasks={tasks} />
            )}

            {activeView === 'mermaid' && (
              <MermaidDiagram tasks={tasks} />
            )}

            {createPortal(
              <DragOverlay>
                {activeTask ? (
                  <Card className="p-4 bg-white shadow-lg border-2 border-blue-500">
                    <div className="flex items-start gap-3">
                      <div className="flex-1">
                        <h3 className="font-semibold text-gray-900">
                          {activeTask.title}
                        </h3>
                        {activeTask.description && (
                          <p className="text-sm text-gray-600 mt-1">
                            {activeTask.description}
                          </p>
                        )}
                        <div className="flex items-center gap-2 mt-2">
                          <Badge
                            variant={
                              activeTask.priority === 'urgent' ? 'destructive' :
                              activeTask.priority === 'high' ? 'default' :
                              activeTask.priority === 'medium' ? 'secondary' : 'outline'
                            }
                          >
                            {activeTask.priority}
                          </Badge>
                          {activeTask.assignee && (
                            <span className="text-sm text-gray-500">
                              @{activeTask.assignee}
                            </span>
                          )}
                        </div>
                      </div>
                    </div>
                  </Card>
                ) : null}
              </DragOverlay>,
              document.body
            )}
          </DndContext>
        </div>
      </div>
    </DashboardLayout>
  )
}
