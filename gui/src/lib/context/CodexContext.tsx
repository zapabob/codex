'use client';

import React, { createContext, useContext, useReducer, useEffect, ReactNode } from 'react';
import {
  Conversation,
  Message,
  Agent,
  MCPConnection,
  SecurityScan,
  ResearchResult,
  SystemMetrics,
  NotificationItem,
  User,
  WebSocketMessage,
} from '../types';
import { apiClient } from '../api/client';
import { getSpecStory } from '../specstory';

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

  // System
  metrics: SystemMetrics | null;

  // UI State
  notifications: NotificationItem[];
  isLoading: boolean;
  error: string | null;

  // WebSocket
  isConnected: boolean;
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
  | { type: 'SET_METRICS'; payload: SystemMetrics }
  | { type: 'ADD_NOTIFICATION'; payload: NotificationItem }
  | { type: 'MARK_NOTIFICATION_READ'; payload: string }
  | { type: 'SET_LOADING'; payload: boolean }
  | { type: 'SET_ERROR'; payload: string | null }
  | { type: 'SET_CONNECTION_STATUS'; payload: boolean };

const initialState: CodexState = {
  user: null,
  isAuthenticated: false,
  conversations: [],
  currentConversation: null,
  messages: [],
  agents: [],
  activeAgents: [],
  mcpConnections: [],
  securityScans: [],
  researchResults: [],
  metrics: null,
  notifications: [],
  isLoading: false,
  error: null,
  isConnected: false,
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

    default:
      return state;
  }
}

type CommandResult = { exitCode: number; stdout: string; stderr: string };

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
  runSecurityScan: (type: string, target: string) => Promise<SecurityScan>;
  runResearch: (
    query: string,
    options?: {
      depth?: number;
      strategy?: 'comprehensive' | 'focused' | 'exploratory';
      useGemini?: boolean;
    },
  ) => Promise<ResearchResult>;
  executeCommand: (command: string, cwd?: string) => Promise<CommandResult>;
  loadMetrics: () => Promise<void>;
  clearError: () => void;
}

const CodexContext = createContext<CodexContextType | undefined>(undefined);

export function CodexProvider({ children }: { children: ReactNode }) {
  const [state, dispatch] = useReducer(codexReducer, initialState);

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

  const runSecurityScan = async (type: string, target: string): Promise<SecurityScan> => {
    try {
      const scan = await apiClient.runSecurityAudit({ path: target });
      dispatch({ type: 'ADD_SECURITY_SCAN', payload: scan });
      return scan;
    } catch (error) {
      console.error('Security scan failed:', error);
      throw error;
    }
  };

  const runResearch = async (query: string, options?: {
    depth?: number;
    strategy?: 'comprehensive' | 'focused' | 'exploratory';
    useGemini?: boolean;
  }): Promise<ResearchResult> => {
    try {
      dispatch({ type: 'SET_LOADING', payload: true });
      
      // Create initial research result with searching status
      const initialResult: ResearchResult = {
        id: `research-${Date.now()}`,
        query,
        status: 'searching',
        sources: [],
        startedAt: new Date(),
      };
      dispatch({ type: 'ADD_RESEARCH_RESULT', payload: initialResult });

      // Use GeminiCLI-based DeepResearch if available
      const researchResult = await apiClient.deepResearch({
        query,
        depth: options?.depth || 3,
        strategy: options?.strategy || 'comprehensive',
        useGemini: options?.useGemini !== false, // Default to true
      });

      // Update research result with completed status
      const completedResult: ResearchResult = {
        id: initialResult.id,
        query: researchResult.query || query,
        status: 'completed',
        summary: researchResult.summary,
        sources: (researchResult.sources || []).map((source, index) => ({
          url: source.url,
          title: source.title || `Source ${index + 1}`,
          snippet: source.snippet || '',
          confidence: source.relevance_score || 0.7,
        })),
        startedAt: initialResult.startedAt,
        completedAt: new Date(),
      };

      dispatch({ type: 'UPDATE_RESEARCH_RESULT', payload: completedResult });
      return completedResult;
    } catch (error) {
      console.error('Research failed:', error);
      
      // Update research result with failed status
      const failedResult: ResearchResult = {
        id: `research-${Date.now()}`,
        query,
        status: 'failed',
        sources: [],
        startedAt: new Date(),
        completedAt: new Date(),
      };
      dispatch({ type: 'ADD_RESEARCH_RESULT', payload: failedResult });
      
      dispatch({
        type: 'SET_ERROR',
        payload: error instanceof Error ? error.message : '研究の実行に失敗しました',
      });
      throw error;
    } finally {
      dispatch({ type: 'SET_LOADING', payload: false });
    }
  };

  const executeCommand = async (command: string, cwd?: string): Promise<CommandResult> => {
    try {
      const result = await apiClient.executeCommand(command.split(' '), cwd);
      return result;
    } catch (error) {
      console.error('Command execution failed:', error);
      throw error;
    }
  };

  const loadMetrics = async () => {
    try {
      const metrics = await apiClient.getSystemMetrics();
      dispatch({ type: 'SET_METRICS', payload: metrics });
    } catch (error) {
      console.error('Failed to load metrics:', error);
    }
  };

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
    executeCommand,
    loadMetrics,
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
