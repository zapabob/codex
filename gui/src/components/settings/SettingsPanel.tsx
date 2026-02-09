import React from "react";
import {
  Box,
  Typography,
  Switch,
  FormControlLabel,
  Select,
  MenuItem,
  FormControl,
  InputLabel,
  Divider,
  Button,
  List,
  ListItem,
  ListItemText,
  ListItemSecondaryAction,
  TextField,
} from "@mui/material";
import {
  DarkMode as DarkModeIcon,
  Notifications as NotificationsIcon,
  Security as SecurityIcon,
  Storage as StorageIcon,
} from "@mui/icons-material";

export const SettingsPanel: React.FC = () => {
  const [theme, setTheme] = React.useState<"dark" | "light">("dark");
  const [notifications, setNotifications] = React.useState(true);
  const [digest, setDigest] = React.useState<"realtime" | "hourly" | "daily">(
    "realtime",
  );

  return (
    <Box sx={{ height: "100%", display: "flex", flexDirection: "column" }}>
      <Typography variant="h6" fontWeight={600} sx={{ mb: 2 }}>
        Settings
      </Typography>

      <Box sx={{ flex: 1, overflow: "auto" }}>
        {/* Appearance */}
        <List>
          <ListItem>
            <ListItemIcon>
              <DarkModeIcon />
            </ListItemIcon>
            <ListItemText
              primary="Theme"
              secondary="Choose your preferred theme"
            />
            <ListItemSecondaryAction>
              <FormControl size="small" sx={{ minWidth: 120 }}>
                <Select
                  value={theme}
                  onChange={(e) => setTheme(e.target.value as "dark" | "light")}
                >
                  <MenuItem value="dark">Dark</MenuItem>
                  <MenuItem value="light">Light</MenuItem>
                </Select>
              </FormControl>
            </ListItemSecondaryAction>
          </ListItem>
        </List>

        <Divider sx={{ my: 2 }} />

        {/* Notifications */}
        <List>
          <ListItem>
            <ListItemIcon>
              <NotificationsIcon />
            </ListItemIcon>
            <ListItemText
              primary="Notifications"
              secondary="Enable desktop notifications"
            />
            <ListItemSecondaryAction>
              <Switch
                checked={notifications}
                onChange={(e) => setNotifications(e.target.checked)}
              />
            </ListItemSecondaryAction>
          </ListItem>

          {notifications && (
            <ListItem sx={{ pl: 4 }}>
              <ListItemText
                primary="Notification digest"
                secondary="How often to receive notification summaries"
              />
              <ListItemSecondaryAction>
                <FormControl size="small" sx={{ minWidth: 120 }}>
                  <Select
                    value={digest}
                    onChange={(e) =>
                      setDigest(
                        e.target.value as "realtime" | "hourly" | "daily",
                      )
                    }
                  >
                    <MenuItem value="realtime">Real-time</MenuItem>
                    <MenuItem value="hourly">Hourly</MenuItem>
                    <MenuItem value="daily">Daily</MenuItem>
                  </Select>
                </FormControl>
              </ListItemSecondaryAction>
            </ListItem>
          )}
        </List>

        <Divider sx={{ my: 2 }} />

        {/* Security */}
        <List>
          <ListItem>
            <ListItemIcon>
              <SecurityIcon />
            </ListItemIcon>
            <ListItemText
              primary="Guardrails"
              secondary="Configure security policies"
            />
            <ListItemSecondaryAction>
              <Button variant="outlined" size="small">
                Configure
              </Button>
            </ListItemSecondaryAction>
          </ListItem>

          <ListItem>
            <ListItemIcon>
              <StorageIcon />
            </ListItemIcon>
            <ListItemText
              primary="Data & Privacy"
              secondary="Manage your data and privacy settings"
            />
            <ListItemSecondaryAction>
              <Button variant="outlined" size="small">
                Manage
              </Button>
            </ListItemSecondaryAction>
          </ListItem>
        </List>

        <Divider sx={{ my: 2 }} />

        {/* Worktree Settings */}
        <Typography
          variant="subtitle2"
          color="text.secondary"
          sx={{ px: 2, mb: 1 }}
        >
          Worktree
        </Typography>

        <List>
          <ListItem>
            <ListItemText
              primary="Auto-sync"
              secondary="Automatically sync changes from main branch"
            />
            <ListItemSecondaryAction>
              <Switch defaultChecked />
            </ListItemSecondaryAction>
          </ListItem>

          <ListItem>
            <ListItemText
              primary="Max worktrees"
              secondary="Maximum number of concurrent worktrees"
            />
            <ListItemSecondaryAction>
              <TextField
                size="small"
                type="number"
                defaultValue={10}
                sx={{ width: 80 }}
              />
            </ListItemSecondaryAction>
          </ListItem>

          <ListItem>
            <ListItemText
              primary="Cleanup after"
              secondary="Remove inactive worktrees after (days)"
            />
            <ListItemSecondaryAction>
              <TextField
                size="small"
                type="number"
                defaultValue={7}
                sx={{ width: 80 }}
              />
            </ListItemSecondaryAction>
          </ListItem>
        </List>

        <Divider sx={{ my: 2 }} />

        {/* Approvals */}
        <Typography
          variant="subtitle2"
          color="text.secondary"
          sx={{ px: 2, mb: 1 }}
        >
          Command Approvals
        </Typography>

        <List>
          <ListItem>
            <ListItemText
              primary="Safe commands"
              description="Always allow safe commands (git status, gh pr view, etc.)"
            />
            <ListItemSecondaryAction>
              <Switch defaultChecked />
            </ListItemSecondaryAction>
          </ListItem>

          <ListItem>
            <ListItemText
              primary="Require approval for git push"
              description="Prompt before pushing to protected branches"
            />
            <ListItemSecondaryAction>
              <Switch defaultChecked />
            </ListItemSecondaryAction>
          </ListItem>

          <ListItem>
            <ListItemText
              primary="Require approval for publish"
              description="Prompt before publishing packages"
            />
            <ListItemSecondaryAction>
              <Switch defaultChecked />
            </ListItemSecondaryAction>
          </ListItem>
        </List>
      </Box>

      {/* Footer */}
      <Box sx={{ pt: 2, borderTop: 1, borderColor: "divider" }}>
        <Typography variant="caption" color="text.secondary">
          Settings are saved to localStorage. Use .codex/rules for team-wide
          rules.
        </Typography>
      </Box>
    </Box>
  );
};

export default SettingsPanel;
