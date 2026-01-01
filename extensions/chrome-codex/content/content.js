const MAX_DEFAULT_CHARS = 5000;
const MAX_HARD_LIMIT = 20000;
const MAX_ACTIONS = 20;
const LOG_SOURCE = "codex-console";
const LOGIN_KEYWORDS = ["login", "sign in", "\u30ed\u30b0\u30a4\u30f3"];
const ARTICLE_PLATFORM_DOMAINS = {
  note: ["note.com"],
  qiita: ["qiita.com"],
  zenn: ["zenn.dev"]
};
const SHOP_PLATFORM_DOMAINS = {
  amazon: ["amazon.com", "amazon.co.jp"],
  mercari: ["mercari.com", "mercari.jp"],
  yahoo_auctions: ["auctions.yahoo.co.jp"]
};

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

function hostMatchesDomain(host, domains) {
  if (!host || !Array.isArray(domains)) {
    return false;
  }
  return domains.some((domain) => host === domain || host.endsWith(`.${domain}`));
}

function ensureHostAllowed(platform, allowlist) {
  const host = window.location.hostname || "";
  const domains = allowlist[platform] || [];
  if (!hostMatchesDomain(host, domains)) {
    throw new Error(`Not on allowed ${platform} domain`);
  }
}

function collectElementHints(element) {
  const hints = [
    element.getAttribute("aria-label"),
    element.getAttribute("placeholder"),
    element.getAttribute("name"),
    element.getAttribute("id"),
    element.getAttribute("data-placeholder")
  ];
  return hints.map(normalizeText).filter(Boolean);
}

function findEditableByKeywords(keywords) {
  const normalized = keywords.map(normalizeText).filter(Boolean);
  if (normalized.length === 0) {
    return null;
  }
  const candidates = Array.from(
    document.querySelectorAll("input, textarea, [contenteditable=\"true\"]")
  );
  for (const element of candidates) {
    const hints = collectElementHints(element);
    if (normalized.some((keyword) => hints.some((hint) => hint.includes(keyword)))) {
      return element;
    }
  }
  return null;
}

function findLargestEditable() {
  const candidates = Array.from(
    document.querySelectorAll("textarea, [contenteditable=\"true\"]")
  );
  let best = null;
  let bestArea = 0;
  for (const element of candidates) {
    const rect = element.getBoundingClientRect();
    const area = rect.width * rect.height;
    if (area > bestArea) {
      bestArea = area;
      best = element;
    }
  }
  return best;
}

function findSelectByKeywords(keywords) {
  const normalized = keywords.map(normalizeText).filter(Boolean);
  if (normalized.length === 0) {
    return null;
  }
  const candidates = Array.from(document.querySelectorAll("select"));
  for (const element of candidates) {
    const hints = collectElementHints(element);
    if (normalized.some((keyword) => hints.some((hint) => hint.includes(keyword)))) {
      return element;
    }
  }
  return null;
}

