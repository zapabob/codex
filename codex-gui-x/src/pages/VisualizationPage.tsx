import * as React from "react";
import { Git4DVisualization } from "../components/visualization/Git4DVisualization";

const repoPath = "c:\\Users\\downl\\Desktop\\codex-main";

export const VisualizationPage: React.FC = () => {
  return (
    <div data-testid="visualization-page">
      <Git4DVisualization repositoryPath={repoPath} />
    </div>
  );
};

export default VisualizationPage;
