export interface QualityMetric {
  id: string
  name: string
  value: number
  unit: string
  target: number
  tolerance: number
  status: 'good' | 'warning' | 'critical'
  trend: 'up' | 'down' | 'stable'
  timestamp: Date
  category: string
}

export interface QCProcess {
  id: string
  name: string
  description: string
  status: 'idle' | 'running' | 'completed' | 'failed'
  progress: number
  startTime?: Date
  endTime?: Date
  metrics: QualityMetric[]
  results?: QCResult
}

export interface QCResult {
  id: string
  processId: string
  anovaResult?: AnovaResult
  overallScore: number
  passed: boolean
  recommendations: string[]
  timestamp: Date
}

export interface AnovaResult {
  fStatistic: number
  pValue: number
  degreesOfFreedom: number
  significance: boolean
  groups: Array<{
    name: string
    mean: number
    variance: number
    count: number
  }>
}

export interface QCAlert {
  id: string
  type: 'warning' | 'critical' | 'info'
  title: string
  message: string
  metricId?: string
  threshold: number
  currentValue: number
  timestamp: Date
  acknowledged: boolean
}
