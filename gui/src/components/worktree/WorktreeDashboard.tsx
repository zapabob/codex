import React, { useState, useCallback } from "react";
import {
  Box,
  Typography,
  Button,
  Card,
  CardContent,
  Chip,
  IconButton,
  Tooltip,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  LinearProgress,
  List,
  
  
  
  Divider,
} from "@mui/material";
import {
  Add as AddIcon,
  Delete as DeleteIcon,
  GitBranch as BranchIcon,
  Play as PlayIcon,
  Stop as StopIcon,
  CheckCircle as SuccessIcon,
  Error as ErrorIcon,
  Schedule as PendingIcon,
  Folder as FolderIcon,
  Refresh as RefreshIcon,
} from "@mui/icons-material";
import { useWorktreeStore } from "../../store/useWorktreeStore";

interface Worktree {
  id: string;
  name: string;
  branch: string;
  status: "idle" | "running" | "error";
  lastActivity: Date;
  path: string;
}

const mockWorktrees: Worktree[] = [
  {
    id: "wt-1",
    name: "feature/auth",
    branch: "feature/authentication",
    status: "running",
    lastActivity: new Date(Date.now() - 1000 * 60 * 5),
    path: ".worktrees/wt-1",
  },
  {
    id: "wt-2",
    name: "bugfix/memory-leak",
    branch: "bugfix/memory-leak-fix",
    status: "idle",
    lastActivity: new Date(Date.now() - 1000 * 60 * 60),
    path: ".worktrees/wt-2",
  },
  {
    id: "wt-3",
    name: "refactor/api",
    branch: "refactor/api-cleanup",
    status: "error",
    lastActivity: new Date(Date.now() - 1000 * 60 * 60 * 2),
    path: ".worktrees/wt-3",
  },
];

