import * as React from "react";
import { useTranslation } from "react-i18next";
import { Box, Typography } from "@mui/material";

export const CodePage: React.FC = () => {
  const { t } = useTranslation();

  return (
    <Box data-testid="code-page">
      <Typography variant="h4" gutterBottom>
        {t("nav.code")}
      </Typography>
      <Typography variant="body1" color="text.secondary">
        Code execution interface coming soon...
      </Typography>
    </Box>
  );
};

export default CodePage;
