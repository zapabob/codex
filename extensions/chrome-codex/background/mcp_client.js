/**
 * MCP (Model Context Protocol) Client for Chrome Extension
 * Connects to MCP bridge server via streamable HTTP
 */

class MCPClient {
    constructor(url) {
        this.url = url;
        this.initialized = false;
        this.requestId = 0;
        this.pendingRequests = new Map();
    }

    /**
     * Connect to MCP bridge server
     */
    async connect() {
        if (this.initialized) {
            return;
        }

        // For streamable HTTP, we'll use fetch API with SSE-like streaming
        // MCP streamable HTTP uses POST requests with JSON-RPC messages
        this.initialized = true;
    }

    /**
     * Initialize MCP connection
     */
    async initialize() {
        const params = {
            protocol_version: "2025-06-18",
            capabilities: {},
            client_info: {
                name: "codex-chrome-extension",
                version: "0.1.0"
            }
        };

        const result = await this.callRequest("initialize", params);
        this.initialized = true;
        return result;
    }

    /**
     * List available tools
     */
    async listTools() {
        return await this.callRequest("tools/list", {});
    }

    /**
     * Call a tool
     */
    async callTool(name, arguments_) {
        const params = {
            name,
            arguments: arguments_
        };
        return await this.callRequest("tools/call", params);
    }

    /**
     * Send a JSON-RPC request
     */
    async callRequest(method, params) {
        const id = this.requestId++;
        const request = {
            jsonrpc: "2.0",
            id,
            method,
            params
        };

        return new Promise((resolve, reject) => {
            this.pendingRequests.set(id, { resolve, reject });

            fetch(this.url, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify(request)
            })
                .then(response => {
                    if (!response.ok) {
                        throw new Error(`HTTP error! status: ${response.status}`);
                    }
                    return response.json();
                })
                .then(data => {
                    const pending = this.pendingRequests.get(id);
                    if (pending) {
                        this.pendingRequests.delete(id);
                        if (data.error) {
                            pending.reject(new Error(data.error.message || "MCP error"));
                        } else {
                            pending.resolve(data.result);
                        }
                    }
                })
                .catch(error => {
                    const pending = this.pendingRequests.get(id);
                    if (pending) {
                        this.pendingRequests.delete(id);
                        pending.reject(error);
                    }
                });
        });
    }

    /**
     * Disconnect from MCP server
     */
    disconnect() {
        this.initialized = false;
        this.pendingRequests.clear();
    }
}

// Export for use in background.js
if (typeof module !== "undefined" && module.exports) {
    module.exports = MCPClient;
}
