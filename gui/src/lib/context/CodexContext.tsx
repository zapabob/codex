'use client';

import React, { createContext, useContext, useReducer, useEffect, useRef, ReactNode } from 'react';
import {
  Conversation,
  Message,
  Agent,
  MCPConnection,
  SecurityScan,
  ResearchResult,
  WebResearchResult,
  SystemMetrics,
  NotificationItem,
  User,
  WebSocketMessage,
} from '../types';
import { apiClient } from '../api/client';
import { getSpecStory } from '../specstory';
import { DualBridge, BridgeConfig } from '../bridge/dual-bridge';
import { AITool, AISession, DevelopmentTask } from '../types/ai-tools';

interface CodexState {
  // Authentication
  user: User | null;
  isAuthenticated: boolean;

  // Conversations
  conversations: Conversation[];
  currentConversation: Conversation | null;
  messages: Message[];

  // Agents
  agents: Agent[];
  activeAgents: Agent[];

  // MCP
  mcpConnections: MCPConnection[];

  // Security
  securityScans: SecurityScan[];

  // Research
  researchResults: ResearchResult[];
  webResearchResults: WebResearchResult[];

  // System
  metrics: SystemMetrics | null;

  // UI State
  notifications: NotificationItem[];
  isLoading: boolean;
  error: string | null;

  // WebSocket
  isConnected: boolean;
  cliBridgeConnected: boolean;
}

type CodexAction =
  | { type: 'SET_USER'; payload: User | null }
  | { type: 'SET_CONVERSATIONS'; payload: Conversation[] }
  | { type: 'SET_CURRENT_CONVERSATION'; payload: Conversation | null }
  | { type: 'ADD_MESSAGE'; payload: Message }
  | { type: 'SET_MESSAGES'; payload: Message[] }
  | { type: 'SET_AGENTS'; payload: Agent[] }
  | { type: 'UPDATE_AGENT'; payload: Agent }
  | { type: 'SET_MCP_CONNECTIONS'; payload: MCPConnection[] }
  | { type: 'UPDATE_MCP_CONNECTION'; payload: MCPConnection }
  | { type: 'ADD_SECURITY_SCAN'; payload: SecurityScan }
  | { type: 'UPDATE_SECURITY_SCAN'; payload: SecurityScan }
  | { type: 'ADD_RESEARCH_RESULT'; payload: ResearchResult }
  | { type: 'UPDATE_RESEARCH_RESULT'; payload: ResearchResult }
  | { type: 'ADD_WEB_RESEARCH_RESULT'; payload: WebResearchResult }
  | { type: 'UPDATE_WEB_RESEARCH_RESULT'; payload: WebResearchResult }
  | { type: 'SET_METRICS'; payload: SystemMetrics }
  | { type: 'ADD_NOTIFICATION'; payload: NotificationItem }
  | { type: 'MARK_NOTIFICATION_READ'; payload: string }
  | { type: 'SET_LOADING'; payload: boolean }
  | { type: 'SET_ERROR'; payload: string | null }
  | { type: 'SET_CONNECTION_STATUS'; payload: boolean }
  | { type: 'SET_BRIDGE_STATUS'; payload: boolean };

