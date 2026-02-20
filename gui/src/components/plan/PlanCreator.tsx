'use client';

import React, { useState, useEffect } from 'react';
import {
  Box,
  Typography,
  TextField,
  Button,
  Select,
  MenuItem,
  FormControl,
  InputLabel,
  Card as MuiCard,
  CardContent,
  CardActions,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  Alert,
  Chip,
  List,
  ListItem,
  ListItemText,
  
  
  
  LinearProgress,
  Tabs,
  Tab,
  Accordion,
  AccordionSummary,
  AccordionDetails,
  
} from '@mui/material';
import Grid from '@/mui/Grid2';
import {
  Plus,
  Edit,
  Play,
  CheckCircle,
  XCircle,
  Clock,
  FileText,
  ChevronDown,
  
  
  Download,
} from 'lucide-react';
import { Card } from '../atoms/Card';
import { CodexAPIClient } from '../../lib/api/client';

interface Plan {
  id: string;
  title: string;
  mode: 'single' | 'orchestrated' | 'competition';
  budgetTokens: number;
  budgetTime: number;
  state: 'drafting' | 'pending' | 'approved' | 'rejected' | 'executing' | 'completed';
  createdAt: Date;
  updatedAt: Date;
  blocks?: PlanBlock[];
}

interface PlanBlock {
  id: string;
  title: string;
  description: string;
  order: number;
  status: 'pending' | 'running' | 'completed' | 'failed';
}

interface PlanCreatorProps {
  onPlanCreated?: (plan: Plan) => void;
  onPlanExecuted?: (planId: string) => void;
}

