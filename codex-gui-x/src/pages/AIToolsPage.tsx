import * as React from "react";
import { useState } from "react";
import { Box } from "@mui/material";
import { AIToolOrchestrator } from "../components/ai-tools/AIToolOrchestrator";
import { TaskDistributor } from "../components/ai-tools/TaskDistributor";
import { ResultIntegrator } from "../components/ai-tools/ResultIntegrator";
import type {
  AITool,
  DevelopmentTask,
  AISession,
  ExecutionResult,
} from "../types/ai-tools";

export const AIToolsPage: React.FC = () => {
  const [activeTab, setActiveTab] = React.useState<
    "orchestrator" | "distributor" | "integrator"
  >("orchestrator");

  const [aiTools] = useState<AITool[]>([
    {
      id: "codex-1",
      name: "Codex Engine v2",
      status: "available",
      capabilities: ["Code Generation", "Refactoring"],
      activeSessions: 0,
      maxSessions: 5,
      performance: { avgResponseTime: 1.2, successRate: 98, resourceUsage: 15 },
    },
    {
      id: "opencode-1",
      name: "OpenCode Llama",
      status: "running",
      capabilities: ["Documentation", "Testing"],
      activeSessions: 1,
      maxSessions: 3,
      performance: { avgResponseTime: 2.5, successRate: 92, resourceUsage: 45 },
    },
    {
      id: "claudecode-1",
      name: "Claude-3.5 Code",
      status: "busy",
      capabilities: ["Architecture", "Logic"],
      activeSessions: 2,
      maxSessions: 2,
      performance: { avgResponseTime: 1.8, successRate: 99, resourceUsage: 80 },
    },
    {
      id: "geminicli-1",
      name: "Gemini-2.0 Flash",
      status: "available",
      capabilities: ["Data Science", "Python"],
      activeSessions: 0,
      maxSessions: 10,
      performance: { avgResponseTime: 0.8, successRate: 96, resourceUsage: 10 },
    },
  ]);
  const [tasks, setTasks] = useState<DevelopmentTask[]>([]);
  const [sessions, setSessions] = useState<AISession[]>([]);
  const [executionResults, setExecutionResults] = useState<ExecutionResult[]>(
    [],
  );

  return (
    <Box data-testid="ai-tools-page" sx={{ height: "100%" }}>
      <Box
        sx={{ display: "flex", borderBottom: 1, borderColor: "divider", mb: 2 }}
      >
        {(["orchestrator", "distributor", "integrator"] as const).map((tab) => (
          <Box
            key={tab}
            onClick={() => setActiveTab(tab)}
            sx={{
              "px": 4,
              "py": 2,
              "cursor": "pointer",
              "borderBottom": 2,
              "borderColor": activeTab === tab ? "primary.main" : "transparent",
              "color": activeTab === tab ? "primary.main" : "text.secondary",
              "fontWeight": activeTab === tab ? 700 : 500,
              "textTransform": "uppercase",
              "fontSize": "0.75rem",
              "letterSpacing": "0.1em",
              "transition": "all 0.2s",
              "&:hover": {
                color: "primary.main",
              },
            }}
          >
            {tab}
          </Box>
        ))}
      </Box>

      <Box sx={{ height: "calc(100% - 50px)" }}>
        {activeTab === "orchestrator" && (
          <AIToolOrchestrator
            aiTools={aiTools}
            tasks={tasks}
            sessions={sessions}
            onTaskExecute={(task) => setTasks((prev) => [...prev, task])}
            onTaskComplete={(taskId, result) => {
              setExecutionResults((prev) => [...prev, result]);
              setTasks((prev) =>
                prev.map((t) =>
                  t.id === taskId
                    ? { ...t, status: "completed", progress: 100 }
                    : t,
                ),
              );
            }}
            onSessionUpdate={(session) =>
              setSessions((prev) => [...prev, session])
            }
          />
        )}
        {activeTab === "distributor" && (
          <TaskDistributor
            tasks={tasks}
            aiTools={aiTools}
            onTaskCreate={(task) => setTasks((prev) => [...prev, task])}
            onTaskUpdate={(task) =>
              setTasks((prev) => prev.map((t) => (t.id === task.id ? task : t)))
            }
          />
        )}
        {activeTab === "integrator" && (
          <ResultIntegrator
            results={executionResults}
            tasks={tasks}
            onResultAccept={(result) => console.log("Accepted:", result)}
            onResultReject={(result) =>
              setExecutionResults((prev) =>
                prev.filter((r) => r.taskId !== result.taskId),
              )
            }
          />
        )}
      </Box>
    </Box>
  );
};

export default AIToolsPage;
