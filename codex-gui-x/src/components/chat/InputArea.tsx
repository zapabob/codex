import React, { useState, useCallback, useRef, useEffect } from 'react';
import {
  Box,
  TextField,
  IconButton,
  Tooltip,
  Paper,
  InputAdornment,
  Menu,
  MenuItem,
  ListItemIcon,
  ListItemText,
  Typography,
  CircularProgress,
} from '@mui/material';
import {
  Send as SendIcon,
  AttachFile as AttachIcon,
  Mic as MicIcon,
  Stop as StopIcon,
  SmartToy as AIIcon,
  AutoAwesome as SparkleIcon,
} from '@mui/icons-material';

interface InputAreaProps {
  value: string;
  onChange: (value: string) => void;
  onSend: () => void;
  disabled?: boolean;
  placeholder?: string;
}

export const InputArea: React.FC<InputAreaProps> = ({
  value,
  onChange,
  onSend,
  disabled = false,
  placeholder = 'Type a message...',
}) => {
  const [isRecording, setIsRecording] = useState(false);
  const [attachMenuAnchor, setAttachMenuAnchor] = useState<HTMLElement | null>(null);
  const inputRef = useRef<HTMLTextAreaElement>(null);

  const handleKeyDown = useCallback(
    (event: React.KeyboardEvent) => {
      if (event.key === 'Enter' && !event.shiftKey) {
        event.preventDefault();
        if (value.trim() && !disabled) {
          onSend();
        }
      }
    },
    [value, disabled, onSend]
  );

  const handleChange = useCallback(
    (event: React.ChangeEvent<HTMLTextAreaElement>) => {
      onChange(event.target.value);
    },
    [onChange]
  );

  const handleAttachClick = useCallback((event: React.MouseEvent<HTMLElement>) => {
    setAttachMenuAnchor(event.currentTarget);
  }, []);

  const handleAttachClose = useCallback(() => {
    setAttachMenuAnchor(null);
  }, []);

  const handleFileAttach = useCallback(() => {
    const input = document.createElement('input');
    input.type = 'file';
    input.multiple = true;
    input.onchange = () => {
      const files = input.files;
      if (files) {
        console.log('Selected files:', files);
      }
    };
    input.click();
    handleAttachClose();
  }, [handleAttachClose]);

  const handleMicClick = useCallback(() => {
    if ('webkitSpeechRecognition' in window || 'SpeechRecognition' in window) {
      setIsRecording(!isRecording);
    } else {
      console.warn('Speech recognition not supported');
    }
  }, [isRecording]);

  useEffect(() => {
    if (!disabled && inputRef.current) {
      inputRef.current.focus();
    }
  }, [disabled]);

  return (
    <Paper
      elevation={0}
      sx={{
        p: 2,
        borderTop: 1,
        borderColor: 'divider',
        bgcolor: 'background.paper',
      }}
    >
      <Box sx={{ display: 'flex', gap: 1, alignItems: 'flex-end' }}>
        <Box sx={{ display: 'flex', gap: 0.5 }}>
          <Tooltip title="Attach file">
            <IconButton
              size="small"
              onClick={handleAttachClick}
              disabled={disabled}
              sx={{ color: 'text.secondary' }}
            >
              <AttachIcon />
            </IconButton>
          </Tooltip>
          <Menu
            anchorEl={attachMenuAnchor}
            open={Boolean(attachMenuAnchor)}
            onClose={handleAttachClose}
            anchorOrigin={{ vertical: 'top', horizontal: 'left' }}
          >
            <MenuItem onClick={handleFileAttach}>
              <ListItemIcon>
                <AttachIcon fontSize="small" />
              </ListItemIcon>
              <ListItemText>Upload Files</ListItemText>
            </MenuItem>
          </Menu>
        </Box>

        <TextField
          ref={inputRef}
          fullWidth
          multiline
          maxRows={6}
          value={value}
          onChange={handleChange}
          onKeyDown={handleKeyDown}
          placeholder={placeholder}
          disabled={disabled}
          variant="outlined"
          sx={{
            '& .MuiOutlinedInput-root': {
              borderRadius: 3,
              bgcolor: 'grey.50',
              '&:hover': {
                bgcolor: 'grey.100',
              },
              '&.Mui-focused': {
                bgcolor: 'background.paper',
              },
            },
          }}
          InputProps={{
            endAdornment: (
              <InputAdornment position="end">
                <Tooltip title={isRecording ? 'Stop recording' : 'Voice input'}>
                  <IconButton
                    size="small"
                    onClick={handleMicClick}
                    disabled={disabled}
                    sx={{
                      color: isRecording ? 'error.main' : 'text.secondary',
                      animation: isRecording ? 'pulse 1s infinite' : 'none',
                      '@keyframes pulse': {
                        '0%, 100%': { opacity: 1 },
                        '50%': { opacity: 0.5 },
                      },
                    }}
                  >
                    <MicIcon />
                  </IconButton>
                </Tooltip>
              </InputAdornment>
            ),
          }}
        />

        <Tooltip title={disabled ? 'Connecting...' : 'Send message'}>
          <span>
            <IconButton
              color="primary"
              onClick={onSend}
              disabled={disabled || !value.trim()}
              sx={{
                bgcolor: 'primary.main',
                color: 'primary.contrastText',
                '&:hover': {
                  bgcolor: 'primary.dark',
                },
                '&.Mui-disabled': {
                  bgcolor: 'grey.300',
                },
              }}
            >
              {disabled ? (
                <CircularProgress size={24} color="inherit" />
              ) : (
                <SendIcon />
              )}
            </IconButton>
          </span>
        </Tooltip>
      </Box>

      {!disabled && (
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mt: 1 }}>
          <Typography variant="caption" color="text.secondary">
            Press
          </Typography>
          <kbd
            style={{
              padding: '2px 6px',
              borderRadius: 4,
              backgroundColor: 'grey.100',
              border: '1px solid grey.300',
              fontSize: 11,
            }}
          >
            Enter
          </kbd>
          <Typography variant="caption" color="text.secondary">
            to send,
          </Typography>
          <kbd
            style={{
              padding: '2px 6px',
              borderRadius: 4,
              backgroundColor: 'grey.100',
              border: '1px solid grey.300',
              fontSize: 11,
            }}
          >
            Shift + Enter
          </kbd>
          <Typography variant="caption" color="text.secondary">
            for new line
          </Typography>
        </Box>
      )}
    </Paper>
  );
};
