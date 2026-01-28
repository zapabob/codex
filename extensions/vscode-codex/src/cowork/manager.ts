/**
 * Cowork Integration Manager
 * 
 * Provides ClaudeCowork-style productivity features:
 * - Browser automation
 * - Document generation (Excel, Word, PowerPoint)
 * - External service connectors (Asana, Notion, etc.)
 * - Session management
 */

import * as vscode from 'vscode';
import { OrchestratorClient } from '@zapabob/codex-protocol-client';
import * as path from 'path';
import * as fs from 'fs';

export interface CoworkTask {
    id: string;
    type: 'browser' | 'document' | 'connector' | 'session';
    description: string;
    status: 'pending' | 'running' | 'completed' | 'failed';
    result?: any;
}

export class CoworkManager {
    private tasks: Map<string, CoworkTask> = new Map();
    
    constructor(
        private client: OrchestratorClient,
        private context: vscode.ExtensionContext
    ) {}
    
    /**
     * Register all Cowork commands
     */
    registerCommands(context: vscode.ExtensionContext): void {
        context.subscriptions.push(
            // Browser automation
            vscode.commands.registerCommand('codex.cowork.browser.navigate', () =>
                this.browserNavigate()
            ),
            vscode.commands.registerCommand('codex.cowork.browser.automate', () =>
                this.browserAutomate()
            ),
            
            // Document generation
            vscode.commands.registerCommand('codex.cowork.document.excel', () =>
                this.generateExcel()
            ),
            vscode.commands.registerCommand('codex.cowork.document.word', () =>
                this.generateWord()
            ),
            vscode.commands.registerCommand('codex.cowork.document.powerpoint', () =>
                this.generatePowerPoint()
            ),
            
            // External connectors
            vscode.commands.registerCommand('codex.cowork.connector.asana', () =>
                this.connectAsana()
            ),
            vscode.commands.registerCommand('codex.cowork.connector.notion', () =>
                this.connectNotion()
            ),
            
            // Session management
            vscode.commands.registerCommand('codex.cowork.session.create', () =>
                this.createSession()
            ),
            vscode.commands.registerCommand('codex.cowork.session.list', () =>
                this.listSessions()
            )
        );
    }
    
    /**
     * Browser navigation
     */
    private async browserNavigate(): Promise<void> {
        const url = await vscode.window.showInputBox({
            prompt: 'Enter URL to navigate',
            placeHolder: 'https://example.com'
        });
        
        if (!url) return;
        
        try {
            await vscode.window.withProgress({
                location: vscode.ProgressLocation.Notification,
                title: 'Navigating browser...',
                cancellable: false
            }, async () => {
                // Call Rust cowork integration
                await this.client.request('cowork.browser.navigate', { url });
            });
            
            vscode.window.showInformationMessage(`✅ Navigated to ${url}`);
        } catch (error) {
            vscode.window.showErrorMessage(`❌ Navigation failed: ${error}`);
        }
    }
    
    /**
     * Browser automation workflow
     */
    private async browserAutomate(): Promise<void> {
        const workflow = await vscode.window.showInputBox({
            prompt: 'Enter workflow steps (JSON)',
            placeHolder: '{"steps": [{"type": "navigate", "url": "..."}, {"type": "click", "selector": "..."}]}'
        });
        
        if (!workflow) return;
        
        try {
            await vscode.window.withProgress({
                location: vscode.ProgressLocation.Notification,
                title: 'Executing browser automation...',
                cancellable: false
            }, async () => {
                await this.client.request('cowork.browser.automate', {
                    workflow: JSON.parse(workflow)
                });
            });
            
            vscode.window.showInformationMessage('✅ Browser automation completed');
        } catch (error) {
            vscode.window.showErrorMessage(`❌ Automation failed: ${error}`);
        }
    }
    
    /**
     * Generate Excel document
     */
    private async generateExcel(): Promise<void> {
        const data = await vscode.window.showInputBox({
            prompt: 'Enter data (JSON array)',
            placeHolder: '[{"name": "John", "age": 30}, ...]'
        });
        
        if (!data) return;
        
        const outputPath = await vscode.window.showSaveDialog({
            defaultUri: vscode.Uri.file('output.xlsx'),
            filters: {
                'Excel Files': ['xlsx']
            }
        });
        
        if (!outputPath) return;
        
        try {
            await vscode.window.withProgress({
                location: vscode.ProgressLocation.Notification,
                title: 'Generating Excel document...',
                cancellable: false
            }, async () => {
                await this.client.request('cowork.document.excel', {
                    data: JSON.parse(data),
                    output_path: outputPath.fsPath
                });
            });
            
            vscode.window.showInformationMessage(`✅ Excel document saved to ${outputPath.fsPath}`);
        } catch (error) {
            vscode.window.showErrorMessage(`❌ Excel generation failed: ${error}`);
        }
    }
    
