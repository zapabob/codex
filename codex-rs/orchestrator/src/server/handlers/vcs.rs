use crate::error_handler::{create_secure_rpc_error, messages};
use crate::rpc::*;
use crate::server::OrchestratorServer;
use crate::server::config::OrchestratorConfig;
use git2::{DiffOptions, Repository};
use std::path::Path;

impl OrchestratorServer {
    pub(crate) async fn handle_vcs_diff(
        request: &RpcRequest,
        config: &OrchestratorConfig,
    ) -> RpcResponse {
        let repo_root = config.codex_dir.parent().unwrap_or_else(|| Path::new("."));

        match Repository::open(repo_root) {
            Ok(repo) => {
                let head = match repo.head() {
                    Ok(head) => head.peel_to_commit().ok(),
                    Err(_) => None,
                };

                let mut diff_options = DiffOptions::new();
                diff_options.include_untracked(true);
                diff_options.include_ignored(false);

                let diff = if let Some(head_commit) = head {
                    let tree = match head_commit.tree() {
                        Ok(t) => t,
                        Err(e) => {
                            return RpcResponse::internal_error(
                                request.id.clone(),
                                &format!("Failed to get tree: {e}"),
                            );
                        }
                    };
                    repo.diff_tree_to_workdir_with_index(Some(&tree), Some(&mut diff_options))
                } else {
                    repo.diff_tree_to_workdir(None, Some(&mut diff_options))
                };

                match diff {
                    Ok(diff) => {
                        let mut diff_text = String::new();
                        if let Err(e) =
                            diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
                                match line.origin() {
                                    ' ' | '+' | '-' | 'F' | 'H' | 'B' => {
                                        diff_text.push(line.origin());
                                        if let Ok(content) = std::str::from_utf8(line.content()) {
                                            diff_text.push_str(content);
                                        }
                                    }
                                    _ => {}
                                }
                                true
                            })
                        {
                            return RpcResponse {
                                id: request.id.clone(),
                                result: None,
                                error: Some(create_secure_rpc_error(
                                    ERROR_INTERNAL,
                                    messages::OPERATION_FAILED,
                                    Some(&e),
                                )),
                            };
                        }

                        RpcResponse {
                            id: request.id.clone(),
                            result: Some(serde_json::json!({ "diff": diff_text })),
                            error: None,
                        }
                    }
                    Err(e) => RpcResponse::internal_error(
                        request.id.clone(),
                        &format!("Failed to compute diff: {e}"),
                    ),
                }
            }
            Err(e) => RpcResponse::internal_error(
                request.id.clone(),
                &format!("Not a git repository or failed to open: {e}"),
            ),
        }
    }
}
