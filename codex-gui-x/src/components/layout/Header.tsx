import * as React from "react";
import { useTranslation } from "react-i18next";
import {
  AppBar,
  Toolbar,
  IconButton,
  Typography,
  Box,
  Avatar,
  Tooltip,
  styled,
} from "@mui/material";
import {
  Menu as MenuIcon,
  Settings as SettingsIcon,
  Terminal,
  Github,
} from "lucide-react";
import { useBridge } from "../../hooks/useBridge";

interface HeaderProps {
  title?: string;
  onMenuClick?: () => void;
  showMenuButton?: boolean;
}

const StyledAppBar = styled(AppBar)(({ theme }) => ({
  background:
    theme.palette.mode === "dark"
      ? "rgba(15, 20, 25, 0.8)"
      : "rgba(255, 255, 255, 0.8)",
  backdropFilter: "blur(20px)",
  borderBottom: `1px solid ${
    theme.palette.mode === "dark"
      ? "rgba(255, 255, 255, 0.1)"
      : "rgba(0, 0, 0, 0.05)"
  }`,
  boxShadow: "none",
}));

const ConnectionBadge = styled(Box)<{ connected: boolean }>(
  ({ theme, connected }) => ({
    display: "flex",
    alignItems: "center",
    gap: "6px",
    padding: "4px 10px",
    borderRadius: "6px",
    fontSize: "10px",
    fontFamily: "monospace",
    fontWeight: 700,
    textTransform: "uppercase",
    letterSpacing: "0.5px",
    backgroundColor: connected
      ? `${theme.palette.success.main}15`
      : `${theme.palette.error.main}15`,
    color: connected ? theme.palette.success.main : theme.palette.error.main,
    border: `1px solid ${
      connected
        ? `${theme.palette.success.main}30`
        : `${theme.palette.error.main}30`
    }`,
    transition: "all 0.3s ease",
  }),
);

export const Header: React.FC<HeaderProps> = ({
  title,
  onMenuClick,
  showMenuButton = true,
}) => {
  const { t } = useTranslation();
  const { connected } = useBridge();

  return (
    <StyledAppBar position="sticky" data-testid="header">
      <Toolbar sx={{ minHeight: "56px", px: 2 }}>
        {showMenuButton && (
          <Tooltip title={t("common.menu")}>
            <IconButton
              edge="start"
              onClick={onMenuClick}
              sx={{ mr: 2 }}
              data-testid="mobile-menu"
            >
              <MenuIcon size={20} />
            </IconButton>
          </Tooltip>
        )}

        <Box sx={{ display: "flex", alignItems: "center", gap: 2, flex: 1 }}>
          <Typography
            variant="h6"
            component="div"
            sx={{
              fontWeight: 700,
              letterSpacing: "-0.5px",
              background: "linear-gradient(45deg, #0061a4, #565f71)",
              backgroundClip: "text",
              WebkitBackgroundClip: "text",
              WebkitTextFillColor: "transparent",
            }}
          >
            {title || t("app.title")}
          </Typography>

          <ConnectionBadge connected={connected}>
            <Terminal size={10} />
            <span>{connected ? "CONNECTED TO BRIDGE" : "DISCONNECTED"}</span>
          </ConnectionBadge>
        </Box>

        <Box sx={{ display: "flex", alignItems: "center", gap: 1 }}>
          <Tooltip title="GitHub">
            <IconButton
              component="a"
              href="https://github.com/zapabob/Codex"
              target="_blank"
              rel="noopener noreferrer"
              sx={{ color: "text.secondary" }}
            >
              <Github size={20} />
            </IconButton>
          </Tooltip>

          <Tooltip title={t("nav.settings")}>
            <IconButton sx={{ color: "text.secondary" }}>
              <SettingsIcon size={20} />
            </IconButton>
          </Tooltip>

          <Avatar
            sx={{
              width: 32,
              height: 32,
              ml: 1,
              background: "linear-gradient(135deg, #667eea 0%, #764ba2 100%)",
              fontSize: "14px",
              fontWeight: 700,
            }}
          >
            A
          </Avatar>
        </Box>
      </Toolbar>
    </StyledAppBar>
  );
};

export default Header;
