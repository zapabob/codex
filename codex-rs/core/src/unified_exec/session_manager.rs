use std::sync::Arc;

use tokio::sync::mpsc;
use tokio::time::Duration;
use tokio::time::Instant;

use crate::exec::SandboxType;
use crate::exec_env::create_env;
use crate::sandboxing::ExecEnv;

use super::ExecCommandRequest;
use super::SessionEntry;
use super::SessionStore;
use super::UnifiedExecContext;
use super::UnifiedExecError;
use super::UnifiedExecResponse;
use super::UnifiedExecSession;
use super::UnifiedExecSessionManager;
use super::WriteStdinRequest;
use super::generate_chunk_id;
use super::resolve_max_tokens;
use super::truncate_output_to_tokens;

impl UnifiedExecSessionManager {
    pub(crate) async fn allocate_process_id(&self) -> String {
        let mut store = self.session_store.lock().await;
        loop {
            let millis = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis();
            let id = format!("proc-{millis}-{}", generate_chunk_id());
            if store.sessions.contains_key(&id) || store.reserved_sessions_id.contains(&id) {
                continue;
            }
            store.reserved_sessions_id.insert(id.clone());
            return id;
        }
    }

    pub(crate) async fn release_process_id(&self, process_id: &str) {
        let entry = {
            let mut store = self.session_store.lock().await;
            store.reserved_sessions_id.remove(process_id);
            store.sessions.remove(process_id)
        };

        if let Some(entry) = entry {
            entry.session.terminate();
        }
    }

    pub(crate) async fn exec_command(
        &self,
        request: ExecCommandRequest,
        context: &UnifiedExecContext,
    ) -> Result<UnifiedExecResponse, UnifiedExecError> {
        let cwd = request
            .workdir
            .clone()
            .unwrap_or_else(|| context.turn.cwd.clone());
        let env = create_env(&context.turn.shell_environment_policy);

        let exec_env = ExecEnv {
            command: request.command.clone(),
            cwd: cwd.clone(),
            env,
            timeout_ms: None,
            sandbox: SandboxType::None,
            sandbox_permissions: request.sandbox_permissions,
            justification: request.justification.clone(),
            arg0: None,
        };

        let session = Arc::new(self.open_session_with_exec_env(&exec_env).await?);

        let (output_buffer, output_notify) = session.output_handles();
        let yield_time_ms = super::clamp_yield_time(request.yield_time_ms);
        let max_tokens = resolve_max_tokens(request.max_output_tokens);

        let started_at = Instant::now();
        let deadline = started_at + Duration::from_millis(yield_time_ms);
        let collected =
            Self::collect_output_until_deadline(&output_buffer, &output_notify, deadline).await;
        let wall_time = Instant::now().saturating_duration_since(started_at);

        let text = String::from_utf8_lossy(&collected).to_string();
        let (output, original_token_count) = truncate_output_to_tokens(&text, max_tokens);
        let exit_code = session.exit_code();
        let has_exited = session.has_exited() || exit_code.is_some();

        if has_exited {
            self.release_process_id(&request.process_id).await;
        } else {
            let now = Instant::now();
            let transcript = Arc::new(tokio::sync::Mutex::new(super::CommandTranscript::default()));
            {
                let mut guard = transcript.lock().await;
                guard.append(&collected);
            }

            let entry = SessionEntry {
                session: Arc::clone(&session),
                session_ref: Arc::clone(&context.session),
                turn_ref: Arc::clone(&context.turn),
                call_id: context.call_id.clone(),
                process_id: request.process_id.clone(),
                command: request.command.clone(),
                cwd,
                started_at,
                last_used: now,
                transcript,
            };

            let mut store = self.session_store.lock().await;
            store.reserved_sessions_id.remove(&request.process_id);
            store.sessions.insert(request.process_id.clone(), entry);
        }

        Ok(UnifiedExecResponse {
            event_call_id: context.call_id.clone(),
            chunk_id: generate_chunk_id(),
            wall_time,
            output,
            raw_output: collected,
            process_id: if has_exited {
                None
            } else {
                Some(request.process_id)
            },
            exit_code,
            original_token_count,
        })
    }