function findFileInput() {
  return document.querySelector("input[type=\"file\"]");
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

function setContentEditable(element, value) {
  element.focus();
  element.textContent = value;
  element.dispatchEvent(new Event("input", { bubbles: true }));
  element.dispatchEvent(new Event("change", { bubbles: true }));
}

function fillTextField(element, value) {
  if (!element || value === null || value === undefined) {
    return;
  }
  if (element.tagName === "INPUT" || element.tagName === "TEXTAREA") {
    setInputValue(element, value);
    return;
  }
  if (element.isContentEditable) {
    setContentEditable(element, value);
    return;
  }
  setInputValue(element, value);
}

function fillTagsField(element, tags) {
  if (!element || !Array.isArray(tags) || tags.length === 0) {
    return;
  }
  if (element.tagName === "INPUT" || element.tagName === "TEXTAREA") {
    element.focus();
    element.value = "";
    element.dispatchEvent(new Event("input", { bubbles: true }));
    for (const tag of tags) {
      if (!tag) {
        continue;
      }
      setInputValue(element, tag);
      pressEnter(element);
    }
    return;
  }
  if (element.isContentEditable) {
    setContentEditable(element, tags.join(", "));
  }
}

function fillCategoriesField(categories) {
  if (!Array.isArray(categories) || categories.length === 0) {
    return false;
  }
  const select = findSelectByKeywords(["category", "\u30ab\u30c6\u30b4\u30ea"]);
  if (select) {
    const category = categories[0];
    const options = Array.from(select.options || []);
    const match = options.find((option) =>
      normalizeText(option.textContent).includes(normalizeText(category))
    );
    if (match) {
      select.value = match.value;
      select.dispatchEvent(new Event("change", { bubbles: true }));
      return true;
    }
  }

  const input = findEditableByKeywords(["category", "\u30ab\u30c6\u30b4\u30ea"]);
  if (input) {
    fillTextField(input, categories.join(", "));
    return true;
  }
  return false;
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

async function runPostArticle(action) {
  const platform = normalizeText(action.platform);
  if (!platform) {
    throw new Error("No article platform provided");
  }
  ensureHostAllowed(platform, ARTICLE_PLATFORM_DOMAINS);

  const title = action.title || "";
  const body = action.body || "";
  const tags = Array.isArray(action.tags) ? action.tags : [];
  const categories = Array.isArray(action.categories) ? action.categories : [];
  const images = Array.isArray(action.images) ? action.images : [];
  const wantsPublish = Boolean(action.publish);

  const titleInput = findEditableByKeywords(["title", "\u30bf\u30a4\u30c8\u30eb", "\u898b\u51fa\u3057"]);
  const bodyInput =
    findEditableByKeywords(["body", "\u672c\u6587", "\u8a18\u4e8b", "content"]) ||
    findLargestEditable();
  const tagInput = findEditableByKeywords(["tag", "\u30bf\u30b0"]);

  if (titleInput && title) {
    fillTextField(titleInput, title);
  }
  if (bodyInput && body) {
    fillTextField(bodyInput, body);
  }
  if (tagInput && tags.length > 0) {
    fillTagsField(tagInput, tags);
  }

  if (categories.length > 0) {
    fillCategoriesField(categories);
  }

  let imagePrompted = false;
  if (images.length > 0) {
    const fileInput = findFileInput();
    if (fileInput) {
      fileInput.click();
      imagePrompted = true;
    }
  }

  let publishReady = false;
  if (wantsPublish) {
    const publishButton =
      findByText("\u516c\u958b") ||
      findByText("publish") ||
      findByText("share") ||
      findByText("\u6295\u7a3f");
    if (publishButton) {
      publishButton.scrollIntoView({ behavior: "smooth", block: "center" });
      publishReady = true;
    }
  }

  return {
    ok: true,
    result: {
      platform,
      publishReady,
      imagePrompted,
      titleFilled: Boolean(titleInput && title),
      bodyFilled: Boolean(bodyInput && body),
      tagsFilled: Boolean(tagInput && tags.length > 0)
    }
  };
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
  ensureHostAllowed("amazon", SHOP_PLATFORM_DOMAINS);

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

async function runMercariPreparePurchase(productName) {
  ensureHostAllowed("mercari", SHOP_PLATFORM_DOMAINS);

  if (productName) {
    const searchInput =
      document.querySelector("input[type='search']") ||
      document.querySelector("input[name='keyword']") ||
      document.querySelector("input[placeholder*='\u691c\u7d22']");
    if (searchInput) {
      setInputValue(searchInput, productName);
      pressEnter(searchInput);
    }
  }

  const firstResult = await waitFor(
    () => document.querySelector("a[href*='/item/']") || document.querySelector("a[href*='/items/']")
  );
  firstResult.click();

  const purchaseButton =
    findByText("\u8cfc\u5165\u624b\u7d9a\u304d\u3078") ||
    findByText("\u8cfc\u5165\u3059\u308b") ||
    findByText("buy");
  if (purchaseButton) {
    purchaseButton.scrollIntoView({ behavior: "smooth", block: "center" });
  }
}

async function runYahooAuctionPreparePurchase(productName) {
  ensureHostAllowed("yahoo_auctions", SHOP_PLATFORM_DOMAINS);

  if (productName) {
    const searchInput =
      document.querySelector("input[name='p']") ||
      document.querySelector("input[type='search']") ||
      document.querySelector("input[placeholder*='\u691c\u7d22']");
    if (searchInput) {
      setInputValue(searchInput, productName);
      pressEnter(searchInput);
    }
  }

  const firstResult = await waitFor(
    () => document.querySelector("a[href*='/auction/']") || document.querySelector("a[href*='/item/']")
  );
  firstResult.click();

  const bidButton =
    findByText("\u5165\u672d\u3059\u308b") ||
    findByText("\u5165\u672d") ||
    findByText("bid");
  if (bidButton) {
    bidButton.scrollIntoView({ behavior: "smooth", block: "center" });
  }
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
    case "post_article":
      return await runPostArticle(action);
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
    case "shop_prepare_purchase": {
      const platform = normalizeText(action.platform);
      if (platform === "mercari") {
        await runMercariPreparePurchase(action.productName || "");
        return { ok: true };
      }
      if (platform === "yahoo_auctions") {
        await runYahooAuctionPreparePurchase(action.productName || "");
        return { ok: true };
      }
      throw new Error("Unsupported shopping platform");
    }
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
