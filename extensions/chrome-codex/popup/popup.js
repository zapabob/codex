const statusEl = document.getElementById("status");
const outputEl = document.getElementById("output");
const queryEl = document.getElementById("query");
const depthEl = document.getElementById("depth");
const selectorEl = document.getElementById("selector");
const nlInputEl = document.getElementById("nlInput");
const nlNoticeEl = document.getElementById("nlNotice");
const nlExecuteBtn = document.getElementById("nlExecute");
const codePromptEl = document.getElementById("codegenPrompt");
const codeNoticeEl = document.getElementById("codeNotice");
const codeRunBtn = document.getElementById("codeRun");

let pendingIntent = null;
let pendingCode = null;

function setStatus(text, isError = false) {
  statusEl.textContent = text;
  statusEl.classList.toggle("error", isError);
}

function setOutput(value) {
  if (typeof value === "string") {
    outputEl.textContent = value;
  } else {
    outputEl.textContent = JSON.stringify(value, null, 2);
  }
}

function setNotice(el, text, isError = false) {
  el.textContent = text || "";
  el.classList.toggle("error", isError);
}

function sendToBackground(payload) {
  return new Promise((resolve) => {
    chrome.runtime.sendMessage(payload, (response) => {
      resolve(response);
    });
  });
}

async function sendNative(message) {
  return await sendToBackground({ type: "native.send", message });
}

function makeNativeMessage(type, payload, origin) {
  return {
    version: "1.0",
    id: crypto.randomUUID(),
    type,
    origin,
    payload
  };
}

