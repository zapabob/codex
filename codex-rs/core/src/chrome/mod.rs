use anyhow::{Result, bail};
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

    let (intent, args, risk) = if contains_any(&normalized, &["tweet", "post", "x "])
        || contains_any(&normalized, &["\\u{}\\u{}\\u{}\\u{}", "\\u{}\\u{}"])
    {
        let text = extract_quoted(utterance).unwrap_or_else(|| utterance.to_string());
        (
            "post_social".to_string(),
            serde_json::json!({
                "platform": "x",
                "text": text,
            }),
            RiskLevel::High,
        )
    } else if contains_any(&normalized, &["amazon", "add to cart", "checkout"])
        || contains_any(
            &normalized,
            &["\\u{}\\u{}\\u{}", "\\u{}\\u{}", "\\u{}\\u{}\\u{}"],
        )
    {
        let product_name = extract_quoted(utterance)
            .or_else(|| extract_after_keyword(utterance, "amazon"))
            .unwrap_or_else(|| utterance.to_string());
        (
            "shop_add_to_cart".to_string(),
            serde_json::json!({
                "product_name": product_name,
            }),
            RiskLevel::High,
        )
    } else if contains_any(&normalized, &["login", "sign in"])
        || contains_any(&normalized, &["\\u{}\\u{}\\u{}\\u{}"])
    {
        (
            "login_start".to_string(),
            serde_json::json!({}),
            RiskLevel::High,
        )
    } else if contains_any(&normalized, &["click"])
        || contains_any(&normalized, &["\\u{}\\u{}\\u{}\\u{}"])
    {
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
    } else if contains_any(&normalized, &["type", "enter", "input"])
        || contains_any(&normalized, &["\\u{}\\u{}", "\\u{}\\u{}\\u{}"])
    {
        let text = extract_quoted(utterance).unwrap_or_default();
        (
            "type".to_string(),
            serde_json::json!({
                "text": text,
                "enter": false,
            }),
            RiskLevel::Medium,
        )
    } else if contains_any(&normalized, &["scroll"])
        || contains_any(&normalized, &["\\u{}\\u{}\\u{}\\u{}\\u{}"])
    {
        let y = if contains_any(&normalized, &["down", "\\u{}"])
            || contains_any(&normalized, &["\\u{}\\u{}"])
        {
            800
        } else if contains_any(&normalized, &["up", "\\u{}"])
            || contains_any(&normalized, &["\\u{}\\u{}"])
        {
            -800
        } else {
            600
        };
        (
            "scroll".to_string(),
            serde_json::json!({ "x": 0, "y": y }),
            RiskLevel::Low,
        )
    } else if contains_any(&normalized, &["wait"])
        || contains_any(&normalized, &["\\u{}\\u{}\\u{}"])
    {
        (
            "wait_for".to_string(),
            serde_json::json!({ "timeout_ms": 8000 }),
            RiskLevel::Low,
        )
    } else {
        bail!("Could not determine intent");
    };

    if matches!(intent.as_str(), "post_social" | "shop_add_to_cart") {
        validate_origin_domain(&request.origin, &intent)?;
    }

    let constraints = request.constraints;
    if let Some(allowed) = constraints
        .as_ref()
        .and_then(|c| c.allowed_intents.as_ref())
    {
        if !allowed.is_empty()
            && !allowed
                .iter()
                .any(|allowed_intent| allowed_intent == &intent)
        {
            bail!("Intent not allowed by constraints");
        }
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
        ("\\u{}", "\\u{}"),
        ("\\u{}", "\\u{}"),
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

    if intent.starts_with("shop_") {
        if host.contains("amazon.") {
            return Ok(());
        }
        bail!("shopping intents allowed only on amazon.* domains");
    }

    Ok(())
}
