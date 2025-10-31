use codex_app_server_protocol::AuthMode;
use codex_common::CliConfigOverrides;
use codex_core::auth::login_with_api_key;
use codex_core::auth::logout;
use codex_core::auth::AuthCredentialsStoreMode;
use codex_core::auth::CLIENT_ID;
use codex_core::auth::gemini::{GeminiAuthProvider, GeminiCredentials, CredentialSource, read_gemini_api_key_from_env};
use codex_core::config::Config;
use codex_core::config::ConfigOverrides;
use codex_core::protocol_config_types::ForcedLoginMethod;
use codex_core::CodexAuth;
use codex_login::run_device_code_login;
use codex_login::run_login_server;
use codex_login::ServerOptions;
use std::io::IsTerminal;
use std::io::Read;
use std::path::PathBuf;

pub async fn login_with_chatgpt(
    codex_home: PathBuf,
    forced_chatgpt_workspace_id: Option<String>,
    cli_auth_credentials_store_mode: AuthCredentialsStoreMode,
) -> std::io::Result<()> {
    let opts = ServerOptions::new(
        codex_home,
        CLIENT_ID.to_string(),
        forced_chatgpt_workspace_id,
        cli_auth_credentials_store_mode,
    );
    let server = run_login_server(opts)?;

    eprintln!(
        "Starting local login server on http://localhost:{}.\nIf your browser did not open, navigate to this URL to authenticate:\n\n{}",
        server.actual_port, server.auth_url,
    );

    server.block_until_done().await
}

pub async fn run_login_with_chatgpt(cli_config_overrides: CliConfigOverrides) -> ! {
    let config = load_config_or_exit(cli_config_overrides).await;

    if matches!(config.forced_login_method, Some(ForcedLoginMethod::Api)) {
        eprintln!("ChatGPT login is disabled. Use API key login instead.");
        std::process::exit(1);
    }

    let forced_chatgpt_workspace_id = config.forced_chatgpt_workspace_id.clone();

    match login_with_chatgpt(
        config.codex_home,
        forced_chatgpt_workspace_id,
        config.cli_auth_credentials_store_mode,
    )
    .await
    {
        Ok(_) => {
            eprintln!("Successfully logged in");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Error logging in: {e}");
            std::process::exit(1);
        }
    }
}

pub async fn run_login_with_api_key(
    cli_config_overrides: CliConfigOverrides,
    api_key: String,
) -> ! {
    let config = load_config_or_exit(cli_config_overrides).await;

    if matches!(config.forced_login_method, Some(ForcedLoginMethod::Chatgpt)) {
        eprintln!("API key login is disabled. Use ChatGPT login instead.");
        std::process::exit(1);
    }

    match login_with_api_key(
        &config.codex_home,
        &api_key,
        config.cli_auth_credentials_store_mode,
    ) {
        Ok(_) => {
            eprintln!("Successfully logged in");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Error logging in: {e}");
            std::process::exit(1);
        }
    }
}

pub fn read_api_key_from_stdin() -> String {
    let mut stdin = std::io::stdin();

    if stdin.is_terminal() {
        eprintln!(
            "--with-api-key expects the API key on stdin. Try piping it, e.g. `printenv OPENAI_API_KEY | codex login --with-api-key`."
        );
        std::process::exit(1);
    }

    eprintln!("Reading API key from stdin...");

    let mut buffer = String::new();
    if let Err(err) = stdin.read_to_string(&mut buffer) {
        eprintln!("Failed to read API key from stdin: {err}");
        std::process::exit(1);
    }

    let api_key = buffer.trim().to_string();
    if api_key.is_empty() {
        eprintln!("No API key provided via stdin.");
        std::process::exit(1);
    }

    api_key
}

/// Login using the OAuth device code flow.
pub async fn run_login_with_device_code(
    cli_config_overrides: CliConfigOverrides,
    issuer_base_url: Option<String>,
    client_id: Option<String>,
) -> ! {
    let config = load_config_or_exit(cli_config_overrides).await;
    if matches!(config.forced_login_method, Some(ForcedLoginMethod::Api)) {
        eprintln!("ChatGPT login is disabled. Use API key login instead.");
        std::process::exit(1);
    }
    let forced_chatgpt_workspace_id = config.forced_chatgpt_workspace_id.clone();
    let mut opts = ServerOptions::new(
        config.codex_home,
        client_id.unwrap_or(CLIENT_ID.to_string()),
        forced_chatgpt_workspace_id,
        config.cli_auth_credentials_store_mode,
    );
    if let Some(iss) = issuer_base_url {
        opts.issuer = iss;
    }
    match run_device_code_login(opts).await {
        Ok(()) => {
            eprintln!("Successfully logged in");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Error logging in with device code: {e}");
            std::process::exit(1);
        }
    }
}

pub async fn run_login_status(cli_config_overrides: CliConfigOverrides) -> ! {
    let config = load_config_or_exit(cli_config_overrides).await;

    match CodexAuth::from_auth_storage(&config.codex_home, config.cli_auth_credentials_store_mode) {
        Ok(Some(auth)) => match auth.mode {
            AuthMode::ApiKey => match auth.get_token().await {
                Ok(api_key) => {
                    eprintln!("Logged in using an API key - {}", safe_format_key(&api_key));
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("Unexpected error retrieving API key: {e}");
                    std::process::exit(1);
                }
            },
            AuthMode::ChatGPT => {
                eprintln!("Logged in using ChatGPT");
                std::process::exit(0);
            }
        },
        Ok(None) => {
            eprintln!("Not logged in");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Error checking login status: {e}");
            std::process::exit(1);
        }
    }
}

