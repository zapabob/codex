'use client'

import { useState, useEffect, useRef } from 'react'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/atoms/Button'
import { VirtualEnvironment, CodeExecution } from '@/app/virtual-os/page'
import {
  Play,
  Square,
  Save,
  Download,
  Upload,
  Terminal,
  FileText,
  Clock,
  CheckCircle,
  XCircle,
  AlertTriangle
} from 'lucide-react'

interface CodeEditorProps {
  selectedEnvironment: VirtualEnvironment | null
  onCodeExecute: (execution: CodeExecution) => void
  executions: CodeExecution[]
}

export function CodeEditor({ selectedEnvironment, onCodeExecute, executions }: CodeEditorProps) {
  const [code, setCode] = useState('// Welcome to the Virtual OS Code Editor\n// Write your code here and execute it in the selected environment\n\nconsole.log("Hello, Virtual OS!");')
  const [language, setLanguage] = useState('javascript')
  const [isExecuting, setIsExecuting] = useState(false)
  const [output, setOutput] = useState('')
  const [error, setError] = useState('')
  const textareaRef = useRef<HTMLTextAreaElement>(null)

  const languages = [
    { value: 'javascript', label: 'JavaScript', icon: '🟨' },
    { value: 'typescript', label: 'TypeScript', icon: '🔷' },
    { value: 'python', label: 'Python', icon: '🐍' },
    { value: 'rust', label: 'Rust', icon: '🦀' },
    { value: 'go', label: 'Go', icon: '🐹' },
    { value: 'cpp', label: 'C++', icon: '🟦' },
    { value: 'java', label: 'Java', icon: '☕' },
    { value: 'bash', label: 'Bash', icon: '🐚' },
  ]

  const codeTemplates = {
    javascript: `// JavaScript Example
function fibonacci(n) {
    if (n <= 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}

console.log('Fibonacci sequence:');
for (let i = 0; i < 10; i++) {
    console.log(\`F(\${i}) = \${fibonacci(i)}\`);
}`,

    python: `# Python Example
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)

print("Fibonacci sequence:")
for i in range(10):
    print(f"F({i}) = {fibonacci(i)}")`,

    rust: `// Rust Example
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    println!("Fibonacci sequence:");
    for i in 0..10 {
        println!("F({}) = {}", i, fibonacci(i));
    }
}`,

    go: `// Go Example
package main

import "fmt"

func fibonacci(n int) int {
    if n <= 1 {
        return n
    }
    return fibonacci(n-1) + fibonacci(n-2)
}

func main() {
    fmt.Println("Fibonacci sequence:")
    for i := 0; i < 10; i++ {
        fmt.Printf("F(%d) = %d\\n", i, fibonacci(i))
    }
}`,
  }

  const handleLanguageChange = (newLanguage: string) => {
    setLanguage(newLanguage)
    // Load template for the new language
    const template = codeTemplates[newLanguage as keyof typeof codeTemplates] || codeTemplates.javascript
    setCode(template)
  }

  const handleExecute = async () => {
    if (!selectedEnvironment) {
      setError('No environment selected. Please select a virtual environment first.')
      return
    }

    if (selectedEnvironment.status !== 'running') {
      setError('Selected environment is not running. Please start the environment first.')
      return
    }

    setIsExecuting(true)
    setOutput('')
    setError('')

    try {
      // Simulate code execution
      await new Promise(resolve => setTimeout(resolve, 1000 + Math.random() * 2000))

      // Mock execution result based on language
      let mockOutput = ''
      let mockError = ''

      if (language === 'javascript') {
        mockOutput = 'Fibonacci sequence:\nF(0) = 0\nF(1) = 1\nF(2) = 1\nF(3) = 2\nF(4) = 3\nF(5) = 5\nF(6) = 8\nF(7) = 13\nF(8) = 21\nF(9) = 34'
      } else if (language === 'python') {
        mockOutput = 'Fibonacci sequence:\nF(0) = 0\nF(1) = 1\nF(2) = 1\nF(3) = 2\nF(4) = 3\nF(5) = 5\nF(6) = 8\nF(7) = 13\nF(8) = 21\nF(9) = 34'
      } else if (language === 'rust') {
        mockOutput = 'Fibonacci sequence:\nF(0) = 0\nF(1) = 1\nF(2) = 1\nF(3) = 2\nF(4) = 3\nF(5) = 5\nF(6) = 8\nF(7) = 13\nF(8) = 21\nF(9) = 34'
      } else {
        // Random execution result
        const success = Math.random() > 0.2
        if (success) {
          mockOutput = `Code executed successfully in ${language}.\nExecution time: ${(Math.random() * 2).toFixed(2)}s`
        } else {
          mockError = `Error: ${language} execution failed. Check your syntax.`
        }
      }

      const execution: CodeExecution = {
        id: `exec-${Date.now()}`,
        environmentId: selectedEnvironment.id,
        code: code,
        language: language,
        status: mockError ? 'failed' : 'completed',
        output: mockOutput,
        error: mockError,
        executionTime: Math.random() * 2,
        timestamp: new Date(),
      }

      onCodeExecute(execution)

      if (mockError) {
        setError(mockError)
      } else {
        setOutput(mockOutput)
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Execution failed'
      setError(errorMessage)

      const execution: CodeExecution = {
        id: `exec-${Date.now()}`,
        environmentId: selectedEnvironment.id,
        code: code,
        language: language,
        status: 'failed',
        output: '',
        error: errorMessage,
        executionTime: 0,
        timestamp: new Date(),
      }

      onCodeExecute(execution)
    } finally {
      setIsExecuting(false)
    }
  }

  const handleSave = () => {
    const blob = new Blob([code], { type: 'text/plain' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `code.${getFileExtension(language)}`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
  }

  const handleLoad = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0]
    if (file) {
      const reader = new FileReader()
      reader.onload = (e) => {
        const content = e.target?.result as string
        setCode(content)

        // Try to detect language from file extension
        const extension = file.name.split('.').pop()?.toLowerCase()
        const detectedLanguage = detectLanguageFromExtension(extension)
        if (detectedLanguage) {
          setLanguage(detectedLanguage)
        }
      }
      reader.readAsText(file)
    }
  }

  const detectLanguageFromExtension = (extension?: string): string | null => {
    const mapping: Record<string, string> = {
      'js': 'javascript',
      'ts': 'typescript',
      'py': 'python',
      'rs': 'rust',
      'go': 'go',
      'cpp': 'cpp',
      'cc': 'cpp',
      'cxx': 'cpp',
      'java': 'java',
      'sh': 'bash',
      'bash': 'bash',
    }
    return extension ? mapping[extension] || null : null
  }

  const getFileExtension = (lang: string): string => {
    const mapping: Record<string, string> = {
      javascript: 'js',
      typescript: 'ts',
      python: 'py',
      rust: 'rs',
      go: 'go',
      cpp: 'cpp',
      java: 'java',
      bash: 'sh',
    }
    return mapping[lang] || 'txt'
  }

  const getLanguageIcon = (lang: string) => {
    return languages.find(l => l.value === lang)?.icon || '📄'
  }

  // Auto-resize textarea
  useEffect(() => {
    if (textareaRef.current) {
      textareaRef.current.style.height = 'auto'
      textareaRef.current.style.height = `${textareaRef.current.scrollHeight}px`
    }
  }, [code])

  return (
    <div className="h-full flex flex-col">
      {/* Toolbar */}
      <div className="flex items-center justify-between p-4 bg-gray-50 border-b">
        <div className="flex items-center gap-4">
          {/* Language Selector */}
          <div className="flex items-center gap-2">
            <span className="text-sm font-medium">Language:</span>
            <select
              value={language}
              onChange={(e) => handleLanguageChange(e.target.value)}
              className="px-3 py-1 border rounded text-sm"
            >
              {languages.map((lang) => (
                <option key={lang.value} value={lang.value}>
                  {lang.icon} {lang.label}
                </option>
              ))}
            </select>
          </div>

          {/* Environment Status */}
          {selectedEnvironment ? (
            <div className="flex items-center gap-2">
              <Badge variant={selectedEnvironment.status === 'running' ? 'secondary' : 'outline'}>
                {getLanguageIcon(language)} {selectedEnvironment.name}
              </Badge>
              <Badge variant={selectedEnvironment.status === 'running' ? 'secondary' : 'destructive'}>
                {selectedEnvironment.status === 'running' ? 'Ready' : 'Not Ready'}
              </Badge>
            </div>
          ) : (
            <Badge variant="outline">No Environment Selected</Badge>
          )}
        </div>

        {/* Actions */}
        <div className="flex items-center gap-2">
          <Button variant="outline" size="sm" onClick={handleSave}>
            <Save className="w-4 h-4 mr-1" />
            Save
          </Button>

          <label className="cursor-pointer">
            <Button variant="outline" size="sm" as="span">
              <Upload className="w-4 h-4 mr-1" />
              Load
            </Button>
            <input
              type="file"
              accept=".js,.ts,.py,.rs,.go,.cpp,.java,.sh,.txt"
              onChange={handleLoad}
              className="hidden"
            />
          </label>

          <Button
            onClick={handleExecute}
            disabled={isExecuting || !selectedEnvironment || selectedEnvironment.status !== 'running'}
            className="px-6"
          >
            {isExecuting ? (
              <>
                <div className="animate-spin w-4 h-4 border-2 border-white border-t-transparent rounded-full mr-2" />
                Executing...
              </>
            ) : (
              <>
                <Play className="w-4 h-4 mr-2" />
                Execute
              </>
            )}
          </Button>
        </div>
      </div>

      {/* Main Content */}
      <div className="flex-1 flex">
        {/* Code Editor */}
        <div className="flex-1 flex flex-col">
          <div className="flex-1 p-4">
            <textarea
              ref={textareaRef}
              value={code}
              onChange={(e) => setCode(e.target.value)}
              className="w-full h-full p-4 border rounded-lg font-mono text-sm resize-none focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="Write your code here..."
              spellCheck={false}
            />
          </div>
        </div>

        {/* Output Panel */}
        <div className="w-96 border-l flex flex-col">
          {/* Output Header */}
          <div className="p-4 bg-gray-50 border-b">
            <h3 className="font-semibold flex items-center gap-2">
              <Terminal className="w-4 h-4" />
              Output
            </h3>
          </div>

          {/* Output Content */}
          <div className="flex-1 p-4 overflow-y-auto">
            {error && (
              <div className="mb-4 p-3 bg-red-50 border border-red-200 rounded">
                <div className="flex items-center gap-2 text-red-800 mb-2">
                  <XCircle className="w-4 h-4" />
                  <span className="font-medium">Execution Error</span>
                </div>
                <pre className="text-sm text-red-700 whitespace-pre-wrap">{error}</pre>
              </div>
            )}

            {output && (
              <div className="mb-4 p-3 bg-green-50 border border-green-200 rounded">
                <div className="flex items-center gap-2 text-green-800 mb-2">
                  <CheckCircle className="w-4 h-4" />
                  <span className="font-medium">Execution Successful</span>
                </div>
                <pre className="text-sm text-green-700 whitespace-pre-wrap">{output}</pre>
              </div>
            )}

            {!error && !output && !isExecuting && (
              <div className="text-center text-gray-500 py-8">
                <Terminal className="w-8 h-8 mx-auto mb-2 opacity-50" />
                <p>Execute code to see output here</p>
              </div>
            )}

            {isExecuting && (
              <div className="text-center text-gray-500 py-8">
                <div className="animate-spin w-8 h-8 border-4 border-blue-500 border-t-transparent rounded-full mx-auto mb-2" />
                <p>Executing code...</p>
              </div>
            )}
          </div>

          {/* Execution History */}
          <div className="border-t bg-gray-50">
            <div className="p-4">
              <h4 className="font-medium mb-3">Recent Executions</h4>
              <div className="space-y-2 max-h-48 overflow-y-auto">
                {executions.slice(0, 5).map((exec) => (
                  <div key={exec.id} className="flex items-center justify-between text-sm">
                    <div className="flex items-center gap-2">
                      {exec.status === 'completed' ? (
                        <CheckCircle className="w-3 h-3 text-green-500" />
                      ) : exec.status === 'failed' ? (
                        <XCircle className="w-3 h-3 text-red-500" />
                      ) : (
                        <Clock className="w-3 h-3 text-blue-500" />
                      )}
                      <span className="font-mono text-xs">{exec.language}</span>
                    </div>
                    <span className="text-gray-500 text-xs">
                      {exec.executionTime.toFixed(2)}s
                    </span>
                  </div>
                ))}

                {executions.length === 0 && (
                  <p className="text-gray-500 text-sm">No executions yet</p>
                )}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
