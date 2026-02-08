import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import { Suspense, lazy } from "react";
import { Box, CircularProgress } from "@mui/material";
import { DashboardLayout } from "./components/layout/DashboardLayout";
import "./i18n";

// Lazy load page components for better performance
const ChatPage = lazy(() => import("./pages/ChatPage"));
const AgentsPage = lazy(() => import("./pages/AgentsPage"));
const CodePage = lazy(() => import("./pages/CodePage"));
const TasksPage = lazy(() => import("./components/tasks/TasksPage"));
const QCPage = lazy(() => import("./components/qc/QCPage"));
const SecurityPage = lazy(() => import("./components/security/SecurityPage"));
const VirtualOSPage = lazy(() => import("./pages/VirtualOSPage"));
const AIToolsPage = lazy(() => import("./pages/AIToolsPage"));
const ResearchPage = lazy(() => import("./pages/ResearchPage"));
const MCPPage = lazy(() => import("./pages/MCPPage"));
const SettingsPage = lazy(() => import("./pages/SettingsPage"));
const OrchestrationPage = lazy(() => import("./pages/OrchestrationPage"));
const AuditorPage = lazy(() => import("./pages/AuditorPage"));
const VisualizationPage = lazy(() => import("./pages/VisualizationPage"));
const VRPage = lazy(() => import("./pages/VRPage"));
const PlansPage = lazy(() => import("./pages/PlansPage"));

// Loading fallback
const PageLoader = () => (
  <Box
    sx={{
      display: "flex",
      justifyContent: "center",
      alignItems: "center",
      height: "100%",
      minHeight: "400px",
    }}
  >
    <CircularProgress />
  </Box>
);

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<DashboardLayout />}>
          <Route
            index
            element={
              <Suspense fallback={<PageLoader />}>
                <ChatPage />
              </Suspense>
            }
          />
          <Route
            path="agents"
            element={
              <Suspense fallback={<PageLoader />}>
                <AgentsPage />
              </Suspense>
            }
          />
          <Route
            path="code"
            element={
              <Suspense fallback={<PageLoader />}>
                <CodePage />
              </Suspense>
            }
          />
          <Route
            path="tasks"
            element={
              <Suspense fallback={<PageLoader />}>
                <TasksPage />
              </Suspense>
            }
          />
          <Route
            path="qc"
            element={
              <Suspense fallback={<PageLoader />}>
                <QCPage />
              </Suspense>
            }
          />
          <Route
            path="security"
            element={
              <Suspense fallback={<PageLoader />}>
                <SecurityPage />
              </Suspense>
            }
          />
          <Route
            path="virtual-os"
            element={
              <Suspense fallback={<PageLoader />}>
                <VirtualOSPage />
              </Suspense>
            }
          />
          <Route
            path="ai-tools"
            element={
              <Suspense fallback={<PageLoader />}>
                <AIToolsPage />
              </Suspense>
            }
          />
          <Route
            path="research"
            element={
              <Suspense fallback={<PageLoader />}>
                <ResearchPage />
              </Suspense>
            }
          />
          <Route
            path="mcp"
            element={
              <Suspense fallback={<PageLoader />}>
                <MCPPage />
              </Suspense>
            }
          />
          <Route
            path="settings"
            element={
              <Suspense fallback={<PageLoader />}>
                <SettingsPage />
              </Suspense>
            }
          />
          <Route
            path="orchestration"
            element={
              <Suspense fallback={<PageLoader />}>
                <OrchestrationPage />
              </Suspense>
            }
          />
          <Route
            path="auditor"
            element={
              <Suspense fallback={<PageLoader />}>
                <AuditorPage />
              </Suspense>
            }
          />
          <Route
            path="visualization"
            element={
              <Suspense fallback={<PageLoader />}>
                <VisualizationPage />
              </Suspense>
            }
          />
          <Route
            path="vr"
            element={
              <Suspense fallback={<PageLoader />}>
                <VRPage />
              </Suspense>
            }
          />
          <Route
            path="plans"
            element={
              <Suspense fallback={<PageLoader />}>
                <PlansPage />
              </Suspense>
            }
          />
          <Route path="*" element={<Navigate to="/" replace />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
