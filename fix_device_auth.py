#!/usr/bin/env python3
"""
codex-rs/login/src/device_code_auth.rsの競合を修正
"""

# Read the file
with open('codex-rs/login/src/device_code_auth.rs', 'r', encoding='utf-8') as f:
    content = f.read()

# Remove the old print_colored_warning_device_code function and update the main function
content = content.replace('''<<<<<<< HEAD
fn print_colored_warning_device_code() {
    let mut stdout = io::stdout().lock();
    let _ = write!(
        stdout,
        "{ANSI_YELLOW}{ANSI_BOLD}Only use device code authentication when browser login is not available.{ANSI_RESET}{ANSI_YELLOW}\n\
{ANSI_BOLD}Keep the code secret; do not share it.{ANSI_RESET}{ANSI_RESET}\n\n"
    );
    let _ = stdout.flush();
}

fn print_device_code_prompt(code: &str, issuer_base_url: &str) {
    println!(
        "\nWelcome to Codex [v{ANSI_GRAY}{version}{ANSI_RESET}]\n{ANSI_GRAY}OpenAI's command-line coding agent{ANSI_RESET}\n\
\nFollow these steps to sign in with ChatGPT using device code authorization:\n\
\n1. Open this link in your browser and sign in to your account\n   {ANSI_BLUE}{issuer_base_url}/codex/device{ANSI_RESET}\n\
\n2. Enter this one-time code {ANSI_GRAY}(expires in 15 minutes){ANSI_RESET}\n   {ANSI_BLUE}{code}{ANSI_RESET}\n\
\n{ANSI_GRAY}Device codes are a common phishing target. Never share this code.{ANSI_RESET}\n",
        version = env!("CARGO_PKG_VERSION"),
        code = code,
        issuer_base_url = issuer_base_url
    );
    let _ = stdout.flush();
}

/// Full device code login flow.
pub async fn run_device_code_login(opts: ServerOptions) -> std::io::Result<()> {
    let client = reqwest::Client::new();
<<<<<<< HEAD
    let base_url = opts.issuer.trim_end_matches('/');
    let api_base_url = format!("{}/api/accounts", opts.issuer.trim_end_matches('/'));
    print_colored_warning_device_code();
    let uc = request_user_code(&client, &api_base_url, &opts.client_id).await?;

    println!(
        "To authenticate:\n  1. Open in your browser: {ANSI_BOLD}https://auth.openai.com/codex/device{ANSI_RESET}\n  2. Enter the one-time code below within 15 minutes:\n\n     {ANSI_BOLD}{}{ANSI_RESET}\n",
        uc.user_code
    );
=======
    let issuer_base_url = opts.issuer.trim_end_matches('/');
    let api_base_url = format!("{issuer_base_url}/api/accounts");
    let uc = request_user_code(&client, &api_base_url, &opts.client_id).await?;

    print_device_code_prompt(&uc.user_code, issuer_base_url);
>>>>>>> upstream/main''', '''fn print_device_code_prompt(code: &str, issuer_base_url: &str) {
    println!(
        "\nWelcome to Codex [v{ANSI_GRAY}{version}{ANSI_RESET}]\n{ANSI_GRAY}OpenAI's command-line coding agent{ANSI_RESET}\n\
\nFollow these steps to sign in with ChatGPT using device code authorization:\n\
\n1. Open this link in your browser and sign in to your account\n   {ANSI_BLUE}{issuer_base_url}/codex/device{ANSI_RESET}\n\
\n2. Enter this one-time code {ANSI_GRAY}(expires in 15 minutes){ANSI_RESET}\n   {ANSI_BLUE}{code}{ANSI_RESET}\n\
\n{ANSI_GRAY}Device codes are a common phishing target. Never share this code.{ANSI_RESET}\n",
        version = env!("CARGO_PKG_VERSION"),
        code = code,
        issuer_base_url = issuer_base_url
    );
    let _ = io::stdout().flush();
}

/// Full device code login flow.
pub async fn run_device_code_login(opts: ServerOptions) -> std::io::Result<()> {
    let client = reqwest::Client::new();
    let issuer_base_url = opts.issuer.trim_end_matches('/');
    let api_base_url = format!("{issuer_base_url}/api/accounts");
    let uc = request_user_code(&client, &api_base_url, &opts.client_id).await?;

    print_device_code_prompt(&uc.user_code, issuer_base_url);''')

# Write back
with open('codex-rs/login/src/device_code_auth.rs', 'w', encoding='utf-8') as f:
    f.write(content)

print("Fixed conflicts in device_code_auth.rs")