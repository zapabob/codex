/**
 * LSP Client for connecting to language servers
 * Provides real-time diagnostics, completion, and symbol search
 */

// LSPメッセージ型
export interface LspRequestMessage {
  jsonrpc: '2.0';
  id: number | string;
  method: string;
  params?: unknown;
}

export interface LspResponseMessage {
  jsonrpc: '2.0';
  id: number | string;
  result?: unknown;
  error?: {
    code: number;
    message: string;
    data?: unknown;
  };
}

export interface LspNotificationMessage {
  jsonrpc: '2.0';
  method: string;
  params?: unknown;
}

export type LspMessage = LspRequestMessage | LspResponseMessage | LspNotificationMessage;

export interface LspDiagnostic {
  range: {
    start: { line: number; character: number };
    end: { line: number; character: number };
  };
  severity: 'error' | 'warning' | 'information' | 'hint';
  code?: string | number;
  source?: string;
  message: string;
  relatedInformation?: Array<{
    location: {
      uri: string;
      range: {
        start: { line: number; character: number };
        end: { line: number; character: number };
      };
    };
    message: string;
  }>;
}

export interface LspCompletionItem {
  label: string;
  kind?: number;
  detail?: string;
  documentation?: string;
  insertText?: string;
  textEdit?: {
    range: {
      start: { line: number; character: number };
      end: { line: number; character: number };
    };
    newText: string;
  };
}

export interface LspHover {
  contents: string | Array<{ language: string; value: string }>;
  range?: {
    start: { line: number; character: number };
    end: { line: number; character: number };
  };
}

export class LspClient {
  private ws: WebSocket | null = null;
  private requestId = 1;
  private pendingRequests = new Map<number, {
    resolve: (value: unknown) => void;
    reject: (error: Error) => void;
  }>();
  private diagnosticsCallbacks: Set<(diagnostics: Map<string, LspDiagnostic[]>) => void> = new Set();
  private diagnosticsCache = new Map<string, LspDiagnostic[]>();

  constructor(private serverUrl: string) {}

  /**
   * Connect to the LSP server via WebSocket
   */
  async connect(): Promise<void> {
    return new Promise((resolve, reject) => {
      try {
        this.ws = new WebSocket(this.serverUrl);
        
        this.ws.onopen = () => {
          console.log('LSP WebSocket connected');
          resolve();
        };

        this.ws.onmessage = (event) => {
          this.handleMessage(JSON.parse(event.data));
        };

        this.ws.onerror = (error) => {
          console.error('LSP WebSocket error:', error);
          reject(new Error('Failed to connect to LSP server'));
        };

        this.ws.onclose = () => {
          console.log('LSP WebSocket closed');
          this.ws = null;
        };
      } catch (error) {
        reject(error);
      }
    });
  }

  /**
   * Disconnect from the LSP server
   */
  disconnect(): void {
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
  }

  /**
   * Handle incoming messages from the LSP server
   */
  private handleMessage(message: LspMessage): void {
    // Handle responses to requests
    if (message.id && this.pendingRequests.has(message.id)) {
      const { resolve, reject } = this.pendingRequests.get(message.id)!;
      this.pendingRequests.delete(message.id);

      if (message.error) {
        reject(new Error(message.error.message || 'LSP request failed'));
      } else {
        resolve(message.result);
      }
      return;
    }

    // Handle notifications
    if (message.method === 'textDocument/publishDiagnostics') {
      this.handleDiagnostics(message.params);
    }
  }

  /**
   * Handle diagnostics notification
   */
  private handleDiagnostics(params: { uri: string; diagnostics: LspDiagnostic[] }): void {
    const uri = params.uri;
    this.diagnosticsCache.set(uri, params.diagnostics);

    // Notify all subscribers
    this.diagnosticsCallbacks.forEach(callback => {
      callback(this.diagnosticsCache);
    });
  }

  /**
   * Subscribe to diagnostics updates
   */
  onDiagnostics(callback: (diagnostics: Map<string, LspDiagnostic[]>) => void): () => void {
    this.diagnosticsCallbacks.add(callback);
    
    // Immediately call with current cache
    callback(this.diagnosticsCache);

    // Return unsubscribe function
    return () => {
      this.diagnosticsCallbacks.delete(callback);
    };
  }

  /**
   * Get diagnostics for a specific document
   */
  getDiagnostics(uri: string): LspDiagnostic[] {
    return this.diagnosticsCache.get(uri) || [];
  }

  /**
   * Get all diagnostics
   */
  getAllDiagnostics(): Map<string, LspDiagnostic[]> {
    return new Map(this.diagnosticsCache);
  }

  /**
   * Send a request to the LSP server
   */
  private async sendRequest<T = unknown>(method: string, params?: unknown): Promise<T> {
    if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
      throw new Error('LSP client not connected');
    }

    const id = this.requestId++;
    const request = {
      jsonrpc: '2.0',
      id,
      method,
      params,
    };

    return new Promise((resolve, reject) => {
      this.pendingRequests.set(id, { resolve, reject });

      this.ws!.send(JSON.stringify(request));

      // Timeout after 30 seconds
      setTimeout(() => {
        if (this.pendingRequests.has(id)) {
          this.pendingRequests.delete(id);
          reject(new Error('LSP request timeout'));
        }
      }, 30000);
    });
  }

  /**
   * Open a text document
   */
  async openDocument(uri: string, languageId: string, text: string): Promise<void> {
    await this.sendRequest('textDocument/didOpen', {
      textDocument: {
        uri,
        languageId,
        version: 0,
        text,
      },
    });
  }

  /**
   * Update text document content
   */
  async changeDocument(uri: string, version: number, changes: Array<{
    range: {
      start: { line: number; character: number };
      end: { line: number; character: number };
    };
    text: string;
  }>): Promise<void> {
    await this.sendRequest('textDocument/didChange', {
      textDocument: {
        uri,
        version,
      },
      contentChanges: changes,
    });
  }

  /**
   * Get completion items at a position
   */
  async getCompletions(
    uri: string,
    line: number,
    character: number
  ): Promise<LspCompletionItem[]> {
    const result = await this.sendRequest('textDocument/completion', {
      textDocument: { uri },
      position: { line, character },
    });

    if (Array.isArray(result)) {
      return result;
    } else if (result?.items) {
      return result.items;
    }
    return [];
  }

  /**
   * Get hover information at a position
   */
  async getHover(uri: string, line: number, character: number): Promise<LspHover | null> {
    return this.sendRequest('textDocument/hover', {
      textDocument: { uri },
      position: { line, character },
    });
  }

  /**
   * Get references to a symbol
   */
  async getReferences(
    uri: string,
    line: number,
    character: number,
    includeDeclaration = false
  ): Promise<Array<{
    uri: string;
    range: {
      start: { line: number; character: number };
      end: { line: number; character: number };
    };
  }>> {
    return this.sendRequest('textDocument/references', {
      textDocument: { uri },
      position: { line, character },
      context: { includeDeclaration },
    });
  }
}
