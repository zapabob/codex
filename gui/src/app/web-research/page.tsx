'use client';

import React, { useState } from 'react';
import {
  Box,
  Typography,
  TextField,
  Button,
  Paper,
  Alert,
  Chip,
  CircularProgress,
} from '@mui/material';
import { Search } from 'lucide-react';
import { DashboardLayout } from '@/components/templates/DashboardLayout';
import { Card } from '@/components/atoms/Card';
import { useCodex } from '@/lib/context/CodexContext';

export default function WebResearchPage() {
  const { state, runWebResearch } = useCodex();
  const [query, setQuery] = useState('');
  const [isRunning, setIsRunning] = useState(false);

  const handleResearch = async () => {
    if (!query.trim()) return;

    setIsRunning(true);
    try {
      await runWebResearch(query);
      setQuery('');
    } catch (error) {
      console.error('Web research failed:', error);
    } finally {
      setIsRunning(false);
    }
  };

  return (
    <DashboardLayout title="Web Research">
      <Box sx={{ p: 3 }}>
        <Typography variant="h4" sx={{ mb: 2, fontWeight: 700 }}>
          Web Research
        </Typography>
        <Typography variant="body1" color="text.secondary" sx={{ mb: 4 }}>
          Official web_search tool for fast web queries and citations.
        </Typography>

        <Card header={
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
            <Search size={20} />
            <Typography variant="h6">New Web Research</Typography>
          </Box>
        }>
          <Box sx={{ display: 'flex', gap: 2, alignItems: 'flex-start' }}>
            <TextField
              fullWidth
              multiline
              rows={3}
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="Enter a web research query..."
              sx={{ flex: 1 }}
            />
            <Button
              variant="contained"
              onClick={handleResearch}
              disabled={!query.trim() || isRunning}
              sx={{ minWidth: 140, height: 56 }}
              startIcon={isRunning ? <CircularProgress size={16} /> : <Search />}
            >
              {isRunning ? 'Searching...' : 'Run Search'}
            </Button>
          </Box>
        </Card>

        {state.error && (
          <Alert severity="error" sx={{ mt: 3 }}>
            {state.error}
          </Alert>
        )}

        <Box sx={{ mt: 4 }}>
          <Typography variant="h5" sx={{ mb: 3, fontWeight: 600 }}>
            Web Research History
          </Typography>

          {state.webResearchResults.length === 0 ? (
            <Paper sx={{ p: 4, textAlign: 'center', color: 'text.secondary' }}>
              <Typography variant="h6" gutterBottom>
                No web research results yet
              </Typography>
              <Typography variant="body2">
                Run a query to see the official web_search output here.
              </Typography>
            </Paper>
          ) : (
            <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
              {state.webResearchResults.map((result) => (
                <Paper key={result.id} sx={{ p: 3 }}>
                  <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mb: 2 }}>
                    <Typography variant="h6" sx={{ flex: 1 }}>
                      {result.query}
                    </Typography>
                    <Chip
                      label={result.status === 'completed' ? 'Completed' : 'Failed'}
                      color={result.status === 'completed' ? 'success' : 'error'}
                      size="small"
                    />
                  </Box>
                  {result.output && (
                    <Paper
                      sx={{
                        p: 2,
                        backgroundColor: 'grey.900',
                        color: 'grey.100',
                        fontFamily: 'monospace',
                        fontSize: '14px',
                        whiteSpace: 'pre-wrap',
                      }}
                    >
                      {result.output}
                    </Paper>
                  )}
                </Paper>
              ))}
            </Box>
          )}
        </Box>
      </Box>
    </DashboardLayout>
  );
}
