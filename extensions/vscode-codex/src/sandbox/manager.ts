/**
 * macOS-style Sandbox Manager
 * 
 * Provides sandbox management UI and controls for Codex execution environment.
 * Supports macOS Seatbelt, Linux seccomp, and Windows Restricted Token.
 */

import * as vscode from 'vscode';
import { OrchestratorClient } from '@zapabob/codex-protocol-client';

export interface SandboxConfig {
    type: 'macos-seatbelt' | 'linux-seccomp' | 'windows-restricted' | 'none';
    networkAccess: boolean;
    filesystemAccess: 'read-only' | 'read-write' | 'isolated';
    allowedPaths: string[];
    deniedPaths: string[];
    internetAccess: boolean;
    justification?: string;
}

export class SandboxManager {
    private currentConfig: SandboxConfig | null = null;
    
    constructor(
        private client: OrchestratorClient,
        private context: vscode.ExtensionContext
    ) {}
    
    /**
     * Register all sandbox commands
     */
    registerCommands(context: vscode.ExtensionContext): void {
        context.subscriptions.push(
            vscode.commands.registerCommand('codex.sandbox.configure', () =>
                this.configureSandbox()
            ),
            vscode.commands.registerCommand('codex.sandbox.status', () =>
                this.showStatus()
            ),
            vscode.commands.registerCommand('codex.sandbox.enable', () =>
                this.enableSandbox()
            ),
            vscode.commands.registerCommand('codex.sandbox.disable', () =>
                this.disableSandbox()
            ),
            vscode.commands.registerCommand('codex.sandbox.allowPath', () =>
                this.allowPath()
            ),
            vscode.commands.registerCommand('codex.sandbox.denyPath', () =>
                this.denyPath()
            )
        );
    }
    
    /**
     * Configure sandbox settings
     */
    private async configureSandbox(): Promise<void> {
        const platform = process.platform;
        let sandboxType: string;
        
        if (platform === 'darwin') {
            sandboxType = 'macos-seatbelt';
        } else if (platform === 'linux') {
            sandboxType = 'linux-seccomp';
        } else if (platform === 'win32') {
            sandboxType = 'windows-restricted';
        } else {
            sandboxType = 'none';
        }
        
        const networkAccess = await vscode.window.showQuickPick(
            ['Yes', 'No'],
            { placeHolder: 'Allow network access?' }
        );
        
        const filesystemAccess = await vscode.window.showQuickPick(
            ['read-only', 'read-write', 'isolated'],
            { placeHolder: 'Filesystem access level' }
        );
        
        const internetAccess = await vscode.window.showQuickPick(
            ['Yes', 'No'],
            { placeHolder: 'Allow internet access?' }
        );
        
        if (!networkAccess || !filesystemAccess || !internetAccess) {
            return;
        }
        
        const config: SandboxConfig = {
            type: sandboxType as any,
            networkAccess: networkAccess === 'Yes',
            filesystemAccess: filesystemAccess as any,
            allowedPaths: [],
            deniedPaths: [],
            internetAccess: internetAccess === 'Yes'
        };
        
        try {
            await this.client.request('sandbox.configure', config);
            this.currentConfig = config;
            
            vscode.window.showInformationMessage(
                `✅ Sandbox configured: ${sandboxType} (${filesystemAccess})`
            );
        } catch (error) {
            vscode.window.showErrorMessage(`❌ Sandbox configuration failed: ${error}`);
        }
    }
    
    /**
     * Show current sandbox status
     */
    private async showStatus(): Promise<void> {
        try {
            const response = await this.client.request('sandbox.status', {});
            
            const status = response.enabled
                ? `🟢 Sandbox: ${response.type}\n` +
                  `Network: ${response.network_access ? 'Allowed' : 'Denied'}\n` +
                  `Filesystem: ${response.filesystem_access}\n` +
                  `Internet: ${response.internet_access ? 'Allowed' : 'Denied'}`
                : '🔴 Sandbox: Disabled';
            
            vscode.window.showInformationMessage(status);
        } catch (error) {
            vscode.window.showErrorMessage(`❌ Failed to get sandbox status: ${error}`);
        }
    }
    
    /**
     * Enable sandbox
     */
    private async enableSandbox(): Promise<void> {
        try {
            await this.client.request('sandbox.enable', {});
            vscode.window.showInformationMessage('✅ Sandbox enabled');
        } catch (error) {
            vscode.window.showErrorMessage(`❌ Failed to enable sandbox: ${error}`);
        }
    }
    
    /**
     * Disable sandbox
     */
    private async disableSandbox(): Promise<void> {
        const confirmed = await vscode.window.showWarningMessage(
            'Disabling sandbox will allow unrestricted access. Continue?',
            { modal: true },
            'Yes',
            'No'
        );
        
        if (confirmed !== 'Yes') {
            return;
        }
        
        try {
            await this.client.request('sandbox.disable', {});
            vscode.window.showInformationMessage('⚠️ Sandbox disabled');
        } catch (error) {
            vscode.window.showErrorMessage(`❌ Failed to disable sandbox: ${error}`);
        }
    }
    
    /**
     * Allow a specific path
     */
    private async allowPath(): Promise<void> {
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (!workspaceFolders) {
            vscode.window.showWarningMessage('No workspace folder open');
            return;
        }
        
        const folderPath = await vscode.window.showOpenDialog({
            canSelectFiles: false,
            canSelectFolders: true,
            defaultUri: workspaceFolders[0].uri
        });
        
        if (!folderPath || folderPath.length === 0) {
            return;
        }
        
        try {
            await this.client.request('sandbox.allowPath', {
                path: folderPath[0].fsPath
            });
            
            vscode.window.showInformationMessage(
                `✅ Path allowed: ${folderPath[0].fsPath}`
            );
        } catch (error) {
            vscode.window.showErrorMessage(`❌ Failed to allow path: ${error}`);
        }
    }
    
    /**
     * Deny a specific path
     */
    private async denyPath(): Promise<void> {
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (!workspaceFolders) {
            vscode.window.showWarningMessage('No workspace folder open');
            return;
        }
        
        const folderPath = await vscode.window.showOpenDialog({
            canSelectFiles: false,
            canSelectFolders: true,
            defaultUri: workspaceFolders[0].uri
        });
        
        if (!folderPath || folderPath.length === 0) {
            return;
        }
        
        try {
            await this.client.request('sandbox.denyPath', {
                path: folderPath[0].fsPath
            });
            
            vscode.window.showInformationMessage(
                `✅ Path denied: ${folderPath[0].fsPath}`
            );
        } catch (error) {
            vscode.window.showErrorMessage(`❌ Failed to deny path: ${error}`);
        }
    }
    
    /**
     * Get current sandbox configuration
     */
    getCurrentConfig(): SandboxConfig | null {
        return this.currentConfig;
    }
}
