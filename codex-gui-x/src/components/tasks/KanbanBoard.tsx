import type { TaskColumn } from '../../types/tasks'
import { KanbanColumn } from './KanbanColumn'

interface KanbanBoardProps {
  columns: TaskColumn[]
}

export function KanbanBoard({ columns }: KanbanBoardProps) {
  return (
    <div className="h-full">
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 h-full items-start">
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