const initialState: CodexState = {
  user: null,
  isAuthenticated: false,
  conversations: [],
  currentConversation: null,
  messages: [],
  agents: [
    {
      id: 'code-reviewer',
      name: 'Code Reviewer',
      type: 'code-reviewer',
      status: 'idle',
      description: 'コードの品質とセキュリティをレビューします',
      capabilities: ['code-review', 'security-audit', 'performance-check'],
      lastUsed: new Date(Date.now() - 3600000).toISOString(), // 1 hour ago
    },
    {
      id: 'test-generator',
      name: 'Test Generator',
      type: 'test-gen',
      status: 'idle',
      description: '自動的にテストコードを生成します',
      capabilities: ['unit-tests', 'integration-tests', 'e2e-tests'],
      lastUsed: new Date(Date.now() - 7200000).toISOString(), // 2 hours ago
    },
    {
      id: 'security-auditor',
      name: 'Security Auditor',
      type: 'sec-audit',
      status: 'idle',
      description: 'セキュリティ脆弱性をスキャンします',
      capabilities: ['vulnerability-scan', 'dependency-check', 'secrets-detection'],
      lastUsed: new Date(Date.now() - 1800000).toISOString(), // 30 minutes ago
    },
    {
      id: 'deep-researcher',
      name: 'Deep Researcher',
      type: 'researcher',
      status: 'idle',
      description: '高度な研究と分析を行います',
      capabilities: ['deep-research', 'data-analysis', 'trend-analysis'],
      lastUsed: new Date(Date.now() - 86400000).toISOString(), // 1 day ago
    },
    {
      id: 'web-research',
      name: 'Web Research',
      type: 'researcher',
      status: 'idle',
      description: '譛譁ｰ縺ｮWeb繧ｽ繝ｼ繧ｹから情報を検索します。',
      capabilities: ['web-search', 'official-research'],
      lastUsed: new Date(Date.now() - 43200000).toISOString(), // 12 hours ago
    },
  ],
  activeAgents: [],
  mcpConnections: [
    {
      id: 'filesystem-1',
      name: 'Local Filesystem',
      type: 'filesystem',
      status: 'connected',
      url: 'file:///',
      lastConnected: new Date().toISOString(),
      requestCount: 42,
      avgResponseTime: 15.7,
    },
    {
      id: 'github-1',
      name: 'GitHub Integration',
      type: 'github',
      status: 'connected',
      url: 'https://api.github.com',
      lastConnected: new Date().toISOString(),
      requestCount: 28,
      avgResponseTime: 120.5,
    },
  ],
  securityScans: [],
  researchResults: [],
  webResearchResults: [],
  metrics: {
    cpuUsage: 45.2,
    memoryUsage: 67.8,
    diskUsage: 23.1,
    activeProcesses: 127,
    uptime: 3600,
  },
  notifications: [],
  isLoading: false,
  error: null,
  isConnected: false,
  cliBridgeConnected: false,
};

function codexReducer(state: CodexState, action: CodexAction): CodexState {
  switch (action.type) {
    case 'SET_USER':
      return {
        ...state,
        user: action.payload,
        isAuthenticated: action.payload !== null,
      };

    case 'SET_CONVERSATIONS':
      return {
        ...state,
        conversations: action.payload,
      };

    case 'SET_CURRENT_CONVERSATION':
      return {
        ...state,
        currentConversation: action.payload,
      };

    case 'ADD_MESSAGE':
      return {
        ...state,
        messages: [...state.messages, action.payload],
      };

    case 'SET_MESSAGES':
      return {
        ...state,
        messages: action.payload,
      };

    case 'SET_AGENTS':
      return {
        ...state,
        agents: action.payload,
      };

    case 'UPDATE_AGENT':
      return {
        ...state,
        agents: state.agents.map(agent =>
          agent.id === action.payload.id ? action.payload : agent
        ),
        activeAgents: state.activeAgents.map(agent =>
          agent.id === action.payload.id ? action.payload : agent
        ),
      };

    case 'SET_MCP_CONNECTIONS':
      return {
        ...state,
        mcpConnections: action.payload,
      };

    case 'UPDATE_MCP_CONNECTION':
      return {
        ...state,
        mcpConnections: state.mcpConnections.map(conn =>
          conn.id === action.payload.id ? action.payload : conn
        ),
      };

    case 'ADD_SECURITY_SCAN':
      return {
        ...state,
        securityScans: [action.payload, ...state.securityScans],
      };

    case 'UPDATE_SECURITY_SCAN':
      return {
        ...state,
        securityScans: state.securityScans.map(scan =>
          scan.id === action.payload.id ? action.payload : scan
        ),
      };

    case 'ADD_RESEARCH_RESULT':
      return {
        ...state,
        researchResults: [action.payload, ...state.researchResults],
      };

    case 'UPDATE_RESEARCH_RESULT':
      return {
        ...state,
        researchResults: state.researchResults.map(result =>
          result.id === action.payload.id ? action.payload : result
        ),
      };

    case 'ADD_WEB_RESEARCH_RESULT':
      return {
        ...state,
        webResearchResults: [action.payload, ...state.webResearchResults],
      };

    case 'UPDATE_WEB_RESEARCH_RESULT':
      return {
        ...state,
        webResearchResults: state.webResearchResults.map(result =>
          result.id === action.payload.id ? action.payload : result
        ),
      };

    case 'SET_METRICS':
      return {
        ...state,
        metrics: action.payload,
      };

    case 'ADD_NOTIFICATION':
      return {
        ...state,
        notifications: [action.payload, ...state.notifications],
      };

    case 'MARK_NOTIFICATION_READ':
      return {
        ...state,
        notifications: state.notifications.map(notif =>
          notif.id === action.payload ? { ...notif, read: true } : notif
        ),
      };

    case 'SET_LOADING':
      return {
        ...state,
        isLoading: action.payload,
      };

    case 'SET_ERROR':
      return {
        ...state,
        error: action.payload,
      };

    case 'SET_CONNECTION_STATUS':
      return {
        ...state,
        isConnected: action.payload,
      };

    case 'SET_BRIDGE_STATUS':
      return {
        ...state,
        cliBridgeConnected: action.payload,
      };

    default:
      return state;
  }
}

