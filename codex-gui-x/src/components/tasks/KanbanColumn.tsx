import type { TaskColumn, TaskStatus } from '../../types/tasks'
import { KanbanCard } from './KanbanCard'
import { Badge } from '../atoms/Badge'

interface KanbanColumnProps {
  column: TaskColumn
}

export function KanbanColumn({ column }: KanbanColumnProps) {
  const getColumnColor = (status: TaskStatus) => {
    switch (status) {
      case 'todo': return 'border-muted-foreground/20'
      case 'in-progress': return 'border-indigo-500/30 bg-indigo-500/5'
      case 'review': return 'border-amber-500/30 bg-amber-500/5'
      case 'done': return 'border-emerald-500/30 bg-emerald-500/5'
      default: return 'border-border'
    }
  }

  const getHeaderBadgeColor = (status: TaskStatus) => {
    switch (status) {
      case 'todo': return 'secondary'
      case 'in-progress': return 'primary'
      case 'review': return 'warning'
      case 'done': return 'success'
      default: return 'secondary'
    }
  }

  return (
    <div className={`flex flex-col h-full min-h-[500px] border-2 rounded-3xl transition-all ${getColumnColor(column.id)} mb-8 backdrop-blur-sm`}>
      <div className="p-5 border-b border-border/50">
        <div className="flex items-center justify-between">
          <h2 className="font-black text-xs uppercase tracking-widest italic">{column.title}</h2>
          <Badge color={getHeaderBadgeColor(column.id)} size="sm">
            {column.tasks.length}
          </Badge>
        </div>
      </div>

      <div className="flex-1 p-4 space-y-4 overflow-y-auto custom-scrollbar">
        {column.tasks.map((task) => (
          <KanbanCard key={task.id} task={task} />
        ))}
        {column.tasks.length === 0 && (
          <div className="flex flex-col items-center justify-center h-32 border-2 border-dashed border-border/50 rounded-2xl opacity-40">
            <p className="text-[10px] font-bold uppercase tracking-tighter">Empty Sector</p>
          </div>
        )}
      </div>
    </div>
  )
}