    pub(crate) async fn write_stdin(
        &self,
        request: WriteStdinRequest<'_>,
    ) -> Result<UnifiedExecResponse, UnifiedExecError> {
        let (call_id, session) = {
            let store = self.session_store.lock().await;
            let Some(entry) = store.sessions.get(request.process_id) else {
                return Err(UnifiedExecError::UnknownProcessId {
                    process_id: request.process_id.to_string(),
                });
            };
            (entry.call_id.clone(), Arc::clone(&entry.session))
        };

        let writer = session.writer_sender();
        let data = request.input.as_bytes();
        if !data.is_empty() {
            Self::send_input(&writer, data).await?;
        }

        let (output_buffer, output_notify) = session.output_handles();
        let yield_time_ms = super::clamp_yield_time(request.yield_time_ms);
        let max_tokens = resolve_max_tokens(request.max_output_tokens);

        let started_at = Instant::now();
        let deadline = started_at + Duration::from_millis(yield_time_ms);
        let collected =
            Self::collect_output_until_deadline(&output_buffer, &output_notify, deadline).await;
        let wall_time = Instant::now().saturating_duration_since(started_at);

        let text = String::from_utf8_lossy(&collected).to_string();
        let (output, original_token_count) = truncate_output_to_tokens(&text, max_tokens);
        let exit_code = session.exit_code();
        let has_exited = session.has_exited() || exit_code.is_some();

        {
            let store = self.session_store.lock().await;
            if let Some(entry) = store.sessions.get(request.process_id) {
                let transcript = Arc::clone(&entry.transcript);
                drop(store);
                let mut guard = transcript.lock().await;
                guard.append(&collected);
            }
        }

        if has_exited {
            self.release_process_id(request.process_id).await;
        } else {
            let mut store = self.session_store.lock().await;
            if let Some(entry) = store.sessions.get_mut(request.process_id) {
                entry.last_used = Instant::now();
            }
        }

        Ok(UnifiedExecResponse {
            event_call_id: call_id,
            chunk_id: generate_chunk_id(),
            wall_time,
            output,
            raw_output: collected,
            process_id: if has_exited {
                None
            } else {
                Some(request.process_id.to_string())
            },
            exit_code,
            original_token_count,
        })
    }

    pub(crate) async fn open_session_with_exec_env(
        &self,
        env: &ExecEnv,
    ) -> Result<UnifiedExecSession, UnifiedExecError> {
        let (program, args) = env
            .command
            .split_first()
            .ok_or(UnifiedExecError::MissingCommandLine)?;

        let spawned = codex_utils_pty::spawn_pty_process(program, args, env.cwd.as_path(), &env.env)
            .await
            .map_err(|err| UnifiedExecError::create_session(err.to_string()))?;

        UnifiedExecSession::from_spawned(spawned, env.sandbox).await
    }

    async fn send_input(writer_tx: &mpsc::Sender<Vec<u8>>, data: &[u8]) -> Result<(), UnifiedExecError> {
        writer_tx
            .send(data.to_vec())
            .await
            .map_err(|_| UnifiedExecError::WriteToStdin)
    }

    async fn collect_output_until_deadline(
        output_buffer: &super::session::OutputBuffer,
        output_notify: &Arc<tokio::sync::Notify>,
        deadline: Instant,
    ) -> Vec<u8> {
        let mut collected: Vec<u8> = Vec::with_capacity(4096);
        loop {
            let drained_chunks;
            let mut wait_for_output = None;
            {
                let mut guard = output_buffer.lock().await;
                drained_chunks = guard.drain();
                if drained_chunks.is_empty() {
                    wait_for_output = Some(output_notify.notified());
                }
            }

            for chunk in drained_chunks {
                collected.extend_from_slice(&chunk);
            }

            if Instant::now() >= deadline {
                break;
            }

            if wait_for_output.is_some() {
                let remaining = deadline.saturating_duration_since(Instant::now());
                if remaining == Duration::ZERO {
                    break;
                }
                let notified = wait_for_output.take().unwrap_or_else(|| output_notify.notified());
                tokio::pin!(notified);
                tokio::select! {
                    _ = &mut notified => {}
                    _ = tokio::time::sleep(remaining) => break,
                }
            }
        }
        collected
    }
}

impl SessionStore {
    #[allow(dead_code)]
    pub(crate) async fn cleanup_exited(&mut self) {
        self.sessions.retain(|_, entry| !entry.session.has_exited());
    }
}


