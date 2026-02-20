import React from "react";
import {
  Box,
  Typography,
  Button,
  Card,
  CardContent,
  
  List,
  
  
  
  Divider,
} from "@mui/material";
import { PlayArrow, CheckCircle, Error, Schedule } from "@mui/icons-material";

interface Action {
  id: string;
  name: string;
  description: string;
  command: string;
  status: "idle" | "running" | "completed" | "failed";
  lastRun?: Date;
}

const actions: Action[] = [
  {
    id: "build",
    name: "Build",
    description: "Build the project",
    command: "pnpm build",
    status: "idle",
  },
  {
    id: "test",
    name: "Test",
    description: "Run tests",
    command: "pnpm test",
    status: "idle",
  },
  {
    id: "lint",
    name: "Lint",
    description: "Run linter",
    command: "pnpm lint",
    status: "idle",
  },
  {
    id: "dev",
    name: "Dev Server",
    description: "Start dev server",
    command: "pnpm dev",
    status: "running",
  },
  {
    id: "format",
    name: "Format",
    description: "Format code",
    command: "pnpm format",
    status: "idle",
  },
];

export const ActionsPanel: React.FC = () => {
  return (
    <Box sx={{ height: "100%", display: "flex", flexDirection: "column" }}>
      <Typography variant="h6" fontWeight={600} sx={{ mb: 2 }}>
        Quick Actions
      </Typography>

      <Box sx={{ flex: 1, overflow: "auto" }}>
        <List sx={{ p: 0 }}>
          {actions.map((action, index) => (
            <React.Fragment key={action.id}>
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
                    <Box sx={{ display: "flex", alignItems: "center", gap: 1 }}>
                      {action.status === "running" ? (
                        <Schedule
                          sx={{ color: "warning.main", fontSize: 20 }}
                        />
                      ) : action.status === "completed" ? (
                        <CheckCircle
                          sx={{ color: "success.main", fontSize: 20 }}
                        />
                      ) : action.status === "failed" ? (
                        <Error sx={{ color: "error.main", fontSize: 20 }} />
                      ) : (
                        <PlayArrow
                          sx={{ color: "text.secondary", fontSize: 20 }}
                        />
                      )}
                      <Box>
                        <Typography variant="subtitle2" fontWeight={600}>
                          {action.name}
                        </Typography>
                        <Typography variant="caption" color="text.secondary">
                          {action.description}
                        </Typography>
                      </Box>
                    </Box>
                    <Button
                      size="small"
                      variant="outlined"
                      startIcon={<PlayIcon />}
                      onClick={() => {
                        console.log("Running:", action.command);
                      }}
                    >
                      Run
                    </Button>
                  </Box>
                </CardContent>
              </Card>
              {index < actions.length - 1 && <Divider sx={{ my: 1 }} />}
            </React.Fragment>
          ))}
        </List>
      </Box>

      <Box sx={{ mt: 2, pt: 2, borderTop: 1, borderColor: "divider" }}>
        <Typography variant="caption" color="text.secondary">
          Actions are defined in .codex/actions/common.yaml
        </Typography>
      </Box>
    </Box>
  );
};

export default ActionsPanel;
