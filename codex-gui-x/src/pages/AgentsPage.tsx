import * as React from "react";
import { useTranslation } from "react-i18next";
import { Box, Typography } from "@mui/material";

export const AgentsPage: React.FC = () => {
  const { t } = useTranslation();

  return (
    <Box data-testid="agents-page">
      <Typography variant="h4" gutterBottom>
        {t("nav.agents")}
      </Typography>
      <Typography variant="body1" color="text.secondary">
        AI Agents management interface coming soon...
      </Typography>
    </Box>
  );
};

export default AgentsPage;