    /**
     * Generate Word document
     */
    private async generateWord(): Promise<void> {
        const content = await vscode.window.showInputBox({
            prompt: 'Enter document content (Markdown)',
            placeHolder: '# Title\n\nContent...'
        });
        
        if (!content) return;
        
        const outputPath = await vscode.window.showSaveDialog({
            defaultUri: vscode.Uri.file('output.docx'),
            filters: {
                'Word Files': ['docx']
            }
        });
        
        if (!outputPath) return;
        
        try {
            await vscode.window.withProgress({
                location: vscode.ProgressLocation.Notification,
                title: 'Generating Word document...',
                cancellable: false
            }, async () => {
                await this.client.request('cowork.document.word', {
                    content,
                    output_path: outputPath.fsPath
                });
            });
            
            vscode.window.showInformationMessage(`✅ Word document saved to ${outputPath.fsPath}`);
        } catch (error) {
            vscode.window.showErrorMessage(`❌ Word generation failed: ${error}`);
        }
    }
    
    /**
     * Generate PowerPoint presentation
     */
    private async generatePowerPoint(): Promise<void> {
        const slides = await vscode.window.showInputBox({
            prompt: 'Enter slides (JSON array)',
            placeHolder: '[{"title": "Slide 1", "content": "..."}, ...]'
        });
        
        if (!slides) return;
        
        const outputPath = await vscode.window.showSaveDialog({
            defaultUri: vscode.Uri.file('output.pptx'),
            filters: {
                'PowerPoint Files': ['pptx']
            }
        });
        
        if (!outputPath) return;
        
        try {
            await vscode.window.withProgress({
                location: vscode.ProgressLocation.Notification,
                title: 'Generating PowerPoint presentation...',
                cancellable: false
            }, async () => {
                await this.client.request('cowork.document.powerpoint', {
                    slides: JSON.parse(slides),
                    output_path: outputPath.fsPath
                });
            });
            
            vscode.window.showInformationMessage(`✅ PowerPoint saved to ${outputPath.fsPath}`);
        } catch (error) {
            vscode.window.showErrorMessage(`❌ PowerPoint generation failed: ${error}`);
        }
    }
    
    /**
     * Connect to Asana
     */
    private async connectAsana(): Promise<void> {
        const apiKey = await vscode.window.showInputBox({
            prompt: 'Enter Asana API key',
            password: true
        });
        
        if (!apiKey) return;
        
        try {
            await this.client.request('cowork.connector.asana.connect', {
                api_key: apiKey
            });
            
            vscode.window.showInformationMessage('✅ Connected to Asana');
        } catch (error) {
            vscode.window.showErrorMessage(`❌ Asana connection failed: ${error}`);
        }
    }
    
    /**
     * Connect to Notion
     */
    private async connectNotion(): Promise<void> {
        const apiKey = await vscode.window.showInputBox({
            prompt: 'Enter Notion API key',
            password: true
        });
        
        if (!apiKey) return;
        
        try {
            await this.client.request('cowork.connector.notion.connect', {
                api_key: apiKey
            });
            
            vscode.window.showInformationMessage('✅ Connected to Notion');
        } catch (error) {
            vscode.window.showErrorMessage(`❌ Notion connection failed: ${error}`);
        }
    }
    
    /**
     * Create new session
     */
    private async createSession(): Promise<void> {
        const name = await vscode.window.showInputBox({
            prompt: 'Enter session name',
            placeHolder: 'my-session'
        });
        
        if (!name) return;
        
        try {
            const response = await this.client.request('cowork.session.create', {
                name
            });
            
            vscode.window.showInformationMessage(`✅ Session created: ${response.session_id}`);
        } catch (error) {
            vscode.window.showErrorMessage(`❌ Session creation failed: ${error}`);
        }
    }
    
    /**
     * List all sessions
     */
    private async listSessions(): Promise<void> {
        try {
            const response = await this.client.request('cowork.session.list', {});
            
            if (response.sessions && response.sessions.length > 0) {
                const sessionList = response.sessions.map((s: any) => 
                    `${s.name} (${s.status})`
                ).join('\n');
                
                vscode.window.showInformationMessage(`Active Sessions:\n${sessionList}`);
            } else {
                vscode.window.showInformationMessage('No active sessions');
            }
        } catch (error) {
            vscode.window.showErrorMessage(`❌ Failed to list sessions: ${error}`);
        }
    }
}
