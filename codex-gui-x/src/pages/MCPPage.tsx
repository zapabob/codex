import * as React from "react";
import { useTranslation } from "react-i18next";
import { Box, Typography } from "@mui/material";

export const MCPPage: React.FC = () => {
  const { t } = useTranslation();

  return (
    <Box data-testid="mcp-page">
      <Typography variant="h4" gutterBottom>
        {t("nav.mcp")}
      </Typography>
      <Typography variant="body1" color="text.secondary">
        MCP Server management interface coming soon...
      </Typography>
    </Box>
  );
};

export default MCPPage;
