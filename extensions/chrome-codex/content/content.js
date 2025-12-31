const MAX_DEFAULT_CHARS = 5000;
const MAX_HARD_LIMIT = 20000;
const MAX_ACTIONS = 20;
const LOG_SOURCE = "codex-console";
const LOGIN_KEYWORDS = ["login", "sign in", "\u30ed\u30b0\u30a4\u30f3"];

function clamp(value, min, max) {
  return Math.min(Math.max(value, min), max);
}

function readDom(selector, maxChars) {
  const element = selector ? document.querySelector(selector) : document.body;
  const rawText = element ? (element.innerText || element.textContent || "") : "";
  const cleanText = rawText.trim();
  const limit = clamp(Number(maxChars) || MAX_DEFAULT_CHARS, 1, MAX_HARD_LIMIT);
  const truncated = cleanText.slice(0, limit);

  return {
    title: document.title,
    url: window.location.href,
    selector: selector || null,
    text: truncated,
    length: cleanText.length,
    truncated: cleanText.length > limit
  };
}

function normalizeText(value) {
  return (value || "").toString().trim().toLowerCase();
}

function findByText(text) {
  const query = normalizeText(text);
  if (!query) {
    return null;
  }

  const candidates = Array.from(
    document.querySelectorAll(
      "button, a, input, textarea, [role=button], [role=link], [data-testid]"
    )
  );

  return candidates.find((el) => {
    const label = normalizeText(el.getAttribute("aria-label"));
    const textContent = normalizeText(el.textContent || el.value);
    return label.includes(query) || textContent.includes(query);
  }) || null;
}

function findByRole(role, name) {
  const roleValue = normalizeText(role);
  if (!roleValue) {
    return null;
  }

  const selector = `[role="${roleValue}"]`;
  const candidates = Array.from(document.querySelectorAll(selector));
  if (!name) {
    return candidates[0] || null;
  }

  const query = normalizeText(name);
  return (
    candidates.find((el) => {
      const label = normalizeText(el.getAttribute("aria-label"));
      const textContent = normalizeText(el.textContent);
      return label.includes(query) || textContent.includes(query);
    }) || null
  );
}

function findByAria(name) {
  const query = normalizeText(name);
  if (!query) {
    return null;
  }

  return (
    Array.from(document.querySelectorAll("[aria-label]"))
      .find((el) => normalizeText(el.getAttribute("aria-label")).includes(query)) || null
  );
}

function resolveTarget(target) {
  if (!target) {
    return null;
  }

  if (target.strategy === "css" && target.query) {
    return document.querySelector(target.query);
  }

  if (target.strategy === "role") {
    return findByRole(target.query, target.name);
  }

  if (target.strategy === "aria") {
    return findByAria(target.query || target.name);
  }

  if (target.strategy === "text") {
    return findByText(target.query || target.name);
  }

  return null;
}

function setInputValue(element, value) {
  element.focus();
  element.value = value;
  element.dispatchEvent(new Event("input", { bubbles: true }));
  element.dispatchEvent(new Event("change", { bubbles: true }));
}

function pressEnter(element) {
  element.dispatchEvent(new KeyboardEvent("keydown", { key: "Enter", bubbles: true }));
  element.dispatchEvent(new KeyboardEvent("keyup", { key: "Enter", bubbles: true }));
}

function waitFor(condition, timeoutMs = 8000, intervalMs = 200) {
  return new Promise((resolve, reject) => {
    const start = Date.now();
    const timer = setInterval(() => {
      const result = condition();
      if (result) {
        clearInterval(timer);
        resolve(result);
        return;
      }
      if (Date.now() - start > timeoutMs) {
        clearInterval(timer);
        reject(new Error("Timed out"));
      }
    }, intervalMs);
  });
}

async function clickTarget(target) {
  const element = resolveTarget(target);
  if (!element) {
    throw new Error("Target not found");
  }
  element.click();
}

async function typeIntoTarget(target, value, pressEnterKey) {
  let element = resolveTarget(target);
  if (!element) {
    element = document.activeElement;
  }
  if (!element) {
    throw new Error("No element to type into");
  }

  if (element.tagName === "INPUT" || element.tagName === "TEXTAREA") {
    setInputValue(element, value);
  } else if (element.isContentEditable) {
    element.focus();
    element.textContent = value;
  } else {
    setInputValue(element, value);
  }

  if (pressEnterKey) {
    pressEnter(element);
  }
}

async function scrollByAmount(action) {
  const x = Number(action.x || 0);
  const y = Number(action.y || 0);
  window.scrollBy(x, y);
}

async function waitForTarget(target, timeoutMs) {
  return await waitFor(() => resolveTarget(target), timeoutMs);
}

async function runPostToX(text) {
  const host = window.location.hostname || "";
  if (!host.includes("x.com") && !host.includes("twitter.com")) {
    throw new Error("Not on x.com or twitter.com");
  }
  if (!text) {
    throw new Error("No text to post");
  }

  const composeButton =
    document.querySelector('[data-testid="SideNav_NewTweet_Button"]') ||
    document.querySelector('a[href="/compose/tweet"]');

  if (composeButton) {
    composeButton.click();
  }

  const editor = await waitFor(() =>
    document.querySelector('[data-testid="tweetTextarea_0"]') ||
      document.querySelector('div[role="textbox"]')
  );
  editor.focus();
  editor.textContent = text;
  editor.dispatchEvent(new Event("input", { bubbles: true }));

  const postButton = await waitFor(() =>
    document.querySelector('[data-testid="tweetButtonInline"]') ||
      document.querySelector('[data-testid="tweetButton"]')
  );
  postButton.click();
}

