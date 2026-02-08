import * as React from "react";
import type { ChipProps } from "@mui/material";
import { Chip, styled } from "@mui/material";

interface StyledBadgeProps extends ChipProps {
  colortype?:
    | "default"
    | "primary"
    | "secondary"
    | "error"
    | "warning"
    | "success"
    | "info";
}

const StyledBadge = styled(Chip, {
  shouldForwardProp: (prop) => prop !== "colortype",
})<StyledBadgeProps>(({ theme, colortype = "default" }) => {
  const colorMap = {
    default: theme.palette.grey[500],
    primary: theme.palette.primary.main,
    secondary: theme.palette.secondary.main,
    error: theme.palette.error.main,
    warning: theme.palette.warning.main,
    success: theme.palette.success.main,
    info: theme.palette.info.main,
  };

  const color = colorMap[colortype];

  return {
    "backgroundColor": `${color}20`,
    "color": color,
    "border": `1px solid ${color}40`,
    "fontWeight": 600,
    "& .MuiChip-label": {
      padding: "0 12px",
    },
  };
});

export interface BadgeProps extends Omit<ChipProps, "color"> {
  color?:
    | "default"
    | "primary"
    | "secondary"
    | "error"
    | "warning"
    | "success"
    | "info";
  children?: React.ReactNode;
}

export const Badge = React.forwardRef<HTMLDivElement, BadgeProps>(
  ({ color = "default", children, ...props }, ref) => {
    return (
      <StyledBadge ref={ref} colortype={color} label={children} {...props} />
    );
  },
);

Badge.displayName = "Badge";

export default Badge;
