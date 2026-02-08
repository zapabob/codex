import * as React from "react";
import { WorktreeDashboard } from "../components/orchestration/WorktreeDashboard";

const repoPath = "c:\\Users\\downl\\Desktop\\codex-main";

export const OrchestrationPage: React.FC = () => {
  return (
    <div data-testid="orchestration-page">
      <WorktreeDashboard repoPath={repoPath} />
    </div>
  );
};

export default OrchestrationPage;
