import { useState } from 'react'
import { KanbanBoard } from './KanbanBoard'
import { GanttChart } from './GanttChart'
import { Task, TaskColumn } from '../../types/tasks'
import { Layout, BarChart, Plus, Filter, Search } from 'lucide-react'
import { Button } from '../atoms/Button'

export function TasksPage() {
  const [activeTab, setActiveTab] = useState<'kanban' | 'gantt'>('kanban')

  const sampleTasks: Task[] = [
    { 
       id: 't1', title: 'Initialize High-Speed Bridge', description: 'Setup bi-directional communication layer between CLI and GUI Clusters', 
       status: 'in-progress', priority: 'urgent', progress: 45, tags: ['core', 'infra'], subtasks: [], assignee: 'Antigravity'
    },
    { 
       id: 't2', title: 'System Hardening Protocol', description: 'Enable hardware-backed encryption modules for all user data streams', 
       status: 'todo', priority: 'high', progress: 0, tags: ['security'], subtasks: [], assignee: 'Sentinel'
    },
    { 
       id: 't3', title: 'Neural Engine Migration', description: 'Port legacy inference engine to the new optimized Vite architecture', 
       status: 'done', priority: 'medium', progress: 100, tags: ['ai', 'optimization'], subtasks: [], assignee: 'Codex-X'
    },
    { 
       id: 't4', title: 'VR Workspace Alpha', description: 'Setup initial spatial tracking for the VR development interface', 
       status: 'review', priority: 'low', progress: 85, tags: ['ux', 'vr'], subtasks: [], assignee: 'Meta-01'
    }
  ]

  const columns: TaskColumn[] = [
    { id: 'todo', title: 'Backlog Cluster', tasks: sampleTasks.filter(t => t.status === 'todo') },
    { id: 'in-progress', title: 'Active Processing', tasks: sampleTasks.filter(t => t.status === 'in-progress') },
    { id: 'review', title: 'Quality Assurance', tasks: sampleTasks.filter(t => t.status === 'review') },
    { id: 'done', title: 'Finalized Modules', tasks: sampleTasks.filter(t => t.status === 'done') }
  ]

  return (
    <div className="max-w-[1600px] mx-auto w-full p-8">
      <div className="flex flex-col lg:flex-row lg:items-center justify-between gap-8 mb-12">
        <div>
          <div className="flex items-center gap-3 mb-2">
            <div className="h-12 w-12 rounded-2xl bg-sky-500/20 text-sky-400 flex items-center justify-center shadow-lg shadow-sky-500/10">
                <Layout size={32} />
            </div>
            <h1 className="text-4xl font-black tracking-tighter uppercase italic">Task Orchestrator</h1>
          </div>
          <p className="text-muted-foreground font-medium pl-1 tracking-tight">Agile development and strategic milestone management</p>
        </div>

        <div className="flex items-center gap-3">
            <div className="flex items-center gap-2 p-1 bg-muted/20 border border-border rounded-2xl px-4 py-2">
                <Search size={16} className="text-muted-foreground" />
                <input placeholder="Filter tasks..." className="bg-transparent border-none outline-none text-sm w-40" />
            </div>
            <Button variant="outlined" size="small"><Filter size={18} /></Button>
            <Button size="small"><Plus size={18} className="mr-2" /> New Task</Button>
        </div>
      </div>

      <nav className="flex gap-2 p-1.5 bg-muted/30 border border-border rounded-3xl w-fit mb-10 backdrop-blur-xl">
        {[
          { id: 'kanban', label: 'Board System', icon: Layout },
          { id: 'gantt', label: 'Progress Gantt', icon: BarChart },
        ].map(tab => (
          <button
            key={tab.id}
            onClick={() => setActiveTab(tab.id as any)}
            className={`flex items-center gap-2 px-6 py-2.5 rounded-2xl text-sm font-bold transition-all ${activeTab === tab.id ? 'bg-primary text-primary-foreground shadow-2xl shadow-primary/30 scale-105' : 'hover:bg-muted text-muted-foreground hover:text-foreground'}`}
          >
            <tab.icon size={16} />
            {tab.label}
          </button>
        ))}
      </nav>

      <div className="relative animate-in fade-in slide-in-from-bottom-4 duration-500">
        {activeTab === 'kanban' && <KanbanBoard columns={columns} />}
        {activeTab === 'gantt' && <GanttChart tasks={sampleTasks} />}
      </div>
    </div>
  )
}