pub async fn run_logout(cli_config_overrides: CliConfigOverrides) -> ! {
    let config = load_config_or_exit(cli_config_overrides).await;

    match logout(&config.codex_home, config.cli_auth_credentials_store_mode) {
        Ok(true) => {
            eprintln!("Successfully logged out");
            std::process::exit(0);
        }
        Ok(false) => {
            eprintln!("Not logged in");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Error logging out: {e}");
            std::process::exit(1);
        }
    }
}

async fn load_config_or_exit(cli_config_overrides: CliConfigOverrides) -> Config {
    let cli_overrides = match cli_config_overrides.parse_overrides() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error parsing -c overrides: {e}");
            std::process::exit(1);
        }
    };

    let config_overrides = ConfigOverrides::default();
    match Config::load_with_cli_overrides(cli_overrides, config_overrides).await {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading configuration: {e}");
            std::process::exit(1);
        }
    }
}

fn safe_format_key(key: &str) -> String {
    if key.len() <= 13 {
        return "***".to_string();
    }
    let prefix = &key[..8];
    let suffix = &key[key.len() - 5..];
    format!("{prefix}***{suffix}")
}

/// Login with Gemini using API key from stdin
pub async fn run_gemini_login_with_api_key(cli_config_overrides: CliConfigOverrides) -> ! {
    let config = load_config_or_exit(cli_config_overrides).await;
    
    let api_key = read_api_key_from_stdin();
    
    let client = codex_core::default_client::create_client();
    let provider = GeminiAuthProvider::new(config.codex_home.clone(), client);
    
    let credentials = GeminiCredentials {
        source: CredentialSource::ApiKey { key: api_key },
        last_refresh: None,
    };
    
    match provider.save_credentials(&credentials) {
        Ok(_) => {
            eprintln!("Successfully logged in to Gemini with API key");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Error saving Gemini credentials: {e}");
            std::process::exit(1);
        }
    }
}

/// Login with Gemini using OAuth 2.0
pub async fn run_gemini_login_with_oauth(cli_config_overrides: CliConfigOverrides) -> ! {
    let _config = load_config_or_exit(cli_config_overrides).await;
    
    eprintln!("OAuth 2.0 login for Gemini is not yet implemented.");
    eprintln!("Please use API key login with: codex login gemini login --with-api-key");
    eprintln!("Set your API key via: export GEMINI_API_KEY=your-api-key");
    std::process::exit(1);
}

/// Show Gemini login status
pub async fn run_gemini_login_status(cli_config_overrides: CliConfigOverrides) -> ! {
    let config = load_config_or_exit(cli_config_overrides).await;
    
    let client = codex_core::default_client::create_client();
    let provider = GeminiAuthProvider::new(config.codex_home.clone(), client);
    
    match provider.resolve_credentials() {
        Ok(Some(credentials)) => {
            match &credentials.source {
                CredentialSource::ApiKey { key } => {
                    eprintln!("Logged in to Gemini using API key: {}", safe_format_key(key));
                    if read_gemini_api_key_from_env().is_some() {
                        eprintln!("Source: GEMINI_API_KEY environment variable");
                    } else {
                        eprintln!("Source: Secure storage");
                    }
                }
                CredentialSource::OAuth { expiry, .. } => {
                    eprintln!("Logged in to Gemini using OAuth 2.0");
                    if let Some(exp) = expiry {
                        eprintln!("Token expires: {}", exp);
                    }
                }
            }
            std::process::exit(0);
        }
        Ok(None) => {
            eprintln!("Not logged in to Gemini");
            eprintln!("Use 'codex login gemini login --with-api-key' to login");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Error checking Gemini login status: {e}");
            std::process::exit(1);
        }
    }
}

/// Logout from Gemini
pub async fn run_gemini_logout(cli_config_overrides: CliConfigOverrides) -> ! {
    let config = load_config_or_exit(cli_config_overrides).await;
    
    let client = codex_core::default_client::create_client();
    let provider = GeminiAuthProvider::new(config.codex_home.clone(), client);
    
    // Try to load current credentials to see if logged in
    match provider.resolve_credentials() {
        Ok(Some(_)) => {
            // Clear the credentials by saving an empty config
            // For now, we'll just inform the user
            eprintln!("To logout from Gemini:");
            eprintln!("1. Unset GEMINI_API_KEY environment variable if set");
            eprintln!("2. Remove gemini credentials from ~/.codex/config.yaml");
            eprintln!("3. Remove credentials from secure storage using: codex logout");
            std::process::exit(0);
        }
        Ok(None) => {
            eprintln!("Not logged in to Gemini");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Error checking Gemini credentials: {e}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::safe_format_key;

    #[test]
    fn formats_long_key() {
        let key = "sk-proj-1234567890ABCDE";
        assert_eq!(safe_format_key(key), "sk-proj-***ABCDE");
    }

    #[test]
    fn short_key_returns_stars() {
        let key = "sk-proj-12345";
        assert_eq!(safe_format_key(key), "***");
    }
}
