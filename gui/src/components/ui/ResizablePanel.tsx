import React, {
  useState,
  useCallback,
  useRef,
  useEffect,
  ReactNode,
} from "react";
import { Box, IconButton, Typography, Tooltip, Collapse } from "@mui/material";
import { ChevronLeft, ChevronRight, Close } from "@mui/icons-material";

interface ResizablePanelProps {
  id: string;
  title: string;
  children: ReactNode;
  icon?: ReactNode;
  defaultWidth?: number;
  minWidth?: number;
  maxWidth?: number;
  defaultHeight?: number;
  minHeight?: number;
  maxHeight?: number;
  resizable?: "horizontal" | "vertical" | "both" | "none";
  collapsible?: boolean;
  closable?: boolean;
  onClose?: () => void;
  onCollapse?: () => void;
  isActive?: boolean;
}

export const ResizablePanel: React.FC<ResizablePanelProps> = ({
  id,
  title,
  children,
  icon,
  defaultWidth = 400,
  minWidth = 200,
  maxWidth = 800,
  defaultHeight,
  minHeight,
  maxHeight,
  resizable = "horizontal",
  collapsible = true,
  closable = false,
  onClose,
  onCollapse,
  isActive = true,
}) => {
  const [width, setWidth] = useState(defaultWidth);
  const [height, setHeight] = useState(defaultHeight);
  const [isCollapsed, setIsCollapsed] = useState(false);
  const [isResizing, setIsResizing] = useState(false);
  const panelRef = useRef<HTMLDivElement>(null);
  const startPos = useRef({ x: 0, y: 0, width: 0, height: 0 });

  const handleMouseDown = useCallback(
    (e: React.MouseEvent) => {
      if (resizable === "none") return;

      e.preventDefault();
      setIsResizing(true);
      startPos.current = {
        x: e.clientX,
        y: e.clientY,
        width,
        height: height || 0,
      };

      const handleMouseMove = (e: MouseEvent) => {
        if (resizable.includes("horizontal")) {
          const newWidth = Math.min(
            maxWidth,
            Math.max(
              minWidth,
              startPos.current.width + (e.clientX - startPos.current.x),
            ),
          );
          setWidth(newWidth);
        }
        if (resizable.includes("vertical") && height) {
          const newHeight = Math.min(
            maxHeight || Infinity,
            Math.max(
              minHeight || 0,
              startPos.current.height + (e.clientY - startPos.current.y),
            ),
          );
          setHeight(newHeight);
        }
      };

      const handleMouseUp = () => {
        setIsResizing(false);
        document.removeEventListener("mousemove", handleMouseMove);
        document.removeEventListener("mouseup", handleMouseUp);
      };

      document.addEventListener("mousemove", handleMouseMove);
      document.addEventListener("mouseup", handleMouseUp);
    },
    [resizable, width, height, minWidth, maxWidth, minHeight, maxHeight],
  );

  const handleCollapse = useCallback(() => {
    setIsCollapsed(!isCollapsed);
    onCollapse?.();
  }, [isCollapsed, onCollapse]);

  return (
    <Box
      ref={panelRef}
      className="resizable-panel"
      sx={{
        width: isCollapsed ? 48 : width,
        height: height || "100%",
        display: "flex",
        flexDirection: "column",
        borderRight: 1,
        borderColor: "divider",
        bgcolor: "background.paper",
        transition: isResizing ? "none" : "width 0.2s ease",
        overflow: "hidden",
        position: "relative",
      }}
    >
      {/* Resize Handle */}
      {resizable.includes("horizontal") && !isCollapsed && (
        <Box
          onMouseDown={handleMouseDown}
          sx={{
            "position": "absolute",
            "left": 0,
            "top": 0,
            "bottom": 0,
            "width": 4,
            "cursor": "col-resize",
            "bgcolor": "transparent",
            "transition": "bgcolor 0.2s",
            "&:hover": {
              bgcolor: "primary.main",
            },
            "zIndex": 10,
          }}
        />
      )}

      {/* Header */}
      <Box
        sx={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          p: 1.5,
          borderBottom: 1,
          borderColor: "divider",
          minHeight: 48,
          cursor: resizable.includes("vertical") ? "row-resize" : "default",
        }}
        onMouseDown={(e) => {
          if (resizable.includes("vertical") && height) {
            handleMouseDown(e);
          }
        }}
      >
        <Box sx={{ display: "flex", alignItems: "center", gap: 1 }}>
          {icon && (
            <Box
              sx={{
                color: "primary.main",
                display: "flex",
                alignItems: "center",
              }}
            >
              {icon}
            </Box>
          )}
          {!isCollapsed && (
            <Typography variant="subtitle2" fontWeight={600}>
              {title}
            </Typography>
          )}
        </Box>

        <Box sx={{ display: "flex", gap: 0.5 }}>
          {collapsible && !isCollapsed && (
            <Tooltip title="Collapse" placement="bottom">
              <IconButton size="small" onClick={handleCollapse}>
                <ChevronLeft fontSize="small" />
              </IconButton>
            </Tooltip>
          )}
          {collapsible && isCollapsed && (
            <Tooltip title={title} placement="right">
              <IconButton size="small" onClick={handleCollapse}>
                <ChevronRight fontSize="small" />
              </IconButton>
            </Tooltip>
          )}
          {closable && !isCollapsed && (
            <Tooltip title="Close" placement="bottom">
              <IconButton size="small" onClick={onClose}>
                <Close fontSize="small" />
              </IconButton>
            </Tooltip>
          )}
        </Box>
      </Box>

      {/* Content */}
      <Collapse in={!isCollapsed}>
        <Box
          sx={{
            flex: 1,
            overflow: "auto",
            p: 2,
          }}
        >
          {children}
        </Box>
      </Collapse>
    </Box>
  );
};

export default ResizablePanel;
