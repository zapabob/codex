import React, { useState, useCallback } from 'react';
import {
  Box,
  Paper,
  Typography,
  IconButton,
  Collapse,
  Avatar,
  Tooltip,
  Chip,
} from '@mui/material';
import {
  ContentCopy as CopyIcon,
  ThumbUp as ThumbUpIcon,
  ThumbDown as ThumbDownIcon,
  Code as CodeIcon,
  ExpandMore as ExpandIcon,
  ExpandLess as CollapseIcon,
} from '@mui/icons-material';
import type { ChatMessage, MessageRole } from '../../types/mcp';

interface MessageBubbleProps {
  message: ChatMessage;
  isStreaming?: boolean;
}

export const MessageBubble: React.FC<MessageBubbleProps> = ({ message, isStreaming = false }) => {
  const [expanded, setExpanded] = useState(true);
  const [copied, setCopied] = useState(false);

  const isUser = message.role === 'user';
  const isAssistant = message.role === 'assistant';

  const handleCopy = useCallback(async () => {
    await navigator.clipboard.writeText(message.content);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  }, [message.content]);

  const roleColors: Record<MessageRole, { bg: string; text: string; avatar: string }> = {
    user: { bg: 'primary.light', text: 'primary.contrastText', avatar: 'U' },
    assistant: { bg: 'grey.200', text: 'text.primary', avatar: 'AI' },
    system: { bg: 'warning.light', text: 'warning.contrastText', avatar: 'S' },
    tool: { bg: 'info.light', text: 'info.contrastText', avatar: 'T' },
  };

  const colors = roleColors[message.role];

  const formatTimestamp = (date: Date | string) => {
    return new Date(date).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  };

  const hasCode = message.content.includes('```');

  return (
    <Box
      sx={{
        display: 'flex',
        gap: 2,
        mb: 2,
        flexDirection: isUser ? 'row-reverse' : 'row',
      }}
    >
      <Tooltip title={message.role}>
        <Avatar
          sx={{
            bgcolor: colors.bg,
            color: colors.text,
            width: 36,
            height: 36,
            fontSize: 14,
            fontWeight: 600,
          }}
        >
          {colors.avatar}
        </Avatar>
      </Tooltip>

      <Box sx={{ maxWidth: '70%', flex: 1 }}>
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mb: 0.5 }}>
          <Typography variant="caption" color="text.secondary">
            {message.role === 'user' ? 'You' : message.role === 'assistant' ? 'Assistant' : message.role}
          </Typography>
          <Typography variant="caption" color="text.secondary">
            {formatTimestamp(message.timestamp)}
          </Typography>
          {isStreaming && (
            <Chip label="Streaming" size="small" color="info" sx={{ height: 20, fontSize: 10 }} />}
        </Box>

        <Paper
          elevation={isStreaming ? 2 : 0}
          sx={{
            p: 2,
            bgcolor: isUser ? 'primary.main' : 'background.paper',
            color: isUser ? 'primary.contrastText' : 'text.primary',
            borderRadius: 2,
            border: isStreaming ? '2px solid' : '1px solid',
            borderColor: isStreaming ? 'info.light' : 'divider',
            position: 'relative',
          }}
        >
          <Box
            sx={{
              whiteSpace: 'pre-wrap',
              wordBreak: 'break-word',
              fontFamily: message.content.includes('```') ? 'monospace' : 'inherit',
            }}
          >
            {message.content}
          </Box>

          {!isUser && message.content.length > 100 && (
            <IconButton
              size="small"
              onClick={() => setExpanded(!expanded)}
              sx={{
                position: 'absolute',
                bottom: -12,
                right: 8,
                bgcolor: 'background.paper',
                boxShadow: 1,
                '&:hover': { bgcolor: 'background.paper' },
              }}
            >
              {expanded ? <CollapseIcon fontSize="small" /> : <ExpandIcon fontSize="small" />}
            </IconButton>
          )}

          {!isStreaming && (
            <Box
              sx={{
                display: 'flex',
                gap: 0.5,
                mt: 1,
                pt: 1,
                borderTop: 1,
                borderColor: isUser ? 'primary.dark' : 'divider',
              }}
            >
              <Tooltip title={copied ? 'Copied!' : 'Copy'}>
                <IconButton
                  size="small"
                  onClick={handleCopy}
                  sx={{ color: isUser ? 'primary.contrastText' : 'text.secondary' }}
                >
                  <CopyIcon fontSize="small" />
                </IconButton>
              </Tooltip>
              {isAssistant && (
                <>
                  <Tooltip title="Good response">
                    <IconButton size="small" sx={{ color: isUser ? 'primary.contrastText' : 'text.secondary' }}>
                      <ThumbUpIcon fontSize="small" />
                    </IconButton>
                  </Tooltip>
                  <Tooltip title="Bad response">
                    <IconButton size="small" sx={{ color: isUser ? 'primary.contrastText' : 'text.secondary' }}>
                      <ThumbDownIcon fontSize="small" />
                    </IconButton>
                  </Tooltip>
                </>
              )}
            </Box>
          )}
        </Paper>
      </Box>
    </Box>
  );
};
