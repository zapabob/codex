import { createTheme, alpha } from "@mui/material/styles";

declare module "@mui/material/styles" {
  interface Palette {
    chatGPT: {
      userBubble: string;
      assistantBubble: string;
      codeBackground: string;
      codeText: string;
      border: string;
      hover: string;
      inputBackground: string;
    };
  }
  interface PaletteOptions {
    chatGPT?: {
      userBubble?: string;
      assistantBubble?: string;
      codeBackground?: string;
      codeText?: string;
      border?: string;
      hover?: string;
      inputBackground?: string;
    };
  }
}

export const chatGPTDarkTheme = createTheme({
  palette: {
    mode: "dark",
    primary: {
      main: "#10a37f",
      light: "#1a7f64",
      dark: "#0d6b53",
      contrastText: "#ffffff",
    },
    secondary: {
      main: "#5e5e5e",
      light: "#7a7a7a",
      dark: "#3d3d3d",
    },
    background: {
      default: "#212121",
      paper: "#2f2f2f",
    },
    text: {
      primary: "#ececec",
      secondary: "#b4b4b4",
    },
    divider: "rgba(255, 255, 255, 0.1)",
    chatGPT: {
      userBubble: "#2f2f2f",
      assistantBubble: "transparent",
      codeBackground: "#1e1e1e",
      codeText: "#d4d4d4",
      border: "rgba(255, 255, 255, 0.1)",
      hover: "rgba(255, 255, 255, 0.05)",
      inputBackground: "#2f2f2f",
    },
  },
  typography: {
    fontFamily:
      '"Söhne", "Segoe UI", "Roboto", "Helvetica", "Arial", sans-serif',
    h1: {
      fontSize: "2rem",
      fontWeight: 600,
      letterSpacing: "-0.02em",
    },
    h2: {
      fontSize: "1.5rem",
      fontWeight: 600,
      letterSpacing: "-0.01em",
    },
    h3: {
      fontSize: "1.25rem",
      fontWeight: 600,
    },
    h4: {
      fontSize: "1rem",
      fontWeight: 600,
    },
    body1: {
      fontSize: "0.9375rem",
      lineHeight: 1.6,
    },
    body2: {
      fontSize: "0.875rem",
      lineHeight: 1.5,
    },
    caption: {
      fontSize: "0.75rem",
      color: "#8e8e8e",
    },
    button: {
      textTransform: "none",
      fontWeight: 500,
    },
  },
  shape: {
    borderRadius: 8,
  },
  components: {
    MuiButton: {
      styleOverrides: {
        root: {
          borderRadius: 8,
          padding: "8px 16px",
        },
        contained: {
          "backgroundColor": "#2f2f2f",
          "&:hover": {
            backgroundColor: "#3d3d3d",
          },
        },
      },
    },
    MuiPaper: {
      styleOverrides: {
        root: {
          backgroundImage: "none",
        },
        outlined: {
          borderColor: "rgba(255, 255, 255, 0.1)",
        },
      },
    },
    MuiTextField: {
      styleOverrides: {
        root: {
          "& .MuiOutlinedInput-root": {
            "backgroundColor": "#2f2f2f",
            "& fieldset": {
              borderColor: "rgba(255, 255, 255, 0.1)",
            },
            "&:hover fieldset": {
              borderColor: "rgba(255, 255, 255, 0.2)",
            },
            "&.Mui-focused fieldset": {
              borderColor: "#10a37f",
            },
          },
        },
      },
    },
    MuiChip: {
      styleOverrides: {
        root: {
          borderRadius: 8,
        },
      },
    },
    MuiTooltip: {
      styleOverrides: {
        tooltip: {
          backgroundColor: "#404040",
          fontSize: "0.8125rem",
          padding: "8px 12px",
        },
      },
    },
    MuiDrawer: {
      styleOverrides: {
        paper: {
          backgroundColor: "#171717",
          borderRight: "1px solid rgba(255, 255, 255, 0.1)",
        },
      },
    },
    MuiListItemButton: {
      styleOverrides: {
        root: {
          "borderRadius": 8,
          "margin": "2px 8px",
          "&.Mui-selected": {
            "backgroundColor": "rgba(16, 163, 127, 0.15)",
            "&:hover": {
              backgroundColor: "rgba(16, 163, 127, 0.25)",
            },
          },
        },
      },
    },
  },
});

export const chatGPTLightTheme = createTheme({
  palette: {
    mode: "light",
    primary: {
      main: "#10a37f",
      light: "#1a7f64",
      dark: "#0d6b53",
      contrastText: "#ffffff",
    },
    secondary: {
      main: "#6e6e6e",
      light: "#8a8a8a",
      dark: "#4a4a4a",
    },
    background: {
      default: "#ffffff",
      paper: "#f7f7f7",
    },
    text: {
      primary: "#2d2d2d",
      secondary: "#666666",
    },
    divider: "rgba(0, 0, 0, 0.08)",
    chatGPT: {
      userBubble: "#f7f7f7",
      assistantBubble: "transparent",
      codeBackground: "#f7f7f7",
      codeText: "#2d2d2d",
      border: "rgba(0, 0, 0, 0.08)",
      hover: "rgba(0, 0, 0, 0.04)",
      inputBackground: "#ffffff",
    },
  },
  typography: chatGPTDarkTheme.typography,
  shape: chatGPTDarkTheme.shape,
  components: {
    ...chatGPTDarkTheme.components,
    MuiDrawer: {
      styleOverrides: {
        paper: {
          backgroundColor: "#f9f9f9",
          borderRight: "1px solid rgba(0, 0, 0, 0.08)",
        },
      },
    },
  },
});

export type ThemeMode = "dark" | "light";

export const getTheme = (mode: ThemeMode) => {
  return mode === "dark" ? chatGPTDarkTheme : chatGPTLightTheme;
};

export const toggleTheme = (
  mode: ThemeMode,
  setMode: (mode: ThemeMode) => void,
) => {
  setMode(mode === "dark" ? "light" : "dark");
};
