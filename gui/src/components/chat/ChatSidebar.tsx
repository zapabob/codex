import React, { useState, useCallback } from "react";
import {
  Box,
  List,
  ListItemButton,
  ListItemText,
  ListItemIcon,
  IconButton,
  Typography,
  Menu,
  MenuItem,
  Dialog,
  TextField,
  Button,
  Tooltip,
  
  InputBase,
  alpha,
} from "@mui/material";
import {
  Add as AddIcon,
  Delete as DeleteIcon,
  MoreVert as MoreIcon,
  ChatBubbleOutline as ChatIcon,
  Search as SearchIcon,
  Settings as SettingsIcon,
  DarkMode as DarkModeIcon,
  Notifications as NotificationsIcon,
  PushPin as PinIcon,
  PushPinOutlined as PinOutlineIcon,
} from "@mui/icons-material";
import { useTheme } from "@mui/material/styles";
import type { ChatThread } from "../../types/mcp";
import { useWorktreeStore } from "../../store/useWorktreeStore";

interface ChatSidebarProps {
  threads: ChatThread[];
  activeThreadId: string | null;
  onSelectThread: (threadId: string) => void;
  onCreateThread: (title?: string) => void;
  onDeleteThread: (threadId: string) => void;
  onRenameThread: (threadId: string, newTitle: string) => void;
  isDarkMode: boolean;
  onToggleTheme: () => void;
  notificationCount?: number;
  onOpenNotifications?: () => void;
}

