'use client'

import { useState, useRef } from 'react'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/atoms/Button'
import { VirtualEnvironment, BrowserSession } from '@/app/virtual-os/page'
import {
  Globe,
  RefreshCw,
  Home,
  ArrowLeft,
  ArrowRight,
  Monitor,
  Smartphone,
  Tablet,
  Settings,
  ExternalLink,
  Camera,
  Network
} from 'lucide-react'

interface BrowserInterfaceProps {
  selectedEnvironment: VirtualEnvironment | null
  browserSession: BrowserSession | null
  onBrowserLaunch: (session: BrowserSession) => void
}

export function BrowserInterface({ selectedEnvironment, browserSession, onBrowserLaunch }: BrowserInterfaceProps) {
  const [url, setUrl] = useState('http://localhost:3000')
  const [viewport, setViewport] = useState<'desktop' | 'tablet' | 'mobile'>('desktop')
  const [isLoading, setIsLoading] = useState(false)
  const [canGoBack, setCanGoBack] = useState(false)
  const [canGoForward, setCanGoForward] = useState(false)
  const [showDevTools, setShowDevTools] = useState(false)
  const iframeRef = useRef<HTMLIFrameElement>(null)

  const viewportSizes = {
    desktop: { width: '100%', height: '600px' },
    tablet: { width: '768px', height: '600px' },
    mobile: { width: '375px', height: '600px' },
  }

  const handleLaunchBrowser = () => {
    if (!selectedEnvironment) {
      alert('Please select a virtual environment first')
      return
    }

    if (selectedEnvironment.status !== 'running') {
      alert('Selected environment is not running')
      return
    }

    setIsLoading(true)

    // Simulate browser launch
    setTimeout(() => {
      const session: BrowserSession = {
        id: `browser-${Date.now()}`,
        environmentId: selectedEnvironment.id,
        url: url,
        status: 'ready',
        console: [
          'Browser launched successfully',
          'Loading webpage...',
          `Navigating to ${url}`,
        ],
        network: [
          {
            url: url,
            status: 200,
            size: 15432,
            time: 245,
          }
        ],
      }

      onBrowserLaunch(session)
      setIsLoading(false)
    }, 2000)
  }

  const handleNavigate = (newUrl: string) => {
    if (browserSession) {
      setUrl(newUrl)
      setIsLoading(true)

      // Simulate navigation
      setTimeout(() => {
        setIsLoading(false)
        // Update browser session with new URL
      }, 1000)
    }
  }

  const handleRefresh = () => {
    if (browserSession) {
      setIsLoading(true)
      setTimeout(() => setIsLoading(false), 800)
    }
  }

  const handleTakeScreenshot = () => {
    if (browserSession) {
      // Simulate screenshot
      alert('Screenshot captured and saved to environment')
    }
  }

  const getViewportIcon = (vp: typeof viewport) => {
    switch (vp) {
      case 'desktop': return <Monitor className="w-4 h-4" />
      case 'tablet': return <Tablet className="w-4 h-4" />
      case 'mobile': return <Smartphone className="w-4 h-4" />
    }
  }

  const samplePages = [
    { name: 'Development Server', url: 'http://localhost:3000', description: 'Local development server' },
    { name: 'Documentation', url: 'http://localhost:8080/docs', description: 'API documentation' },
    { name: 'Admin Panel', url: 'http://localhost:8080/admin', description: 'System administration' },
    { name: 'Test Suite', url: 'http://localhost:3001', description: 'Automated test results' },
  ]

  return (
    <div className="h-full flex flex-col">
      {/* Browser Toolbar */}
      <div className="p-4 bg-gray-50 border-b">
        <div className="flex items-center gap-4 mb-4">
          {/* Environment Status */}
          {selectedEnvironment ? (
            <Badge variant={selectedEnvironment.status === 'running' ? 'secondary' : 'outline'}>
              🌐 {selectedEnvironment.name}
            </Badge>
          ) : (
            <Badge variant="outline">No Environment Selected</Badge>
          )}

          {/* Viewport Controls */}
          <div className="flex items-center gap-2">
            <span className="text-sm font-medium">Viewport:</span>
            {(['desktop', 'tablet', 'mobile'] as const).map((vp) => (
              <Button
                key={vp}
                variant={viewport === vp ? 'primary' : 'outline'}
                size="sm"
                onClick={() => setViewport(vp)}
              >
                {getViewportIcon(vp)}
              </Button>
            ))}
          </div>

          {/* Browser Actions */}
          <div className="flex items-center gap-2 ml-auto">
            <Button
              variant="outline"
              size="sm"
              onClick={handleTakeScreenshot}
              disabled={!browserSession}
            >
              <Camera className="w-4 h-4 mr-1" />
              Screenshot
            </Button>

            <Button
              variant="outline"
              size="sm"
              onClick={() => setShowDevTools(!showDevTools)}
              disabled={!browserSession}
            >
              <Settings className="w-4 h-4 mr-1" />
              DevTools
            </Button>

            <Button
              variant="outline"
              size="sm"
              disabled={!browserSession}
            >
              <ExternalLink className="w-4 h-4 mr-1" />
              Open External
            </Button>
          </div>
        </div>

        {/* Address Bar */}
        <div className="flex items-center gap-2">
          <Button
            variant="outline"
            size="sm"
            disabled={!browserSession || !canGoBack}
            onClick={() => setCanGoBack(false)}
          >
            <ArrowLeft className="w-4 h-4" />
          </Button>

          <Button
            variant="outline"
            size="sm"
            disabled={!browserSession || !canGoForward}
            onClick={() => setCanGoForward(false)}
          >
            <ArrowRight className="w-4 h-4" />
          </Button>

          <Button
            variant="outline"
            size="sm"
            onClick={handleRefresh}
            disabled={!browserSession}
          >
            <RefreshCw className={`w-4 h-4 ${isLoading ? 'animate-spin' : ''}`} />
          </Button>

          <Button
            variant="outline"
            size="sm"
            disabled={!browserSession}
          >
            <Home className="w-4 h-4" />
          </Button>

          <div className="flex-1 flex items-center gap-2">
            <div className="flex-1 relative">
              <Globe className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-gray-400" />
              <input
                type="url"
                value={url}
                onChange={(e) => setUrl(e.target.value)}
                onKeyDown={(e) => {
                  if (e.key === 'Enter') {
                    handleNavigate(url)
                  }
                }}
                className="w-full pl-10 pr-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                placeholder="Enter URL..."
                disabled={!browserSession}
              />
            </div>

            {!browserSession ? (
              <Button onClick={handleLaunchBrowser} disabled={!selectedEnvironment || selectedEnvironment.status !== 'running'}>
                <Globe className="w-4 h-4 mr-2" />
                Launch Browser
              </Button>
            ) : (
              <Button variant="outline" onClick={() => handleNavigate(url)}>
                Go
              </Button>
            )}
          </div>
        </div>
      </div>

      {/* Browser Content */}
      <div className="flex-1 flex">
        {/* Main Browser View */}
        <div className="flex-1 p-4">
          {!browserSession ? (
            <Card className="h-full flex items-center justify-center">
              <div className="text-center">
                <Globe className="w-16 h-16 text-gray-400 mx-auto mb-4" />
                <h3 className="text-xl font-bold text-gray-900 mb-2">Browser Not Launched</h3>
                <p className="text-gray-600 mb-6">
                  Launch a browser in your virtual environment to start browsing.
                </p>

                {selectedEnvironment && selectedEnvironment.status === 'running' && (
                  <div className="space-y-4">
                    <Button onClick={handleLaunchBrowser} className="px-8">
                      <Globe className="w-5 h-5 mr-2" />
                      Launch Browser
                    </Button>

                    <div className="mt-6">
                      <h4 className="font-semibold mb-3">Quick Access</h4>
                      <div className="grid grid-cols-2 gap-3">
                        {samplePages.map((page) => (
                          <button
                            key={page.name}
                            onClick={() => {
                              setUrl(page.url)
                              handleLaunchBrowser()
                            }}
                            className="p-3 border rounded-lg hover:bg-gray-50 text-left transition-colors"
                          >
                            <div className="font-medium text-sm">{page.name}</div>
                            <div className="text-xs text-gray-600">{page.description}</div>
                          </button>
                        ))}
                      </div>
                    </div>
                  </div>
                )}

                {(!selectedEnvironment || selectedEnvironment.status !== 'running') && (
                  <p className="text-sm text-gray-500">
                    Please select and start a virtual environment first.
                  </p>
                )}
              </div>
            </Card>
          ) : (
            <div className="h-full flex flex-col">
              {/* Browser Frame */}
              <div className="flex-1 bg-white border rounded-lg overflow-hidden">
                <div
                  className="mx-auto bg-gray-100 relative"
                  style={{
                    width: viewportSizes[viewport].width,
                    height: viewportSizes[viewport].height,
                    maxWidth: '100%',
                  }}
                >
                  {isLoading && (
                    <div className="absolute inset-0 flex items-center justify-center bg-white bg-opacity-75 z-10">
                      <div className="flex items-center gap-2">
                        <div className="animate-spin w-6 h-6 border-2 border-blue-500 border-t-transparent rounded-full" />
                        <span className="text-sm text-gray-600">Loading...</span>
                      </div>
                    </div>
                  )}

                  {/* Mock Browser Content */}
                  <div className="w-full h-full bg-white border">
                    <div className="p-4 border-b bg-gray-50">
                      <div className="flex items-center gap-2 text-sm text-gray-600">
                        <Globe className="w-4 h-4" />
                        <span>{browserSession.url}</span>
                        <Badge variant="secondary" className="ml-auto">
                          {browserSession.status}
                        </Badge>
                      </div>
                    </div>

                    <div className="p-8 text-center">
                      <div className="text-4xl mb-4">🌐</div>
                      <h3 className="text-xl font-bold text-gray-900 mb-2">Browser Active</h3>
                      <p className="text-gray-600 mb-4">
                        Connected to {browserSession.url} in {selectedEnvironment?.name}
                      </p>

                      <div className="bg-gray-50 rounded p-4 text-left max-w-md mx-auto">
                        <h4 className="font-semibold mb-2">Connection Details</h4>
                        <div className="text-sm space-y-1">
                          <div><strong>Environment:</strong> {selectedEnvironment?.name}</div>
                          <div><strong>Container:</strong> {selectedEnvironment?.containerId}</div>
                          <div><strong>Viewport:</strong> {viewport}</div>
                          <div><strong>Status:</strong> {browserSession.status}</div>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          )}
        </div>

        {/* DevTools Panel */}
        {showDevTools && browserSession && (
          <div className="w-96 border-l bg-gray-50 flex flex-col">
            {/* DevTools Tabs */}
            <div className="flex border-b bg-white">
              {['Console', 'Network', 'Elements'].map((tab) => (
                <button
                  key={tab}
                  className="px-4 py-2 text-sm border-b-2 border-transparent hover:border-gray-300"
                >
                  {tab}
                </button>
              ))}
            </div>

            {/* Console */}
            <div className="flex-1 p-4 overflow-y-auto">
              <h4 className="font-semibold mb-3 flex items-center gap-2">
                <Terminal className="w-4 h-4" />
                Console
              </h4>

              <div className="space-y-2 font-mono text-sm">
                {browserSession.console.map((log, index) => (
                  <div key={index} className="text-gray-700">
                    <span className="text-gray-500">&gt;</span> {log}
                  </div>
                ))}
              </div>
            </div>

            {/* Network */}
            <div className="border-t bg-white">
              <div className="p-4">
                <h4 className="font-semibold mb-3 flex items-center gap-2">
                  <Network className="w-4 h-4" />
                  Network ({browserSession.network.length})
                </h4>

                <div className="space-y-2 max-h-48 overflow-y-auto">
                  {browserSession.network.map((request, index) => (
                    <div key={index} className="flex items-center justify-between text-sm">
                      <div className="flex-1 truncate">
                        <div className="font-medium truncate">{request.url}</div>
                      </div>
                      <div className="flex items-center gap-2 ml-2">
                        <Badge
                          variant={request.status < 400 ? 'secondary' : 'destructive'}
                          className="text-xs"
                        >
                          {request.status}
                        </Badge>
                        <span className="text-gray-500 text-xs">
                          {request.time}ms
                        </span>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  )
}
