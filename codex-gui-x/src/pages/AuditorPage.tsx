import * as React from "react";
import { QAAuditor } from "../components/orchestration/QAAuditor";

export const AuditorPage: React.FC = () => {
  return (
    <div data-testid="auditor-page">
      <QAAuditor />
    </div>
  );
};

export default AuditorPage;
