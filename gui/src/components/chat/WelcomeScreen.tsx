import React from "react";
import { Box, Typography, Grid, Paper, Chip } from "@mui/material";
import {
  Sparkles,
  Code,
  Bug,
  Optimize,
  Document,
  Translate,
} from "@mui/icons-material";

interface Suggestion {
  id: string;
  icon: React.ReactNode;
  title: string;
  description: string;
  prompt: string;
  tags: string[];
}

interface WelcomeScreenProps {
  onSelectSuggestion: (prompt: string) => void;
}

const suggestions: Suggestion[] = [
  {
    id: "explain",
    icon: <Code />,
    title: "Explain code",
    description: "Understand how this code works",
    prompt: "Explain how this code works: ",
    tags: ["Code", "Education"],
  },
  {
    id: "debug",
    icon: <Bug />,
    title: "Debug this code",
    description: "Find and fix bugs in your code",
    prompt: "Debug this code and explain the issues: ",
    tags: ["Debug", "Fix"],
  },
  {
    id: "refactor",
    icon: <Optimize />,
    title: "Refactor code",
    description: "Improve code quality and performance",
    prompt: "Refactor this code for better quality and performance: ",
    tags: ["Optimization", "Best Practices"],
  },
  {
    id: "test",
    icon: <Document />,
    title: "Write tests",
    description: "Generate comprehensive tests",
    prompt: "Write comprehensive tests for this code: ",
    tags: ["Testing", "Coverage"],
  },
  {
    id: "translate",
    icon: <Translate />,
    title: "Translate code",
    description: "Convert code between languages",
    prompt: "Convert this code to a different programming language: ",
    tags: ["Translation", "Migration"],
  },
  {
    id: "analyze",
    icon: <Sparkles />,
    title: "Analyze architecture",
    description: "Review system design",
    prompt:
      "Analyze the architecture of this codebase and suggest improvements: ",
    tags: ["Architecture", "Design"],
  },
];

export const WelcomeScreen: React.FC<WelcomeScreenProps> = ({
  onSelectSuggestion,
}) => {
  return (
    <Box
      sx={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        height: "100%",
        p: 4,
        maxWidth: 800,
        mx: "auto",
      }}
    >
      <Box sx={{ textAlign: "center", mb: 6 }}>
        <Typography
          variant="h3"
          component="h1"
          sx={{
            fontWeight: 600,
            mb: 2,
            background: "linear-gradient(45deg, #10a37f 30%, #1a7f64 90%)",
            backgroundClip: "text",
            WebkitBackgroundClip: "text",
            WebkitTextFillColor: "transparent",
          }}
        >
          What can I help you with today?
        </Typography>
        <Typography variant="body1" color="text.secondary" sx={{ mb: 4 }}>
          Ask me to write code, debug issues, explain concepts, or help with
          your development workflow.
        </Typography>
      </Box>

      <Grid container spacing={2} sx={{ width: "100%" }}>
        {suggestions.map((suggestion) => (
          <Grid item xs={12} sm={6} md={4} key={suggestion.id}>
            <Paper
              elevation={0}
              onClick={() => onSelectSuggestion(suggestion.prompt)}
              sx={{
                "p": 3,
                "height": "100%",
                "cursor": "pointer",
                "border": 1,
                "borderColor": "divider",
                "borderRadius": 2,
                "transition": "all 0.2s ease",
                "&:hover": {
                  borderColor: "primary.main",
                  bgcolor: "action.hover",
                  transform: "translateY(-2px)",
                },
              }}
            >
              <Box
                sx={{
                  display: "flex",
                  alignItems: "center",
                  gap: 1.5,
                  mb: 1.5,
                }}
              >
                <Box
                  sx={{
                    p: 1,
                    borderRadius: 1.5,
                    bgcolor: "primary.main",
                    color: "primary.contrastText",
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "center",
                  }}
                >
                  {suggestion.icon}
                </Box>
                <Typography variant="subtitle1" fontWeight={600}>
                  {suggestion.title}
                </Typography>
              </Box>
              <Typography
                variant="body2"
                color="text.secondary"
                sx={{ mb: 1.5 }}
              >
                {suggestion.description}
              </Typography>
              <Box sx={{ display: "flex", gap: 0.5, flexWrap: "wrap" }}>
                {suggestion.tags.map((tag) => (
                  <Chip
                    key={tag}
                    label={tag}
                    size="small"
                    sx={{
                      height: 20,
                      fontSize: "0.7rem",
                      bgcolor: "action.selected",
                    }}
                  />
                ))}
              </Box>
            </Paper>
          </Grid>
        ))}
      </Grid>

      <Box sx={{ mt: 6, textAlign: "center" }}>
        <Typography variant="caption" color="text.secondary">
          Pro tip: Use{" "}
          <kbd
            style={{
              px: 0.5,
              py: 0.25,
              borderRadius: 4,
              bgcolor: "action.selected",
              border: "1px solid",
              borderColor: "divider",
              fontSize: "0.75rem",
            }}
          >
            @
          </kbd>{" "}
          to mention specific skills or tools
        </Typography>
      </Box>
    </Box>
  );
};

export default WelcomeScreen;
