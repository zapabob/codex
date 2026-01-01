const HOST_NAME = "com.codex.chrome";
const CONTENT_SCRIPT_PATH = "content/content.js";
const MAX_LOGS = 200;
const MCP_BRIDGE_URL = "http://127.0.0.1:8788/mcp";
const ACTION_DOMAIN_ALLOWLIST = {
  post_social: ["x.com", "twitter.com"],
  post_article: ["note.com", "qiita.com", "zenn.dev"],
  shop_add_to_cart: ["amazon.com", "amazon.co.jp"],
  shop_prepare_purchase: ["mercari.com", "mercari.jp", "auctions.yahoo.co.jp"]
};
const OPTIONAL_HOST_PERMISSIONS = [
  "*://*.x.com/*",
  "*://*.twitter.com/*",
  "*://*.note.com/*",
  "*://*.qiita.com/*",
  "*://*.zenn.dev/*",
  "*://*.amazon.com/*",
  "*://*.amazon.co.jp/*",
  "*://*.mercari.com/*",
  "*://*.mercari.jp/*",
  "*://auctions.yahoo.co.jp/*"
];

let nativePort = null;
const nativeQueue = [];
const consoleLogs = [];
const networkLogs = [];
const networkSubscriptions = new Map();
let networkListenersReady = false;
let mcpClient = null;

// Import MCP client (inline for Manifest V3 service worker)
// In a full implementation, this would be imported as a module
const MCPClient = class {
    constructor(url) {
        this.url = url;
        this.initialized = false;
        this.requestId = 0;
        this.pendingRequests = new Map();
    }

    async connect() {
        if (this.initialized) {
            return;
        }
        this.initialized = true;
    }

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

    async listTools() {
        return await this.callRequest("tools/list", {});
    }

    async callTool(name, arguments_) {
        const params = {
            name,
            arguments: arguments_
        };
        return await this.callRequest("tools/call", params);
    }

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

    disconnect() {
        this.initialized = false;
        this.pendingRequests.clear();
    }
};

function pushLog(buffer, entry) {
  buffer.push(entry);
  if (buffer.length > MAX_LOGS) {
    buffer.shift();
  }
}

function connectNative() {
  if (nativePort) {
    return nativePort;
  }

  try {
    nativePort = chrome.runtime.connectNative(HOST_NAME);

    nativePort.onMessage.addListener((message) => {
      const pending = nativeQueue.shift();
      if (pending) {
        pending.resolve(message);
      } else {
        // Handle unsolicited messages from native host (e.g., from CLI)
        // These messages might be requests that need to be processed
        if (message && message.type) {
          handleUnsolicitedNativeMessage(message);
        }
      }
    });

    nativePort.onDisconnect.addListener(() => {
      const error = chrome.runtime.lastError?.message || "Native host disconnected";
      while (nativeQueue.length > 0) {
        const pending = nativeQueue.shift();
        pending.reject(new Error(error));
      }
      nativePort = null;
      // Attempt to reconnect after a short delay
      setTimeout(() => {
        if (!nativePort) {
          connectNative();
        }
      }, 1000);
    });
  } catch (err) {
    console.error("Failed to connect to native host:", err);
    nativePort = null;
    throw err;
  }

  return nativePort;
}

// Handle unsolicited messages from native host (e.g., CLI-initiated requests)
function handleUnsolicitedNativeMessage(message) {
  // This function handles messages that come from the native host
  // without a corresponding pending request in the queue.
  // This can happen when CLI initiates a request.
  console.log("Received unsolicited message from native host:", message);
  
  // For now, we just log these messages.
  // In a full implementation, we would process CLI-initiated requests here.
  // However, since Native Messaging Host is typically called by the extension,
  // CLI-initiated requests would need a different mechanism.
}

function sendNativeMessage(message) {
  return new Promise((resolve, reject) => {
    let port;
    try {
      port = connectNative();
    } catch (err) {
      reject(err);
      return;
    }

    nativeQueue.push({ resolve, reject });

    try {
      port.postMessage(message);
    } catch (err) {
      nativeQueue.pop();
      reject(err);
    }
  });
}