export const ChatSidebar: React.FC<ChatSidebarProps> = ({
  threads,
  activeThreadId,
  onSelectThread,
  onCreateThread,
  onDeleteThread,
  onRenameThread,
  isDarkMode,
  onToggleTheme,
  notificationCount = 0,
  onOpenNotifications,
}) => {
  const theme = useTheme();
  const { pinnedThreads } = useWorktreeStore();

  const [anchorEl, setAnchorEl] = useState<HTMLElement | null>(null);
  const [selectedThreadId, setSelectedThreadId] = useState<string | null>(null);
  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [newThreadTitle, setNewThreadTitle] = useState("");
  const [renameDialogOpen, setRenameDialogOpen] = useState(false);
  const [newThreadName, setNewThreadName] = useState("");
  const [searchQuery, setSearchQuery] = useState("");
  const [collapsed, setCollapsed] = useState(false);

  const pinned = threads.filter((t) => pinnedThreads.includes(t.id));
  const unpinned = threads.filter((t) => !pinnedThreads.includes(t.id));

  const filteredThreads = searchQuery
    ? threads.filter((t) =>
        (t.title || "Untitled")
          .toLowerCase()
          .includes(searchQuery.toLowerCase()),
      )
    : threads;

  const handleMenuOpen = (
    event: React.MouseEvent<HTMLElement>,
    threadId: string,
  ) => {
    event.stopPropagation();
    setAnchorEl(event.currentTarget);
    setSelectedThreadId(threadId);
  };

  const handleMenuClose = () => {
    setAnchorEl(null);
    setSelectedThreadId(null);
  };

  const handleDelete = () => {
    if (selectedThreadId) {
      onDeleteThread(selectedThreadId);
    }
    handleMenuClose();
  };

  const handleRenameOpen = () => {
    const thread = threads.find((t) => t.id === selectedThreadId);
    if (thread) {
      setNewThreadName(thread.title || "Untitled");
      setRenameDialogOpen(true);
    }
    handleMenuClose();
  };

  const handleRename = () => {
    if (selectedThreadId && newThreadName.trim()) {
      onRenameThread(selectedThreadId, newThreadName.trim());
    }
    setRenameDialogOpen(false);
  };

  const handleCreate = () => {
    onCreateThread(newThreadTitle.trim() || undefined);
    setNewThreadTitle("");
    setCreateDialogOpen(false);
  };

  const formatDate = useCallback((date: Date | string) => {
    const d = new Date(date);
    const now = new Date();
    const diff = now.getTime() - d.getTime();
    const days = Math.floor(diff / (1000 * 60 * 60 * 24));

    if (days === 0)
      return d.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
    if (days === 1) return "Yesterday";
    if (days < 7) return d.toLocaleDateString([], { weekday: "short" });
    return d.toLocaleDateString([], { month: "short", day: "numeric" });
  }, []);

  if (collapsed) {
    return (
      <Box
        sx={{
          width: 64,
          borderRight: 1,
          borderColor: "divider",
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          py: 2,
          bgcolor: "background.paper",
        }}
      >
        <IconButton onClick={() => setCollapsed(false)} sx={{ mb: 2 }}>
          <ChatIcon />
        </IconButton>
        <Box sx={{ flex: 1 }}>
          {filteredThreads.slice(0, 10).map((thread) => (
            <Tooltip
              key={thread.id}
              title={thread.title || "Untitled"}
              placement="right"
            >
              <IconButton
                size="small"
                onClick={() => onSelectThread(thread.id)}
                sx={{
                  mb: 0.5,
                  opacity: thread.id === activeThreadId ? 1 : 0.6,
                }}
              >
                <ChatIcon fontSize="small" />
              </IconButton>
            </Tooltip>
          ))}
        </Box>
        <IconButton onClick={onToggleTheme} size="small" sx={{ mt: 2 }}>
          <DarkModeIcon fontSize="small" />
        </IconButton>
      </Box>
    );
  }

  return (
    <Box
      sx={{
        width: 280,
        borderRight: 1,
        borderColor: "divider",
        display: "flex",
        flexDirection: "column",
        bgcolor: "background.paper",
      }}
    >
      {/* Header */}
      <Box sx={{ p: 2, borderBottom: 1, borderColor: "divider" }}>
        <Box
          sx={{
            display: "flex",
            alignItems: "center",
            justifyContent: "space-between",
            mb: 2,
          }}
        >
          <Box sx={{ display: "flex", alignItems: "center", gap: 1 }}>
            <ChatIcon color="primary" />
            <Typography variant="h6" fontWeight={600}>
              Codex
            </Typography>
          </Box>
          <IconButton size="small" onClick={() => setCollapsed(true)}>
            <ChatIcon fontSize="small" />
          </IconButton>
        </Box>

        {/* New Chat button */}
        <Button
          fullWidth
          variant="outlined"
          startIcon={<AddIcon />}
          onClick={() => setCreateDialogOpen(true)}
          sx={{
            "mb": 2,
            "justifyContent": "flex-start",
            "borderColor": "divider",
            "color": "text.primary",
            "&:hover": {
              bgcolor: "action.hover",
              borderColor: "divider",
            },
          }}
        >
          New chat
        </Button>

        {/* Search */}
        <Box
          sx={{
            "position": "relative",
            "borderRadius": 1,
            "bgcolor": alpha(theme.palette.text.primary, 0.04),
            "&:hover": {
              bgcolor: alpha(theme.palette.text.primary, 0.08),
            },
          }}
        >
          <Box
            sx={{
              px: 1.5,
              height: 36,
              display: "flex",
              alignItems: "center",
              position: "absolute",
              pointerEvents: "none",
            }}
          >
            <SearchIcon sx={{ fontSize: 18, color: "text.secondary" }} />
          </Box>
          <InputBase
            placeholder="Search conversations..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            sx={{
              "color": "text.primary",
              "width": "100%",
              "& input": {
                pl: 5,
                py: 0.75,
                fontSize: "0.875rem",
              },
            }}
          />
        </Box>
      </Box>

      {/* Threads list */}
      <List sx={{ flex: 1, overflow: "auto", py: 1 }}>
        {/* Pinned threads */}
        {pinned.length > 0 && (
          <>
            <Typography
              variant="overline"
              sx={{ px: 2, py: 1, color: "text.secondary", fontSize: "0.7rem" }}
            >
              Pinned
            </Typography>
            {pinned.map((thread) => (
              <ThreadItem
                key={thread.id}
                thread={thread}
                isActive={thread.id === activeThreadId}
                onSelect={() => onSelectThread(thread.id)}
                onMenuOpen={(e) => handleMenuOpen(e, thread.id)}
                formatDate={formatDate}
              />
            ))}
          </>
        )}

        {/* Recent threads */}
        {unpinned.length > 0 && (
          <>
            <Typography
              variant="overline"
              sx={{ px: 2, py: 1, color: "text.secondary", fontSize: "0.7rem" }}
            >
              Recent
            </Typography>
            {filteredThreads
              .filter((t) => !pinnedThreads.includes(t.id))
              .map((thread) => (
                <ThreadItem
                  key={thread.id}
                  thread={thread}
                  isActive={thread.id === activeThreadId}
                  onSelect={() => onSelectThread(thread.id)}
                  onMenuOpen={(e) => handleMenuOpen(e, thread.id)}
                  formatDate={formatDate}
                />
              ))}
          </>
        )}

        {filteredThreads.length === 0 && searchQuery && (
          <Box sx={{ p: 3, textAlign: "center" }}>
            <Typography variant="body2" color="text.secondary">
              No conversations found
            </Typography>
          </Box>
        )}
      </List>

      {/* Footer */}
      <Box sx={{ p: 2, borderTop: 1, borderColor: "divider" }}>
        <Box sx={{ display: "flex", gap: 1 }}>
          <Tooltip title="Notifications">
            <IconButton size="small" onClick={onOpenNotifications}>
              <NotificationsIcon />
              {notificationCount > 0 && (
                <Box
                  sx={{
                    position: "absolute",
                    top: 4,
                    right: 4,
                    width: 8,
                    height: 8,
                    borderRadius: "50%",
                    bgcolor: "error.main",
                  }}
                />
              )}
            </IconButton>
          </Tooltip>
          <Tooltip title="Settings">
            <IconButton size="small">
              <SettingsIcon />
            </IconButton>
          </Tooltip>
          <Tooltip title={isDarkMode ? "Light mode" : "Dark mode"}>
            <IconButton size="small" onClick={onToggleTheme}>
              <DarkModeIcon />
            </IconButton>
          </Tooltip>
        </Box>
      </Box>

      {/* Context menu */}
      <Menu
        anchorEl={anchorEl}
        open={Boolean(anchorEl)}
        onClose={handleMenuClose}
      >
        <MenuItem onClick={handleRenameOpen}>
          <ListItemIcon>
            <EditIcon fontSize="small" />
          </ListItemIcon>
          Rename
        </MenuItem>
        <MenuItem onClick={handleDelete}>
          <ListItemIcon>
            <DeleteIcon fontSize="small" />
          </ListItemIcon>
          Delete
        </MenuItem>
      </Menu>

      {/* Create dialog */}
      <Dialog
        open={createDialogOpen}
        onClose={() => setCreateDialogOpen(false)}
        maxWidth="xs"
        fullWidth
      >
        <Box sx={{ p: 3 }}>
          <Typography variant="h6" gutterBottom>
            New chat
          </Typography>
          <TextField
            autoFocus
            fullWidth
            label="Title (optional)"
            value={newThreadTitle}
            onChange={(e) => setNewThreadTitle(e.target.value)}
            sx={{ mb: 2 }}
          />
          <Box sx={{ display: "flex", justifyContent: "flex-end", gap: 1 }}>
            <Button onClick={() => setCreateDialogOpen(false)}>Cancel</Button>
            <Button variant="contained" onClick={handleCreate}>
              Create
            </Button>
          </Box>
        </Box>
      </Dialog>

      {/* Rename dialog */}
      <Dialog
        open={renameDialogOpen}
        onClose={() => setRenameDialogOpen(false)}
        maxWidth="xs"
        fullWidth
      >
        <Box sx={{ p: 3 }}>
          <Typography variant="h6" gutterBottom>
            Rename conversation
          </Typography>
          <TextField
            autoFocus
            fullWidth
            value={newThreadName}
            onChange={(e) => setNewThreadName(e.target.value)}
            sx={{ mb: 2 }}
          />
          <Box sx={{ display: "flex", justifyContent: "flex-end", gap: 1 }}>
            <Button onClick={() => setRenameDialogOpen(false)}>Cancel</Button>
            <Button variant="contained" onClick={handleRename}>
              Save
            </Button>
          </Box>
        </Box>
      </Dialog>
    </Box>
  );
};

