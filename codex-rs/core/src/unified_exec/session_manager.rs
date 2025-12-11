use std::sync::Arc;

use tokio::sync::Notify;
use tokio::sync::mpsc;
use tokio::time::Duration;
use tokio::time::Instant;

<<<<<<< HEAD
use crate::exec::ExecToolCallOutput;
use crate::exec::StreamOutput;
use crate::exec_env::create_env;
use crate::sandboxing::ExecEnv;
use crate::tools::events::ToolEmitter;
use crate::tools::events::ToolEventCtx;
use crate::tools::events::ToolEventStage;
=======
use crate::bash::extract_bash_command;
use crate::codex::Session;
use crate::codex::TurnContext;
use crate::exec_env::create_env;
use crate::exec_policy::create_exec_approval_requirement_for_command;
use crate::protocol::BackgroundEventEvent;
use crate::protocol::EventMsg;
use crate::sandboxing::ExecEnv;
use crate::sandboxing::SandboxPermissions;
>>>>>>> upstream/main
use crate::tools::orchestrator::ToolOrchestrator;
use crate::tools::runtimes::unified_exec::UnifiedExecRequest as UnifiedExecToolRequest;
use crate::tools::runtimes::unified_exec::UnifiedExecRuntime;
use crate::tools::sandboxing::ToolCtx;

use super::CommandTranscript;
use super::ExecCommandRequest;
use super::MIN_YIELD_TIME_MS;
use super::SessionEntry;
use super::UnifiedExecContext;
use super::UnifiedExecError;
use super::UnifiedExecResponse;
use super::UnifiedExecSessionManager;
use super::WriteStdinRequest;
use super::async_watcher::emit_exec_end_for_unified_exec;
use super::async_watcher::spawn_exit_watcher;
use super::async_watcher::start_streaming_output;
use super::clamp_yield_time;
use super::generate_chunk_id;
use super::resolve_max_tokens;
use super::session::OutputBuffer;
use super::session::UnifiedExecSession;
<<<<<<< HEAD
use super::truncate_output_to_tokens;
=======

const UNIFIED_EXEC_ENV: [(&str, &str); 8] = [
    ("NO_COLOR", "1"),
    ("TERM", "dumb"),
    ("LANG", "C.UTF-8"),
    ("LC_CTYPE", "C.UTF-8"),
    ("LC_ALL", "C.UTF-8"),
    ("COLORTERM", ""),
    ("PAGER", "cat"),
    ("GIT_PAGER", "cat"),
];

fn apply_unified_exec_env(mut env: HashMap<String, String>) -> HashMap<String, String> {
    for (key, value) in UNIFIED_EXEC_ENV {
        env.insert(key.to_string(), value.to_string());
    }
    env
}

struct PreparedSessionHandles {
    writer_tx: mpsc::Sender<Vec<u8>>,
    output_buffer: OutputBuffer,
    output_notify: Arc<Notify>,
    cancellation_token: CancellationToken,
    session_ref: Arc<Session>,
    turn_ref: Arc<TurnContext>,
    command: Vec<String>,
    process_id: String,
}
>>>>>>> upstream/main