function queryActiveTab() {
  return new Promise((resolve, reject) => {
    chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
      if (chrome.runtime.lastError) {
        reject(new Error(chrome.runtime.lastError.message));
        return;
      }

      if (!tabs || tabs.length === 0) {
        reject(new Error("No active tab"));
        return;
      }

      resolve(tabs[0]);
    });
  });
}

function injectContentScript(tabId) {
  return new Promise((resolve, reject) => {
    chrome.scripting.executeScript(
      {
        target: { tabId },
        files: [CONTENT_SCRIPT_PATH]
      },
      () => {
        if (chrome.runtime.lastError) {
          reject(new Error(chrome.runtime.lastError.message));
          return;
        }
        resolve();
      }
    );
  });
}

function sendMessageToTab(tabId, message) {
  return new Promise((resolve, reject) => {
    chrome.tabs.sendMessage(tabId, message, (response) => {
      if (chrome.runtime.lastError) {
        reject(new Error(chrome.runtime.lastError.message));
        return;
      }
      resolve(response);
    });
  });
}

async function handleDomRead(request) {
  const tab = await queryActiveTab();
  await injectContentScript(tab.id);
  return await sendMessageToTab(tab.id, {
    type: "dom.read",
    selector: request.selector || null,
    maxChars: request.maxChars || 5000
  });
}

async function handleActionExecute(request) {
  const highRisk = new Set([
    "post_social",
    "post_article",
    "login_start",
    "shop_add_to_cart",
    "shop_prepare_purchase",
    "eval"
  ]);
  const actions = Array.isArray(request.actions) ? request.actions : [];
  const needsConfirmation = actions.some((action) => highRisk.has(action.type));
  if (needsConfirmation && !request.confirmed) {
    throw new Error("Confirmation required for high-risk actions");
  }
  const tab = await queryActiveTab();
  ensureAllowedDomains(actions, tab.url || "");
  await injectContentScript(tab.id);
  return await sendMessageToTab(tab.id, {
    type: "action.execute",
    actions
  });
}

async function handleConsoleSubscribe() {
  const tab = await queryActiveTab();
  await injectContentScript(tab.id);
  return await sendMessageToTab(tab.id, { type: "console.subscribe" });
}

function ensureNetworkListeners() {
  if (networkListenersReady) {
    return;
  }

  const filter = { urls: ["<all_urls>"] };

  chrome.webRequest.onCompleted.addListener((details) => {
    handleNetworkEvent(details, "completed");
  }, filter, ["responseHeaders"]);

  chrome.webRequest.onErrorOccurred.addListener((details) => {
    handleNetworkEvent(details, "error");
  }, filter);

  networkListenersReady = true;
}

function handleNetworkEvent(details, status) {
  const subscription = networkSubscriptions.get(details.tabId);
  if (!subscription) {
    return;
  }

  const url = details.url || "";
  if (subscription.filter && !url.includes(subscription.filter)) {
    return;
  }

  pushLog(networkLogs, {
    tabId: details.tabId,
    url,
    method: details.method,
    status,
    statusCode: details.statusCode || null,
    timeStamp: details.timeStamp
  });
}

function requestNetworkPermissions() {
  return new Promise((resolve) => {
    chrome.permissions.request(
      {
        permissions: ["webRequest"],
        origins: OPTIONAL_HOST_PERMISSIONS
      },
      (granted) => {
        resolve(Boolean(granted));
      }
    );
  });
}

function ensureAllowedDomains(actions, url) {
  if (!url) {
    return;
  }
  let host = "";
  try {
    host = new URL(url).hostname;
  } catch (err) {
    throw new Error("Invalid tab URL");
  }

  for (const action of actions) {
    const allowed = ACTION_DOMAIN_ALLOWLIST[action.type];
    if (!allowed || allowed.length === 0) {
      continue;
    }
    if (!allowed.some((domain) => host === domain || host.endsWith(`.${domain}`))) {
      throw new Error(`Action ${action.type} not allowed on ${host}`);
    }
  }
}

chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
  if (!request || !request.type) {
    return;
  }

  if (request.type === "console.event") {
    const entry = {
      tabId: sender.tab?.id || null,
      level: request.level || "log",
      message: request.message || "",
      timeStamp: Date.now()
    };
    pushLog(consoleLogs, entry);
    return;
  }

  if (request.type === "native.send") {
    sendNativeMessage(request.message)
      .then((response) => {
        sendResponse({ ok: true, response });
      })
      .catch((error) => {
        const errorMessage = error.message || String(error);
        // Provide more helpful error messages
        let userMessage = errorMessage;
        if (errorMessage.includes("Native host has exited") || errorMessage.includes("disconnected")) {
          userMessage = "Native host disconnected. Please ensure codex-chrome-host is installed and running.";
        } else if (errorMessage.includes("Could not connect")) {
          userMessage = "Could not connect to native host. Please check the installation.";
        }
        sendResponse({ ok: false, error: userMessage, originalError: errorMessage });
      });
    return true;
  }

  if (request.type === "dom.read.request") {
    (async () => {
      try {
        // Try MCP bridge first, fall back to native messaging
        if (request.useMCP !== false) {
          try {
            if (!mcpClient) {
              mcpClient = new MCPClient(MCP_BRIDGE_URL);
              await mcpClient.connect();
              await mcpClient.initialize();
            }

            const mcpResult = await mcpClient.callTool("dom_read", {
              selector: request.selector || null,
              max_chars: request.maxChars || 5000
            });

            sendResponse({ ok: true, result: mcpResult, source: "mcp" });
            return;
          } catch (mcpError) {
            console.warn("MCP bridge failed, falling back to native messaging:", mcpError);
          }
        }

        // Fallback to native messaging host
        try {
          connectNative();
        } catch (err) {
          console.warn("Native host not connected, processing request locally:", err);
        }

        const result = await handleDomRead({
          selector: request.selector || null,
          maxChars: request.maxChars || 5000
        });
        
        // Send result to native host if needed (for CLI-initiated requests)
        if (request.sendToNative || request.fromCLI) {
          try {
            const nativeMessage = {
              version: "1.0",
              id: request.id || crypto.randomUUID(),
              type: "dom.read.response",
              origin: request.origin || {},
              payload: {
                success: true,
                data: result
              }
            };
            await sendNativeMessage(nativeMessage);
          } catch (err) {
            console.warn("Failed to send result to native host:", err);
            // Continue even if native host communication fails
          }
        }
        sendResponse({ ok: true, result, source: "native" });
      } catch (error) {
        sendResponse({ ok: false, error: error.message || String(error) });
      }
    })();
    return true;
  }

  if (request.type === "console.get_logs.request") {
    (async () => {
      try {
        // Try MCP bridge first, fall back to native messaging
        if (request.useMCP !== false) {
          try {
            if (!mcpClient) {
              mcpClient = new MCPClient(MCP_BRIDGE_URL);
              await mcpClient.connect();
              await mcpClient.initialize();
            }

            const mcpResult = await mcpClient.callTool("console_get_logs", {
              level: request.level || null,
              filter: request.filter || null,
              limit: request.limit || 50
            });

            sendResponse({ ok: true, result: mcpResult, source: "mcp" });
            return;
          } catch (mcpError) {
            console.warn("MCP bridge failed, falling back to native messaging:", mcpError);
          }
        }

        // Fallback to native messaging host
        try {
          connectNative();
        } catch (err) {
          console.warn("Native host not connected, processing request locally:", err);
        }

        let logs = consoleLogs.slice();
        
        // Filter by level if specified
        if (request.level) {
          logs = logs.filter(log => log.level === request.level);
        }
        
        // Filter by message content if specified
        if (request.filter) {
          logs = logs.filter(log => log.message.includes(request.filter));
        }
        
        // Limit results
        const limit = request.limit || 50;
        logs = logs.slice(-limit);
        
        const result = { logs };
        
        // Send result to native host if needed (for CLI-initiated requests)
        if (request.sendToNative || request.fromCLI) {
          try {
            const nativeMessage = {
              version: "1.0",
              id: request.id || crypto.randomUUID(),
              type: "console.get_logs.response",
              origin: request.origin || {},
              payload: {
                success: true,
                data: result
              }
            };
            await sendNativeMessage(nativeMessage);
          } catch (err) {
            console.warn("Failed to send result to native host:", err);
            // Continue even if native host communication fails
          }
        }
        sendResponse({ ok: true, result, source: "native" });
      } catch (error) {
        sendResponse({ ok: false, error: error.message || String(error) });
      }
    })();
    return true;
  }

  if (request.type === "network.get_logs.request") {
    (async () => {
      try {
        // Try MCP bridge first, fall back to native messaging
        if (request.useMCP !== false) {
          try {
            if (!mcpClient) {
              mcpClient = new MCPClient(MCP_BRIDGE_URL);
              await mcpClient.connect();
              await mcpClient.initialize();
            }

            const mcpResult = await mcpClient.callTool("network_get_logs", {
              filter: request.filter || null,
              limit: request.limit || 50
            });

            sendResponse({ ok: true, result: mcpResult, source: "mcp" });
            return;
          } catch (mcpError) {
            console.warn("MCP bridge failed, falling back to native messaging:", mcpError);
          }
        }

        // Fallback to native messaging host
        try {
          connectNative();
        } catch (err) {
          console.warn("Native host not connected, processing request locally:", err);
        }

        let logs = networkLogs.slice();
        
        // Filter by URL pattern if specified
        if (request.filter) {
          logs = logs.filter(log => log.url.includes(request.filter));
        }
        
        // Limit results
        const limit = request.limit || 50;
        logs = logs.slice(-limit);
        
        const result = { logs };
        
        // Send result to native host if needed (for CLI-initiated requests)
        if (request.sendToNative || request.fromCLI) {
          try {
            const nativeMessage = {
              version: "1.0",
              id: request.id || crypto.randomUUID(),
              type: "network.get_logs.response",
              origin: request.origin || {},
              payload: {
                success: true,
                data: result
              }
            };
            await sendNativeMessage(nativeMessage);
          } catch (err) {
            console.warn("Failed to send result to native host:", err);
            // Continue even if native host communication fails
          }
        }
        sendResponse({ ok: true, result, source: "native" });
      } catch (error) {
        sendResponse({ ok: false, error: error.message || String(error) });
      }
    })();
    return true;
  }

  if (request.type === "dom.read") {
    handleDomRead(request)
      .then((result) => {
        sendResponse({ ok: true, result });
      })
      .catch((error) => {
        sendResponse({ ok: false, error: error.message || String(error) });
      });
    return true;
  }

  if (request.type === "action.execute") {
    handleActionExecute(request)
      .then((result) => {
        sendResponse({ ok: true, result });
      })
      .catch((error) => {
        sendResponse({ ok: false, error: error.message || String(error) });
      });
    return true;
  }

  if (request.type === "console.subscribe") {
    handleConsoleSubscribe()
      .then((result) => {
        sendResponse({ ok: true, result });
      })
      .catch((error) => {
        sendResponse({ ok: false, error: error.message || String(error) });
      });
    return true;
  }

  if (request.type === "console.get_logs") {
    sendResponse({ ok: true, logs: consoleLogs.slice() });
    return;
  }

  if (request.type === "network.subscribe") {
    (async () => {
      const granted = await requestNetworkPermissions();
      if (!granted) {
        sendResponse({ ok: false, error: "Permission denied" });
        return;
      }

      ensureNetworkListeners();
      const tab = await queryActiveTab();
      networkSubscriptions.set(tab.id, { filter: request.filter || null });
      sendResponse({ ok: true, message: "Subscribed" });
    })().catch((error) => {
      sendResponse({ ok: false, error: error.message || String(error) });
    });
    return true;
  }

  if (request.type === "network.unsubscribe") {
    (async () => {
      const tab = await queryActiveTab();
      networkSubscriptions.delete(tab.id);
      sendResponse({ ok: true, message: "Unsubscribed" });
    })().catch((error) => {
      sendResponse({ ok: false, error: error.message || String(error) });
    });
    return true;
  }

  if (request.type === "network.get_logs") {
    sendResponse({ ok: true, logs: networkLogs.slice() });
    return;
  }

  if (request.type === "tab.active") {
    queryActiveTab()
      .then((tab) => {
        sendResponse({ ok: true, tab });
      })
      .catch((error) => {
        sendResponse({ ok: false, error: error.message || String(error) });
      });
    return true;
  }
});
