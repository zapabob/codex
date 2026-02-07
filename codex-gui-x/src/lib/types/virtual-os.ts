export type VirtualEnvironment = {
  id: string
  name: string
  status: 'creating' | 'running' | 'stopped' | 'error'
  containerId: string
  image: string
  ports: Record<number, number>
  resources: {
    cpu: number
    memory: number
    disk: number
  }
  createdAt: Date
  lastAccessed: Date
}

export type CodeExecution = {
  id: string
  environmentId: string
  code: string
  language: string
  status: 'pending' | 'running' | 'completed' | 'failed'
  output: string
  error: string
  executionTime: number
  timestamp: Date
}

export type BrowserSession = {
  id: string
  environmentId: string
  url: string
  status: 'loading' | 'ready' | 'error'
  screenshot?: string
  console: string[]
  network: Array<{
    url: string
    status: number
    size: number
    time: number
  }>
}