interface ThreadItemProps {
  thread: ChatThread;
  isActive: boolean;
  onSelect: () => void;
  onMenuOpen: (event: React.MouseEvent<HTMLElement>) => void;
  formatDate: (date: Date | string) => string;
  isPinned?: boolean;
}

const ThreadItem: React.FC<ThreadItemProps> = ({
  thread,
  isActive,
  onSelect,
  onMenuOpen,
  formatDate,
  isPinned = false,
}) => {
  return (
    <ListItemButton
      selected={isActive}
      onClick={onSelect}
      sx={{
        borderRadius: 1,
        mx: 1,
        mb: 0.5,
        opacity: isActive ? 1 : 0.85,
      }}
    >
      <ListItemIcon sx={{ minWidth: 36 }}>
        <ChatIcon fontSize="small" color={isActive ? "primary" : "inherit"} />
      </ListItemIcon>
      <ListItemText
        primary={thread.title || "Untitled"}
        secondary={
          thread.lastMessageAt ? formatDate(thread.lastMessageAt) : "New"
        }
        primaryTypographyProps={{
          noWrap: true,
          fontWeight: isActive ? 600 : 400,
          fontSize: "0.875rem",
        }}
        secondaryTypographyProps={{
          noWrap: true,
          fontSize: "0.75rem",
        }}
      />
      <IconButton size="small" onClick={onMenuOpen} sx={{ opacity: 0, ml: 1 }}>
        <MoreIcon fontSize="small" />
      </IconButton>
    </ListItemButton>
  );
};

export default ChatSidebar;
