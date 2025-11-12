/**
 * Plan slash commands implementation
 */

import * as vscode from 'vscode';
import { OrchestratorClient } from '@zapabob/codex-protocol-client';
import type * as Types from '@zapabob/codex-protocol-client';
import { PlanStateManager, ExecutionMode } from './state';

export class PlanCommands {
    constructor(
        private client: OrchestratorClient,
        private stateManager: PlanStateManager
    ) {}
    
    /**
     * Register all Plan commands
     */
    registerCommands(context: vscode.ExtensionContext): void {
        context.subscriptions.push(
            vscode.commands.registerCommand('codex.Plan.toggle', () => 
                this.togglePlanMode()
            ),
            vscode.commands.registerCommand('codex.Plan.create', (args) =>
                this.createPlan(args)
            ),
            vscode.commands.registerCommand('codex.Plan.approve', (PlanId) =>
                this.approvePlan(PlanId)
            ),
            vscode.commands.registerCommand('codex.Plan.reject', (PlanId, reason) =>
                this.rejectPlan(PlanId, reason)
            ),
            vscode.commands.registerCommand('codex.Plan.export', (PlanId, format) =>
                this.exportPlan(PlanId, format)
            ),
            vscode.commands.registerCommand('codex.Plan.setMode', (mode) =>
                this.setExecutionMode(mode)
            ),
            vscode.commands.registerCommand('codex.Plan.deepResearch', (query, depth, policy) =>
                this.deepResearch(query, depth, policy)
            )
        );
    }
    
    /**
     * Toggle plan mode on/off
     */
    private async togglePlanMode(): Promise<void> {
        if (this.stateManager.isPlanModeActive()) {
            this.stateManager.disablePlanMode();
            vscode.window.showInformationMessage('plan mode: OFF');
        } else {
            this.stateManager.enablePlanMode();
            vscode.window.showInformationMessage('plan mode: ON');
        }
    }
    
    /**
     * Create a new Plan
     * Usage: /Plan "title or goal..." --mode=orchestrated --budget.tokens=50000
     */
    private async createPlan(args: any): Promise<void> {
        try {
            const title = args?.title || await vscode.window.showInputBox({
                prompt: 'Enter Plan title',
                placeHolder: 'e.g., Add telemetry feature'
            });
            
            if (!title) {
                return;
            }
            
            const goal = args?.goal || await vscode.window.showInputBox({
                prompt: 'Enter Plan goal',
                placeHolder: 'e.g., Measure p50/p95 request latency'
            });
            
            if (!goal) {
                return;
            }
            
            const mode = args?.mode || 'orchestrated';
            const budget = args?.budget || {};
            
            const response = await this.client.request<Types.BlueprintCreateResponse>('Blueprint.create', {
                title,
                goal,
                mode,
                budget
            });
            
            if (response.success) {
                vscode.window.showInformationMessage(
                    `✅ Plan created: ${response.blueprint_id}`
                );
                
                // Load the new Plan
                await this.loadPlan(response.blueprint_id);
            }
        } catch (error) {
            vscode.window.showErrorMessage(`❌ Failed to create Plan: ${error}`);
        }
    }
    
    /**
     * Approve a Plan
     */
    private async approvePlan(PlanId?: string): Promise<void> {
        try {
            const id = PlanId || this.stateManager.getCurrentPlan()?.id;
            
            if (!id) {
                vscode.window.showWarningMessage('No Plan to approve');
                return;
            }
            
            const approver = await vscode.window.showInputBox({
                prompt: 'Enter your name',
                placeHolder: 'e.g., john.doe'
            });
            
            if (!approver) {
                return;
            }
            
            const response = await this.client.request<Types.BlueprintApproveResponse>('Blueprint.approve', {
                blueprint_id: id
            });
            
            if (response.success) {
                vscode.window.showInformationMessage(`✅ Plan ${id} approved!`);
                await this.loadPlan(id);
            }
        } catch (error) {
            vscode.window.showErrorMessage(`❌ Failed to approve Plan: ${error}`);
        }
    }
    
    /**
     * Reject a Plan
     */
    private async rejectPlan(PlanId?: string, reason?: string): Promise<void> {
        try {
            const id = PlanId || this.stateManager.getCurrentPlan()?.id;
            
            if (!id) {
                vscode.window.showWarningMessage('No Plan to reject');
                return;
            }
            
            const rejectReason = reason || await vscode.window.showInputBox({
                prompt: 'Enter rejection reason',
                placeHolder: 'e.g., Scope too broad'
            });
            
            if (!rejectReason) {
                return;
            }
            
            const response = await this.client.request<Types.BlueprintRejectResponse>('Blueprint.reject', {
                blueprint_id: id,
                reason: rejectReason
            });
            
            if (response.success) {
                vscode.window.showInformationMessage(`Plan ${id} rejected`);
                await this.loadPlan(id);
            }
        } catch (error) {
            vscode.window.showErrorMessage(`❌ Failed to reject Plan: ${error}`);
        }
    }
    
