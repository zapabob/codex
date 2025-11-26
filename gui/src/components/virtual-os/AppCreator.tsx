'use client';

import React, { useState } from 'react';
import {
  Box,
  Typography,
  Card,
  CardContent,
  TextField,
  Button,
  Select,
  MenuItem,
  FormControl,
  InputLabel,
  Chip,
  Alert,
  LinearProgress,
  Paper,
  IconButton,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
} from '@mui/material';
import Grid from '@/mui/Grid2';
import {
  Code,
  Play,
  FileText,
  Monitor,
  Terminal,
  Library,
  Gamepad2,
  X,
  CheckCircle,
} from 'lucide-react';
import { CodexAPIClient } from '@/lib/api/client';

interface AppTemplate {
  id: string;
  name: string;
  description: string;
  icon: React.ReactNode;
}

const templates: AppTemplate[] = [
  {
    id: 'webapp',
    name: 'Web Application',
    description: 'HTML/CSS/JavaScript based web app',
    icon: <Code size={24} />,
  },
  {
    id: 'desktop',
    name: 'Desktop Application',
    description: 'Rust-based desktop app',
    icon: <Monitor size={24} />,
  },
  {
    id: 'cli',
    name: 'Command Line Tool',
    description: 'CLI application',
    icon: <Terminal size={24} />,
  },
  {
    id: 'library',
    name: 'Library',
    description: 'Reusable library project',
    icon: <Library size={24} />,
  },
  {
    id: 'game',
    name: 'Game',
    description: 'Game project',
    icon: <Gamepad2 size={24} />,
  },
];

