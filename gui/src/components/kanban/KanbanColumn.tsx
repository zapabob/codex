'use client'

import { useDroppable } from '@dnd-kit/core'
import {
  SortableContext,
  verticalListSortingStrategy,
} from '@dnd-kit/sortable'
import { TaskColumn, TaskStatus } from '@/app/tasks/page'
import { KanbanCard } from './KanbanCard'
import { Badge } from '@/components/ui/badge'

interface KanbanColumnProps {
  column: TaskColumn
}

export function KanbanColumn({ column }: KanbanColumnProps) {
  const { setNodeRef, isOver } = useDroppable({
    id: `column-${column.id}`,
  })

  const getColumnColor = (status: TaskStatus) => {
    switch (status) {
      case 'todo':
        return 'border-gray-300 bg-gray-50'
      case 'in-progress':
        return 'border-blue-300 bg-blue-50'
      case 'review':
        return 'border-yellow-300 bg-yellow-50'
      case 'done':
        return 'border-green-300 bg-green-50'
      default:
        return 'border-gray-300 bg-gray-50'
    }
  }

  const getColumnHeaderColor = (status: TaskStatus) => {
    switch (status) {
      case 'todo':
        return 'bg-gray-100 text-gray-800'
      case 'in-progress':
        return 'bg-blue-100 text-blue-800'
      case 'review':
        return 'bg-yellow-100 text-yellow-800'
      case 'done':
        return 'bg-green-100 text-green-800'
      default:
        return 'bg-gray-100 text-gray-800'
    }
  }

  return (
    <div
      ref={setNodeRef}
      className={`flex flex-col h-full border-2 rounded-lg transition-colors ${
        isOver ? 'border-blue-400 bg-blue-25' : getColumnColor(column.id)
      }`}
    >
      {/* Column Header */}
      <div className={`p-4 border-b rounded-t-lg ${getColumnHeaderColor(column.id)}`}>
        <div className="flex items-center justify-between">
          <h2 className="font-semibold text-lg">{column.title}</h2>
          <Badge variant="secondary" className="ml-2">
            {column.tasks.length}
          </Badge>
        </div>
      </div>

      {/* Column Content */}
      <div className="flex-1 p-4 space-y-3 overflow-y-auto">
        <SortableContext
          items={column.tasks.map(task => task.id)}
          strategy={verticalListSortingStrategy}
        >
          {column.tasks.map((task) => (
            <KanbanCard key={task.id} task={task} />
          ))}
        </SortableContext>

        {/* Empty state */}
        {column.tasks.length === 0 && (
          <div className="flex items-center justify-center h-32 border-2 border-dashed border-gray-300 rounded-lg">
            <p className="text-gray-500 text-sm">Drop tasks here</p>
          </div>
        )}
      </div>
    </div>
  )
}