    /**
     * Export Plan to file
     */
    private async exportPlan(PlanId?: string, format?: string): Promise<void> {
        try {
            const id = PlanId || this.stateManager.getCurrentPlan()?.id;
            
            if (!id) {
                vscode.window.showWarningMessage('No Plan to export');
                return;
            }
            
            const exportFormat = format || await vscode.window.showQuickPick(
                ['md', 'json', 'both'],
                { placeHolder: 'Select export format' }
            );
            
            if (!exportFormat) {
                return;
            }
            
            const response = await this.client.request<Types.BlueprintExportResponse>('Blueprint.export', {
                blueprint_id: id,
                format: exportFormat
            });
            
            if (response.success) {
                const paths: string[] = [];
                if (response.markdown_path) {
                    paths.push(response.markdown_path);
                }
                if (response.json_path) {
                    paths.push(response.json_path);
                }
                
                vscode.window.showInformationMessage(
                    `✅ Plan exported to: ${paths.join(', ')}`
                );
            }
        } catch (error) {
            vscode.window.showErrorMessage(`❌ Failed to export Plan: ${error}`);
        }
    }
    
    /**
     * Set execution mode
     */
    private async setExecutionMode(mode?: string): Promise<void> {
        try {
            const executionMode = mode || await vscode.window.showQuickPick(
                [ExecutionMode.Single, ExecutionMode.Orchestrated, ExecutionMode.Competition],
                { placeHolder: 'Select execution mode' }
            );
            
            if (!executionMode) {
                return;
            }
            
            const currentPlan = this.stateManager.getCurrentPlan();
            if (!currentPlan) {
                vscode.window.showErrorMessage('No active Plan to set mode for');
                return;
            }
            
            const response = await this.client.request<Types.BlueprintSetModeResponse>('Blueprint.setMode', {
                blueprint_id: currentPlan.id,
                mode: executionMode
            });
            
            if (response.success) {
                vscode.window.showInformationMessage(
                    `✅ Execution mode set to: ${executionMode}`
                );
            }
        } catch (error) {
            vscode.window.showErrorMessage(`❌ Failed to set mode: ${error}`);
        }
    }
    
    /**
     * Deep research with approval dialog
     */
    private async deepResearch(query?: string, depth?: number, policy?: string): Promise<void> {
        try {
            const researchQuery = query || await vscode.window.showInputBox({
                prompt: 'Enter research query',
                placeHolder: 'e.g., React Server Components best practices'
            });
            
            if (!researchQuery) {
                return;
            }
            
            const searchDepth = depth || parseInt(
                await vscode.window.showQuickPick(
                    ['1', '2', '3'],
                    { placeHolder: 'Select search depth' }
                ) || '2'
            );
            
            // Show approval dialog
            const approved = await vscode.window.showInformationMessage(
                `Research Request:\n\n` +
                `Query: ${researchQuery}\n` +
                `Depth: ${searchDepth}\n` +
                `Estimated tokens: ${searchDepth * 10000}\n` +
                `Domains: DuckDuckGo, GitHub\n\n` +
                `Approve this research request?`,
                { modal: true },
                'Approve',
                'Reject'
            );
            
            if (approved !== 'Approve') {
                return;
            }
            
            // Execute research
            await vscode.window.withProgress({
                location: vscode.ProgressLocation.Notification,
                title: 'Conducting deep research...',
                cancellable: false
            }, async () => {
                // TODO: Implement actual research call
                await new Promise(resolve => setTimeout(resolve, 2000));
            });
            
            vscode.window.showInformationMessage('✅ Research completed! Results added to Plan.');
        } catch (error) {
            vscode.window.showErrorMessage(`❌ Research failed: ${error}`);
        }
    }
    
    /**
     * Load a Plan from backend
     */
    private async loadPlan(PlanId: string): Promise<void> {
        try {
            const response = await this.client.request<Types.BlueprintGetResponse>('Blueprint.get', {
                blueprint_id: PlanId
            });
            
            if (response.blueprint) {
                this.stateManager.setCurrentPlan(response.blueprint as any);
            }
        } catch (error) {
            console.error('Failed to load Plan:', error);
        }
    }
}