export function AppCreator() {
  const [appName, setAppName] = useState('');
  const [selectedTemplate, setSelectedTemplate] = useState<string>('');
  const [description, setDescription] = useState('');
  const [language, setLanguage] = useState('rust');
  const [framework, setFramework] = useState('');
  const [features, setFeatures] = useState<string[]>([]);
  const [isGenerating, setIsGenerating] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const [previewDialogOpen, setPreviewDialogOpen] = useState(false);
  const [generatedFiles, setGeneratedFiles] = useState<string[]>([]);

  const apiClient = new CodexAPIClient();

  const handleTemplateSelect = (templateId: string) => {
    setSelectedTemplate(templateId);
    // Set default language based on template
    if (templateId === 'webapp') {
      setLanguage('javascript');
      setFramework('react');
    } else if (templateId === 'desktop' || templateId === 'cli' || templateId === 'library') {
      setLanguage('rust');
      setFramework('');
    }
  };

  const handleFeatureToggle = (feature: string) => {
    setFeatures(prev =>
      prev.includes(feature)
        ? prev.filter(f => f !== feature)
        : [...prev, feature]
    );
  };

  const handleGenerate = async () => {
    if (!appName.trim() || !selectedTemplate) {
      setError('アプリ名とテンプレートを選択してください');
      return;
    }

    setIsGenerating(true);
    setError(null);
    setSuccess(null);

    try {
      // TODO: Call actual API
      // const result = await apiClient.createApp({
      //   name: appName,
      //   template: selectedTemplate,
      //   description,
      //   language,
      //   framework,
      //   features,
      // });

      // Simulate generation
      await new Promise(resolve => setTimeout(resolve, 2000));

      const mockFiles = [
        `${appName}/src/main.rs`,
        `${appName}/Cargo.toml`,
        `${appName}/README.md`,
      ];
      setGeneratedFiles(mockFiles);
      setSuccess('アプリケーションが正常に生成されました');
      setPreviewDialogOpen(true);
    } catch (err: any) {
      setError(err.message || 'アプリケーション生成に失敗しました');
    } finally {
      setIsGenerating(false);
    }
  };

  const handleBuild = async () => {
    // TODO: Implement build functionality
    setSuccess('ビルドを開始しました');
  };

  const handleRun = async () => {
    // TODO: Implement run functionality
    setSuccess('アプリケーションを実行しました');
  };

  const selectedTemplateData = templates.find(t => t.id === selectedTemplate);

  return (
    <Box sx={{ p: 3 }}>
      <Typography variant="h5" sx={{ mb: 3, fontWeight: 700 }}>
        アプリケーション作成
      </Typography>

      {error && (
        <Alert severity="error" sx={{ mb: 2 }} onClose={() => setError(null)}>
          {error}
        </Alert>
      )}

      {success && (
        <Alert severity="success" sx={{ mb: 2 }} onClose={() => setSuccess(null)}>
          {success}
        </Alert>
      )}

      <Grid container spacing={3}>
        {/* Template Selection */}
        <Grid xs={12} md={4}>
          <Card>
            <CardContent>
              <Typography variant="h6" sx={{ mb: 2 }}>
                テンプレート選択
              </Typography>
              <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
                {templates.map(template => (
                  <Paper
                    key={template.id}
                    onClick={() => handleTemplateSelect(template.id)}
                    sx={{
                      p: 2,
                      cursor: 'pointer',
                      bgcolor: selectedTemplate === template.id ? 'primary.main' : 'background.paper',
                      color: selectedTemplate === template.id ? 'white' : 'text.primary',
                      '&:hover': {
                        bgcolor: selectedTemplate === template.id ? 'primary.dark' : 'action.hover',
                      },
                      transition: 'all 0.2s',
                    }}
                  >
                    <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
                      {template.icon}
                      <Box>
                        <Typography variant="subtitle1" sx={{ fontWeight: 600 }}>
                          {template.name}
                        </Typography>
                        <Typography variant="caption" color="text.secondary">
                          {template.description}
                        </Typography>
                      </Box>
                    </Box>
                  </Paper>
                ))}
              </Box>
            </CardContent>
          </Card>
        </Grid>

        {/* App Configuration */}
        <Grid xs={12} md={8}>
          <Card>
            <CardContent>
              <Typography variant="h6" sx={{ mb: 2 }}>
                アプリケーション設定
              </Typography>

              <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
                <TextField
                  label="アプリ名"
                  value={appName}
                  onChange={(e) => setAppName(e.target.value)}
                  placeholder="my-app"
                  fullWidth
                  required
                />

                <TextField
                  label="説明"
                  value={description}
                  onChange={(e) => setDescription(e.target.value)}
                  placeholder="アプリケーションの説明"
                  fullWidth
                  multiline
                  rows={3}
                />

                <FormControl fullWidth>
                  <InputLabel>言語</InputLabel>
                  <Select
                    value={language}
                    onChange={(e) => setLanguage(e.target.value)}
                    label="言語"
                  >
                    <MenuItem value="rust">Rust</MenuItem>
                    <MenuItem value="javascript">JavaScript</MenuItem>
                    <MenuItem value="typescript">TypeScript</MenuItem>
                    <MenuItem value="python">Python</MenuItem>
                  </Select>
                </FormControl>

                {selectedTemplate === 'webapp' && (
                  <FormControl fullWidth>
                    <InputLabel>フレームワーク</InputLabel>
                    <Select
                      value={framework}
                      onChange={(e) => setFramework(e.target.value)}
                      label="フレームワーク"
                    >
                      <MenuItem value="react">React</MenuItem>
                      <MenuItem value="vue">Vue</MenuItem>
                      <MenuItem value="svelte">Svelte</MenuItem>
                      <MenuItem value="vanilla">Vanilla JS</MenuItem>
                    </Select>
                  </FormControl>
                )}

                <Box>
                  <Typography variant="subtitle2" sx={{ mb: 1 }}>
                    機能
                  </Typography>
                  <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 1 }}>
                    {['Authentication', 'Database', 'API', 'UI Components', 'Testing'].map(feature => (
                      <Chip
                        key={feature}
                        label={feature}
                        onClick={() => handleFeatureToggle(feature)}
                        color={features.includes(feature) ? 'primary' : 'default'}
                        variant={features.includes(feature) ? 'filled' : 'outlined'}
                      />
                    ))}
                  </Box>
                </Box>

                {isGenerating && <LinearProgress />}

                <Box sx={{ display: 'flex', gap: 2 }}>
                  <Button
                    variant="contained"
                    startIcon={<Code />}
                    onClick={handleGenerate}
                    disabled={isGenerating || !appName.trim() || !selectedTemplate}
                    fullWidth
                  >
                    {isGenerating ? '生成中...' : 'コード生成'}
                  </Button>
                  {generatedFiles.length > 0 && (
                    <>
                      <Button
                        variant="outlined"
                        startIcon={<Play />}
                        onClick={handleRun}
                      >
                        実行
                      </Button>
                      <Button
                        variant="outlined"
                        startIcon={<FileText />}
                        onClick={handleBuild}
                      >
                        ビルド
                      </Button>
                    </>
                  )}
                </Box>
              </Box>
            </CardContent>
          </Card>
        </Grid>
      </Grid>

      {/* Preview Dialog */}
      <Dialog
        open={previewDialogOpen}
        onClose={() => setPreviewDialogOpen(false)}
        maxWidth="md"
        fullWidth
      >
        <DialogTitle>
          <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
            <Typography variant="h6">生成されたファイル</Typography>
            <IconButton onClick={() => setPreviewDialogOpen(false)}>
              <X />
            </IconButton>
          </Box>
        </DialogTitle>
        <DialogContent>
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
            {generatedFiles.map((file, index) => (
              <Paper key={index} sx={{ p: 2 }}>
                <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                  <CheckCircle size={16} color="green" />
                  <Typography variant="body2" sx={{ fontFamily: 'monospace' }}>
                    {file}
                  </Typography>
                </Box>
              </Paper>
            ))}
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setPreviewDialogOpen(false)}>閉じる</Button>
          <Button variant="contained" onClick={handleBuild}>
            ビルド
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
}

