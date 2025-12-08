'use client'

import { useDroppable } from '@dnd-kit/core'
import {
  SortableContext,
  verticalListSortingStrategy,
} from '@dnd-kit/sortable'
import { Task, TaskColumn, TaskStatus } from '@/app/tasks/page'
import { KanbanColumn } from './KanbanColumn'
import { KanbanCard } from './KanbanCard'

interface KanbanBoardProps {
  columns: TaskColumn[]
}

export function KanbanBoard({ columns }: KanbanBoardProps) {
  return (
    <div className="h-full p-6">
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 h-full">
        {columns.map((column) => (
          <KanbanColumn
            key={column.id}
            column={column}
          />
        ))}
      </div>
    </div>
  )
}
