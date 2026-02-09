import React, { useCallback, useRef, useEffect } from "react";
import { Box, Typography, CircularProgress } from "@mui/material";
import { useChatStore } from "../../store/useChatStore";
import { ChatBubble } from "./ChatBubble";
import { InputArea } from "./InputArea";
import { WelcomeScreen } from "./WelcomeScreen";
import { useMCPBridge } from "../../hooks/useMCPBridge";

interface ChatContainerProps {
  welcomeMode?: boolean;
}

export const ChatContainer: React.FC<ChatContainerProps> = ({
  welcomeMode = false,
}) => {
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
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, []);

  useEffect(() => {
    scrollToBottom();
  }, [activeMessages, scrollToBottom]);

  useEffect(() => {
    if (activeThreadId && bridge?.connected) {
      bridge
        .request("chat/thread/messages", { threadId: activeThreadId })
        .then((messages) => setMessages(activeThreadId, messages as any[]))
        .catch((err) => setError(err.message));
    }
  }, [activeThreadId, bridge, setMessages, setError]);

  const handleSendMessage = useCallback(async () => {
    if (!inputText.trim() || !bridge?.connected) return;

    const userMessage = {
      id: crypto.randomUUID(),
      role: "user" as const,
      content: inputText,
      timestamp: new Date(),
      attachments: [],
    };

    addMessage(activeThreadId!, userMessage);
    setInputText("");
    setIsLoading(true);

    try {
      setStreaming(true);

      for await (const chunk of bridge.streamChat([
        ...activeMessages.map((m) => ({
          role: m.role,
          content: m.content,
        })),
        { role: "user", content: inputText },
      ])) {
        appendStreamingMessage(chunk);
      }

      finishStreaming();

      const assistantMessage = {
        id: crypto.randomUUID(),
        role: "assistant" as const,
        content: useChatStore.getState().streamingMessage,
        timestamp: new Date(),
        attachments: [],
      };

      addMessage(activeThreadId!, assistantMessage);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to send message");
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

  const handleCreateThread = useCallback(
    async (title?: string) => {
      if (!bridge?.connected) return;

      try {
        const thread = await bridge.createThread(title);
        useChatStore.getState().addThread(thread);
        setActiveThread(thread.id);
      } catch (err) {
        setError(
          err instanceof Error ? err.message : "Failed to create thread",
        );
      }
    },
    [bridge, setActiveThread, setError],
  );

  const handleDeleteThread = useCallback(
    async (threadId: string) => {
      if (!bridge?.connected) return;

      try {
        await bridge.deleteThread(threadId);
        useChatStore.getState().deleteThread(threadId);
        if (activeThreadId === threadId) {
          setActiveThread(null);
        }
      } catch (err) {
        setError(
          err instanceof Error ? err.message : "Failed to delete thread",
        );
      }
    },
    [bridge, activeThreadId, setActiveThread, setError],
  );

  const handleSelectSuggestion = useCallback(
    (prompt: string) => {
      if (!activeThreadId) {
        handleCreateThread(prompt.slice(0, 50));
      }
      setInputText(prompt);
    },
    [activeThreadId, handleCreateThread, setInputText],
  );

  return (
    <Box
      sx={{
        display: "flex",
        flexDirection: "column",
        height: "100%",
        overflow: "hidden",
      }}
    >
      {/* Messages Area */}
      <Box
        sx={{
          flex: 1,
          overflow: "auto",
          px: 2,
          py: 2,
          bgcolor: "background.default",
        }}
      >
        {!activeThreadId && welcomeMode ? (
          <WelcomeScreen onSelectSuggestion={handleSelectSuggestion} />
        ) : !activeThreadId ? (
          <Box
            sx={{
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
              height: "100%",
            }}
          >
            <Typography variant="body1" color="text.secondary">
              Select a thread or start a new conversation
            </Typography>
          </Box>
        ) : (
          <>
            {activeMessages.map((message) => (
              <ChatBubble
                key={message.id}
                message={message}
                isStreaming={false}
              />
            ))}
            {isStreaming && (
              <ChatBubble
                message={{
                  id: "streaming",
                  role: "assistant",
                  content: useChatStore.getState().streamingMessage,
                  timestamp: new Date(),
                  attachments: [],
                }}
                isStreaming
              />
            )}
            {isLoading && !isStreaming && (
              <Box
                sx={{
                  display: "flex",
                  alignItems: "center",
                  gap: 1,
                  p: 2,
                }}
              >
                <CircularProgress size={16} />
                <Typography variant="body2" color="text.secondary">
                  Thinking...
                </Typography>
              </Box>
            )}
            {error && (
              <Box
                sx={{
                  p: 2,
                  bgcolor: "error.main",
                  color: "error.contrastText",
                  borderRadius: 1,
                  mt: 2,
                }}
              >
                <Typography variant="body2">{error}</Typography>
              </Box>
            )}
            <div ref={messagesEndRef} />
          </>
        )}
      </Box>

      {/* Input Area */}
      {activeThreadId && (
        <InputArea
          value={inputText}
          onChange={setInputText}
          onSend={handleSendMessage}
          disabled={!connected || isLoading}
          placeholder={connected ? "Type a message..." : "Connecting..."}
        />
      )}
    </Box>
  );
};

export default ChatContainer;