function getActiveTab() {
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

async function onPing() {
  setStatus("Connecting...");
  const response = await sendNative(makeNativeMessage("ping", {}, {}));
  if (!response || !response.ok) {
    setStatus("Disconnected", true);
    setOutput(response?.error || "Failed to connect to native host");
    return;
  }

  setStatus("Connected");
  setOutput(response.response || { message: "pong" });
}

async function onResearch() {
  const query = queryEl.value.trim();
  if (!query) {
    setOutput("Enter a research query.");
    return;
  }

  setStatus("Running...");
  const depth = Number(depthEl.value) || 2;
  const response = await sendNative(
    makeNativeMessage(
      "deep_research.request",
      {
        query,
        options: { depth }
      },
      {}
    )
  );

  if (!response || !response.ok || !response.response?.success) {
    setStatus("Disconnected", true);
    setOutput(response?.response?.error || response?.error || "Research failed");
    return;
  }

  setStatus("Connected");
  setOutput(response.response.data || {});
}

async function onReadDom() {
  setStatus("Reading...");
  const selector = selectorEl.value.trim();
  const response = await sendToBackground({
    type: "dom.read",
    selector
  });

  if (!response || !response.ok) {
    setStatus("Error", true);
    setOutput(response?.error || "DOM read failed");
    return;
  }

  setStatus("Connected");
  setOutput(response.result || {});
}

async function onParseNl() {
  const utterance = nlInputEl.value.trim();
  if (!utterance) {
    setNotice(nlNoticeEl, "Enter an instruction.", true);
    return;
  }

  setNotice(nlNoticeEl, "");
  nlExecuteBtn.disabled = true;
  pendingIntent = null;

  setStatus("Parsing...");
  let tab;
  try {
    tab = await getActiveTab();
  } catch (err) {
    setStatus("Error", true);
    setNotice(nlNoticeEl, err.message || String(err), true);
    return;
  }

  const message = makeNativeMessage(
    "nl_command.request",
    {
      utterance,
      constraints: {
        allowed_intents: [
          "click",
          "type",
          "scroll",
          "wait_for",
          "read_dom",
          "post_social",
          "login_start",
          "shop_add_to_cart",
          "post_article",
          "shop_prepare_purchase"
        ],
        require_confirmation_for: [
          "post_social",
          "login_start",
          "shop_add_to_cart",
          "post_article",
          "shop_prepare_purchase"
        ]
      }
    },
    {
      tab_id: tab.id,
      url: tab.url,
      frame_id: 0
    }
  );

  const response = await sendNative(message);
  if (!response || !response.ok || !response.response?.success) {
    setStatus("Error", true);
    setNotice(
      nlNoticeEl,
      response?.response?.error || response?.error || "Parse failed",
      true
    );
    return;
  }

  const data = response.response.data || {};
  if (data.success === false) {
    setStatus("Error", true);
    setNotice(nlNoticeEl, data.error || "Parse failed", true);
    setOutput(data);
    return;
  }
  const intent = data.intent || data.data?.intent;
  const warnings = data.warnings || data.data?.warnings || [];
  if (!intent) {
    setStatus("Error", true);
    setNotice(nlNoticeEl, "No intent returned.", true);
    setOutput(data);
    return;
  }
  pendingIntent = intent;

  const requiresConfirmation = Boolean(intent.requires_confirmation);
  nlExecuteBtn.textContent = requiresConfirmation ? "Confirm & Execute" : "Execute";
  nlExecuteBtn.disabled = false;
  setStatus("Parsed");
  setOutput(intent);

  const baseNotice = requiresConfirmation
    ? "High-risk action. Confirmation required."
    : "Ready to execute.";
  const combinedNotice = warnings.length > 0 ? `${baseNotice} ${warnings.join(" ")}` : baseNotice;
  setNotice(nlNoticeEl, combinedNotice);
}

function intentToActions(intent) {
  if (!intent || !intent.intent) {
    return [];
  }

  switch (intent.intent) {
    case "click":
      return [{ type: "click", target: intent.args?.target }];
    case "type":
      return [{
        type: "type",
        target: intent.args?.target,
        text: intent.args?.text || "",
        enter: Boolean(intent.args?.enter)
      }];
    case "scroll":
      return [{ type: "scroll", x: intent.args?.x || 0, y: intent.args?.y || 600 }];
    case "wait_for":
      return [{ type: "wait_for", target: intent.args?.target, timeoutMs: intent.args?.timeout_ms }];
    case "read_dom":
      return [{ type: "read_dom", selector: intent.args?.selector, maxChars: intent.args?.max_chars }];
    case "post_social":
      return [{ type: "post_social", platform: intent.args?.platform, text: intent.args?.text }];
    case "post_article":
      return [{
        type: "post_article",
        platform: intent.args?.platform,
        title: intent.args?.title,
        body: intent.args?.body,
        tags: intent.args?.tags || [],
        categories: intent.args?.categories || [],
        images: intent.args?.images || [],
        publish: Boolean(intent.args?.publish)
      }];
    case "login_start":
      return [{ type: "login_start" }];
    case "shop_add_to_cart":
      return [{ type: "shop_add_to_cart", productName: intent.args?.product_name }];
    case "shop_prepare_purchase":
      return [{
        type: "shop_prepare_purchase",
        platform: intent.args?.platform,
        productName: intent.args?.product_name
      }];
    default:
      return [];
  }
}

async function onExecuteNl() {
  if (!pendingIntent) {
    return;
  }

  nlExecuteBtn.disabled = true;
  setStatus("Executing...");

  const actions = intentToActions(pendingIntent);
  const response = await sendToBackground({
    type: "action.execute",
    actions,
    confirmed: Boolean(pendingIntent.requires_confirmation)
  });

  if (!response || !response.ok) {
    setStatus("Error", true);
    setOutput(response?.error || "Action failed");
    nlExecuteBtn.disabled = false;
    return;
  }

  setStatus("Done");
  setOutput(response.result || response);
  nlExecuteBtn.disabled = false;
}

async function onCodegen() {
  const task = codePromptEl.value.trim();
  if (!task) {
    setNotice(codeNoticeEl, "Enter a task.", true);
    return;
  }

  setNotice(codeNoticeEl, "");
  codeRunBtn.disabled = true;
  pendingCode = null;

  setStatus("Generating...");
  const response = await sendNative(
    makeNativeMessage("codegen.request", { task, language: "javascript" }, {})
  );

  if (!response || !response.ok || !response.response?.success) {
    setStatus("Error", true);
    setNotice(
      codeNoticeEl,
      response?.response?.error || response?.error || "Codegen failed",
      true
    );
    return;
  }

  const data = response.response.data || {};
  pendingCode = data.code || data.output || "";
  if (!pendingCode) {
    setNotice(codeNoticeEl, "No code returned.", true);
    return;
  }

  codeRunBtn.disabled = false;
  setStatus("Generated");
  setOutput({ code: pendingCode });
  setNotice(codeNoticeEl, "Run with caution. Confirmation required.");
}

async function onRunCode() {
  if (!pendingCode) {
    return;
  }

  setStatus("Executing...");
  const response = await sendToBackground({
    type: "action.execute",
    actions: [{ type: "eval", code: pendingCode }],
    confirmed: true
  });

  if (!response || !response.ok) {
    setStatus("Error", true);
    setOutput(response?.error || "Code execution failed");
    return;
  }

  setStatus("Done");
  setOutput(response.result || response);
}

async function onConsoleSubscribe() {
  setStatus("Subscribing...");
  const response = await sendToBackground({ type: "console.subscribe" });
  if (!response || !response.ok) {
    setStatus("Error", true);
    setOutput(response?.error || "Failed to subscribe");
    return;
  }
  setStatus("Console active");
  setOutput(response.result || response);
}

async function onConsoleFetch() {
  const response = await sendToBackground({ type: "console.get_logs" });
  if (!response || !response.ok) {
    setStatus("Error", true);
    setOutput(response?.error || "Failed to fetch logs");
    return;
  }
  setStatus("Logs");
  setOutput(response.logs || []);
}

async function onNetworkSubscribe() {
  setStatus("Subscribing...");
  const response = await sendToBackground({ type: "network.subscribe" });
  if (!response || !response.ok) {
    setStatus("Error", true);
    setOutput(response?.error || "Failed to subscribe");
    return;
  }
  setStatus("Network active");
  setOutput(response || {});
}

async function onNetworkFetch() {
  const response = await sendToBackground({ type: "network.get_logs" });
  if (!response || !response.ok) {
    setStatus("Error", true);
    setOutput(response?.error || "Failed to fetch network logs");
    return;
  }
  setStatus("Logs");
  setOutput(response.logs || []);
}

document.getElementById("ping").addEventListener("click", onPing);
document.getElementById("research").addEventListener("click", onResearch);
document.getElementById("readDom").addEventListener("click", onReadDom);
document.getElementById("nlParse").addEventListener("click", onParseNl);
document.getElementById("nlExecute").addEventListener("click", onExecuteNl);
document.getElementById("codegen").addEventListener("click", onCodegen);
document.getElementById("codeRun").addEventListener("click", onRunCode);
document.getElementById("consoleSubscribe").addEventListener("click", onConsoleSubscribe);
document.getElementById("consoleFetch").addEventListener("click", onConsoleFetch);
document.getElementById("networkSubscribe").addEventListener("click", onNetworkSubscribe);
document.getElementById("networkFetch").addEventListener("click", onNetworkFetch);
