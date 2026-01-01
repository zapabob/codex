const HOST_NAME = "com.codex.chrome";
const CONTENT_SCRIPT_PATH = "content/content.js";
const MAX_LOGS = 200;
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

  nativePort = chrome.runtime.connectNative(HOST_NAME);

  nativePort.onMessage.addListener((message) => {
    const pending = nativeQueue.shift();
    if (pending) {
      pending.resolve(message);
    }
  });

  nativePort.onDisconnect.addListener(() => {
    const error = chrome.runtime.lastError?.message || "Native host disconnected";
    while (nativeQueue.length > 0) {
      const pending = nativeQueue.shift();
      pending.reject(new Error(error));
    }
    nativePort = null;
  });

  return nativePort;
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