async function runLoginClick() {
  const textTarget = LOGIN_KEYWORDS.find((keyword) => findByText(keyword));
  if (textTarget) {
    await clickTarget({ strategy: "text", query: textTarget });
    return;
  }

  const loginButton =
    document.querySelector('[data-testid="loginButton"]') ||
    findByText("login");
  if (!loginButton) {
    throw new Error("Login button not found");
  }
  loginButton.click();
}

async function runAmazonAddToCart(productName) {
  const host = window.location.hostname || "";
  if (!host.includes("amazon.")) {
    throw new Error("Not on Amazon domain");
  }

  if (productName) {
    const searchInput =
      document.querySelector("#twotabsearchtextbox") ||
      document.querySelector("input[name='field-keywords']");
    if (!searchInput) {
      throw new Error("Amazon search box not found");
    }
    setInputValue(searchInput, productName);
    pressEnter(searchInput);
  }

  const firstResult = await waitFor(() =>
    document.querySelector("div[data-component-type='s-search-result'] h2 a")
  );
  firstResult.click();

  const addToCartButton = await waitFor(() =>
    document.querySelector("#add-to-cart-button") ||
      document.querySelector("input[name='submit.add-to-cart']")
  );
  addToCartButton.click();
}

async function runAction(action) {
  switch (action.type) {
    case "click":
      await clickTarget(action.target);
      return { ok: true };
    case "type":
      await typeIntoTarget(action.target, action.text || "", action.enter || false);
      return { ok: true };
    case "scroll":
      await scrollByAmount(action);
      return { ok: true };
    case "wait_for":
      await waitForTarget(action.target, action.timeoutMs || 8000);
      return { ok: true };
    case "read_dom":
      return { ok: true, result: readDom(action.selector, action.maxChars) };
    case "eval": {
      const fn = new Function(action.code || "");
      const result = fn();
      return { ok: true, result };
    }
    case "post_social":
      if ((action.platform || "").toLowerCase() === "x") {
        await runPostToX(action.text || "");
        return { ok: true };
      }
      throw new Error("Unsupported social platform");
    case "login_start":
      await runLoginClick();
      return { ok: true };
    case "shop_add_to_cart":
      await runAmazonAddToCart(action.productName || "");
      return { ok: true };
    default:
      throw new Error(`Unsupported action: ${action.type}`);
  }
}

async function executeActions(actions) {
  if (!Array.isArray(actions)) {
    throw new Error("Actions must be an array");
  }
  if (actions.length > MAX_ACTIONS) {
    throw new Error("Too many actions");
  }

  const results = [];
  for (const action of actions) {
    const id = action.id || null;
    try {
      const result = await runAction(action);
      results.push({ id, ok: true, result: result.result });
    } catch (err) {
      results.push({ id, ok: false, error: err.message || String(err) });
      break;
    }
  }

  return { ok: true, results };
}

function installConsoleHook() {
  if (window.__codexConsoleHookInstalled) {
    return;
  }
  window.__codexConsoleHookInstalled = true;

  const script = document.createElement("script");
  script.textContent = `(() => {
    if (window.__codexConsoleInjected) return;
    window.__codexConsoleInjected = true;
    const methods = ["log", "info", "warn", "error"];
    methods.forEach((method) => {
      const original = console[method];
      console[method] = (...args) => {
        try {
          window.postMessage({
            source: "${LOG_SOURCE}",
            level: method,
            message: args.map((arg) => {
              if (typeof arg === "string") return arg;
              try { return JSON.stringify(arg); } catch { return String(arg); }
            }).join(" ")
          }, "*");
        } catch (err) {
          // Ignore console hook errors.
        }
        original.apply(console, args);
      };
    });
  })();`;
  document.documentElement.appendChild(script);
  script.remove();
}

window.addEventListener("message", (event) => {
  if (event.source !== window) {
    return;
  }
  const data = event.data;
  if (!data || data.source !== LOG_SOURCE) {
    return;
  }

  chrome.runtime.sendMessage({
    type: "console.event",
    level: data.level,
    message: data.message
  });
});

chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
  if (!request || !request.type) {
    return;
  }

  if (request.type === "dom.read") {
    try {
      const result = readDom(request.selector, request.maxChars);
      sendResponse({ ok: true, result });
    } catch (err) {
      sendResponse({ ok: false, error: err.message || String(err) });
    }
    return;
  }

  if (request.type === "action.execute") {
    executeActions(request.actions)
      .then((result) => {
        sendResponse(result);
      })
      .catch((error) => {
        sendResponse({ ok: false, error: error.message || String(error) });
      });
    return true;
  }

  if (request.type === "console.subscribe") {
    installConsoleHook();
    sendResponse({ ok: true });
    return;
  }
});