interface CodexContextType {
  state: CodexState;
  dispatch: React.Dispatch<CodexAction>;

  // Actions
  login: (email: string, password: string) => Promise<void>;
  logout: () => Promise<void>;
  createConversation: (model: string, initialMessage: string) => Promise<void>;
  sendMessage: (content: string) => Promise<void>;
  loadConversations: () => Promise<void>;
  selectConversation: (conversation: Conversation) => Promise<void>;
  runAgent: (agentId: string, context: any) => Promise<void>;
  runSecurityScan: (type: string, target: string) => Promise<void>;
  runResearch: (query: string) => Promise<void>;
  runWebResearch: (query: string) => Promise<void>;
  executeCommand: (command: string, cwd?: string) => Promise<{ exitCode: number; stdout: string; stderr: string }>;
  loadMetrics: () => Promise<void>;
  clearError: () => void;
  loadAITools: () => Promise<AITool[]>;
  loadAISessions: () => Promise<AISession[]>;
  loadDevelopmentTasks: () => Promise<DevelopmentTask[]>;
}

const CodexContext = createContext<CodexContextType | undefined>(undefined);

export function CodexProvider({ children }: { children: ReactNode }) {
  const [state, dispatch] = useReducer(codexReducer, initialState);
  const cliBridgeRef = useRef<DualBridge | null>(null);

  // Initialize WebSocket connection
  useEffect(() => {
    const handleWebSocketMessage = (message: WebSocketMessage) => {
      switch (message.type) {
        case 'conversation_update':
          // Handle conversation updates
          if (message.data.conversation) {
            dispatch({
              type: 'SET_CURRENT_CONVERSATION',
              payload: message.data.conversation,
            });
          }
          break;

        case 'agent_status':
          if (message.data.agent) {
            dispatch({
              type: 'UPDATE_AGENT',
              payload: message.data.agent,
            });
          }
          break;

        case 'system_metrics':
          dispatch({
            type: 'SET_METRICS',
            payload: message.data.metrics,
          });
          break;

        case 'notification':
          dispatch({
            type: 'ADD_NOTIFICATION',
            payload: message.data.notification,
          });
          break;
      }
    };

    apiClient.connectWebSocket(handleWebSocketMessage);

    return () => {
      apiClient.disconnectWebSocket();
    };
  }, []);

  // Establish GUI-CLI bridge without requiring manual steps
  useEffect(() => {
    const bridgeUrl = process.env.NEXT_PUBLIC_CLI_BRIDGE_URL
      || apiClient.getBridgeWebSocketUrl('/cli/bridge');

    const config: BridgeConfig = {
      websocketUrl: bridgeUrl,
      mcpRegistryUrl: process.env.NEXT_PUBLIC_MCP_REGISTRY_URL,
      reconnectInterval: 5000,
      maxRetries: 5,
      heartbeatInterval: 10000,
    };

    const bridge = new DualBridge(config);
    cliBridgeRef.current = bridge;

    const handleHandshake = () => dispatch({ type: 'SET_BRIDGE_STATUS', payload: true });
    const handleBridgeError = () => dispatch({ type: 'SET_BRIDGE_STATUS', payload: false });

    bridge.on('handshake', handleHandshake);
    bridge.on('error', handleBridgeError);

    bridge.connect().catch((error) => {
      console.error('CLI bridge connection failed:', error);
      dispatch({ type: 'SET_BRIDGE_STATUS', payload: false });
    });

    return () => {
      bridge.off('handshake', handleHandshake);
      bridge.off('error', handleBridgeError);
      bridge.disconnect();
    };
  }, [dispatch]);

  // Load initial data
  useEffect(() => {
    const initialize = async () => {
      try {
        dispatch({ type: 'SET_LOADING', payload: true });

        // Initialize SpecStory
        const specStory = getSpecStory({
          enableAutoSave: true,
          saveInterval: 30000, // 30 seconds
        });
        await specStory.initialize();

        // Load conversations from SpecStory
        const savedConversations = await specStory.getConversations();
        if (savedConversations.length > 0) {
          dispatch({ type: 'SET_CONVERSATIONS', payload: savedConversations });
        }

        // Load user account
        const account = await apiClient.getAccount();
        if (account) {
          dispatch({
            type: 'SET_USER',
            payload: {
              id: account.id || 'user',
              email: account.email,
              name: account.name,
              plan: account.plan || 'free',
            },
          });
        }

        // Load conversations from API if not loaded from SpecStory
        if (savedConversations.length === 0) {
          await loadConversations();
        }

        // Load agents
        const agents = await apiClient.getAgents();
        dispatch({ type: 'SET_AGENTS', payload: agents });

        // Load MCP connections
        const connections = await apiClient.getMCPConnections();
        dispatch({ type: 'SET_MCP_CONNECTIONS', payload: connections });

        // Load system metrics
        await loadMetrics();

      } catch (error) {
        console.error('Failed to initialize:', error);
        dispatch({
          type: 'SET_ERROR',
          payload: error instanceof Error ? error.message : '初期化に失敗しました',
        });
      } finally {
        dispatch({ type: 'SET_LOADING', payload: false });
      }
    };

    initialize();
  }, []);

  const login = async (email: string, password: string) => {
    try {
      dispatch({ type: 'SET_LOADING', payload: true });
      dispatch({ type: 'SET_ERROR', payload: null });

      // For now, use API key login
      await apiClient.login({
        method: 'api-key',
        apiKey: password, // password field contains API key
      });

      // Load user data
      const account = await apiClient.getAccount();
      dispatch({
        type: 'SET_USER',
        payload: {
          id: account.id || 'user',
          email: account.email,
          name: account.name,
          plan: account.plan || 'free',
        },
      });

    } catch (error) {
      dispatch({
        type: 'SET_ERROR',
        payload: error instanceof Error ? error.message : 'ログインに失敗しました',
      });
      throw error;
    } finally {
      dispatch({ type: 'SET_LOADING', payload: false });
    }
  };

  const logout = async () => {
    try {
      await apiClient.logout();
      dispatch({ type: 'SET_USER', payload: null });
      dispatch({ type: 'SET_CONVERSATIONS', payload: [] });
      dispatch({ type: 'SET_CURRENT_CONVERSATION', payload: null });
      dispatch({ type: 'SET_MESSAGES', payload: [] });
    } catch (error) {
      console.error('Logout error:', error);
    }
  };

  const createConversation = async (model: string, initialMessage: string) => {
    try {
      dispatch({ type: 'SET_LOADING', payload: true });
      const conversation = await apiClient.createConversation({
        model,
        initialMessage,
      });

      // Save to SpecStory
      const specStory = getSpecStory();
      await specStory.saveConversation(conversation);

      dispatch({
        type: 'SET_CURRENT_CONVERSATION',
        payload: conversation,
      });

      dispatch({
        type: 'SET_CONVERSATIONS',
        payload: [conversation, ...state.conversations],
      });

    } catch (error) {
      dispatch({
        type: 'SET_ERROR',
        payload: error instanceof Error ? error.message : '会話の作成に失敗しました',
      });
      throw error;
    } finally {
      dispatch({ type: 'SET_LOADING', payload: false });
    }
  };

  const sendMessage = async (content: string) => {
    if (!state.currentConversation) return;

    try {
      const message = await apiClient.sendMessage(state.currentConversation.id, content);

      // Save to SpecStory
      const specStory = getSpecStory();
      await specStory.saveMessage(state.currentConversation.id, message);

      dispatch({ type: 'ADD_MESSAGE', payload: message });
    } catch (error) {
      dispatch({
        type: 'SET_ERROR',
        payload: error instanceof Error ? error.message : 'メッセージの送信に失敗しました',
      });
      throw error;
    }
  };

  const loadConversations = async () => {
    try {
      const conversations = await apiClient.listConversations();
      dispatch({ type: 'SET_CONVERSATIONS', payload: conversations });
    } catch (error) {
      console.error('Failed to load conversations:', error);
    }
  };

  const selectConversation = async (conversation: Conversation) => {
    dispatch({ type: 'SET_CURRENT_CONVERSATION', payload: conversation });
    // Load messages for this conversation
    // This would require additional API endpoint
    dispatch({ type: 'SET_MESSAGES', payload: [] });
  };

  const runAgent = async (agentId: string, context: any) => {
    try {
      const result = await apiClient.runAgent(agentId, context);

      // Update agent status
      const updatedAgent = state.agents.find(a => a.id === agentId);
      if (updatedAgent) {
        dispatch({
          type: 'UPDATE_AGENT',
          payload: { ...updatedAgent, status: 'completed' as const },
        });
      }

      return result;
    } catch (error) {
      // Update agent status to error
      const updatedAgent = state.agents.find(a => a.id === agentId);
      if (updatedAgent) {
        dispatch({
          type: 'UPDATE_AGENT',
          payload: { ...updatedAgent, status: 'error' as const },
        });
      }
      throw error;
    }
  };

  const runSecurityScan = async (type: string, target: string) => {
    try {
      const scan = await apiClient.runSecurityAudit({ path: target });
      dispatch({ type: 'ADD_SECURITY_SCAN', payload: scan });
      return scan;
    } catch (error) {
      console.error('Security scan failed:', error);
      throw error;
    }
  };

  const runResearch = async (query: string) => {
    try {
      const result = await apiClient.runResearch({ query });
      dispatch({ type: 'ADD_RESEARCH_RESULT', payload: result });
      return result;
    } catch (error) {
      console.error('Research failed:', error);
      throw error;
    }
  };

  const runWebResearch = async (query: string) => {
    try {
      const result = await apiClient.runWebResearch({ query });
      dispatch({ type: 'ADD_WEB_RESEARCH_RESULT', payload: result });
      return result;
    } catch (error) {
      console.error('Web research failed:', error);
      throw error;
    }
  };

  const executeCommand = async (command: string, cwd?: string) => {
    const bridge = cliBridgeRef.current;

    if (bridge) {
      try {
        if (!state.cliBridgeConnected) {
          await bridge.connect();
        }

        return await bridge.executeCommand(command, { cwd });
      } catch (bridgeError) {
        console.warn('CLI bridge execution failed, falling back to API client:', bridgeError);
      }
    }

    try {
      return await apiClient.executeCommand(command.split(' '), cwd);
    } catch (error) {
      console.error('Command execution failed:', error);
      throw error;
    }
  };

  const loadMetrics = async () => {
    try {
      // Try CLI/TUI bridge first for real-time metrics
      const bridge = cliBridgeRef.current;
      if (bridge && state.cliBridgeConnected) {
        try {
          const cliMetrics = await bridge.getSystemMetrics();
          if (cliMetrics) {
            const metrics = {
              cpuUsage: cliMetrics.cpu_usage || cliMetrics.cpuUsage || 0,
              memoryUsage: cliMetrics.memory_usage || cliMetrics.memoryUsage || 0,
              diskUsage: cliMetrics.disk_usage || cliMetrics.diskUsage || 0,
              activeProcesses: cliMetrics.active_processes || cliMetrics.activeProcesses || 0,
              uptime: cliMetrics.uptime || 0,
              gpuUsage: cliMetrics.gpu_usage || cliMetrics.gpuUsage,
              gpuMemoryUsed: cliMetrics.gpu_memory_used || cliMetrics.gpuMemoryUsed,
              gpuMemoryTotal: cliMetrics.gpu_memory_total || cliMetrics.gpuMemoryTotal,
              gpuMemoryUsage: cliMetrics.gpu_memory_usage || cliMetrics.gpuMemoryUsage,
              gpuTemperature: cliMetrics.gpu_temperature || cliMetrics.gpuTemperature,
              gpuName: cliMetrics.gpu_name || cliMetrics.gpuName,
              gpuVendor: cliMetrics.gpu_vendor || cliMetrics.gpuVendor,
            };
            dispatch({ type: 'SET_METRICS', payload: metrics });
            return;
          }
        } catch (bridgeError) {
          console.warn('CLI bridge metrics failed, falling back to API:', bridgeError);
        }
      }

      // Fallback to API
      const metrics = await apiClient.getSystemMetrics();
      dispatch({ type: 'SET_METRICS', payload: metrics });
    } catch (error) {
      console.error('Failed to load metrics:', error);
    }
  };

  const loadAITools = async () => apiClient.listAITools();
  const loadAISessions = async () => apiClient.listAISessions();
  const loadDevelopmentTasks = async () => apiClient.listDevelopmentTasks();

  const clearError = () => {
    dispatch({ type: 'SET_ERROR', payload: null });
  };

  const value: CodexContextType = {
    state,
    dispatch,
    login,
    logout,
    createConversation,
    sendMessage,
    loadConversations,
    selectConversation,
    runAgent,
    runSecurityScan,
    runResearch,
    runWebResearch,
    executeCommand,
    loadMetrics,
    loadAITools,
    loadAISessions,
    loadDevelopmentTasks,
    clearError,
  };

  return (
    <CodexContext.Provider value={value}>
      {children}
    </CodexContext.Provider>
  );
}

export function useCodex() {
  const context = useContext(CodexContext);
  if (context === undefined) {
    throw new Error('useCodex must be used within a CodexProvider');
  }
  return context;
}
