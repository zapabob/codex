import type { Task, TaskPriority } from '../../types/tasks'
import { Card } from '../atoms/Card'
import { Badge } from '../atoms/Badge'
import { Progress } from '../atoms/Progress'
import { Calendar, Clock, Tag, MoreHorizontal } from 'lucide-react'

interface KanbanCardProps {
  task: Task
}

export function KanbanCard({ task }: KanbanCardProps) {
  const getPriorityColor = (priority: TaskPriority) => {
    switch (priority) {
      case 'urgent': return 'error'
      case 'high': return 'warning'
      case 'medium': return 'primary'
      case 'low': return 'success'
      default: return 'secondary'
    }
  }

  return (
    <Card animated hover className="group">
      <div className="p-4 space-y-4">
        <div className="flex items-start justify-between gap-2">
            <h3 className="font-bold text-sm leading-tight group-hover:text-primary transition-colors tracking-tight">
                {task.title}
            </h3>
            <button className="text-muted-foreground hover:text-foreground opacity-0 group-hover:opacity-100 transition-opacity">
                <MoreHorizontal size={14} />
            </button>
        </div>

        {task.description && (
          <p className="text-[11px] text-muted-foreground line-clamp-2 leading-relaxed">
            {task.description}
          </p>
        )}

        <div className="flex flex-wrap gap-1.5">
          {task.tags.map((tag, idx) => (
            <div key={idx} className="flex items-center gap-1 px-2 py-0.5 rounded-full bg-muted/50 border border-border text-[9px] font-bold text-muted-foreground uppercase">
                <Tag size={8} /> {tag}
            </div>
          ))}
        </div>

        <div className="flex items-center justify-between">
            <Badge color={getPriorityColor(task.priority)} size="sm" className="font-black text-[9px]">{task.priority.toUpperCase()}</Badge>
            <div className="flex items-center -space-x-2">
                <div className="h-6 w-6 rounded-lg bg-indigo-500 flex items-center justify-center text-[10px] font-bold text-white border-2 border-card ring-1 ring-border">
                    {task.assignee?.[0] || 'A'}
                </div>
            </div>
        </div>

        <div className="space-y-1.5 pt-2 border-t border-border/50">
            <div className="flex justify-between text-[9px] font-bold uppercase text-muted-foreground">
                <span>Phase Progress</span>
                <span>{task.progress}%</span>
            </div>
            <Progress value={task.progress} className="h-1.5" />
        </div>

        <div className="flex items-center gap-4 text-[10px] text-muted-foreground font-bold uppercase tracking-wider">
            {task.dueDate && (
                <div className="flex items-center gap-1"><Calendar size={12} className="text-rose-500" /> {task.dueDate.toLocaleDateString()}</div>
            )}
            {task.estimatedHours && (
                <div className="flex items-center gap-1"><Clock size={12} className="text-sky-500" /> {task.estimatedHours}h</div>
            )}
        </div>
      </div>
    </Card>
  )
}
