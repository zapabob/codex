import * as React from "react";
import { VirtualEnvironmentManager } from "../components/virtual-os/VirtualEnvironmentManager";

export const VirtualOSPage: React.FC = () => {
  return (
    <div data-testid="virtual-os-page">
      <VirtualEnvironmentManager
        environments={[]}
        onEnvironmentSelect={() => {}}
        onEnvironmentCreate={() => {}}
        onEnvironmentDelete={() => {}}
      />
    </div>
  );
};

export default VirtualOSPage;
