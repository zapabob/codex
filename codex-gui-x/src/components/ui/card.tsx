import * as React from "react";
import type { CardProps as MuiCardProps } from "@mui/material";
import { Card as MuiCard, styled } from "@mui/material";

const StyledCard = styled(MuiCard)(({ theme }) => ({
  "background":
    theme.palette.mode === "dark"
      ? "rgba(30, 30, 35, 0.6)"
      : "rgba(255, 255, 255, 0.7)",
  "backdropFilter": "blur(10px)",
  "border": `1px solid ${
    theme.palette.mode === "dark"
      ? "rgba(255, 255, 255, 0.1)"
      : "rgba(0, 0, 0, 0.05)"
  }`,
  "borderRadius": "16px",
  "transition": "all 0.3s cubic-bezier(0.4, 0, 0.2, 1)",
  "&:hover": {
    transform: "translateY(-2px)",
    boxShadow:
      theme.palette.mode === "dark"
        ? "0 20px 40px rgba(0, 0, 0, 0.4)"
        : "0 20px 40px rgba(0, 0, 0, 0.1)",
  },
}));

export interface CardProps extends MuiCardProps {
  children?: React.ReactNode;
}

export const Card = React.forwardRef<HTMLDivElement, CardProps>(
  ({ children, ...props }, ref) => {
    return (
      <StyledCard ref={ref} {...props}>
        {children}
      </StyledCard>
    );
  },
);

Card.displayName = "Card";

export default Card;
