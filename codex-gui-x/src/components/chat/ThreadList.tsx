import React, { useState, useCallback } from 'react';
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
} from '@mui/material';
import {
  Add as AddIcon,
  Delete as DeleteIcon,
  MoreVert as MoreIcon,
  ChatBubbleOutline as ChatIcon,
  AutoAwesome as AIIcon,
} from '@mui/icons-material';
import type { ChatThread } from '../../types/mcp';

interface ThreadListProps {
  threads: ChatThread[];
  activeThreadId: string | null;
  onSelectThread: (threadId: string) => void;
  onCreateThread: (title?: string) => void;
  onDeleteThread: (threadId: string) => void;
}

export const ThreadList: React.FC<ThreadListProps> = ({
  threads,
  activeThreadId,
  onSelectThread,
  onCreateThread,
  onDeleteThread,
}) => {
  const [anchorEl, setAnchorEl] = useState<HTMLElement | null>(null);
  const [selectedThreadId, setSelectedThreadId] = useState<string | null>(null);
  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [newThreadTitle, setNewThreadTitle] = useState('');

  const handleMenuOpen = useCallback((event: React.MouseEvent<HTMLElement>, threadId: string) => {
    event.stopPropagation();
    setAnchorEl(event.currentTarget);
    setSelectedThreadId(threadId);
  }, []);

  const handleMenuClose = useCallback(() => {
    setAnchorEl(null);
    setSelectedThreadId(null);
  }, []);

  const handleDelete = useCallback(() => {
    if (selectedThreadId) {
      onDeleteThread(selectedThreadId);
    }
    handleMenuClose();
  }, [selectedThreadId, onDeleteThread, handleMenuClose]);

  const handleCreate = useCallback(() => {
    onCreateThread(newThreadTitle.trim() || undefined);
    setNewThreadTitle('');
    setCreateDialogOpen(false);
  }, [newThreadTitle, onCreateThread]);

  const formatDate = useCallback((date: Date | string) => {
    const d = new Date(date);
    const now = new Date();
    const diff = now.getTime() - d.getTime();
    const days = Math.floor(diff / (1000 * 60 * 60 * 24));

    if (days === 0) return d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    if (days === 1) return 'Yesterday';
    if (days < 7) return d.toLocaleDateString([], { weekday: 'short' });
    return d.toLocaleDateString([], { month: 'short', day: 'numeric' });
  }, []);

  return (
    <Box
      sx={{
        width: 280,
        borderRight: 1,
        borderColor: 'divider',
        display: 'flex',
        flexDirection: 'column',
        bgcolor: 'background.paper',
      }}
    >
      <Box sx={{ p: 2, borderBottom: 1, borderColor: 'divider' }}>
        <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', mb: 1 }}>
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
            <AIIcon color="primary" />
            <Typography variant="h6" fontWeight={600}>
              Codex
            </Typography>
          </Box>
          <Tooltip title="New Thread">
            <IconButton size="small" onClick={() => setCreateDialogOpen(true)}>
              <AddIcon />
            </IconButton>
          </Tooltip>
        </Box>
      </Box>

      <List sx={{ flex: 1, overflow: 'auto', py: 1 }}>
        {threads.length === 0 ? (
          <Box sx={{ p: 2, textAlign: 'center' }}>
            <Typography variant="body2" color="text.secondary">
              No threads yet
            </Typography>
          </Box>
        ) : (
          threads.map((thread) => (
            <ListItemButton
              key={thread.id}
              selected={thread.id === activeThreadId}
              onClick={() => onSelectThread(thread.id)}
              sx={{ borderRadius: 1, mx: 1, mb: 0.5 }}
            >
              <ListItemIcon sx={{ minWidth: 36 }}>
                <ChatIcon fontSize="small" color={thread.id === activeThreadId ? 'primary' : 'inherit'} />
              </ListItemIcon>
              <ListItemText
                primary={thread.title || 'Untitled'}
                secondary={thread.lastMessageAt ? formatDate(thread.lastMessageAt) : 'New'}
                primaryTypographyProps={{
                  noWrap: true,
                  fontWeight: thread.id === activeThreadId ? 600 : 400,
                }}
                secondaryTypographyProps={{ noWrap: true }}
              />
              <IconButton
                size="small"
                onClick={(e) => handleMenuOpen(e, thread.id)}
                sx={{ opacity: thread.id === activeThreadId ? 1 : 0 }}
              >
                <MoreIcon fontSize="small" />
              </IconButton>
            </ListItemButton>
          ))
        )}
      </List>

      <Menu anchorEl={anchorEl} open={Boolean(anchorEl)} onClose={handleMenuClose}>
        <MenuItem onClick={handleDelete}>
          <DeleteIcon fontSize="small" sx={{ mr: 1 }} />
          Delete
        </MenuItem>
      </Menu>

      <Dialog open={createDialogOpen} onClose={() => setCreateDialogOpen(false)} maxWidth="xs" fullWidth>
        <Box sx={{ p: 3 }}>
          <Typography variant="h6" gutterBottom>
            New Thread
          </Typography>
          <TextField
            autoFocus
            fullWidth
            label="Title (optional)"
            value={newThreadTitle}
            onChange={(e) => setNewThreadTitle(e.target.value)}
            sx={{ mb: 2 }}
          />
          <Box sx={{ display: 'flex', justifyContent: 'flex-end', gap: 1 }}>
            <Button onClick={() => setCreateDialogOpen(false)}>Cancel</Button>
            <Button variant="contained" onClick={handleCreate}>
              Create
            </Button>
          </Box>
        </Box>
      </Dialog>
    </Box>
  );
};
