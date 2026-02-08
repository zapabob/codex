import * as React from "react";
import { useTranslation } from "react-i18next";
import { Box, Typography } from "@mui/material";

export const ChatPage: React.FC = () => {
  const { t } = useTranslation();

  return (
    <Box>
      <Typography variant="h4" gutterBottom>
        {t("app.welcome")}
      </Typography>
      <Typography variant="body1" color="text.secondary">
        Chat interface coming soon...
      </Typography>
    </Box>
  );
};

export default ChatPage;
