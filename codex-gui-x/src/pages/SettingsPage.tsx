import * as React from "react";
import { useTranslation } from "react-i18next";
import { Box, Typography } from "@mui/material";

export const SettingsPage: React.FC = () => {
  const { t } = useTranslation();

  return (
    <Box data-testid="settings-page">
      <Typography variant="h4" gutterBottom>
        {t("nav.settings")}
      </Typography>
      <Typography variant="body1" color="text.secondary">
        Settings interface coming soon...
      </Typography>
    </Box>
  );
};

export default SettingsPage;