export const WorktreeDashboard: React.FC = () => {
  const { worktrees, createWorktree, deleteWorktree } = useWorktreeStore();
  const [loading, setLoading] = useState(false);
  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [newBranchName, setNewBranchName] = useState("");
  const [newWorktreeName, setNewWorktreeName] = useState("");

  const handleCreate = useCallback(async () => {
    if (!newBranchName.trim()) return;

    setLoading(true);
    try {
      await createWorktree(".", newBranchName);
      setNewBranchName("");
      setNewWorktreeName("");
      setCreateDialogOpen(false);
    } catch (error) {
      console.error("Failed to create worktree:", error);
    } finally {
      setLoading(false);
    }
  }, [newBranchName, createWorktree]);

  const handleDelete = useCallback(
    async (id: string) => {
      if (window.confirm("Are you sure you want to delete this worktree?")) {
        await deleteWorktree(id);
      }
    },
    [deleteWorktree],
  );

  const formatTime = (date: Date) => {
    const diff = Date.now() - date.getTime();
    const minutes = Math.floor(diff / 60000);
    const hours = Math.floor(diff / 3600000);
    const days = Math.floor(diff / 86400000);

    if (minutes < 1) return "Just now";
    if (minutes < 60) return `${minutes}m ago`;
    if (hours < 24) return `${hours}h ago`;
    return `${days}d ago`;
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case "running":
        return <PlayIcon sx={{ color: "success.main", fontSize: 18 }} />;
      case "error":
        return <ErrorIcon sx={{ color: "error.main", fontSize: 18 }} />;
      default:
        return <PendingIcon sx={{ color: "text.secondary", fontSize: 18 }} />;
    }
  };

  const getStatusColor = (status: string): string => {
    switch (status) {
      case "running":
        return "success";
      case "error":
        return "error";
      default:
        return "default";
    }
  };

  const allWorktrees = [...mockWorktrees, ...worktrees];

  return (
    <Box sx={{ height: "100%", display: "flex", flexDirection: "column" }}>
      {/* Header */}
      <Box
        sx={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          mb: 2,
        }}
      >
        <Box sx={{ display: "flex", alignItems: "center", gap: 1 }}>
          <Typography variant="h6" fontWeight={600}>
            Worktrees
          </Typography>
          <Chip
            label={allWorktrees.length}
            size="small"
            sx={{ height: 20, fontSize: "0.75rem" }}
          />
        </Box>
        <Button
          variant="contained"
          size="small"
          startIcon={<AddIcon />}
          onClick={() => setCreateDialogOpen(true)}
        >
          New Worktree
        </Button>
      </Box>

      {/* Worktree List */}
      <Box sx={{ flex: 1, overflow: "auto" }}>
        {allWorktrees.length === 0 ? (
          <Box
            sx={{
              display: "flex",
              flexDirection: "column",
              alignItems: "center",
              justifyContent: "center",
              py: 4,
            }}
          >
            <FolderIcon sx={{ fontSize: 48, color: "text.disabled", mb: 2 }} />
            <Typography variant="body2" color="text.secondary">
              No worktrees yet
            </Typography>
            <Button
              variant="outlined"
              size="small"
              startIcon={<AddIcon />}
              onClick={() => setCreateDialogOpen(true)}
              sx={{ mt: 2 }}
            >
              Create your first worktree
            </Button>
          </Box>
        ) : (
          <List sx={{ p: 0 }}>
            {allWorktrees.map((worktree, index) => (
              <React.Fragment key={worktree.id}>
                <Card
                  sx={{
                    "mb": 1,
                    "bgcolor": "background.default",
                    "&:hover": {
                      bgcolor: "action.hover",
                    },
                  }}
                >
                  <CardContent sx={{ "py": 1.5, "&:last-child": { pb: 1.5 } }}>
                    <Box
                      sx={{
                        display: "flex",
                        alignItems: "center",
                        justifyContent: "space-between",
                      }}
                    >
                      <Box
                        sx={{
                          display: "flex",
                          alignItems: "center",
                          gap: 1.5,
                          flex: 1,
                        }}
                      >
                        {getStatusIcon(worktree.status)}
                        <Box sx={{ flex: 1, minWidth: 0 }}>
                          <Typography
                            variant="subtitle2"
                            fontWeight={600}
                            noWrap
                          >
                            {worktree.name}
                          </Typography>
                          <Box
                            sx={{
                              display: "flex",
                              alignItems: "center",
                              gap: 0.5,
                              mt: 0.25,
                            }}
                          >
                            <BranchIcon
                              sx={{ fontSize: 12, color: "text.secondary" }}
                            />
                            <Typography
                              variant="caption"
                              color="text.secondary"
                              noWrap
                            >
                              {worktree.branch}
                            </Typography>
                          </Box>
                        </Box>
                      </Box>

                      <Box
                        sx={{ display: "flex", alignItems: "center", gap: 1 }}
                      >
                        <Chip
                          label={worktree.status}
                          size="small"
                          color={getStatusColor(worktree.status) as any}
                          sx={{
                            height: 20,
                            fontSize: "0.7rem",
                            textTransform: "capitalize",
                          }}
                        />
                        <Typography variant="caption" color="text.disabled">
                          {formatTime(worktree.lastActivity)}
                        </Typography>
                        <Tooltip title="Delete worktree">
                          <IconButton
                            size="small"
                            onClick={() => handleDelete(worktree.id)}
                          >
                            <DeleteIcon fontSize="small" />
                          </IconButton>
                        </Tooltip>
                      </Box>
                    </Box>

                    {worktree.status === "running" && (
                      <Box sx={{ mt: 1 }}>
                        <LinearProgress
                          sx={{
                            height: 4,
                            borderRadius: 2,
                            bgcolor: "action.hover",
                          }}
                        />
                        <Typography
                          variant="caption"
                          color="text.secondary"
                          sx={{ mt: 0.5, display: "block" }}
                        >
                          Running: cargo test
                        </Typography>
                      </Box>
                    )}
                  </CardContent>
                </Card>
                {index < allWorktrees.length - 1 && <Divider sx={{ my: 1 }} />}
              </React.Fragment>
            ))}
          </List>
        )}
      </Box>

      {/* Footer */}
      <Box sx={{ pt: 2, borderTop: 1, borderColor: "divider" }}>
        <Typography variant="caption" color="text.secondary">
          Worktrees are isolated development environments. Use Ctrl/Cmd + Click
          to open in terminal.
        </Typography>
      </Box>

      {/* Create Dialog */}
      <Dialog
        open={createDialogOpen}
        onClose={() => setCreateDialogOpen(false)}
        maxWidth="sm"
        fullWidth
      >
        <DialogTitle>Create New Worktree</DialogTitle>
        <DialogContent>
          <Box sx={{ display: "flex", flexDirection: "column", gap: 2, mt: 1 }}>
            <TextField
              label="Worktree Name"
              value={newWorktreeName}
              onChange={(e) => setNewWorktreeName(e.target.value)}
              placeholder="feature/my-feature"
              fullWidth
            />
            <TextField
              label="Branch Name"
              value={newBranchName}
              onChange={(e) => setNewBranchName(e.target.value)}
              placeholder="feature/my-feature"
              helperText="Enter or select a branch name"
              fullWidth
            />
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setCreateDialogOpen(false)}>Cancel</Button>
          <Button
            variant="contained"
            onClick={handleCreate}
            disabled={!newBranchName.trim() || loading}
          >
            {loading ? "Creating..." : "Create"}
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
};

export default WorktreeDashboard;
