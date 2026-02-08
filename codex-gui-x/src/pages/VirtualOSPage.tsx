import * as React from "react";
import { useTranslation } from "react-i18next";
import { VirtualEnvironmentManager } from "../components/virtual-os/VirtualEnvironmentManager";

export const VirtualOSPage: React.FC = () => {
  const { t } = useTranslation();

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
