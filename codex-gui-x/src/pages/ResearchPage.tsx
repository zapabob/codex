import * as React from "react";
import { useTranslation } from "react-i18next";
import { Box, Typography } from "@mui/material";

export const ResearchPage: React.FC = () => {
  const { t } = useTranslation();

  return (
    <Box data-testid="research-page">
      <Typography variant="h4" gutterBottom>
        {t("nav.research")}
      </Typography>
      <Typography variant="body1" color="text.secondary">
        Deep Research interface coming soon...
      </Typography>
    </Box>
  );
};

export default ResearchPage;
