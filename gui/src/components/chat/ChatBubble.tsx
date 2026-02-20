import React, { useState, useCallback } from "react";
import {
  Box,
  Paper,
  Typography,
  IconButton,
  Tooltip,
  
  Chip,
  Menu,
  MenuItem,
  ListItemIcon,
} from "@mui/material";
import {
  ContentCopy as CopyIcon,
  ThumbUp as ThumbUpIcon,
  ThumbDown as ThumbDownIcon,
  Refresh as RegenerateIcon,
  Edit as EditIcon,
  MoreVert as MoreIcon,
  Check as CheckIcon,
} from "@mui/icons-material";
import type { ChatMessage, MessageRole } from "../../types/mcp";

interface ChatBubbleProps {
  message: ChatMessage;
  isStreaming?: boolean;
  onEdit?: (newContent: string) => void;
  onRegenerate?: () => void;
  onCopy?: () => void;
}

export const ChatBubble: React.FC<ChatBubbleProps> = ({
  message,
  isStreaming = false,
  onEdit,
  onRegenerate,
  onCopy,
}) => {
  const [expanded, setExpanded] = useState(true);
  const [copied, setCopied] = useState(false);
  const [editing, setEditing] = useState(false);
  const [editContent, setEditContent] = useState(message.content);
  const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null);

  const isUser = message.role === "user";
  const isAssistant = message.role === "assistant";

  const handleCopy = useCallback(async () => {
    await navigator.clipboard.writeText(message.content);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
    onCopy?.();
  }, [message.content, onCopy]);

  const handleMenuOpen = (event: React.MouseEvent<HTMLElement>) => {
    setAnchorEl(event.currentTarget);
  };

  const handleMenuClose = () => {
    setAnchorEl(null);
  };

  const handleSaveEdit = () => {
    if (editContent.trim() !== message.content) {
      onEdit?.(editContent.trim());
    }
    setEditing(false);
  };

  const handleCancelEdit = () => {
    setEditContent(message.content);
    setEditing(false);
  };

  const formatTimestamp = (date: Date | string) => {
    return new Date(date).toLocaleTimeString([], {
      hour: "2-digit",
      minute: "2-digit",
    });
  };

  const hasCode = message.content.includes("```");
  const shouldTruncate = message.content.length > 500 && !expanded;

  return (
    <Box
      sx={{
        display: "flex",
        gap: 2,
        mb: 2,
        px: 2,
        flexDirection: isUser ? "row-reverse" : "row",
      }}
    >
      {/* Role indicator (no avatar) */}
      <Box
        sx={{
          width: 32,
          display: "flex",
          alignItems: "flex-start",
          justifyContent: "center",
          pt: 0.5,
        }}
      >
        <Typography
          variant="caption"
          sx={{
            fontWeight: isUser ? 600 : 500,
            color: isUser ? "text.secondary" : "primary.main",
          }}
        >
          {isUser ? "You" : isAssistant ? "Claude" : message.role}
        </Typography>
      </Box>

      {/* Message content */}
      <Box sx={{ maxWidth: "75%", flex: 1 }}>
        {/* Metadata */}
        <Box sx={{ display: "flex", alignItems: "center", gap: 1, mb: 0.5 }}>
          <Typography variant="caption" color="text.secondary">
            {formatTimestamp(message.timestamp)}
          </Typography>
          {isStreaming && (
            <Chip
              label="Generating..."
              size="small"
              sx={{
                "height": 18,
                "fontSize": "0.65rem",
                "bgcolor": "primary.main",
                "color": "primary.contrastText",
                "animation": "pulse 1s infinite",
                "@keyframes pulse": {
                  "0%, 100%": { opacity: 1 },
                  "50%": { opacity: 0.7 },
                },
              }}
            />
          )}
        </Box>

        <Paper
          elevation={isStreaming ? 1 : 0}
          sx={{
            "p": 2,
            "bgcolor": isUser
              ? "chatGPT.userBubble"
              : "chatGPT.assistantBubble",
            "border": 1,
            "borderColor": isStreaming ? "primary.main" : "chatGPT.border",
            "borderRadius": 2,
            "position": "relative",
            "&:hover": {
              "& .message-actions": {
                opacity: 1,
              },
            },
          }}
        >
          {/* Edit mode */}
          {editing && isUser ? (
            <Box>
              <textarea
                value={editContent}
                onChange={(e) => setEditContent(e.target.value)}
                style={{
                  width: "100%",
                  minHeight: 100,
                  padding: 8,
                  borderRadius: 4,
                  border: "1px solid",
                  borderColor: "primary.main",
                  backgroundColor: "transparent",
                  color: "inherit",
                  fontFamily: "inherit",
                  fontSize: "inherit",
                  resize: "vertical",
                }}
              />
              <Box
                sx={{
                  display: "flex",
                  gap: 1,
                  mt: 1,
                  justifyContent: "flex-end",
                }}
              >
                <Typography
                  variant="caption"
                  color="text.secondary"
                  sx={{ mr: "auto", alignSelf: "center" }}
                >
                  {message.content.length} → {editContent.length} characters
                </Typography>
                <Typography
                  variant="caption"
                  color="text.secondary"
                  onClick={handleCancelEdit}
                  sx={{ cursor: "pointer", alignSelf: "center", mr: 2 }}
                >
                  Cancel
                </Typography>
                <Typography
                  variant="caption"
                  color="primary"
                  onClick={handleSaveEdit}
                  sx={{
                    cursor: "pointer",
                    alignSelf: "center",
                    fontWeight: 600,
                  }}
                >
                  Save & Submit
                </Typography>
              </Box>
            </Box>
          ) : (
            <>
              {/* Message text */}
              <Box
                sx={{
                  whiteSpace: "pre-wrap",
                  wordBreak: "break-word",
                  fontFamily: hasCode ? "monospace" : "inherit",
                  fontSize: "0.9375rem",
                  lineHeight: 1.6,
                }}
              >
                {shouldTruncate
                  ? message.content.slice(0, 500)
                  : message.content}
                {shouldTruncate && (
                  <Typography
                    component="span"
                    color="primary.main"
                    sx={{ cursor: "pointer", ml: 1, fontWeight: 500 }}
                    onClick={() => setExpanded(true)}
                  >
                    ...show more
                  </Typography>
                )}
              </Box>

              {/* Collapse button for long messages */}
              {!isUser && message.content.length > 500 && expanded && (
                <IconButton
                  size="small"
                  onClick={() => setExpanded(false)}
                  className="message-actions"
                  sx={{
                    "position": "absolute",
                    "bottom": -12,
                    "right": 8,
                    "bgcolor": "background.paper",
                    "boxShadow": 1,
                    "opacity": 0,
                    "transition": "opacity 0.2s",
                    "&:hover": { bgcolor: "action.hover" },
                  }}
                >
                  <Typography variant="caption" sx={{ px: 0.5 }}>
                    Show less
                  </Typography>
                </IconButton>
              )}

              {/* Actions bar */}
              {!isStreaming && (
                <Box
                  className="message-actions"
                  sx={{
                    display: "flex",
                    gap: 0.5,
                    mt: 1,
                    pt: 1,
                    borderTop: 1,
                    borderColor: "chatGPT.border",
                    opacity: 0,
                    transition: "opacity 0.2s",
                  }}
                >
                  {/* Copy button */}
                  <Tooltip title={copied ? "Copied!" : "Copy"}>
                    <IconButton size="small" onClick={handleCopy}>
                      {copied ? (
                        <CheckIcon fontSize="small" />
                      ) : (
                        <CopyIcon fontSize="small" />
                      )}
                    </IconButton>
                  </Tooltip>

                  {/* Assistant-specific actions */}
                  {isAssistant && (
                    <>
                      <Tooltip title="Good response">
                        <IconButton size="small">
                          <ThumbUpIcon fontSize="small" />
                        </IconButton>
                      </Tooltip>
                      <Tooltip title="Bad response">
                        <IconButton size="small">
                          <ThumbDownIcon fontSize="small" />
                        </IconButton>
                      </Tooltip>
                      <Tooltip title="Regenerate response">
                        <IconButton size="small" onClick={onRegenerate}>
                          <RegenerateIcon fontSize="small" />
                        </IconButton>
                      </Tooltip>
                    </>
                  )}

                  {/* User-specific actions */}
                  {isUser && (
                    <>
                      <Tooltip title="Edit message">
                        <IconButton
                          size="small"
                          onClick={() => setEditing(true)}
                        >
                          <EditIcon fontSize="small" />
                        </IconButton>
                      </Tooltip>
                      <Tooltip title="More options">
                        <IconButton size="small" onClick={handleMenuOpen}>
                          <MoreIcon fontSize="small" />
                        </IconButton>
                      </Tooltip>
                    </>
                  )}
                </Box>
              )}
            </>
          )}
        </Paper>
      </Box>

      {/* Context menu */}
      <Menu
        anchorEl={anchorEl}
        open={Boolean(anchorEl)}
        onClose={handleMenuClose}
        transformOrigin={{ horizontal: "right", vertical: "top" }}
        anchorOrigin={{ horizontal: "right", vertical: "bottom" }}
      >
        <MenuItem onClick={handleMenuClose}>
          <ListItemIcon>
            <CopyIcon fontSize="small" />
          </ListItemIcon>
          Copy
        </MenuItem>
        <MenuItem onClick={handleMenuClose}>
          <ListItemIcon>
            <EditIcon fontSize="small" />
          </ListItemIcon>
          Edit
        </MenuItem>
      </Menu>
    </Box>
  );
};

export default ChatBubble;
