import React, { useEffect, useCallback, useRef } from 'react';
import { Box, Paper, Typography } from '@mui/material';
import { useChatStore } from '../../store';
import { ThreadList } from './ThreadList';
import { MessageBubble } from './MessageBubble';
import { InputArea } from './InputArea';
import { useMCPBridge } from '../../hooks/useMCPBridge';

export const ChatContainer: React.FC = () => {
  const {
    threads,
    activeThreadId,
    setActiveThread,
    addMessage,
    setMessages,
    isStreaming,
    setStreaming,
    appendStreamingMessage,
    finishStreaming,
    inputText,
    setInputText,
    isLoading,
    setIsLoading,
    error,
    setError,
    getActiveMessages,
  } = useChatStore();

  const messagesEndRef = useRef<HTMLDivElement>(null);
  const { bridge, connected } = useMCPBridge();

  const activeMessages = getActiveMessages();

  const scrollToBottom = useCallback(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, []);

  useEffect(() => {
    scrollToBottom();
  }, [activeMessages, scrollToBottom]);

  useEffect(() => {
    if (activeThreadId && bridge?.connected) {
      bridge?.request('chat/thread/messages', { threadId: activeThreadId })
        .then((messages) => setMessages(activeThreadId, messages as any[]))
        .catch((err) => setError(err.message));
    }
  }, [activeThreadId, bridge, setMessages, setError]);

  const handleSendMessage = useCallback(async () => {
    if (!inputText.trim() || !bridge?.connected) return;

    const userMessage = {
      id: crypto.randomUUID(),
      role: 'user' as const,
      content: inputText,
      timestamp: new Date(),
      attachments: [],
    };

    addMessage(activeThreadId!, userMessage);
    setInputText('');
    setIsLoading(true);

    try {
      setStreaming(true);

      for await (const chunk of bridge.streamChat([
        ...activeMessages.map((m) => ({
          role: m.role,
          content: m.content,
        })),
        { role: 'user', content: inputText },
      ])) {
        appendStreamingMessage(chunk);
      }

      finishStreaming();

      const assistantMessage = {
        id: crypto.randomUUID(),
        role: 'assistant' as const,
        content: useChatStore.getState().streamingMessage,
        timestamp: new Date(),
        attachments: [],
      };

      addMessage(activeThreadId!, assistantMessage);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to send message');
    } finally {
      setIsLoading(false);
      setStreaming(false);
    }
  }, [
    inputText,
    bridge,
    activeThreadId,
    activeMessages,
    addMessage,
    setInputText,
    setIsLoading,
    setStreaming,
    appendStreamingMessage,
    finishStreaming,
    setError,
  ]);

  const handleCreateThread = useCallback(async (title?: string) => {
    if (!bridge?.connected) return;

    try {
      const thread = await bridge.createThread(title);
      useChatStore.getState().addThread(thread);
      setActiveThread(thread.id);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create thread');
    }
  }, [bridge, setActiveThread, setError]);

  const handleDeleteThread = useCallback(async (threadId: string) => {
    if (!bridge?.connected) return;

    try {
      await bridge.deleteThread(threadId);
      useChatStore.getState().deleteThread(threadId);
      if (activeThreadId === threadId) {
        setActiveThread(null);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to delete thread');
    }
  }, [bridge, activeThreadId, setActiveThread, setError]);

  return (
    <Box sx={{ display: 'flex', height: '100%', overflow: 'hidden' }}>
      <ThreadList
        threads={threads}
        activeThreadId={activeThreadId}
        onSelectThread={setActiveThread}
        onCreateThread={handleCreateThread}
        onDeleteThread={handleDeleteThread}
      />

      <Box sx={{ flex: 1, display: 'flex', flexDirection: 'column', overflow: 'hidden' }}>
        <Box sx={{ flex: 1, overflow: 'auto', p: 2 }}>
          {!activeThreadId ? (
            <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'center', height: '100%' }}>
              <Typography variant="h6" color="text.secondary">
                Select a thread or create a new one
              </Typography>
            </Box>
          ) : (
            <>
              {activeMessages.map((message) => (
                <MessageBubble key={message.id} message={message} />
              ))}
              {isStreaming && (
                <MessageBubble
                  message={{
                    id: 'streaming',
                    role: 'assistant',
                    content: useChatStore.getState().streamingMessage,
                    timestamp: new Date(),
                    attachments: [],
                  }}
                  isStreaming
                />
              )}
              {isLoading && !isStreaming && (
                <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, p: 2 }}>
                  <Typography variant="body2" color="text.secondary">
                    Thinking...
                  </Typography>
                </Box>
              )}
              {error && (
                <Paper sx={{ p: 2, bgcolor: 'error.light', color: 'error.contrastText' }}>
                  <Typography>{error}</Typography>
                </Paper>
              )}
              <div ref={messagesEndRef} />
            </>
          )}
        </Box>

        {activeThreadId && (
          <InputArea
            value={inputText}
            onChange={setInputText}
            onSend={handleSendMessage}
            disabled={!connected || isLoading}
            placeholder={connected ? 'Type a message...' : 'Connecting...'}
          />
        )}
      </Box>
    </Box>
  );
};
