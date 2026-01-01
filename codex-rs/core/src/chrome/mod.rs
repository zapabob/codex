use anyhow::{Result, bail};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChromeOrigin {
    pub tab_id: Option<i64>,
    pub frame_id: Option<i64>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChromeConstraints {
    pub allowed_intents: Option<Vec<String>>,
    pub require_confirmation_for: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChromeNlRequest {
    pub utterance: String,
    pub origin: Option<ChromeOrigin>,
    pub constraints: Option<ChromeConstraints>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChromeIntent {
    pub intent: String,
    pub args: serde_json::Value,
    pub risk: RiskLevel,
    pub requires_confirmation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChromeNlResponse {
    pub intent: ChromeIntent,
    pub warnings: Vec<String>,
}

pub fn parse_nl_command(request: ChromeNlRequest) -> Result<ChromeNlResponse> {
    let utterance = request.utterance.trim();
    if utterance.is_empty() {
        bail!("Empty instruction");
    }

    let normalized = utterance.to_lowercase();
    let mut warnings = Vec::new();

    let article_platform = detect_article_platform(&normalized);
    let shop_platform = detect_shop_platform(&normalized);
    let wants_publish = contains_any(
        &normalized,
        &[
            "publish",
            "public",
            "post",
            "\u{516c}\u{958b}",
            "\u{6295}\u{7a3f}",
            "\u{9001}\u{4fe1}",
        ],
    );

    let (intent, args, risk) = if is_x_post(&normalized) {
        let text = extract_quoted(utterance).unwrap_or_else(|| utterance.to_string());
        (
            "post_social".to_string(),
            serde_json::json!({
                "platform": "x",
                "text": text,
            }),
            RiskLevel::High,
        )
    } else if let Some(platform) = article_platform {
        let title = extract_label_value(utterance, &["title", "\u{30bf}\u{30a4}\u{30c8}\u{30eb}"])
            .or_else(|| extract_quoted(utterance));
        let body = extract_label_value(
            utterance,
            &[
                "body",
                "\u{672c}\u{6587}",
                "\u{5185}\u{5bb9}",
                "\u{8a18}\u{4e8b}",
            ],
        );
        let tags = extract_list(utterance, &["tags", "\u{30bf}\u{30b0}"]);
        let categories = extract_list(
            utterance,
            &["category", "categories", "\u{30ab}\u{30c6}\u{30b4}\u{30ea}"],
        );
        let images = extract_images(utterance);

        if title.is_none() {
            warnings.push("Missing title".to_string());
        }
        if body.is_none() {
            warnings.push("Missing body".to_string());
        }
        if !images.is_empty() {
            warnings.push("Image attachments require manual file selection".to_string());
        }

        let risk = if wants_publish {
            RiskLevel::High
        } else {
            RiskLevel::Medium
        };

        (
            "post_article".to_string(),
            serde_json::json!({
                "platform": platform,
                "title": title,
                "body": body,
                "tags": tags,
                "categories": categories,
                "images": images,
                "publish": wants_publish,
            }),
            risk,
        )
    } else if let Some(platform) = shop_platform {
        let product_name = extract_quoted(utterance)
            .or_else(|| extract_after_keyword(utterance, "amazon"))
            .or_else(|| extract_after_keyword(utterance, "mercari"))
            .or_else(|| extract_after_keyword(utterance, "yahoo"))
            .unwrap_or_else(|| utterance.to_string());
        let intent = if platform == "amazon" {
            "shop_add_to_cart"
        } else {
            "shop_prepare_purchase"
        };
        (
            intent.to_string(),
            serde_json::json!({
                "platform": platform,
                "product_name": product_name,
            }),
            RiskLevel::High,
        )
    } else if contains_any(
        &normalized,
        &["login", "sign in", "\u{30ed}\u{30b0}\u{30a4}\u{30f3}"],
    ) {
        (
            "login_start".to_string(),
            serde_json::json!({}),
            RiskLevel::High,
        )
    } else if contains_any(&normalized, &["click", "\u{30af}\u{30ea}\u{30c3}\u{30af}"]) {
        let target = extract_quoted(utterance)
            .map(|query| {
                serde_json::json!({
                    "strategy": "text",
                    "query": query,
                })
            })
            .unwrap_or_else(|| serde_json::json!({}));
        (
            "click".to_string(),
            serde_json::json!({ "target": target }),
            RiskLevel::Low,
        )
    } else if contains_any(
        &normalized,
        &[
            "type",
            "enter",
            "input",
            "\u{5165}\u{529b}",
            "\u{5165}\u{308c}\u{308b}",
        ],
    ) {
        let text = extract_quoted(utterance).unwrap_or_default();
        (
            "type".to_string(),
            serde_json::json!({
                "text": text,
                "enter": false,
            }),
            RiskLevel::Medium,
        )
    } else if contains_any(
        &normalized,
        &["scroll", "\u{30b9}\u{30af}\u{30ed}\u{30fc}\u{30eb}"],
    ) {
        let y = if contains_any(&normalized, &["down", "\u{4e0b}", "\u{4e0b}\u{306b}"]) {
            800
        } else if contains_any(&normalized, &["up", "\u{4e0a}", "\u{4e0a}\u{306b}"]) {
            -800
        } else {
            600
        };
        (
            "scroll".to_string(),
            serde_json::json!({ "x": 0, "y": y }),
            RiskLevel::Low,
        )
    } else if contains_any(
        &normalized,
        &["wait", "\u{5f85}\u{6a5f}", "\u{5f85}\u{3064}"],
    ) {
        (
            "wait_for".to_string(),
            serde_json::json!({ "timeout_ms": 8000 }),
            RiskLevel::Low,
        )
    } else {
        bail!("Could not determine intent");
    };

    if matches!(
        intent.as_str(),
        "post_social" | "post_article" | "shop_add_to_cart" | "shop_prepare_purchase"
    ) {
        validate_origin_domain(&request.origin, &intent)?;
    }

    let constraints = request.constraints;
    if let Some(allowed) = constraints
        .as_ref()
        .and_then(|c| c.allowed_intents.as_ref())
        && !allowed.is_empty()
            && !allowed
                .iter()
                .any(|allowed_intent| allowed_intent == &intent)
        {
            bail!("Intent not allowed by constraints");
        }

    let requires_confirmation = match risk {
        RiskLevel::High => true,
        _ => constraints
            .as_ref()
            .and_then(|c| c.require_confirmation_for.as_ref())
            .map(|list| list.iter().any(|entry| entry == &intent))
            .unwrap_or(false),
    };

    if matches!(risk, RiskLevel::Medium) && !requires_confirmation {
        warnings.push("Medium-risk action without explicit confirmation".to_string());
    }

    Ok(ChromeNlResponse {
        intent: ChromeIntent {
            intent,
            args,
            risk,
            requires_confirmation,
        },
        warnings,
    })
}

fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}

fn extract_quoted(input: &str) -> Option<String> {
    let pairs = [
        ("\"", "\""),
        ("'", "'"),
        ("\u{300c}", "\u{300d}"),
        ("\u{300e}", "\u{300f}"),
    ];
    for (open, close) in pairs {
        if let Some(start) = input.find(open) {
            let rest = &input[start + open.len()..];
            if let Some(end_rel) = rest.find(close) {
                let end = start + open.len() + end_rel;
                return Some(input[start + open.len()..end].trim().to_string());
            }
        }
    }
    None
}

fn extract_label_value(input: &str, labels: &[&str]) -> Option<String> {
    let open1 = "\u{300c}";
    let close1 = "\u{300d}";
    let open2 = "\u{300e}";
    let close2 = "\u{300f}";

    for label in labels {
        let pattern = format!(
            "(?i){}\\s*[:\\uff1a]?\\s*(?P<value>\\\"[^\\\"]+\\\"|'[^']+'|{}[^{}]+{}|{}[^{}]+{}|[^,\\n]+)",
            regex::escape(label),
            open1,
            close1,
            close1,
            open2,
            close2,
            close2
        );
        if let Ok(regex) = Regex::new(&pattern)
            && let Some(caps) = regex.captures(input)
                && let Some(value) = caps.name("value") {
                    let raw = value.as_str().trim();
                    if !raw.is_empty() {
                        return Some(strip_quotes(raw));
                    }
                }
    }
    None
}

fn extract_list(input: &str, labels: &[&str]) -> Vec<String> {
    let value = extract_label_value(input, labels).unwrap_or_default();
    if value.is_empty() {
        return Vec::new();
    }

    value
        .split([',', '\u{3001}', '\u{ff0c}'])
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn extract_images(input: &str) -> Vec<String> {
    let value =
        extract_label_value(input, &["image", "images", "\u{753b}\u{50cf}"]).unwrap_or_default();
    if value.is_empty() {
        return Vec::new();
    }

    value
        .split([',', '\u{3001}', '\u{ff0c}'])
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn strip_quotes(input: &str) -> String {
    let trimmed = input.trim();
    let pairs = [
        ("\"", "\""),
        ("'", "'"),
        ("\u{300c}", "\u{300d}"),
        ("\u{300e}", "\u{300f}"),
    ];

    for (open, close) in pairs {
        if trimmed.starts_with(open)
            && trimmed.ends_with(close)
            && trimmed.len() >= open.len() + close.len()
        {
            let end = trimmed.len() - close.len();
            return trimmed[open.len()..end].trim().to_string();
        }
    }
    trimmed.to_string()
}

fn is_x_post(normalized: &str) -> bool {
    contains_any(
        normalized,
        &[
            "tweet",
            "twitter",
            "x.com",
            "post on x",
            "x\u{3078}",
            "x\u{306b}",
            "\u{30c4}\u{30a4}\u{30fc}\u{30c8}",
            "\u{30dd}\u{30b9}\u{30c8}",
        ],
    )
}

fn detect_article_platform(normalized: &str) -> Option<&'static str> {
    if contains_any(normalized, &["note", "\u{30ce}\u{30fc}\u{30c8}"]) {
        return Some("note");
    }
    if contains_any(normalized, &["qiita"]) {
        return Some("qiita");
    }
    if contains_any(normalized, &["zenn"]) {
        return Some("zenn");
    }
    None
}

fn detect_shop_platform(normalized: &str) -> Option<&'static str> {
    if contains_any(normalized, &["amazon", "\u{30a2}\u{30de}\u{30be}\u{30f3}"]) {
        return Some("amazon");
    }
    if contains_any(normalized, &["mercari", "\u{30e1}\u{30eb}\u{30ab}\u{30ea}"]) {
        return Some("mercari");
    }
    if contains_any(
        normalized,
        &[
            "yahoo",
            "auctions.yahoo.co.jp",
            "yahoo auction",
            "\u{30e4}\u{30d5}\u{30aa}\u{30af}",
        ],
    ) {
        return Some("yahoo_auctions");
    }
    None
}

fn extract_after_keyword(input: &str, keyword: &str) -> Option<String> {
    let lower = input.to_lowercase();
    let keyword_lower = keyword.to_lowercase();
    let start = lower.find(&keyword_lower)?;
    let value = input[start + keyword_lower.len()..].trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn validate_origin_domain(origin: &Option<ChromeOrigin>, intent: &str) -> Result<()> {
    let url = origin
        .as_ref()
        .and_then(|value| value.url.as_ref())
        .ok_or_else(|| anyhow::anyhow!("Origin URL is required for this intent"))?;

    let parsed = url::Url::parse(url).map_err(|_| anyhow::anyhow!("Invalid origin URL"))?;
    let host = parsed
        .host_str()
        .ok_or_else(|| anyhow::anyhow!("Origin URL missing host"))?;

    if intent == "post_social" {
        if host.contains("x.com") || host.contains("twitter.com") {
            return Ok(());
        }
        bail!("post_social allowed only on x.com or twitter.com");
    }

    if intent == "post_article" {
        if host.contains("note.com") || host.contains("qiita.com") || host.contains("zenn.dev") {
            return Ok(());
        }
        bail!("post_article allowed only on note.com, qiita.com, or zenn.dev");
    }

    if intent == "shop_add_to_cart" {
        if host.contains("amazon.com") || host.contains("amazon.co.jp") {
            return Ok(());
        }
        bail!("shop_add_to_cart allowed only on amazon.com or amazon.co.jp");
    }

    if intent == "shop_prepare_purchase" {
        if host.contains("mercari.com")
            || host.contains("mercari.jp")
            || host.contains("auctions.yahoo.co.jp")
        {
            return Ok(());
        }
        bail!("shop_prepare_purchase allowed only on mercari or auctions.yahoo.co.jp");
    }

    Ok(())
}
