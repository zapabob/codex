import * as React from "react";
import VRInterface from "../components/vr/VRInterface";

export const VRPage: React.FC = () => {
  return (
    <div data-testid="vr-page">
      <VRInterface commits={[]} />
    </div>
  );
};

export default VRPage;
