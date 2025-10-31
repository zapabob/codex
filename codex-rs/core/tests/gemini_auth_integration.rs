// Integration tests for Gemini authentication
use codex_core::auth::gemini::{CredentialSource, GeminiAuthProvider, GeminiCredentials};
use codex_core::default_client::create_client;
use serial_test::serial;
use std::env;
use tempfile::tempdir;

#[test]
fn test_gemini_credentials_serialization() {
    let api_key_creds = GeminiCredentials {
        source: CredentialSource::ApiKey {
            key: "test-api-key".to_string(),
        },
        last_refresh: None,
    };

    let json = serde_json::to_string(&api_key_creds).expect("Serialization should succeed");
    assert!(json.contains("api_key"));
    assert!(json.contains("test-api-key"));

    let deserialized: GeminiCredentials =
        serde_json::from_str(&json).expect("Deserialization should succeed");
    assert_eq!(api_key_creds, deserialized);
}

#[test]
#[serial(gemini_env)]
fn test_gemini_auth_provider_resolve_credentials_from_env() {
    let dir = tempdir().expect("tempdir should succeed");
    let client = create_client();
    let provider = GeminiAuthProvider::new(dir.path().to_path_buf(), client);

    // Set environment variable
    unsafe {
        env::set_var("GEMINI_API_KEY", "test-env-key");
    }

    let result = provider
        .resolve_credentials()
        .expect("resolve_credentials should succeed");

    unsafe {
        env::remove_var("GEMINI_API_KEY");
    }

    assert!(result.is_some());
    let creds = result.unwrap();
    match creds.source {
        CredentialSource::ApiKey { key } => {
            assert_eq!(key, "test-env-key");
        }
        _ => panic!("Expected API key credential"),
    }
}

#[test]
#[serial(gemini_env)]
fn test_gemini_auth_provider_no_credentials() {
    let dir = tempdir().expect("tempdir should succeed");
    let client = create_client();
    let provider = GeminiAuthProvider::new(dir.path().to_path_buf(), client);

    // Ensure environment variable is not set
    unsafe {
        env::remove_var("GEMINI_API_KEY");
    }

    let result = provider
        .resolve_credentials()
        .expect("resolve_credentials should succeed");

    assert!(result.is_none());
}

#[test]
#[serial(gemini_env)]
fn test_gemini_credentials_save_and_load() {
    let dir = tempdir().expect("tempdir should succeed");
    let client = create_client();
    let provider = GeminiAuthProvider::new(dir.path().to_path_buf(), client.clone());

    let creds = GeminiCredentials {
        source: CredentialSource::ApiKey {
            key: "saved-api-key".to_string(),
        },
        last_refresh: None,
    };

    // Save credentials
    provider
        .save_credentials(&creds)
        .expect("save_credentials should succeed");

    // Create a new provider to test loading
    let provider2 = GeminiAuthProvider::new(dir.path().to_path_buf(), client);

    // Ensure environment variable is not interfering
    unsafe {
        env::remove_var("GEMINI_API_KEY");
    }

    let loaded = provider2
        .resolve_credentials()
        .expect("resolve_credentials should succeed");

    assert!(loaded.is_some());
    let loaded_creds = loaded.unwrap();
    match loaded_creds.source {
        CredentialSource::ApiKey { key } => {
            assert_eq!(key, "saved-api-key");
        }
        _ => panic!("Expected API key credential"),
    }
}

#[test]
fn test_attach_auth_with_api_key() {
    let dir = tempdir().expect("tempdir should succeed");
    let client = create_client();
    let provider = GeminiAuthProvider::new(dir.path().to_path_buf(), client.clone());

    let creds = GeminiCredentials {
        source: CredentialSource::ApiKey {
            key: "test-api-key".to_string(),
        },
        last_refresh: None,
    };

    let request = client.get("https://generativelanguage.googleapis.com/v1/models");
    let _request_with_auth = provider.attach_auth(request, &creds);

    // We can't easily inspect headers, but we verify it doesn't panic
}

#[test]
fn test_attach_auth_with_oauth() {
    let dir = tempdir().expect("tempdir should succeed");
    let client = create_client();
    let provider = GeminiAuthProvider::new(dir.path().to_path_buf(), client.clone());

    let creds = GeminiCredentials {
        source: CredentialSource::OAuth {
            access_token: "test-access-token".to_string(),
            refresh_token: Some("test-refresh-token".to_string()),
            expiry: None,
        },
        last_refresh: None,
    };

    let request = client.get("https://generativelanguage.googleapis.com/v1/models");
    let _request_with_auth = provider.attach_auth(request, &creds);

    // We can't easily inspect headers, but we verify it doesn't panic
}
