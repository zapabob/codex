'use client'

import { useEffect, useRef, useState } from 'react'
import mermaid from 'mermaid'
import { Task } from '@/app/tasks/page'
import { Card } from '@/components/ui/card'
import { Button } from '@/components/atoms/Button'
import { Badge } from '@/components/ui/badge'

interface MermaidDiagramProps {
  tasks: Task[]
}

type DiagramType = 'flowchart' | 'gantt' | 'timeline' | 'mindmap'

export function MermaidDiagram({ tasks }: MermaidDiagramProps) {
  const [diagramType, setDiagramType] = useState<DiagramType>('flowchart')
  const [svgContent, setSvgContent] = useState<string>('')
  const [error, setError] = useState<string | null>(null)
  const containerRef = useRef<HTMLDivElement>(null)

  // Initialize Mermaid
  useEffect(() => {
    mermaid.initialize({
      startOnLoad: true,
      theme: 'default',
      securityLevel: 'loose',
      fontFamily: 'arial',
      flowchart: {
        useMaxWidth: true,
        htmlLabels: true,
        curve: 'basis',
      },
      gantt: {
        titleTopMargin: 25,
        barHeight: 20,
        barGap: 4,
        topPadding: 50,
        leftPadding: 75,
        gridLineStartPadding: 35,
        fontSize: 11,
        fontFamily: 'arial',
        numberSectionStyles: 4,
        axisFormat: '%Y-%m-%d',
      },
    })
  }, [])

  // Generate Mermaid diagram
  useEffect(() => {
    const generateDiagram = async () => {
      try {
        setError(null)

        let diagramCode = ''

        switch (diagramType) {
          case 'flowchart':
            diagramCode = generateFlowchart(tasks)
            break
          case 'gantt':
            diagramCode = generateGanttChart(tasks)
            break
          case 'timeline':
            diagramCode = generateTimeline(tasks)
            break
          case 'mindmap':
            diagramCode = generateMindmap(tasks)
            break
        }

        // Generate SVG
        const { svg } = await mermaid.render('mermaid-diagram', diagramCode)
        setSvgContent(svg)
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to generate diagram')
        console.error('Mermaid generation error:', err)
      }
    }

    if (tasks.length > 0) {
      generateDiagram()
    }
  }, [tasks, diagramType])

  const generateFlowchart = (tasks: Task[]): string => {
    let flowchart = 'flowchart TD\n'

    // Add nodes
    tasks.forEach(task => {
      const statusColor = getStatusColor(task.status)
      const priorityIcon = getPriorityIcon(task.priority)

      flowchart += `    ${task.id}["${priorityIcon} ${task.title}<br/>Status: ${task.status}<br/>Assignee: ${task.assignee || 'Unassigned'}"]\n`
      flowchart += `    style ${task.id} fill:${statusColor}\n`
    })

    // Add connections based on dependencies
    tasks.forEach(task => {
      task.dependencies.forEach(depId => {
        flowchart += `    ${depId} --> ${task.id}\n`
      })
    })

    // Add subtasks
    tasks.forEach(task => {
      if (task.subtasks.length > 0) {
        task.subtasks.forEach((subtaskId, index) => {
          flowchart += `    ${task.id} --> ${task.id}_sub${index}["${subtaskId}"]\n`
        })
      }
    })

    return flowchart
  }

  const generateGanttChart = (tasks: Task[]): string => {
    let gantt = 'gantt\n'
    gantt += '    title Task Timeline\n'
    gantt += '    dateFormat YYYY-MM-DD\n'
    gantt += '    section Tasks\n'

    tasks.forEach(task => {
      const startDate = task.createdAt.toISOString().split('T')[0]
      const endDate = task.dueDate
        ? task.dueDate.toISOString().split('T')[0]
        : new Date(task.createdAt.getTime() + 7 * 24 * 60 * 60 * 1000).toISOString().split('T')[0]

      const progress = task.estimatedHours && task.actualHours
        ? Math.min(100, Math.round((task.actualHours / task.estimatedHours) * 100))
        : 0

      gantt += `    ${task.title} : ${task.status}, ${startDate}, ${endDate}\n`
    })

    return gantt
  }

  const generateTimeline = (tasks: Task[]): string => {
    let timeline = 'timeline\n'

    // Group tasks by status
    const statusGroups = tasks.reduce((acc, task) => {
      if (!acc[task.status]) acc[task.status] = []
      acc[task.status].push(task)
      return acc
    }, {} as Record<string, Task[]>)

    Object.entries(statusGroups).forEach(([status, statusTasks]) => {
      timeline += `    ${status}\n`
      statusTasks.forEach(task => {
        const date = task.updatedAt.toLocaleDateString()
        timeline += `        ${date} : ${task.title}\n`
      })
    })

    return timeline
  }

  const generateMindmap = (tasks: Task[]): string => {
    let mindmap = 'mindmap\n'
    mindmap += '  root((Project))\n'

    // Group by status
    const statusGroups = tasks.reduce((acc, task) => {
      if (!acc[task.status]) acc[task.status] = []
      acc[task.status].push(task)
      return acc
    }, {} as Record<string, Task[]>)

    Object.entries(statusGroups).forEach(([status, statusTasks]) => {
      mindmap += `    ${status}\n`
      statusTasks.forEach(task => {
        mindmap += `        ${task.title}\n`
        if (task.subtasks.length > 0) {
          task.subtasks.forEach(subtask => {
            mindmap += `            ${subtask}\n`
          })
        }
      })
    })

    return mindmap
  }

  const getStatusColor = (status: Task['status']): string => {
    switch (status) {
      case 'todo': return '#e5e7eb'
      case 'in-progress': return '#dbeafe'
      case 'review': return '#fef3c7'
      case 'done': return '#d1fae5'
      default: return '#f3f4f6'
    }
  }

  const getPriorityIcon = (priority: Task['priority']): string => {
    switch (priority) {
      case 'urgent': return '🚨'
      case 'high': return '🔴'
      case 'medium': return '🟡'
      case 'low': return '🟢'
      default: return '⚪'
    }
  }

  return (
    <div className="h-full p-6 space-y-6">
      {/* Controls */}
      <div className="flex items-center justify-between">
        <div className="flex gap-2">
          {(['flowchart', 'gantt', 'timeline', 'mindmap'] as DiagramType[]).map((type) => (
            <Button
              key={type}
              variant={diagramType === type ? 'primary' : 'secondary'}
              onClick={() => setDiagramType(type)}
              className="capitalize"
            >
              {type}
            </Button>
          ))}
        </div>

        <div className="flex items-center gap-4">
          <Badge variant="outline">
            {tasks.length} tasks
          </Badge>
          <Badge variant="outline">
            {diagramType}
          </Badge>
        </div>
      </div>

      {/* Diagram */}
      <Card className="flex-1 p-6">
        {error ? (
          <div className="flex items-center justify-center h-full">
            <div className="text-center">
              <div className="text-red-500 text-lg font-semibold mb-2">
                Diagram Generation Error
              </div>
              <div className="text-gray-600 text-sm">
                {error}
              </div>
            </div>
          </div>
        ) : svgContent ? (
          <div
            ref={containerRef}
            className="w-full h-full overflow-auto"
            dangerouslySetInnerHTML={{ __html: svgContent }}
          />
        ) : (
          <div className="flex items-center justify-center h-full">
            <div className="text-center">
              <div className="text-gray-500 text-lg font-semibold mb-2">
                Generating Diagram...
              </div>
              <div className="text-gray-400 text-sm">
                Please wait while we create your {diagramType}
              </div>
            </div>
          </div>
        )}
      </Card>

      {/* Legend */}
      <Card className="p-4">
        <h3 className="text-sm font-semibold mb-2">Legend</h3>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-2 text-xs">
          <div className="flex items-center gap-2">
            <div className="w-3 h-3 bg-gray-200 rounded"></div>
            <span>To Do</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-3 h-3 bg-blue-200 rounded"></div>
            <span>In Progress</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-3 h-3 bg-yellow-200 rounded"></div>
            <span>Review</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-3 h-3 bg-green-200 rounded"></div>
            <span>Done</span>
          </div>
        </div>
      </Card>
    </div>
  )
}
