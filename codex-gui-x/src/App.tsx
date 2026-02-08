import { useState } from 'react'
import { 
  Plus, 
  MessageSquare, 
  Settings, 
  Terminal, 
  Github, 
  Cpu, 
  Zap,
  MoreVertical,
  Send,
  User,
  Bot,
  Layout,
  ShieldCheck,
  Eye,
  Box as BoxIcon,
  ClipboardList,
  Monitor
} from 'lucide-react'

import { useBridge } from './hooks/useBridge'
import { WorktreeDashboard } from './components/orchestration/WorktreeDashboard'
import { QAAuditor } from './components/orchestration/QAAuditor'
import { Git4DVisualization } from './components/visualization/Git4DVisualization'
import VRInterface from './components/vr/VRInterface'
import { PlanCreator } from './components/plan/PlanCreator'
import { VirtualEnvironmentManager } from './components/virtual-os/VirtualEnvironmentManager'
import { QCPage } from './components/qc/QCPage'
import { SecurityPage } from './components/security/SecurityPage'
import { TasksPage } from './components/tasks/TasksPage'

type View = 'chat' | 'orchestration' | 'auditor' | 'visualization' | 'vr' | 'plans' | 'virtual-os' | 'qc' | 'security' | 'tasks'

function App() {
  const { connected } = useBridge()
  const [currentView, setCurrentView] = useState<View>('chat')
  const [messages, setMessages] = useState([
    { id: 1, role: 'agent', content: "Hello! I am your Codex-X orchestrator. How can I help you build today?", sender: 'System' },
  ])
  const [input, setInput] = useState('')

  const handleSend = () => {
    if (!input.trim()) return
    setMessages([...messages, { id: Date.now(), role: 'user', content: input, sender: 'You' }])
    setInput('')
    // TODO: Connect to bi-directional bridge
  }

  // Placeholder repo path
  const repoPath = 'c:\\Users\\downl\\Desktop\\codex-main'

  const renderContent = () => {
    switch (currentView) {
      case 'orchestration':
        return <WorktreeDashboard repoPath={repoPath} />;
      case 'auditor':
        return <QAAuditor />;
      case 'visualization':
        return <Git4DVisualization repositoryPath={repoPath} />;
      case 'vr':
        return <VRInterface commits={[]} />;
      case 'plans':
        return <PlanCreator />;
      case 'virtual-os':
        return (
          <VirtualEnvironmentManager 
            environments={[]} 
            onEnvironmentSelect={() => {}} 
            onEnvironmentCreate={() => {}} 
            onEnvironmentDelete={() => {}} 
          />
        );
      case 'qc':
        return <QCPage />;
      case 'security':
        return <SecurityPage />;
      case 'tasks':
        return <TasksPage />;
      case 'chat':
      default:
        return (
          <>
            {/* Chat Area */}
            <div className="flex-1 overflow-y-auto p-6 space-y-8 max-w-4xl mx-auto w-full">
              {messages.map((m) => (
                <div key={m.id} className={`flex gap-4 ${m.role === 'user' ? 'justify-end' : ''}`}>
                  {m.role === 'agent' && (
                    <div className="h-9 w-9 rounded-lg bg-primary/20 flex items-center justify-center text-primary shrink-0 transition-transform hover:scale-110">
                      <Bot size={20} />
                    </div>
                  )}
                  <div className={m.role === 'user' ? 'chat-bubble-user' : 'chat-bubble-agent'}>
                    {m.content}
                  </div>
                  {m.role === 'user' && (
                    <div className="h-9 w-9 rounded-lg bg-muted flex items-center justify-center text-foreground shrink-0 border border-border">
                      <User size={20} />
                    </div>
                  )}
                </div>
              ))}
            </div>

            {/* Input area */}
            <div className="p-6">
              <div className="max-w-3xl mx-auto relative">
                <textarea 
                  value={input}
                  onChange={(e) => setInput(e.target.value)}
                  placeholder="Ask Codex anything..."
                  className="w-full bg-card border border-border rounded-2xl py-4 pl-4 pr-12 focus:outline-none focus:ring-1 focus:ring-primary/50 resize-none min-h-[56px] transition-all shadow-sm focus:shadow-indigo-500/10"
                  rows={1}
                  onKeyDown={(e) => {
                    if (e.key === 'Enter' && !e.shiftKey) {
                      e.preventDefault()
                      handleSend()
                    }
                  }}
                />
                <button 
                  onClick={handleSend}
                  disabled={!input.trim()}
                  className="absolute right-2 bottom-2 p-2 bg-primary text-primary-foreground rounded-xl disabled:opacity-50 hover:scale-105 active:scale-95 transition-all shadow-lg shadow-primary/20"
                >
                  <Send size={18} />
                </button>
              </div>
              <p className="text-[10px] text-center text-muted-foreground mt-2">
                Codex-X can make mistakes. Check important code.
              </p>
            </div>
          </>
        );
    }
  }

  return (
    <div className="flex h-screen bg-background text-foreground overflow-hidden font-sans">
      {/* Sidebar */}
      <aside className="w-64 bg-card border-r border-border flex flex-col">
        <div className="p-4 flex items-center justify-between">
          <button className="flex items-center gap-2 border border-border rounded-lg px-3 py-2 w-full hover:bg-muted transition-colors">
            <Plus size={16} />
            <span className="text-sm font-medium">New Thread</span>
          </button>
        </div>

        <nav className="flex-1 overflow-y-auto px-2 space-y-1">
          <div className="text-[10px] font-bold text-muted-foreground uppercase px-3 py-2 tracking-wider">
            Views
          </div>
          <button 
            onClick={() => setCurrentView('chat')}
            className={`flex items-center gap-2 w-full px-3 py-2 rounded-lg transition-colors text-sm ${currentView === 'chat' ? 'bg-primary/20 text-primary' : 'hover:bg-muted'}`}
          >
            <MessageSquare size={14} />
            <span>Chat</span>
          </button>
          <button 
            onClick={() => setCurrentView('orchestration')}
            className={`flex items-center gap-2 w-full px-3 py-2 rounded-lg transition-colors text-sm ${currentView === 'orchestration' ? 'bg-primary/20 text-primary' : 'hover:bg-muted'}`}
          >
            <Layout size={14} />
            <span>Orchestration</span>
          </button>
          <button 
            onClick={() => setCurrentView('auditor')}
            className={`flex items-center gap-2 w-full px-3 py-2 rounded-lg transition-all text-sm group ${currentView === 'auditor' ? 'bg-emerald-500/15 text-emerald-400 border border-emerald-500/30' : 'hover:bg-muted/50 hover:translate-x-1'}`}
          >
            <ShieldCheck size={14} className="group-hover:scale-110 transition-transform" />
            <span>QA Auditor</span>
          </button>

          <button 
            onClick={() => setCurrentView('qc')}
            className={`flex items-center gap-2 w-full px-3 py-2 rounded-lg transition-all text-sm group ${currentView === 'qc' ? 'bg-amber-500/15 text-amber-400 border border-amber-500/30' : 'hover:bg-muted/50 hover:translate-x-1'}`}
          >
            <Zap size={14} className="group-hover:scale-110 transition-transform" />
            <span>QC管理</span>
          </button>

          <button 
            onClick={() => setCurrentView('security')}
            className={`flex items-center gap-2 w-full px-3 py-2 rounded-lg transition-all text-sm group ${currentView === 'security' ? 'bg-red-500/15 text-red-400 border border-red-500/30' : 'hover:bg-muted/50 hover:translate-x-1'}`}
          >
            <ShieldCheck size={14} className="group-hover:scale-110 transition-transform" />
            <span>セキュリティ</span>
          </button>

          <button 
            onClick={() => setCurrentView('tasks')}
            className={`flex items-center gap-2 w-full px-3 py-2 rounded-lg transition-all text-sm group ${currentView === 'tasks' ? 'bg-blue-500/15 text-blue-400 border border-blue-500/30' : 'hover:bg-muted/50 hover:translate-x-1'}`}
          >
            <ClipboardList size={14} className="group-hover:scale-110 transition-transform" />
            <span>タスク管理</span>
          </button>

          <button 
            onClick={() => setCurrentView('visualization')}
            className={`flex items-center gap-2 w-full px-3 py-2 rounded-lg transition-colors text-sm ${currentView === 'visualization' ? 'bg-blue-500/10 text-blue-400 border border-blue-500/20' : 'hover:bg-muted'}`}
          >
            <Eye size={14} />
            <span>Visualization</span>
          </button>

          <button 
            onClick={() => setCurrentView('vr')}
            className={`flex items-center gap-2 w-full px-3 py-2 rounded-lg transition-colors text-sm ${currentView === 'vr' ? 'bg-purple-500/10 text-purple-400 border border-purple-500/20' : 'hover:bg-muted'}`}
          >
            <BoxIcon size={14} />
            <span>VR/AR</span>
          </button>

          <button 
            onClick={() => setCurrentView('plans')}
            className={`flex items-center gap-2 w-full px-3 py-2 rounded-lg transition-colors text-sm ${currentView === 'plans' ? 'bg-amber-500/10 text-amber-400 border border-amber-500/20' : 'hover:bg-muted'}`}
          >
            <ClipboardList size={14} />
            <span>Plans</span>
          </button>

          <button 
            onClick={() => setCurrentView('virtual-os')}
            className={`flex items-center gap-2 w-full px-3 py-2 rounded-lg transition-colors text-sm ${currentView === 'virtual-os' ? 'bg-rose-500/10 text-rose-400 border border-rose-500/20' : 'hover:bg-muted'}`}
          >
            <Monitor size={14} />
            <span>Virtual OS</span>
          </button>

          <div className="text-[10px] font-bold text-muted-foreground uppercase px-3 py-2 tracking-wider mt-4">
            Recent Threads
          </div>
          {[1, 2].map((i) => (
            <button key={i} className="flex items-center gap-2 w-full px-3 py-2 rounded-lg hover:bg-muted text-sm group">
              <MessageSquare size={14} className="text-muted-foreground" />
              <span className="flex-1 text-left truncate">Parallel Worktree #{i}</span>
              <MoreVertical size={14} className="opacity-0 group-hover:opacity-100 transition-opacity" />
            </button>
          ))}
        </nav>

        <div className="p-4 border-t border-border space-y-2">
          <button className="flex items-center gap-3 w-full px-3 py-2 rounded-lg hover:bg-muted text-sm">
            <Cpu size={18} />
            <span>Settings</span>
          </button>
          <button className="flex items-center gap-3 w-full px-3 py-2 rounded-lg hover:bg-muted text-sm">
            <Settings size={18} />
            <span>Configurations</span>
          </button>
        </div>
      </aside>

      {/* Main Content */}
      <main className="flex-1 flex flex-col relative">
        {/* Header */}
        <header className="h-14 border-b border-border flex items-center justify-between px-6 bg-background/50 backdrop-blur-md sticky top-0 z-10 font-sans">
          <div className="flex items-center gap-4">
            <div className="flex items-center gap-2 font-bold tracking-tighter">
              <Zap size={20} className="text-primary fill-primary" />
              <span className="text-lg">CODEX-X</span>
            </div>
            <div className={`flex items-center gap-2 px-2 py-1 rounded-md text-[10px] font-mono transition-colors ${connected ? 'bg-primary/10 text-primary border border-primary/20' : 'bg-red-500/10 text-red-500 border border-red-500/20'}`}>
              <Terminal size={10} />
              <span>{connected ? 'CONNECTED TO BRIDGE' : 'DISCONNECTED'}</span>
            </div>
          </div>
          <div className="flex items-center gap-4">
             <button className="p-2 hover:bg-muted rounded-full transition-colors text-muted-foreground">
               <Github size={20} />
             </button>
             <div className="h-8 w-8 rounded-full bg-primary flex items-center justify-center text-primary-foreground font-bold border border-primary-foreground/20 shadow-lg shadow-primary/20">
               A
             </div>
          </div>
        </header>

        {renderContent()}
      </main>
    </div>
  )
}

export default App