impl UnifiedExecSessionManager {
    pub(crate) async fn exec_command(
        &self,
        request: ExecCommandRequest<'_>,
        context: &UnifiedExecContext,
    ) -> Result<UnifiedExecResponse, UnifiedExecError> {
        let shell_flag = if request.login { "-lc" } else { "-c" };
        let command = vec![
            request.shell.to_string(),
            shell_flag.to_string(),
            request.command.to_string(),
        ];

<<<<<<< HEAD
        let session = self.open_session_with_sandbox(command, context).await?;
=======
        let session = self
            .open_session_with_sandbox(
                &request.command,
                cwd.clone(),
                request.sandbox_permissions,
                request.justification,
                context,
            )
            .await;

        let session = match session {
            Ok(session) => Arc::new(session),
            Err(err) => {
                self.release_process_id(&request.process_id).await;
                return Err(err);
            }
        };
>>>>>>> upstream/main

        let transcript = Arc::new(tokio::sync::Mutex::new(CommandTranscript::default()));
        start_streaming_output(&session, context, Arc::clone(&transcript));

        let max_tokens = resolve_max_tokens(request.max_output_tokens);
        let yield_time_ms =
            clamp_yield_time(Some(request.yield_time_ms.unwrap_or(MIN_YIELD_TIME_MS)));

        let start = Instant::now();
<<<<<<< HEAD
        let (output_buffer, output_notify) = session.output_handles();
=======
        // For the initial exec_command call, we both stream output to events
        // (via start_streaming_output above) and collect a snapshot here for
        // the tool response body.
        let OutputHandles {
            output_buffer,
            output_notify,
            cancellation_token,
        } = session.output_handles();
>>>>>>> upstream/main
        let deadline = start + Duration::from_millis(yield_time_ms);
        let collected =
            Self::collect_output_until_deadline(&output_buffer, &output_notify, deadline).await;
        let wall_time = Instant::now().saturating_duration_since(start);

        let text = String::from_utf8_lossy(&collected).to_string();
<<<<<<< HEAD
        let (output, original_token_count) = truncate_output_to_tokens(&text, max_tokens);
        let chunk_id = generate_chunk_id();
        let exit_code = session.exit_code();
        let session_id = if session.has_exited() {
            None
        } else {
            Some(
                self.store_session(session, context, request.command, start)
                    .await,
=======
        let output = formatted_truncate_text(&text, TruncationPolicy::Tokens(max_tokens));
        let exit_code = session.exit_code();
        let has_exited = session.has_exited() || exit_code.is_some();
        let chunk_id = generate_chunk_id();
        let process_id = request.process_id.clone();
        if has_exited {
            // Short‑lived command: emit ExecCommandEnd immediately using the
            // same helper as the background watcher, so all end events share
            // one implementation.
            self.release_process_id(&request.process_id).await;
            let exit = exit_code.unwrap_or(-1);
            emit_exec_end_for_unified_exec(
                Arc::clone(&context.session),
                Arc::clone(&context.turn),
                context.call_id.clone(),
                request.command.clone(),
                cwd,
                Some(process_id),
                Arc::clone(&transcript),
                output.clone(),
                exit,
                wall_time,
            )
            .await;

            session.check_for_sandbox_denial_with_text(&text).await?;
        } else {
            // Long‑lived command: persist the session so write_stdin can reuse
            // it, and register a background watcher that will emit
            // ExecCommandEnd when the PTY eventually exits (even if no further
            // tool calls are made).
            self.store_session(
                Arc::clone(&session),
                context,
                &request.command,
                cwd.clone(),
                start,
                process_id,
                Arc::clone(&transcript),
>>>>>>> upstream/main
            )
        };

        let response = UnifiedExecResponse {
            event_call_id: context.call_id.clone(),
            chunk_id,
            wall_time,
            output,
<<<<<<< HEAD
            session_id,
=======
            raw_output: collected,
            process_id: if has_exited {
                None
            } else {
                Some(request.process_id.clone())
            },
>>>>>>> upstream/main
            exit_code,
            original_token_count,
        };

        // If the command completed during this call, emit an ExecCommandEnd via the emitter.
        if response.session_id.is_none() {
            let exit = response.exit_code.unwrap_or(-1);
            Self::emit_exec_end_from_context(
                context,
                request.command.to_string(),
                response.output.clone(),
                exit,
                response.wall_time,
            )
            .await;
        }

        Ok(response)
    }

    pub(crate) async fn write_stdin(
        &self,
        request: WriteStdinRequest<'_>,
    ) -> Result<UnifiedExecResponse, UnifiedExecError> {
        let session_id = request.session_id;

<<<<<<< HEAD
        let (writer_tx, output_buffer, output_notify) =
            self.prepare_session_handles(session_id).await?;

        if !request.input.is_empty() {
            Self::send_input(&writer_tx, request.input.as_bytes()).await?;
=======
        let PreparedSessionHandles {
            writer_tx,
            output_buffer,
            output_notify,
            cancellation_token,
            session_ref,
            turn_ref,
            command: session_command,
            process_id,
            ..
        } = self.prepare_session_handles(process_id.as_str()).await?;

        if !request.input.is_empty() {
            Self::send_input(&writer_tx, request.input.as_bytes()).await?;
            // Give the remote process a brief window to react so that we are
            // more likely to capture its output in the poll below.
>>>>>>> upstream/main
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        let max_tokens = resolve_max_tokens(request.max_output_tokens);
        let yield_time_ms = clamp_yield_time(request.yield_time_ms);
        let start = Instant::now();
        let deadline = start + Duration::from_millis(yield_time_ms);
        let collected =
            Self::collect_output_until_deadline(&output_buffer, &output_notify, deadline).await;
        let wall_time = Instant::now().saturating_duration_since(start);

        let text = String::from_utf8_lossy(&collected).to_string();
        let (output, original_token_count) = truncate_output_to_tokens(&text, max_tokens);
        let chunk_id = generate_chunk_id();

<<<<<<< HEAD
        let status = self.refresh_session_state(session_id).await;
        let (session_id, exit_code, completion_entry, event_call_id) = match status {
            SessionStatus::Alive { exit_code, call_id } => {
                (Some(session_id), exit_code, None, call_id)
            }
=======
        // After polling, refresh_session_state tells us whether the PTY is
        // still alive or has exited and been removed from the store; we thread
        // that through so the handler can tag TerminalInteraction with an
        // appropriate process_id and exit_code.
        let status = self.refresh_session_state(process_id.as_str()).await;
        let (process_id, exit_code, event_call_id) = match status {
            SessionStatus::Alive {
                exit_code,
                call_id,
                process_id,
            } => (Some(process_id), exit_code, call_id),
>>>>>>> upstream/main
            SessionStatus::Exited { exit_code, entry } => {
                let call_id = entry.call_id.clone();
                (None, exit_code, call_id)
            }
            SessionStatus::Unknown => {
                return Err(UnifiedExecError::UnknownSessionId { session_id });
            }
        };

        let response = UnifiedExecResponse {
            event_call_id,
            chunk_id,
            wall_time,
            output,
<<<<<<< HEAD
            session_id,
=======
            raw_output: collected,
            process_id,
>>>>>>> upstream/main
            exit_code,
            original_token_count,
        };

<<<<<<< HEAD
        if let (Some(exit), Some(entry)) = (response.exit_code, completion_entry) {
            let total_duration = Instant::now().saturating_duration_since(entry.started_at);
            Self::emit_exec_end_from_entry(entry, response.output.clone(), exit, total_duration)
                .await;
=======
        if response.process_id.is_some() {
            Self::emit_waiting_status(&session_ref, &turn_ref, &session_command).await;
>>>>>>> upstream/main
        }

        Ok(response)
    }

    async fn refresh_session_state(&self, session_id: i32) -> SessionStatus {
        let mut sessions = self.sessions.lock().await;
        let Some(entry) = sessions.get(&session_id) else {
            return SessionStatus::Unknown;
        };

        let exit_code = entry.session.exit_code();

        if entry.session.has_exited() {
            let Some(entry) = sessions.remove(&session_id) else {
                return SessionStatus::Unknown;
            };
            SessionStatus::Exited {
                exit_code,
                entry: Box::new(entry),
            }
        } else {
            SessionStatus::Alive {
                exit_code,
                call_id: entry.call_id.clone(),
            }
        }
    }

    async fn prepare_session_handles(
        &self,
        session_id: i32,
    ) -> Result<(mpsc::Sender<Vec<u8>>, OutputBuffer, Arc<Notify>), UnifiedExecError> {
        let sessions = self.sessions.lock().await;
        let (output_buffer, output_notify, writer_tx) =
            if let Some(entry) = sessions.get(&session_id) {
                let (buffer, notify) = entry.session.output_handles();
                (buffer, notify, entry.session.writer_sender())
            } else {
                return Err(UnifiedExecError::UnknownSessionId { session_id });
            };

<<<<<<< HEAD
        Ok((writer_tx, output_buffer, output_notify))
=======
        Ok(PreparedSessionHandles {
            writer_tx: entry.session.writer_sender(),
            output_buffer,
            output_notify,
            cancellation_token,
            session_ref: Arc::clone(&entry.session_ref),
            turn_ref: Arc::clone(&entry.turn_ref),
            command: entry.command.clone(),
            process_id: entry.process_id.clone(),
        })
>>>>>>> upstream/main
    }

    async fn send_input(
        writer_tx: &mpsc::Sender<Vec<u8>>,
        data: &[u8],
    ) -> Result<(), UnifiedExecError> {
        writer_tx
            .send(data.to_vec())
            .await
            .map_err(|_| UnifiedExecError::WriteToStdin)
    }

    async fn store_session(
        &self,
        session: Arc<UnifiedExecSession>,
        context: &UnifiedExecContext,
        command: &str,
        started_at: Instant,
<<<<<<< HEAD
    ) -> i32 {
        let session_id = self
            .next_session_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
=======
        process_id: String,
        transcript: Arc<tokio::sync::Mutex<CommandTranscript>>,
    ) {
>>>>>>> upstream/main
        let entry = SessionEntry {
            session: Arc::clone(&session),
            session_ref: Arc::clone(&context.session),
            turn_ref: Arc::clone(&context.turn),
            call_id: context.call_id.clone(),
<<<<<<< HEAD
            command: command.to_string(),
            cwd: context.turn.cwd.clone(),
            started_at,
        };
        self.sessions.lock().await.insert(session_id, entry);
        session_id
    }

    async fn emit_exec_end_from_entry(
        entry: SessionEntry,
        aggregated_output: String,
        exit_code: i32,
        duration: Duration,
    ) {
        let output = ExecToolCallOutput {
            exit_code,
            stdout: StreamOutput::new(aggregated_output.clone()),
            stderr: StreamOutput::new(String::new()),
            aggregated_output: StreamOutput::new(aggregated_output),
            duration,
            timed_out: false,
        };
        let event_ctx = ToolEventCtx::new(
            entry.session_ref.as_ref(),
            entry.turn_ref.as_ref(),
            &entry.call_id,
            None,
        );
        let emitter = ToolEmitter::unified_exec(entry.command, entry.cwd, true);
        emitter
            .emit(event_ctx, ToolEventStage::Success(output))
            .await;
    }

    async fn emit_exec_end_from_context(
        context: &UnifiedExecContext,
        command: String,
        aggregated_output: String,
        exit_code: i32,
        duration: Duration,
    ) {
        let output = ExecToolCallOutput {
            exit_code,
            stdout: StreamOutput::new(aggregated_output.clone()),
            stderr: StreamOutput::new(String::new()),
            aggregated_output: StreamOutput::new(aggregated_output),
            duration,
            timed_out: false,
        };
        let event_ctx = ToolEventCtx::new(
            context.session.as_ref(),
            context.turn.as_ref(),
            &context.call_id,
            None,
        );
        let emitter = ToolEmitter::unified_exec(command, context.turn.cwd.clone(), true);
        emitter
            .emit(event_ctx, ToolEventStage::Success(output))
            .await;
=======
            process_id: process_id.clone(),
            command: command.to_vec(),
            last_used: started_at,
        };
        let number_sessions = {
            let mut store = self.session_store.lock().await;
            Self::prune_sessions_if_needed(&mut store);
            store.sessions.insert(process_id.clone(), entry);
            store.sessions.len()
        };

        if number_sessions >= WARNING_UNIFIED_EXEC_SESSIONS {
            context
                .session
                .record_model_warning(
                    format!("The maximum number of unified exec sessions you can keep open is {WARNING_UNIFIED_EXEC_SESSIONS} and you currently have {number_sessions} sessions open. Reuse older sessions or close them to prevent automatic pruning of old session"),
                    &context.turn
                )
                .await;
        };

        spawn_exit_watcher(
            Arc::clone(&session),
            Arc::clone(&context.session),
            Arc::clone(&context.turn),
            context.call_id.clone(),
            command.to_vec(),
            cwd,
            process_id,
            transcript,
            started_at,
        );
>>>>>>> upstream/main
    }

    pub(crate) async fn open_session_with_exec_env(
        &self,
        env: &ExecEnv,
    ) -> Result<UnifiedExecSession, UnifiedExecError> {
        let (program, args) = env
            .command
            .split_first()
            .ok_or(UnifiedExecError::MissingCommandLine)?;
        let spawned =
            codex_utils_pty::spawn_pty_process(program, args, env.cwd.as_path(), &env.env)
                .await
                .map_err(|err| UnifiedExecError::create_session(err.to_string()))?;
        UnifiedExecSession::from_spawned(spawned, env.sandbox).await
    }

    pub(super) async fn open_session_with_sandbox(
        &self,
<<<<<<< HEAD
        command: Vec<String>,
=======
        command: &[String],
        cwd: PathBuf,
        sandbox_permissions: SandboxPermissions,
        justification: Option<String>,
>>>>>>> upstream/main
        context: &UnifiedExecContext,
    ) -> Result<UnifiedExecSession, UnifiedExecError> {
        let mut orchestrator = ToolOrchestrator::new();
        let mut runtime = UnifiedExecRuntime::new(self);
<<<<<<< HEAD
        let req = UnifiedExecToolRequest::new(
            command,
            context.turn.cwd.clone(),
            create_env(&context.turn.shell_environment_policy),
=======
        let exec_approval_requirement = create_exec_approval_requirement_for_command(
            &context.turn.exec_policy,
            &features,
            command,
            context.turn.approval_policy,
            &context.turn.sandbox_policy,
            sandbox_permissions,
        )
        .await;
        let req = UnifiedExecToolRequest::new(
            command.to_vec(),
            cwd,
            env,
            sandbox_permissions,
            justification,
            exec_approval_requirement,
>>>>>>> upstream/main
        );
        let tool_ctx = ToolCtx {
            session: context.session.as_ref(),
            turn: context.turn.as_ref(),
            call_id: context.call_id.clone(),
            tool_name: "exec_command".to_string(),
        };
        orchestrator
            .run(
                &mut runtime,
                &req,
                &tool_ctx,
                context.turn.as_ref(),
                context.turn.approval_policy,
            )
            .await
            .map_err(|e| UnifiedExecError::create_session(format!("{e:?}")))
    }

    pub(super) async fn collect_output_until_deadline(
        output_buffer: &OutputBuffer,
        output_notify: &Arc<Notify>,
        deadline: Instant,
    ) -> Vec<u8> {
<<<<<<< HEAD
=======
        const POST_EXIT_OUTPUT_GRACE: Duration = Duration::from_millis(50);

>>>>>>> upstream/main
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

            if drained_chunks.is_empty() {
                let remaining = deadline.saturating_duration_since(Instant::now());
                if remaining == Duration::ZERO {
                    break;
                }

                let notified = wait_for_output.unwrap_or_else(|| output_notify.notified());
                tokio::pin!(notified);
                tokio::select! {
                    _ = &mut notified => {}
                    _ = tokio::time::sleep(remaining) => break,
                }
                continue;
            }

            for chunk in drained_chunks {
                collected.extend_from_slice(&chunk);
            }

            if Instant::now() >= deadline {
                break;
            }
        }

        collected
    }
<<<<<<< HEAD
=======

    fn prune_sessions_if_needed(store: &mut SessionStore) -> bool {
        if store.sessions.len() < MAX_UNIFIED_EXEC_SESSIONS {
            return false;
        }

        let meta: Vec<(String, Instant, bool)> = store
            .sessions
            .iter()
            .map(|(id, entry)| (id.clone(), entry.last_used, entry.session.has_exited()))
            .collect();

        if let Some(session_id) = Self::session_id_to_prune_from_meta(&meta) {
            if let Some(entry) = store.remove(&session_id) {
                entry.session.terminate();
            }
            return true;
        }

        false
    }

    // Centralized pruning policy so we can easily swap strategies later.
    fn session_id_to_prune_from_meta(meta: &[(String, Instant, bool)]) -> Option<String> {
        if meta.is_empty() {
            return None;
        }

        let mut by_recency = meta.to_vec();
        by_recency.sort_by_key(|(_, last_used, _)| Reverse(*last_used));
        let protected: HashSet<String> = by_recency
            .iter()
            .take(8)
            .map(|(process_id, _, _)| process_id.clone())
            .collect();

        let mut lru = meta.to_vec();
        lru.sort_by_key(|(_, last_used, _)| *last_used);

        if let Some((process_id, _, _)) = lru
            .iter()
            .find(|(process_id, _, exited)| !protected.contains(process_id) && *exited)
        {
            return Some(process_id.clone());
        }

        lru.into_iter()
            .find(|(process_id, _, _)| !protected.contains(process_id))
            .map(|(process_id, _, _)| process_id)
    }

    pub(crate) async fn terminate_all_sessions(&self) {
        let entries: Vec<SessionEntry> = {
            let mut sessions = self.session_store.lock().await;
            let entries: Vec<SessionEntry> =
                sessions.sessions.drain().map(|(_, entry)| entry).collect();
            sessions.reserved_sessions_id.clear();
            entries
        };

        for entry in entries {
            entry.session.terminate();
        }
    }
>>>>>>> upstream/main
}

enum SessionStatus {
    Alive {
        exit_code: Option<i32>,
        call_id: String,
    },
    Exited {
        exit_code: Option<i32>,
        entry: Box<SessionEntry>,
    },
    Unknown,
}
