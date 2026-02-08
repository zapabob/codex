import { create } from 'zustand';
import { devtools, persist } from 'zustand/middleware';
import type { ChatThread, ChatMessage, MessageRole, Attachment } from '../types/mcp';

interface ChatState {
  threads: ChatThread[];
  activeThreadId: string | null;
  messages: Map<string, ChatMessage[]>;
  isStreaming: boolean;
  streamingMessage: string;
  inputText: string;
  isLoading: boolean;
  error: string | null;
  suggestions: string[];
  quickActions: Array<{ id: string; label: string; icon: string }>;

  setThreads: (threads: ChatThread[]) => void;
  addThread: (thread: ChatThread) => void;
  updateThread: (threadId: string, updates: Partial<ChatThread>) => void;
  deleteThread: (threadId: string) => void;
  setActiveThread: (threadId: string | null) => void;

  setMessages: (threadId: string, messages: ChatMessage[]) => void;
  addMessage: (threadId: string, message: ChatMessage) => void;
  updateMessage: (threadId: string, messageId: string, updates: Partial<ChatMessage>) => void;
  deleteMessage: (threadId: string, messageId: string) => void;

  setStreaming: (streaming: boolean) => void;
  setStreamingMessage: (message: string) => void;
  appendStreamingMessage: (chunk: string) => void;
  finishStreaming: () => void;

  setInputText: (text: string) => void;
  setIsLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  setSuggestions: (suggestions: string[]) => void;
  setQuickActions: (actions: Array<{ id: string; label: string; icon: string }>) => void;

  clearChat: () => void;
  getActiveMessages: () => ChatMessage[];
  getActiveThread: () => ChatThread | null;
}

export const useChatStore = create<ChatState>()(
  devtools(
    persist(
      (set, get) => ({
        threads: [],
        activeThreadId: null,
        messages: new Map(),
        isStreaming: false,
        streamingMessage: '',
        inputText: '',
        isLoading: false,
        error: null,
        suggestions: [],
        quickActions: [
          { id: 'explain', label: 'Explain code', icon: 'info' },
          { id: 'refactor', label: 'Refactor', icon: 'refresh' },
          { id: 'debug', label: 'Debug', icon: 'bug' },
          { id: 'test', label: 'Write tests', icon: 'check' },
        ],

        setThreads: (threads) => set({ threads }),
        addThread: (thread) => set((state) => ({
          threads: [...state.threads, thread],
        })),
        updateThread: (threadId, updates) => set((state) => ({
          threads: state.threads.map((t) =>
            t.id === threadId ? { ...t, ...updates } : t
          ),
        })),
        deleteThread: (threadId) => set((state) => ({
          threads: state.threads.filter((t) => t.id !== threadId),
          activeThreadId: state.activeThreadId === threadId ? null : state.activeThreadId,
        })),
        setActiveThread: (threadId) => set({ activeThreadId: threadId }),

        setMessages: (threadId, messages) => set((state) => {
          const newMessages = new Map(state.messages);
          newMessages.set(threadId, messages);
          return { messages: newMessages };
        }),
        addMessage: (threadId, message) => set((state) => {
          const newMessages = new Map(state.messages);
          const existing = newMessages.get(threadId) || [];
          newMessages.set(threadId, [...existing, message]);
          return { messages: newMessages };
        }),
        updateMessage: (threadId, messageId, updates) => set((state) => {
          const newMessages = new Map(state.messages);
          const existing = newMessages.get(threadId) || [];
          newMessages.set(threadId, existing.map((m) =>
            m.id === messageId ? { ...m, ...updates } : m
          ));
          return { messages: newMessages };
        }),
        deleteMessage: (threadId, messageId) => set((state) => {
          const newMessages = new Map(state.messages);
          const existing = newMessages.get(threadId) || [];
          newMessages.set(threadId, existing.filter((m) => m.id !== messageId));
          return { messages: newMessages };
        }),

        setStreaming: (streaming) => set({ isStreaming: streaming }),
        setStreamingMessage: (message) => set({ streamingMessage: message }),
        appendStreamingMessage: (chunk) => set((state) => ({
          streamingMessage: state.streamingMessage + chunk,
        })),
        finishStreaming: () => set((state) => {
          if (state.activeThreadId && state.streamingMessage) {
            const newMessages = new Map(state.messages);
            const existing = newMessages.get(state.activeThreadId) || [];
            const lastMessage = existing[existing.length - 1];
            if (lastMessage?.role === 'assistant') {
              newMessages.set(state.activeThreadId, [
                ...existing.slice(0, -1),
                { ...lastMessage, content: state.streamingMessage },
              ]);
            }
            return { messages: newMessages, streamingMessage: '', isStreaming: false };
          }
          return { streamingMessage: '', isStreaming: false };
        }),

        setInputText: (text) => set({ inputText: text }),
        setIsLoading: (loading) => set({ isLoading: loading }),
        setError: (error) => set({ error }),
        setSuggestions: (suggestions) => set({ suggestions }),
        setQuickActions: (actions) => set({ quickActions: actions }),

        clearChat: () => set({
          inputText: '',
          streamingMessage: '',
          isStreaming: false,
          isLoading: false,
          error: null,
        }),
        getActiveMessages: () => {
          const state = get();
          return state.activeThreadId
            ? state.messages.get(state.activeThreadId) || []
            : [];
        },
        getActiveThread: () => {
          const state = get();
          return state.activeThreadId
            ? state.threads.find((t) => t.id === state.activeThreadId) || null
            : null;
        },
      }),
      {
        name: 'codex-chat-store',
        partialize: (state) => ({
          threads: state.threads.slice(-50),
          activeThreadId: state.activeThreadId,
        }),
      }
    ),
    { name: 'ChatStore' }
  )
);
