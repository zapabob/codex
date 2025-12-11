/*
Runtime: shell

Executes shell requests under the orchestrator: asks for approval when needed,
builds a CommandSpec, and runs it under the current SandboxAttempt.
*/
use crate::command_safety::is_dangerous_command::requires_initial_appoval;
use crate::exec::ExecToolCallOutput;
<<<<<<< HEAD
use crate::protocol::SandboxPolicy;
=======
use crate::sandboxing::SandboxPermissions;
>>>>>>> upstream/main
use crate::sandboxing::execute_env;
use crate::tools::runtimes::build_command_spec;
use crate::tools::runtimes::maybe_wrap_shell_lc_with_snapshot;
use crate::tools::sandboxing::Approvable;
use crate::tools::sandboxing::ApprovalCtx;
<<<<<<< HEAD
use crate::tools::sandboxing::ProvidesSandboxRetryData;
use crate::tools::sandboxing::SandboxAttempt;
use crate::tools::sandboxing::SandboxRetryData;
=======
use crate::tools::sandboxing::ExecApprovalRequirement;
use crate::tools::sandboxing::SandboxAttempt;
use crate::tools::sandboxing::SandboxOverride;
>>>>>>> upstream/main
use crate::tools::sandboxing::Sandboxable;
use crate::tools::sandboxing::SandboxablePreference;
use crate::tools::sandboxing::ToolCtx;
use crate::tools::sandboxing::ToolError;
use crate::tools::sandboxing::ToolRuntime;
use crate::tools::sandboxing::with_cached_approval;
use codex_protocol::protocol::AskForApproval;
use codex_protocol::protocol::ReviewDecision;
use futures::future::BoxFuture;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct ShellRequest {
    pub command: Vec<String>,
    pub cwd: PathBuf,
    pub timeout_ms: Option<u64>,
    pub env: std::collections::HashMap<String, String>,
    pub sandbox_permissions: SandboxPermissions,
    pub justification: Option<String>,
}

#[derive(Default)]
pub struct ShellRuntime;

#[derive(serde::Serialize, Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct ApprovalKey {
    command: Vec<String>,
    cwd: PathBuf,
    sandbox_permissions: SandboxPermissions,
}

impl ShellRuntime {
    pub fn new() -> Self {
        Self
    }

    fn stdout_stream(ctx: &ToolCtx<'_>) -> Option<crate::exec::StdoutStream> {
        Some(crate::exec::StdoutStream {
            sub_id: ctx.turn.sub_id.clone(),
            call_id: ctx.call_id.clone(),
            tx_event: ctx.session.get_tx_event(),
        })
    }
}

impl Sandboxable for ShellRuntime {
    fn sandbox_preference(&self) -> SandboxablePreference {
        SandboxablePreference::Auto
    }
    fn escalate_on_failure(&self) -> bool {
        true
    }
}

impl Approvable<ShellRequest> for ShellRuntime {
    type ApprovalKey = ApprovalKey;

    fn approval_key(&self, req: &ShellRequest) -> Self::ApprovalKey {
        ApprovalKey {
            command: req.command.clone(),
            cwd: req.cwd.clone(),
            sandbox_permissions: req.sandbox_permissions,
        }
    }

    fn start_approval_async<'a>(
        &'a mut self,
        req: &'a ShellRequest,
        ctx: ApprovalCtx<'a>,
    ) -> BoxFuture<'a, ReviewDecision> {
        let key = self.approval_key(req);
        let command = req.command.clone();
        let cwd = req.cwd.clone();
        let reason = ctx
            .retry_reason
            .clone()
            .or_else(|| req.justification.clone());
        let session = ctx.session;
        let turn = ctx.turn;
        let call_id = ctx.call_id.to_string();
        Box::pin(async move {
            with_cached_approval(&session.services, key, move || async move {
                session
<<<<<<< HEAD
                    .request_command_approval(turn, call_id, command, cwd, reason, risk)
=======
                    .request_command_approval(
                        turn,
                        call_id,
                        command,
                        cwd,
                        reason,
                        req.exec_approval_requirement
                            .proposed_execpolicy_amendment()
                            .cloned(),
                    )
>>>>>>> upstream/main
                    .await
            })
            .await
        })
    }

    fn wants_initial_approval(
        &self,
        req: &ShellRequest,
        policy: AskForApproval,
        sandbox_policy: &SandboxPolicy,
    ) -> bool {
        requires_initial_appoval(
            policy,
            sandbox_policy,
            &req.command,
            req.with_escalated_permissions.unwrap_or(false),
        )
    }

<<<<<<< HEAD
    fn wants_escalated_first_attempt(&self, req: &ShellRequest) -> bool {
        req.with_escalated_permissions.unwrap_or(false)
=======
    fn sandbox_mode_for_first_attempt(&self, req: &ShellRequest) -> SandboxOverride {
        if req.sandbox_permissions.requires_escalated_permissions()
            || matches!(
                req.exec_approval_requirement,
                ExecApprovalRequirement::Skip {
                    bypass_sandbox: true,
                    ..
                }
            )
        {
            SandboxOverride::BypassSandboxFirstAttempt
        } else {
            SandboxOverride::NoOverride
        }
>>>>>>> upstream/main
    }
}

impl ToolRuntime<ShellRequest, ExecToolCallOutput> for ShellRuntime {
    async fn run(
        &mut self,
        req: &ShellRequest,
        attempt: &SandboxAttempt<'_>,
        ctx: &ToolCtx<'_>,
    ) -> Result<ExecToolCallOutput, ToolError> {
        let base_command = &req.command;
        let session_shell = ctx.session.user_shell();
        let command = maybe_wrap_shell_lc_with_snapshot(base_command, session_shell.as_ref());

        let spec = build_command_spec(
            &command,
            &req.cwd,
            &req.env,
<<<<<<< HEAD
            req.timeout_ms,
            req.with_escalated_permissions,
=======
            req.timeout_ms.into(),
            req.sandbox_permissions,
>>>>>>> upstream/main
            req.justification.clone(),
        )?;
        let env = attempt
            .env_for(&spec)
            .map_err(|err| ToolError::Codex(err.into()))?;
        let out = execute_env(&env, attempt.policy, Self::stdout_stream(ctx))
            .await
            .map_err(ToolError::Codex)?;
        Ok(out)
    }
}
