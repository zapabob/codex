'use client'

import { useSortable } from '@dnd-kit/sortable'
import { CSS } from '@dnd-kit/utilities'
import { Task } from '@/app/tasks/page'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Calendar, Clock, User, Tag } from 'lucide-react'

interface KanbanCardProps {
  task: Task
}

export function KanbanCard({ task }: KanbanCardProps) {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({
    id: task.id,
  })

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
  }

  const getPriorityColor = (priority: Task['priority']) => {
    switch (priority) {
      case 'urgent':
        return 'bg-red-100 text-red-800 border-red-200'
      case 'high':
        return 'bg-orange-100 text-orange-800 border-orange-200'
      case 'medium':
        return 'bg-yellow-100 text-yellow-800 border-yellow-200'
      case 'low':
        return 'bg-green-100 text-green-800 border-green-200'
      default:
        return 'bg-gray-100 text-gray-800 border-gray-200'
    }
  }

  const formatDate = (date: Date) => {
    return new Intl.DateTimeFormat('ja-JP', {
      month: 'short',
      day: 'numeric',
    }).format(date)
  }

  return (
    <Card
      ref={setNodeRef}
      style={style}
      className={`p-4 cursor-grab active:cursor-grabbing transition-shadow hover:shadow-md ${
        isDragging ? 'opacity-50 shadow-lg' : ''
      }`}
      {...attributes}
      {...listeners}
    >
      <div className="space-y-3">
        {/* Title */}
        <h3 className="font-semibold text-gray-900 leading-tight">
          {task.title}
        </h3>

        {/* Description */}
        {task.description && (
          <p className="text-sm text-gray-600 line-clamp-2">
            {task.description}
          </p>
        )}

        {/* Tags */}
        {task.tags.length > 0 && (
          <div className="flex flex-wrap gap-1">
            {task.tags.slice(0, 3).map((tag, index) => (
              <Badge key={index} variant="outline" className="text-xs px-2 py-0">
                <Tag className="w-3 h-3 mr-1" />
                {tag}
              </Badge>
            ))}
            {task.tags.length > 3 && (
              <Badge variant="outline" className="text-xs px-2 py-0">
                +{task.tags.length - 3}
              </Badge>
            )}
          </div>
        )}

        {/* Priority */}
        <Badge className={`text-xs ${getPriorityColor(task.priority)}`}>
          {task.priority.toUpperCase()}
        </Badge>

        {/* Metadata */}
        <div className="flex items-center justify-between text-xs text-gray-500">
          <div className="flex items-center gap-3">
            {/* Assignee */}
            {task.assignee && (
              <div className="flex items-center gap-1">
                <User className="w-3 h-3" />
                <span>{task.assignee}</span>
              </div>
            )}

            {/* Due Date */}
            {task.dueDate && (
              <div className="flex items-center gap-1">
                <Calendar className="w-3 h-3" />
                <span>{formatDate(task.dueDate)}</span>
              </div>
            )}

            {/* Estimated Hours */}
            {task.estimatedHours && (
              <div className="flex items-center gap-1">
                <Clock className="w-3 h-3" />
                <span>{task.estimatedHours}h</span>
              </div>
            )}
          </div>

          {/* Subtasks count */}
          {task.subtasks.length > 0 && (
            <Badge variant="outline" className="text-xs">
              {task.subtasks.length} subtasks
            </Badge>
          )}
        </div>

        {/* Progress indicator for tasks with subtasks */}
        {task.subtasks.length > 0 && (
          <div className="w-full bg-gray-200 rounded-full h-1.5">
            <div
              className="bg-blue-600 h-1.5 rounded-full transition-all duration-300"
              style={{
                width: `${Math.min(100, (task.subtasks.length * 25))}%`, // Placeholder progress
              }}
            />
          </div>
        )}
      </div>
    </Card>
  )
}