export const PlanCreator: React.FC<PlanCreatorProps> = ({
  onPlanCreated,
  onPlanExecuted,
}) => {
  const [plans, setPlans] = useState<Plan[]>([]);
  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [editDialogOpen, setEditDialogOpen] = useState(false);
  const [executeDialogOpen, setExecuteDialogOpen] = useState(false);
  const [selectedPlan, setSelectedPlan] = useState<Plan | null>(null);
  const [activeTab, setActiveTab] = useState(0);
  const [isLoading, setIsLoading] = useState(false);

  const [newPlan, setNewPlan] = useState({
    title: '',
    mode: 'orchestrated' as 'single' | 'orchestrated' | 'competition',
    budgetTokens: 100000,
    budgetTime: 30,
  });

  const apiClient = React.useMemo(() => new CodexAPIClient(), []);

  useEffect(() => {
    loadPlans();
  }, []);

  const loadPlans = async () => {
    try {
      setIsLoading(true);
      const plans = await apiClient.listPlans();
      setPlans(plans || []);
    } catch (error) {
      console.error('Failed to load plans:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleCreatePlan = async () => {
    if (!newPlan.title.trim()) return;

    try {
      setIsLoading(true);
      const result = await apiClient.createPlan({
        title: newPlan.title,
        mode: newPlan.mode,
        budgetTokens: newPlan.budgetTokens,
        budgetTime: newPlan.budgetTime,
      });

      const createdPlan: Plan = {
        id: result.id || `plan-${Date.now()}`,
        title: result.title || newPlan.title,
        mode: result.mode || newPlan.mode,
        budgetTokens: result.budgetTokens || newPlan.budgetTokens,
        budgetTime: result.budgetTime || newPlan.budgetTime,
        state: result.state || 'drafting',
        createdAt: result.createdAt ? new Date(result.createdAt) : new Date(),
        updatedAt: result.updatedAt ? new Date(result.updatedAt) : new Date(),
      };

      setPlans([createdPlan, ...plans]);
      setCreateDialogOpen(false);
      setNewPlan({ title: '', mode: 'orchestrated', budgetTokens: 100000, budgetTime: 30 });
      onPlanCreated?.(createdPlan);
    } catch (error) {
      console.error('Failed to create plan:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleApprovePlan = async (planId: string) => {
    try {
      setIsLoading(true);
      await apiClient.approvePlan(planId);
      
      setPlans(plans.map(p => 
        p.id === planId ? { ...p, state: 'approved' as const, updatedAt: new Date() } : p
      ));
    } catch (error) {
      console.error('Failed to approve plan:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleExecutePlan = async (planId: string) => {
    try {
      setIsLoading(true);
      await apiClient.executePlan(planId);
      
      setPlans(plans.map(p => 
        p.id === planId ? { ...p, state: 'executing' as const, updatedAt: new Date() } : p
      ));
      
      setExecuteDialogOpen(false);
      onPlanExecuted?.(planId);
    } catch (error) {
      console.error('Failed to execute plan:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleRejectPlan = async (planId: string, reason: string) => {
    try {
      setIsLoading(true);
      await apiClient.rejectPlan(planId, reason);
      
      setPlans(plans.map(p => 
        p.id === planId ? { ...p, state: 'rejected' as const, updatedAt: new Date() } : p
      ));
    } catch (error) {
      console.error('Failed to reject plan:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const getStateIcon = (state: Plan['state']) => {
    switch (state) {
      case 'approved':
        return <CheckCircle size={16} color="#4caf50" />;
      case 'rejected':
        return <XCircle size={16} color="#f44336" />;
      case 'executing':
        return <Clock size={16} color="#ff9800" />;
      case 'completed':
        return <CheckCircle size={16} color="#2196f3" />;
      default:
        return <FileText size={16} />;
    }
  };

  const getStateColor = (state: Plan['state']) => {
    switch (state) {
      case 'approved':
        return 'success';
      case 'rejected':
        return 'error';
      case 'executing':
        return 'warning';
      case 'completed':
        return 'info';
      default:
        return 'default';
    }
  };

  const filteredPlans = plans.filter(plan => {
    if (activeTab === 0) return plan.state === 'drafting' || plan.state === 'pending';
    if (activeTab === 1) return plan.state === 'approved';
    if (activeTab === 2) return plan.state === 'executing' || plan.state === 'completed';
    return true;
  });

  return (
    <Box>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
        <Typography variant="h5" sx={{ fontWeight: 700 }}>
          Execution Plans
        </Typography>
        <Button
          variant="contained"
          startIcon={<Plus size={20} />}
          onClick={() => setCreateDialogOpen(true)}
        >
          Create Plan
        </Button>
      </Box>

      <Tabs value={activeTab} onChange={(_, v) => setActiveTab(v)} sx={{ mb: 3 }}>
        <Tab label="Drafting" />
        <Tab label="Approved" />
        <Tab label="Executing/Completed" />
        <Tab label="All" />
      </Tabs>

      {isLoading && <LinearProgress sx={{ mb: 2 }} />}

      {filteredPlans.length === 0 ? (
        <Card>
          <Box sx={{ p: 4, textAlign: 'center', color: 'text.secondary' }}>
            <FileText size={48} style={{ marginBottom: 16, opacity: 0.5 }} />
            <Typography variant="h6" gutterBottom>
              No plans found
            </Typography>
            <Typography variant="body2">
              Create a new plan to get started
            </Typography>
          </Box>
        </Card>
      ) : (
        <Grid container spacing={2}>
          {filteredPlans.map((plan) => (
            <Grid xs={12} md={6} key={plan.id}>
              <MuiCard>
                <CardContent>
                  <Box sx={{ display: 'flex', alignItems: 'center', mb: 2 }}>
                    {getStateIcon(plan.state)}
                    <Typography variant="h6" sx={{ ml: 1, flex: 1 }}>
                      {plan.title}
                    </Typography>
                    <Chip
                      label={plan.state}
                      color={getStateColor(plan.state)}
                      size="small"
                    />
                  </Box>

                  <Box sx={{ mb: 2 }}>
                    <Typography variant="body2" color="text.secondary" gutterBottom>
                      Mode: <strong>{plan.mode}</strong>
                    </Typography>
                    <Typography variant="body2" color="text.secondary" gutterBottom>
                      Budget: {plan.budgetTokens.toLocaleString()} tokens, {plan.budgetTime} min
                    </Typography>
                    <Typography variant="caption" color="text.secondary">
                      Created: {plan.createdAt.toLocaleString()}
                    </Typography>
                  </Box>

                  {plan.blocks && plan.blocks.length > 0 && (
                    <Accordion>
                      <AccordionSummary expandIcon={<ChevronDown />}>
                        <Typography variant="subtitle2">
                          Blocks ({plan.blocks.length})
                        </Typography>
                      </AccordionSummary>
                      <AccordionDetails>
                        <List dense>
                          {plan.blocks.map((block) => (
                            <ListItem key={block.id}>
                              <ListItemText
                                primary={block.title}
                                secondary={block.description}
                              />
                              <Chip
                                label={block.status}
                                size="small"
                                color={block.status === 'completed' ? 'success' : 'default'}
                              />
                            </ListItem>
                          ))}
                        </List>
                      </AccordionDetails>
                    </Accordion>
                  )}
                </CardContent>
                <CardActions>
                  {plan.state === 'drafting' || plan.state === 'pending' ? (
                    <>
                      <Button
                        size="small"
                        onClick={() => handleApprovePlan(plan.id)}
                      >
                        Approve
                      </Button>
                      <Button
                        size="small"
                        color="error"
                        onClick={() => handleRejectPlan(plan.id, 'User rejected')}
                      >
                        Reject
                      </Button>
                    </>
                  ) : plan.state === 'approved' ? (
                    <Button
                      size="small"
                      variant="contained"
                      startIcon={<Play size={16} />}
                      onClick={() => {
                        setSelectedPlan(plan);
                        setExecuteDialogOpen(true);
                      }}
                    >
                      Execute
                    </Button>
                  ) : null}
                  <Button size="small" startIcon={<Edit size={16} />}>
                    Edit
                  </Button>
                  <Button size="small" startIcon={<Download size={16} />}>
                    Export
                  </Button>
                </CardActions>
              </MuiCard>
            </Grid>
          ))}
        </Grid>
      )}

      {/* Create Plan Dialog */}
      <Dialog open={createDialogOpen} onClose={() => setCreateDialogOpen(false)} maxWidth="sm" fullWidth>
        <DialogTitle>Create New Plan</DialogTitle>
        <DialogContent>
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2, pt: 2 }}>
            <TextField
              fullWidth
              label="Plan Title"
              value={newPlan.title}
              onChange={(e) => setNewPlan({ ...newPlan, title: e.target.value })}
              placeholder="e.g., Refactor authentication system"
            />
            <FormControl fullWidth>
              <InputLabel>Execution Mode</InputLabel>
              <Select
                value={newPlan.mode}
                onChange={(e) => setNewPlan({ ...newPlan, mode: e.target.value as any })}
                label="Execution Mode"
              >
                <MenuItem value="single">Single</MenuItem>
                <MenuItem value="orchestrated">Orchestrated</MenuItem>
                <MenuItem value="competition">Competition</MenuItem>
              </Select>
            </FormControl>
            <TextField
              fullWidth
              type="number"
              label="Token Budget"
              value={newPlan.budgetTokens}
              onChange={(e) => setNewPlan({ ...newPlan, budgetTokens: parseInt(e.target.value) || 0 })}
            />
            <TextField
              fullWidth
              type="number"
              label="Time Budget (minutes)"
              value={newPlan.budgetTime}
              onChange={(e) => setNewPlan({ ...newPlan, budgetTime: parseInt(e.target.value) || 0 })}
            />
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setCreateDialogOpen(false)}>Cancel</Button>
          <Button
            variant="contained"
            onClick={handleCreatePlan}
            disabled={!newPlan.title.trim() || isLoading}
          >
            Create
          </Button>
        </DialogActions>
      </Dialog>

      {/* Execute Plan Dialog */}
      <Dialog open={executeDialogOpen} onClose={() => setExecuteDialogOpen(false)} maxWidth="sm" fullWidth>
        <DialogTitle>Execute Plan</DialogTitle>
        <DialogContent>
          {selectedPlan && (
            <Box>
              <Alert severity="info" sx={{ mb: 2 }}>
                Are you sure you want to execute "{selectedPlan.title}"?
              </Alert>
              <Typography variant="body2" color="text.secondary">
                This will start the execution of all approved blocks in the plan.
              </Typography>
            </Box>
          )}
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setExecuteDialogOpen(false)}>Cancel</Button>
          <Button
            variant="contained"
            onClick={() => selectedPlan && handleExecutePlan(selectedPlan.id)}
            disabled={isLoading}
          >
            Execute
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
};

PlanCreator.displayName = 'PlanCreator';

export default PlanCreator;

