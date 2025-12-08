'use client'

import { useEffect, useRef } from 'react'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Title,
  Tooltip,
  Legend,
  ChartOptions,
  ChartData,
} from 'chart.js'
import { Bar } from 'react-chartjs-2'
import { Task } from '@/app/tasks/page'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Title,
  Tooltip,
  Legend
)

interface GanttChartProps {
  tasks: Task[]
}

export function GanttChart({ tasks }: GanttChartProps) {
  const chartRef = useRef<ChartJS<'bar'>>(null)

  // Prepare data for Gantt chart
  const prepareChartData = (): ChartData<'bar'> => {
    // Sort tasks by creation date
    const sortedTasks = [...tasks].sort((a, b) =>
      a.createdAt.getTime() - b.createdAt.getTime()
    )

    const labels = sortedTasks.map(task => task.title)

    // Calculate progress data
    const progressData = sortedTasks.map(task => {
      if (!task.estimatedHours) return 0
      if (!task.actualHours) return 0
      return Math.min(100, (task.actualHours / task.estimatedHours) * 100)
    })

    return {
      labels,
      datasets: [
        {
          label: 'Progress (%)',
          data: progressData,
          backgroundColor: sortedTasks.map(task => {
            switch (task.status) {
              case 'done':
                return 'rgba(34, 197, 94, 0.8)' // green
              case 'in-progress':
                return 'rgba(59, 130, 246, 0.8)' // blue
              case 'review':
                return 'rgba(245, 158, 11, 0.8)' // yellow
              case 'todo':
              default:
                return 'rgba(156, 163, 175, 0.8)' // gray
            }
          }),
          borderColor: sortedTasks.map(task => {
            switch (task.status) {
              case 'done':
                return 'rgb(34, 197, 94)'
              case 'in-progress':
                return 'rgb(59, 130, 246)'
              case 'review':
                return 'rgb(245, 158, 11)'
              case 'todo':
              default:
                return 'rgb(156, 163, 175)'
            }
          }),
          borderWidth: 1,
        },
      ],
    }
  }

  const options: ChartOptions<'bar'> = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: {
        position: 'top' as const,
      },
      title: {
        display: true,
        text: 'Task Progress Gantt Chart',
        font: {
          size: 16,
          weight: 'bold',
        },
      },
      tooltip: {
        callbacks: {
          label: function(context) {
            const task = tasks[context.dataIndex]
            const progress = context.parsed.y
            let label = `${context.dataset.label}: ${progress.toFixed(1)}%`

            if (task.estimatedHours) {
              label += ` (${task.actualHours || 0}/${task.estimatedHours}h)`
            }

            return label
          },
        },
      },
    },
    scales: {
      y: {
        beginAtZero: true,
        max: 100,
        title: {
          display: true,
          text: 'Progress (%)',
        },
      },
      x: {
        title: {
          display: true,
          text: 'Tasks',
        },
        ticks: {
          maxRotation: 45,
          minRotation: 45,
        },
      },
    },
  }

  // Calculate task statistics
  const stats = {
    total: tasks.length,
    completed: tasks.filter(t => t.status === 'done').length,
    inProgress: tasks.filter(t => t.status === 'in-progress').length,
    overdue: tasks.filter(t =>
      t.dueDate && t.dueDate < new Date() && t.status !== 'done'
    ).length,
    totalEstimated: tasks.reduce((sum, t) => sum + (t.estimatedHours || 0), 0),
    totalActual: tasks.reduce((sum, t) => sum + (t.actualHours || 0), 0),
  }

  return (
    <div className="h-full p-6 space-y-6">
      {/* Statistics Cards */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        <Card className="p-4">
          <div className="text-2xl font-bold text-blue-600">{stats.total}</div>
          <div className="text-sm text-gray-600">Total Tasks</div>
        </Card>
        <Card className="p-4">
          <div className="text-2xl font-bold text-green-600">{stats.completed}</div>
          <div className="text-sm text-gray-600">Completed</div>
        </Card>
        <Card className="p-4">
          <div className="text-2xl font-bold text-yellow-600">{stats.inProgress}</div>
          <div className="text-sm text-gray-600">In Progress</div>
        </Card>
        <Card className="p-4">
          <div className="text-2xl font-bold text-red-600">{stats.overdue}</div>
          <div className="text-sm text-gray-600">Overdue</div>
        </Card>
      </div>

      {/* Time Statistics */}
      <div className="grid grid-cols-2 gap-4">
        <Card className="p-4">
          <div className="text-xl font-bold text-blue-600">{stats.totalEstimated}h</div>
          <div className="text-sm text-gray-600">Estimated Hours</div>
        </Card>
        <Card className="p-4">
          <div className="text-xl font-bold text-green-600">{stats.totalActual}h</div>
          <div className="text-sm text-gray-600">Actual Hours</div>
        </Card>
      </div>

      {/* Gantt Chart */}
      <Card className="p-6">
        <div className="h-96">
          <Bar ref={chartRef} data={prepareChartData()} options={options} />
        </div>
      </Card>

      {/* Task Dependencies Visualization */}
      <Card className="p-6">
        <h3 className="text-lg font-semibold mb-4">Task Dependencies</h3>
        <div className="space-y-2">
          {tasks
            .filter(task => task.dependencies.length > 0)
            .map(task => (
              <div key={task.id} className="flex items-center gap-2 p-2 bg-gray-50 rounded">
                <Badge variant="outline" className="min-w-fit">
                  {task.title}
                </Badge>
                <span className="text-gray-500">depends on</span>
                <div className="flex gap-1">
                  {task.dependencies.map(depId => {
                    const depTask = tasks.find(t => t.id === depId)
                    return depTask ? (
                      <Badge key={depId} variant="secondary">
                        {depTask.title}
                      </Badge>
                    ) : null
                  })}
                </div>
              </div>
            ))}
        </div>
      </Card>
    </div>
  )
}
