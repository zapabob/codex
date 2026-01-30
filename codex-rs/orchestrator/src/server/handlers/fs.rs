use crate::rpc::*;
use crate::server::OrchestratorServer;
use crate::server::config::OrchestratorConfig;
use std::path::PathBuf;

impl OrchestratorServer {
    pub(crate) async fn handle_fs_read(
        request: &RpcRequest,
        config: &OrchestratorConfig,
    ) -> RpcResponse {
        let params: Result<FsReadRequest, _> = serde_json::from_value(request.params.clone());
        match params {
            Ok(params) => {
                let allowed_bases = Self::get_allowed_base_directories(config);
                let path = match Self::validate_path_against_base(&params.path, &allowed_bases) {
                    Ok(p) => p,
                    Err(e) => return RpcResponse::invalid_params(request.id.clone(), &e),
                };

                match tokio::fs::read_to_string(&path).await {
                    Ok(content) => RpcResponse {
                        id: request.id.clone(),
                        result: Some(serde_json::json!({ "content": content })),
                        error: None,
                    },
                    Err(e) => RpcResponse::internal_error(
                        request.id.clone(),
                        &format!("Failed to read file: {e}"),
                    ),
                }
            }
            Err(e) => {
                RpcResponse::invalid_params(request.id.clone(), format!("Invalid params: {e}"))
            }
        }
    }

    pub(crate) async fn handle_fs_write(
        request: &RpcRequest,
        config: &OrchestratorConfig,
    ) -> RpcResponse {
        let params: Result<FsWriteRequest, _> = serde_json::from_value(request.params.clone());
        match params {
            Ok(params) => {
                let allowed_bases = Self::get_allowed_base_directories(config);
                let path = match Self::validate_path_against_base(&params.path, &allowed_bases) {
                    Ok(p) => p,
                    Err(e) => return RpcResponse::invalid_params(request.id.clone(), &e),
                };

                if let Some(expected_sha) = &params.preimage_sha {
                    if path.exists() {
                        match tokio::fs::read_to_string(&path).await {
                            Ok(existing_content) => {
                                use sha2::{Digest, Sha256};
                                let mut hasher = Sha256::new();
                                hasher.update(existing_content.as_bytes());
                                let current_sha = format!("{:x}", hasher.finalize());

                                if current_sha != *expected_sha {
                                    return RpcResponse::error(
                                        request.id.clone(),
                                        ERROR_CONFLICT,
                                        &format!(
                                            "File was modified. Expected SHA256: {}, got: {}",
                                            expected_sha, current_sha
                                        ),
                                    );
                                }
                            }
                            Err(e) => {
                                return RpcResponse::internal_error(
                                    request.id.clone(),
                                    &format!("Failed to read existing file: {e}"),
                                );
                            }
                        }
                    }
                }

                let temp_path = path.with_extension(".tmp");
                match tokio::fs::write(&temp_path, &params.content).await {
                    Ok(_) => match tokio::fs::rename(&temp_path, &path).await {
                        Ok(_) => {
                            use sha2::{Digest, Sha256};
                            let mut hasher = Sha256::new();
                            hasher.update(params.content.as_bytes());
                            let new_sha = format!("{:x}", hasher.finalize());

                            RpcResponse {
                                id: request.id.clone(),
                                result: Some(
                                    serde_json::json!({ "success": true, "new_sha": new_sha }),
                                ),
                                error: None,
                            }
                        }
                        Err(e) => {
                            let _ = tokio::fs::remove_file(&temp_path).await;
                            RpcResponse::internal_error(
                                request.id.clone(),
                                &format!("Failed to rename temp file: {e}"),
                            )
                        }
                    },
                    Err(e) => RpcResponse::internal_error(
                        request.id.clone(),
                        &format!("Failed to write file: {e}"),
                    ),
                }
            }
            Err(e) => {
                RpcResponse::invalid_params(request.id.clone(), format!("Invalid params: {e}"))
            }
        }
    }

    pub(crate) async fn handle_fs_patch(request: &RpcRequest) -> RpcResponse {
        let params: Result<FsPatchRequest, _> = serde_json::from_value(request.params.clone());
        match params {
            Ok(params) => {
                let diff_lines: Vec<&str> = params.unified_diff.lines().collect();
                let mut applied_files = Vec::new();
                let mut current_file: Option<PathBuf> = None;
                let mut file_content: Vec<String> = Vec::new();
                let mut line_num = 0;

                for line in diff_lines {
                    if line.starts_with("+++ ") {
                        let file_path = line.strip_prefix("+++ ").unwrap_or("").trim();
                        if !file_path.is_empty() {
                            current_file = Some(PathBuf::from(file_path));
                            file_content.clear();
                            line_num = 0;
                        }
                    } else if let Some(ref file_path) = current_file {
                        if line.starts_with("+") && !line.starts_with("++") {
                            file_content.push(line[1..].to_string());
                            line_num += 1;
                        } else if line.starts_with("-") && !line.starts_with("--") {
                            line_num += 1;
                        } else if !line.starts_with("\\") && !line.starts_with("@@") {
                            file_content.push(line.to_string());
                            line_num += 1;
                        }

                        if line_num > 0 && !file_content.is_empty() {
                            let content = file_content.join("\n");
                            match tokio::fs::write(file_path, content).await {
                                Ok(_) => applied_files.push(file_path.clone()),
                                Err(e) => {
                                    return RpcResponse::internal_error(
                                        request.id.clone(),
                                        &format!(
                                            "Failed to apply patch to {}: {e}",
                                            file_path.display()
                                        ),
                                    );
                                }
                            }
                        }
                    }
                }

                RpcResponse {
                    id: request.id.clone(),
                    result: Some(serde_json::json!({
                        "success": true,
                        "applied_files": applied_files.iter().map(|p| p.to_string_lossy().to_string()).collect::<Vec<_>>(),
                    })),
                    error: None,
                }
            }
            Err(e) => {
                RpcResponse::invalid_params(request.id.clone(), &format!("Invalid params: {e}"))
            }
        }
    }
}
