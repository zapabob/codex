import * as React from "react";
import { PlanCreator } from "../components/plan/PlanCreator";

export const PlansPage: React.FC = () => {
  return (
    <div data-testid="plans-page">
      <PlanCreator />
    </div>
  );
};

export default PlansPage;
