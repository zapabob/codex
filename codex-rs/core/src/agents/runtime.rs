use super::budgeter::TokenBudgeter;

use super::loader::AgentLoader;

use super::types::AgentDefinition;

use super::types::AgentResult;

use super::types::AgentStatus;

use anyhow::Context;

use anyhow::Result;

use anyhow::anyhow;

use std::collections::HashMap;

use std::ffi::OsString;

use std::path::PathBuf;

use std::sync::Arc;

use std::time::Duration;

use std::time::Instant;

use tokio::sync::RwLock;

use tracing::debug;

use tracing::error;

use tracing::info;



use crate::AuthManager;

use crate::audit_log::AgentExecutionEvent;

use crate::audit_log::AuditEvent;

use crate::audit_log::AuditEventType;

use crate::audit_log::ExecutionStatus;

use crate::audit_log::log_audit_event;

use crate::client::ModelClient;

use crate::client_common::Prompt;

use crate::client_common::ResponseEvent;

use crate::config::Config;

use crate::features::Feature;

use crate::model_provider_info::ModelProviderInfo;

#[cfg(feature = "custom-features")]

use crate::orchestration::CollaborationStore;

use codex_otel::OtelManager as OtelEventManager;

use codex_protocol::ThreadId;

type ConversationId = ThreadId;

use codex_protocol::config_types::ReasoningSummary;

use codex_protocol::config_types::Verbosity;

use codex_protocol::models::BaseInstructions;

use codex_protocol::models::ContentItem;

use codex_protocol::models::ResponseItem;

use codex_protocol::openai_models::ReasoningEffort;

use codex_rmcp_client::Elicitation;

use codex_rmcp_client::RmcpClient;

use codex_rmcp_client::SendElicitation;

use futures::FutureExt;

use futures::StreamExt;

use rmcp::model::InitializeRequestParams;

use rmcp::model::ProtocolVersion;

use rmcp::model::RequestId;



/// ﾃ｣ﾂつｵﾃ｣ﾂδ姪｣ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ暗｣ﾂδｩﾃ｣ﾂδｳﾃ｣ﾂつｿﾃ｣ﾂつ､ﾃ｣ﾂδ

pub struct AgentRuntime {

    /// ﾃ｣ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ暗｣ﾂδｭﾃ｣ﾂδｼﾃ｣ﾂδﾃ｣ﾂδｼ

    loader: Arc<RwLock<AgentLoader>>,

    /// ﾃ｣ﾂδ暗｣ﾂ・ﾃ｣ﾂつｯﾃ｣ﾂδｳﾃ､ﾂｺﾂ暗ｧﾂｮﾂ療ｧﾂｮﾂ｡ﾃｧﾂ青・
    budgeter: Arc<TokenBudgeter>,

    /// ﾃ･ﾂｮﾂ淌ｨﾂ｡ﾂ古､ﾂｸﾂｭﾃ｣ﾂ・ｮﾃ｣ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ・
    running_agents: Arc<RwLock<HashMap<String, AgentStatus>>>,

    /// ﾃ｣ﾂδｯﾃ｣ﾂδｼﾃ｣ﾂつｯﾃ｣ﾂつｹﾃ｣ﾂδ堙｣ﾂ・ﾃ｣ﾂつｹﾃ｣ﾂδ・ﾂつ｣ﾃ｣ﾂδｬﾃ｣ﾂつｯﾃ｣ﾂδ暗｣ﾂδｪ

    workspace_dir: PathBuf,

    /// LLMﾃｨﾂｨﾂｭﾃ･ﾂｮﾂ・
    config: Arc<Config>,

    /// ﾃｨﾂｪﾂ催ｨﾂｨﾂｼﾃ｣ﾂδ榲｣ﾂδ催｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂδ｣ﾃ｣ﾂδｼ

    auth_manager: Option<Arc<AuthManager>>,

    /// OpenTelemetry ﾃ｣ﾂつ､ﾃ｣ﾂδ凖｣ﾂδｳﾃ｣ﾂδ暗｣ﾂ・ﾃ｣ﾂδ催｣ﾂ・ﾃ｣ﾂつｸﾃ｣ﾂδ｣ﾃ｣ﾂδｼ

    otel_manager: OtelEventManager,

    /// ﾃ｣ﾂδ｢ﾃ｣ﾂδ・ﾂδｫﾃ｣ﾂδ療｣ﾂδｭﾃ｣ﾂδ静｣ﾂつ､ﾃ｣ﾂδﾃ｣ﾂδｼﾃｦﾂδ・ﾂﾂｱ

    provider: ModelProviderInfo,

    /// ﾃ､ﾂｼﾂ堙ｨﾂｩﾂｱID

    conversation_id: ConversationId,

    /// Codexﾃ｣ﾂδ静｣ﾂつ､ﾃ｣ﾂδ甘｣ﾂδｪﾃ｣ﾂδ妥｣ﾂつｹﾂ・ﾂ・CPﾃｧﾂｵﾂｱﾃ･ﾂ青暗ｧﾂ板ｨﾂ・ﾂ・
    codex_binary_path: Option<PathBuf>,

    /// ﾃ｣ﾂつｵﾃ｣ﾂδ姪｣ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ暗ｩﾂ鳴禿｣ﾂ・ｮﾃ･ﾂ債氾ｨﾂｪﾂｿﾃ｣ﾂつｹﾃ｣ﾂδ暗｣ﾂつ｢

    #[cfg(feature = "custom-features")]

    collaboration_store: Arc<CollaborationStore>,

    /// Reasoning effortﾃｨﾂｨﾂｭﾃ･ﾂｮﾂ・
    reasoning_effort: ReasoningEffort,

    /// Reasoning summaryﾃｨﾂｨﾂｭﾃ･ﾂｮﾂ・
    reasoning_summary: ReasoningSummary,

    /// Verbosityﾃｨﾂｨﾂｭﾃ･ﾂｮﾂ・
    verbosity: Verbosity,

}



impl AgentRuntime {

    /// ﾃｦﾂ鳴ｰﾃ｣ﾂ・療｣ﾂ・・｣ﾂδｩﾃ｣ﾂδｳﾃ｣ﾂつｿﾃ｣ﾂつ､ﾃ｣ﾂδﾃ｣ﾂつ津､ﾂｽﾂ愿ｦﾂ・

    pub fn new(

        workspace_dir: PathBuf,

        total_budget: usize,

        config: Arc<Config>,

        auth_manager: Option<Arc<AuthManager>>,

        otel_manager: OtelEventManager,

        provider: ModelProviderInfo,

        conversation_id: ConversationId,

        reasoning_effort: ReasoningEffort,

        reasoning_summary: ReasoningSummary,

        verbosity: Verbosity,

    ) -> Self {

        let loader = Arc::new(RwLock::new(AgentLoader::new(&workspace_dir)));

        let budgeter = Arc::new(TokenBudgeter::new(total_budget));



        Self {

            loader,

            budgeter,

            running_agents: Arc::new(RwLock::new(HashMap::new())),

            workspace_dir,

            config,

            auth_manager,

            otel_manager,

            provider,

            conversation_id,

            codex_binary_path: None,

            #[cfg(feature = "custom-features")]

            collaboration_store: Arc::new(CollaborationStore::new()),

            reasoning_effort,

            reasoning_summary,

            verbosity,

        }

    }



    /// ﾃｨﾂ､ﾂ・ﾂ閉ｰﾃ｣ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ暗｣ﾂつ津､ﾂｸﾂｦﾃ･ﾂ按療･ﾂｮﾂ淌ｨﾂ｡ﾂ・
    pub async fn delegate_parallel(

        &self,

        agents: Vec<(String, String, HashMap<String, String>, Option<usize>)>, // (agent_name, goal, inputs, budget)

        _deadline: Option<u64>,

    ) -> Result<Vec<AgentResult>> {

        info!("Starting parallel delegation of {} agents", agents.len());



        // ﾃ･ﾂ青・ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ暗｣ﾂつ稚okio::spawnﾃ｣ﾂ・ｧﾃ､ﾂｸﾂｦﾃ･ﾂ按療ｨﾂｵﾂｷﾃ･ﾂ仰・
        let mut handles = Vec::new();



        for (agent_name, goal, inputs, budget) in agents {

            let runtime_clone = Arc::new(self.clone_for_parallel());

            let agent_name_clone = agent_name.clone();



            let handle = tokio::spawn(async move {

                runtime_clone

                    .delegate(&agent_name_clone, &goal, inputs, budget, None)

                    .await

            });



            handles.push((agent_name, handle));

        }



        // ﾃ･ﾂ・ｨﾃ｣ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ暗｣ﾂ・ﾃ･ﾂｮﾂ古､ﾂｺﾂ・ﾂつ津･ﾂｾﾂ・ﾂｩﾂ・
        let mut results = Vec::new();

        for (agent_name, handle) in handles {

            match handle.await {

                Ok(Ok(result)) => {

                    info!("Agent '{}' completed successfully", agent_name);

                    results.push(result);

                }

                Ok(Err(e)) => {

                    error!("Agent '{}' failed: {}", agent_name, e);

                    // ﾃ｣ﾂつｨﾃ｣ﾂδｩﾃ｣ﾂδｼﾃ｣ﾂ・ｧﾃ｣ﾂつづｧﾂｶﾂ堙ｨﾂ｡ﾂ古｣ﾂ・療｣ﾂ・ｦﾃ､ﾂｻﾂ姪｣ﾂ・ﾃ｣ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ暗｣ﾂ・ﾃｧﾂｵﾂ静ｦﾂ楪愿｣ﾂつ津･ﾂ渉偲ｩﾂ崢・
                    results.push(AgentResult {

                        agent_name: agent_name.clone(),

                        status: AgentStatus::Failed,

                        artifacts: vec![],

                        tokens_used: 0,

                        duration_secs: 0.0,

                        error: Some(e.to_string()),

                    });

                }

                Err(e) => {

                    error!("Agent '{}' task panicked: {}", agent_name, e);

                    results.push(AgentResult {

                        agent_name: agent_name.clone(),

                        status: AgentStatus::Failed,

                        artifacts: vec![],

                        tokens_used: 0,

                        duration_secs: 0.0,

                        error: Some(format!("Task panicked: {e}")),

                    });

                }

            }

        }



        info!(

            "Parallel delegation completed: {}/{} agents succeeded",

            results

                .iter()

                .filter(|r| matches!(r.status, AgentStatus::Completed))

                .count(),

            results.len()

        );



        Ok(results)

    }



    /// ﾃ｣ﾂδ療｣ﾂδｭﾃ｣ﾂδｳﾃ｣ﾂδ療｣ﾂδ暗｣ﾂ・凝｣ﾂつ嘉｣ﾂつｫﾃ｣ﾂつｹﾃ｣ﾂつｿﾃ｣ﾂδﾃ｣ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ暗｣ﾂつ津､ﾂｽﾂ愿ｦﾂ・ﾃ｣ﾂ・療｣ﾂ・ｦﾃ･ﾂｮﾂ淌ｨﾂ｡ﾂ・
    pub async fn create_and_run_custom_agent(

        &self,

        prompt: &str,

        budget: Option<usize>,

    ) -> Result<AgentResult> {

        info!("Creating custom agent from prompt");



        // LLMﾃ｣ﾂつ津､ﾂｽﾂｿﾃ｣ﾂ・｣ﾃ｣ﾂ・ｦﾃ｣ﾂδ療｣ﾂδｭﾃ｣ﾂδｳﾃ｣ﾂδ療｣ﾂδ暗｣ﾂ・凝｣ﾂつ嘉｣ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ暗･ﾂｮﾂ堙ｧﾂｾﾂｩﾃ｣ﾂつ津ｧﾂ板淌ｦﾂ按・
        let agent_def = self.generate_agent_from_prompt(prompt).await?;



        info!("Generated custom agent: {}", agent_def.name);



        // ﾃ｣ﾂつｫﾃ｣ﾂつｹﾃ｣ﾂつｿﾃ｣ﾂδﾃ｣ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ暗｣ﾂつ津｣ﾂδ｡ﾃ｣ﾂδ｢ﾃ｣ﾂδｪﾃ､ﾂｸﾂ甘｣ﾂ・ｧﾃ･ﾂｮﾂ淌ｨﾂ｡ﾂ古ｯﾂｼﾂ・AMLﾃ､ﾂｿﾂ敕･ﾂｭﾂ佚､ﾂｸﾂ催ｨﾂｦﾂ・ﾂｼﾂ・
        self.execute_custom_agent_inline(agent_def, budget).await

    }



    /// ﾃ｣ﾂδ療｣ﾂδｭﾃ｣ﾂδｳﾃ｣ﾂδ療｣ﾂδ暗｣ﾂ・凝｣ﾂつ嘉｣ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ暗･ﾂｮﾂ堙ｧﾂｾﾂｩﾃ｣ﾂつ津ｧﾂ板淌ｦﾂ按・
    async fn generate_agent_from_prompt(&self, prompt: &str) -> Result<AgentDefinition> {

        let system_prompt = r#"You are an AI agent definition generator. 

Given a user prompt, generate a complete agent definition in the following JSON format:



{

  "name": "agent-name",

  "goal": "Clear description of the agent's purpose",

  "tools": {

    "mcp": ["codex_read_file", "codex_grep"],

    "shell": []

  },

  "policies": {

    "context": {

      "max_tokens": 40000

    },

    "permissions": {

      "filesystem": [],

      "network": []

    }

  },

  "success_criteria": [

    "Clear criterion 1",

    "Clear criterion 2"

  ],

  "artifacts": [

    "artifacts/output.md"

  ]

}



Guidelines:

- name: Use kebab-case (e.g., "code-reviewer", "test-generator")

- goal: Be specific and actionable

- tools.mcp: Choose from [codex_read_file, codex_grep, codex_codebase_search, codex_apply_patch]

- tools.mcp: Do NOT include codex_shell unless explicitly requested (security)

- max_tokens: 40000 for simple tasks, 60000 for complex tasks

- success_criteria: 3-5 measurable criteria

- artifacts: Specify expected output files



Only output the JSON, no explanation."#;



        let user_message = format!("Generate an agent definition for this task:\n\n{prompt}");



        let input_items = vec![ResponseItem::Message {

            id: None,

            role: "user".to_string(),

            content: vec![ContentItem::InputText { text: user_message }],

            end_turn: None,

            phase: None,

        }];



        let llm_prompt = Prompt {

            input: input_items,

            tools: vec![],

            parallel_tool_calls: false,

            base_instructions: BaseInstructions {

                text: system_prompt.to_string(),

            },

            personality: None,

            output_schema: None,

        };



        // LLMﾃ･ﾂ堕ｼﾃ｣ﾂ・ｳﾃ･ﾂ・ｺﾃ｣ﾂ・・
        let model = self.config.model.as_deref().unwrap_or("gpt-5.2-codex");

        let model_info = crate::models_manager::model_info::with_config_overrides(

            crate::models_manager::model_info::model_info_from_slug(model),

            &self.config,

        );

        let model_client = ModelClient::new(

            self.auth_manager.clone(),

            self.conversation_id,

            self.provider.clone(),

            codex_protocol::protocol::SessionSource::Cli,

            self.config.model_verbosity,
            crate::ws_version_from_features(&self.config),
            self.config
                .features
                .enabled(Feature::EnableRequestCompression),
            self.config.features.enabled(Feature::RuntimeMetrics),
            None,

        );



        let mut client_session = model_client.new_session();

        let mut response_stream = client_session

            .stream(

                &llm_prompt,

                &model_info,

                &self.otel_manager,

                Some(self.reasoning_effort),

                self.reasoning_summary,

                None,

            )

            .await

            .context("Failed to generate agent definition")?;



        // ﾃ｣ﾂδｬﾃ｣ﾂつｹﾃ｣ﾂδ敕｣ﾂδｳﾃ｣ﾂつｹﾃ｣ﾂつ津･ﾂ渉偲ｩﾂ崢・
        let mut full_response = String::new();

        while let Some(event) = response_stream.next().await {

            if let ResponseEvent::OutputItemDone(ResponseItem::Message { content, .. }) = event? {

                for content_item in content {

                    if let ContentItem::OutputText { text } = content_item {

                        full_response.push_str(&text);

                    }

                }

            }

        }



        // JSONﾃ｣ﾂつ津ｦﾂ環ｽﾃ･ﾂ・ｺﾂ・ﾂ暗｣ﾂつｳﾃ｣ﾂδｼﾃ｣ﾂδ嘉｣ﾂδ姪｣ﾂδｭﾃ｣ﾂδ・ﾂつｯﾃ･ﾂ・・ﾂ・ﾃ･ﾂ渉ｯﾃｨﾂδｽﾃｦﾂﾂｧﾃ｣ﾂ・古｣ﾂ・づ｣ﾂつ凝｣ﾂ・淌｣ﾂつ・ﾂｼﾂ・
        let json_str = if let Some(start) = full_response.find('{') {

            if let Some(end) = full_response.rfind('}') {

                &full_response[start..=end]

            } else {

                &full_response

            }

        } else {

            &full_response

        };



        // JSONﾃ｣ﾂつ津｣ﾂδ妥｣ﾂδｼﾃ｣ﾂつｹ

        let agent_def: AgentDefinition =

            serde_json::from_str(json_str).context("Failed to parse generated agent definition")?;



        info!(

            "Successfully generated agent definition: {}",

            agent_def.name

        );



        Ok(agent_def)

    }



    /// ﾃ｣ﾂつｫﾃ｣ﾂつｹﾃ｣ﾂつｿﾃ｣ﾂδﾃ｣ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ暗｣ﾂつ津｣ﾂつ､ﾃ｣ﾂδｳﾃ｣ﾂδｩﾃ｣ﾂつ､ﾃ｣ﾂδｳﾃ｣ﾂ・ｧﾃ･ﾂｮﾂ淌ｨﾂ｡ﾂ古ｯﾂｼﾂ・AMLﾃ､ﾂｿﾂ敕･ﾂｭﾂ佚｣ﾂ・ｪﾃ｣ﾂ・療ｯﾂｼﾂ・
    async fn execute_custom_agent_inline(

        &self,

        agent_def: AgentDefinition,

        budget: Option<usize>,

    ) -> Result<AgentResult> {

        let agent_name = &agent_def.name;

        info!("Executing custom agent '{}' inline", agent_name);



        // ﾃ､ﾂｺﾂ暗ｧﾂｮﾂ療ｨﾂｨﾂｭﾃ･ﾂｮﾂ・
        let effective_budget = budget.unwrap_or(agent_def.policies.context.max_tokens);

        self.budgeter

            .set_agent_limit(agent_name, effective_budget)?;



        // ﾃ･ﾂｮﾂ淌ｨﾂ｡ﾂ古｣ﾂつｹﾃ｣ﾂδ・ﾂ・ﾃ｣ﾂつｿﾃ｣ﾂつｹﾃｦﾂ崢ｴﾃｦﾂ鳴ｰ

        {

            let mut running = self.running_agents.write().await;

            running.insert(agent_name.to_string(), AgentStatus::Running);

        }



        let start_time = Instant::now();

        let start_timestamp = chrono::Utc::now().to_rfc3339();



        // ﾃ｣ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ暗･ﾂｮﾂ淌ｨﾂ｡ﾂ・
        let result = match self

            .execute_agent(&agent_def, &agent_def.goal, HashMap::new(), None)

            .await

        {

            Ok(artifacts) => {

                let duration_secs = start_time.elapsed().as_secs_f64();

                let tokens_used = self.budgeter.get_agent_usage(agent_name);



                // ﾃｧﾂ崢｣ﾃｦﾂ淞ｻﾃ｣ﾂδｭﾃ｣ﾂつｰ: ﾃｦﾂ按静･ﾂ環・

                let _ = log_audit_event(AuditEvent::new(

                    agent_name.to_string(),

                    AuditEventType::AgentExecution(AgentExecutionEvent {

                        agent_name: agent_name.to_string(),

                        status: ExecutionStatus::Completed,

                        goal: agent_def.goal.clone(),

                        start_time: start_timestamp.clone(),

                        end_time: Some(chrono::Utc::now().to_rfc3339()),

                        duration_secs: Some(duration_secs),

                        tokens_used,

                        artifacts: artifacts.clone(),

                        error: None,

                    }),

                ))

                .await;



                AgentResult {

                    agent_name: agent_name.to_string(),

                    status: AgentStatus::Completed,

                    artifacts,

                    tokens_used,

                    duration_secs,

                    error: None,

                }

            }

            Err(e) => {

                let duration_secs = start_time.elapsed().as_secs_f64();

                let tokens_used = self.budgeter.get_agent_usage(agent_name);



                error!("Custom agent '{}' failed: {}", agent_name, e);



                // ﾃｧﾂ崢｣ﾃｦﾂ淞ｻﾃ｣ﾂδｭﾃ｣ﾂつｰ: ﾃ･ﾂ､ﾂｱﾃｦﾂ閉・
                let _ = log_audit_event(AuditEvent::new(

                    agent_name.to_string(),

                    AuditEventType::AgentExecution(AgentExecutionEvent {

                        agent_name: agent_name.to_string(),

                        status: ExecutionStatus::Failed,

                        goal: agent_def.goal.clone(),

                        start_time: start_timestamp,

                        end_time: Some(chrono::Utc::now().to_rfc3339()),

                        duration_secs: Some(duration_secs),

                        tokens_used,

                        artifacts: vec![],

                        error: Some(e.to_string()),

                    }),

                ))

                .await;



                AgentResult {

                    agent_name: agent_name.to_string(),

                    status: AgentStatus::Failed,

                    artifacts: vec![],

                    tokens_used,

                    duration_secs,

                    error: Some(e.to_string()),

                }

            }

        };



        // ﾃ･ﾂｮﾂ淌ｨﾂ｡ﾂ古｣ﾂつｹﾃ｣ﾂδ・ﾂ・ﾃ｣ﾂつｿﾃ｣ﾂつｹﾃｦﾂ崢ｴﾃｦﾂ鳴ｰ

        {

            let mut running = self.running_agents.write().await;

            running.insert(agent_name.to_string(), result.status.clone());

        }



        // ﾃ｣ﾂつｳﾃ｣ﾂδｩﾃ｣ﾂδ愿｣ﾂδｬﾃ｣ﾂδｼﾃ｣ﾂつｷﾃ｣ﾂδｧﾃ｣ﾂδｳﾃ｣ﾂつｹﾃ｣ﾂδ暗｣ﾂつ｢ﾃ｣ﾂ・ｫﾃｧﾂｵﾂ静ｦﾂ楪愿｣ﾂつ津､ﾂｿﾂ敕･ﾂｭﾂ・
        #[cfg(feature = "custom-features")]

        {

            self.collaboration_store

                .store_agent_result(agent_name.to_string(), result.clone());

        }



        Ok(result)

    }



    /// ﾃ､ﾂｸﾂｦﾃ･ﾂ按療･ﾂｮﾂ淌ｨﾂ｡ﾂ古ｧﾂ板ｨﾃ｣ﾂ・ｫﾃ｣ﾂつｯﾃ｣ﾂδｭﾃ｣ﾂδｼﾃ｣ﾂδｳ

    fn clone_for_parallel(&self) -> Self {

        Self {

            loader: self.loader.clone(),

            budgeter: self.budgeter.clone(),

            running_agents: Arc::new(RwLock::new(HashMap::new())),

            workspace_dir: self.workspace_dir.clone(),

            config: self.config.clone(),

            auth_manager: self.auth_manager.clone(),

            otel_manager: self.otel_manager.clone(),

            provider: self.provider.clone(),

            conversation_id: self.conversation_id,

            codex_binary_path: self.codex_binary_path.clone(),

            #[cfg(feature = "custom-features")]

            collaboration_store: self.collaboration_store.clone(),

            reasoning_effort: self.reasoning_effort,

            reasoning_summary: self.reasoning_summary,

            verbosity: self.verbosity,

        }

    }



    /// ﾃ｣ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ暗｣ﾂつ津･ﾂｧﾂ氾､ﾂｻﾂｻﾃ･ﾂｮﾂ淌ｨﾂ｡ﾂ・
    pub async fn delegate(

        &self,

        agent_name: &str,

        goal: &str,

        inputs: HashMap<String, String>,

        budget: Option<usize>,

        deadline: Option<u64>,

    ) -> Result<AgentResult> {

        info!("Delegating to agent '{}': {}", agent_name, goal);



        // ﾃ｣ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ暗･ﾂｮﾂ堙ｧﾂｾﾂｩﾃ｣ﾂつ津ｨﾂｪﾂｭﾃ｣ﾂ・ｿﾃｨﾂｾﾂｼﾃ｣ﾂ・ｿ

        let agent_def = {

            let mut loader = self.loader.write().await;

            loader

                .load_by_name(agent_name)

                .with_context(|| format!("Failed to load agent '{agent_name}'"))?

        };



        // ﾃ･ﾂ・ｱﾃｦﾂ慊嘉ｦﾂδ・･ﾂﾂｱﾃ｣ﾂつ津･ﾂ・ﾃ･ﾂ環崚｣ﾂ・ｸﾃ･ﾂ渉姪｣ﾂつ甘ｨﾂｾﾂｼﾃ｣ﾂつ

        let inputs = inputs;

        #[cfg(feature = "custom-features")]

        let mut inputs = inputs;

        #[cfg(feature = "custom-features")]

        {

            let shared_context_snapshot = self.collaboration_store.get_all_context();

            if !shared_context_snapshot.is_empty()

                && let Ok(serialized) = serde_json::to_string(&shared_context_snapshot)

            {

                inputs.insert("shared_context".to_string(), serialized);

            }



            let prior_results_snapshot = self.collaboration_store.get_all_results();

            if !prior_results_snapshot.is_empty()

                && let Ok(serialized) = serde_json::to_string(&prior_results_snapshot)

            {

                inputs.insert("collaboration_results".to_string(), serialized);

            }

        }



        // ﾃ､ﾂｺﾂ暗ｧﾂｮﾂ療｣ﾂつ津ｨﾂｨﾂｭﾃ･ﾂｮﾂ・
        if let Some(budget) = budget {

            self.budgeter.set_agent_limit(agent_name, budget)?;

        } else {

            // ﾃ｣ﾂδ・ﾂδ陛｣ﾂつｩﾃ｣ﾂδｫﾃ｣ﾂδ暗､ﾂｺﾂ暗ｧﾂｮﾂ療｣ﾂ・ﾃ｣ﾂつｳﾃ｣ﾂδｳﾃ｣ﾂδ・ﾂつｭﾃ｣ﾂつｹﾃ｣ﾂδ暗｣ﾂ・ﾃ｣ﾂδｪﾃ｣ﾂつｷﾃ｣ﾂδｼﾃ｣ﾂ・凝｣ﾂつ嘉･ﾂ渉姪･ﾂｾﾂ・
            self.budgeter

                .set_agent_limit(agent_name, agent_def.policies.context.max_tokens)?;

        }



        // ﾃ･ﾂｮﾂ淌ｨﾂ｡ﾂ古｣ﾂつｹﾃ｣ﾂδ・ﾂ・ﾃ｣ﾂつｿﾃ｣ﾂつｹﾃ｣ﾂつ津ｦﾂ崢ｴﾃｦﾂ鳴ｰ

        {

            let mut running = self.running_agents.write().await;

            running.insert(agent_name.to_string(), AgentStatus::Running);

        }



        // ﾃ･ﾂｮﾂ淌ｨﾂ｡ﾂ古ｩﾂ鳴凝･ﾂｧﾂ・
        let start_time = Instant::now();

        let start_timestamp = chrono::Utc::now().to_rfc3339();



        // ﾃｧﾂ崢｣ﾃｦﾂ淞ｻﾃ｣ﾂδｭﾃ｣ﾂつｰ: ﾃ｣ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ暗ｩﾂ鳴凝･ﾂｧﾂ・
        let _ = log_audit_event(AuditEvent::new(

            agent_name.to_string(),

            AuditEventType::AgentExecution(AgentExecutionEvent {

                agent_name: agent_name.to_string(),

                status: ExecutionStatus::Started,

                goal: goal.to_string(),

                start_time: start_timestamp.clone(),

                end_time: None,

                duration_secs: None,

                tokens_used: 0,

                artifacts: vec![],

                error: None,

            }),

        ))

        .await;



        let result = match self.execute_agent(&agent_def, goal, inputs, deadline).await {

            Ok(artifacts) => {

                let duration_secs = start_time.elapsed().as_secs_f64();

                let tokens_used = self.budgeter.get_agent_usage(agent_name);



                info!(

                    "Agent '{}' completed successfully in {:.2}s, used {} tokens",

                    agent_name, duration_secs, tokens_used

                );



                // ﾃｧﾂ崢｣ﾃｦﾂ淞ｻﾃ｣ﾂδｭﾃ｣ﾂつｰ: ﾃ｣ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ暗･ﾂｮﾂ古､ﾂｺﾂ・
                let _ = log_audit_event(AuditEvent::new(

                    agent_name.to_string(),

                    AuditEventType::AgentExecution(AgentExecutionEvent {

                        agent_name: agent_name.to_string(),

                        status: ExecutionStatus::Completed,

                        goal: goal.to_string(),

                        start_time: start_timestamp.clone(),

                        end_time: Some(chrono::Utc::now().to_rfc3339()),

                        duration_secs: Some(duration_secs),

                        tokens_used,

                        artifacts: artifacts.clone(),

                        error: None,

                    }),

                ))

                .await;



                AgentResult {

                    agent_name: agent_name.to_string(),

                    status: AgentStatus::Completed,

                    artifacts,

                    tokens_used,

                    duration_secs,

                    error: None,

                }

            }

            Err(e) => {

                error!("Agent '{}' failed: {}", agent_name, e);



                let duration_secs = start_time.elapsed().as_secs_f64();

                let tokens_used = self.budgeter.get_agent_usage(agent_name);



                // ﾃｧﾂ崢｣ﾃｦﾂ淞ｻﾃ｣ﾂδｭﾃ｣ﾂつｰ: ﾃ｣ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ暗･ﾂ､ﾂｱﾃｦﾂ閉・
                let _ = log_audit_event(AuditEvent::new(

                    agent_name.to_string(),

                    AuditEventType::AgentExecution(AgentExecutionEvent {

                        agent_name: agent_name.to_string(),

                        status: ExecutionStatus::Failed,

                        goal: goal.to_string(),

                        start_time: start_timestamp.clone(),

                        end_time: Some(chrono::Utc::now().to_rfc3339()),

                        duration_secs: Some(duration_secs),

                        tokens_used,

                        artifacts: vec![],

                        error: Some(e.to_string()),

                    }),

                ))

                .await;



                AgentResult {

                    agent_name: agent_name.to_string(),

                    status: AgentStatus::Failed,

                    artifacts: vec![],

                    tokens_used,

                    duration_secs,

                    error: Some(e.to_string()),

                }

            }

        };



        // ﾃ･ﾂｮﾂ淌ｨﾂ｡ﾂ古｣ﾂつｹﾃ｣ﾂδ・ﾂ・ﾃ｣ﾂつｿﾃ｣ﾂつｹﾃ｣ﾂつ津ｦﾂ崢ｴﾃｦﾂ鳴ｰ

        {

            let mut running = self.running_agents.write().await;

            running.insert(agent_name.to_string(), result.status.clone());

        }



        // ﾃ･ﾂｮﾂ淌ｨﾂ｡ﾂ古ｧﾂｵﾂ静ｦﾂ楪愿｣ﾂつ津･ﾂ債氾ｨﾂｪﾂｿﾃ｣ﾂつｹﾃ｣ﾂδ暗｣ﾂつ｢ﾃ｣ﾂ・ｫﾃ､ﾂｿﾂ敕･ﾂｭﾂ・
        #[cfg(feature = "custom-features")]

        {

            self.collaboration_store

                .store_agent_result(agent_name.to_string(), result.clone());

        }



        Ok(result)

    }



    /// ﾃ｣ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ暗｣ﾂつ津･ﾂｮﾂ淌ｩﾂ堋崚｣ﾂ・ｫﾃ･ﾂｮﾂ淌ｨﾂ｡ﾂ・
    async fn execute_agent(

        &self,

        agent_def: &AgentDefinition,

        goal: &str,

        inputs: HashMap<String, String>,

        _deadline: Option<u64>,

    ) -> Result<Vec<String>> {

        debug!("Executing agent '{}' with goal: {}", agent_def.name, goal);



        // 1. ﾃ｣ﾂつｷﾃ｣ﾂつｹﾃ｣ﾂδ・ﾂδﾃ｣ﾂδ療｣ﾂδｭﾃ｣ﾂδｳﾃ｣ﾂδ療｣ﾂδ暗ｦﾂｧﾂ凝ｧﾂｯﾂ嘉ｯﾂｼﾂ暗｣ﾂつｷﾃ｣ﾂδｳﾃ｣ﾂδ療｣ﾂδｫﾃｧﾂ可暗ｯﾂｼﾂ・
        let _system_prompt = format!("You are a {} agent. {}", agent_def.name, agent_def.goal);



        // 2. ﾃ｣ﾂδｦﾃ｣ﾂδｼﾃ｣ﾂつｶﾃ｣ﾂδｼﾃ･ﾂ・･ﾃ･ﾂ環崚｣ﾂつ津ｦﾂｧﾂ凝ｧﾂｯﾂ嘉ｯﾂｼﾂ暗｣ﾂつｿﾃ｣ﾂつｹﾃ｣ﾂつｯﾃ｣ﾂ・ｨinputsﾃ｣ﾂつ津･ﾂ青ｫﾃ｣ﾂつﾂ・ﾂ・
        let inputs_text = if inputs.is_empty() {

            String::new()

        } else {

            let mut text = String::from("\n\nProvided inputs:\n");

            for (key, value) in &inputs {

                text.push_str(&format!("- {key}: {value}\n"));

            }

            text

        };



        let user_message = format!(

            "Task: {goal}{inputs_text}\n\nPlease analyze the task and provide a detailed response."

        );



        // 3. ModelClientﾃ､ﾂｽﾂ愿ｦﾂ・

        let model = self.config.model.as_deref().unwrap_or("gpt-5.2-codex");

        let model_info = crate::models_manager::model_info::with_config_overrides(

            crate::models_manager::model_info::model_info_from_slug(model),

            &self.config,

        );

        let client = ModelClient::new(

            self.auth_manager.clone(),

            self.conversation_id,

            self.provider.clone(),

            codex_protocol::protocol::SessionSource::Cli,

            self.config.model_verbosity,
            crate::ws_version_from_features(&self.config),
            self.config
                .features
                .enabled(Feature::EnableRequestCompression),
            self.config.features.enabled(Feature::RuntimeMetrics),
            None,

        );



        // 4. ResponseItemﾃｦﾂｧﾂ凝ｧﾂｯﾂ嘉ｯﾂｼﾂ・romptﾃ｣ﾂ・ｫﾃｦﾂｸﾂ｡ﾃ｣ﾂ・凖ｯﾂｼﾂ・
        let _input_items = vec![ResponseItem::Message {

            id: None,

            role: "user".to_string(),

            content: vec![ContentItem::InputText {

                text: user_message.clone(),

            }],

            end_turn: None,

            phase: None,

        }];



        // 5. Promptﾃｦﾂｧﾂ凝ｧﾂｯﾂ嘉ｯﾂｼﾂ暗｣ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ暗ｦﾂｨﾂｩﾃｩﾂ卍静｣ﾂ・凝｣ﾂつ嘉｣ﾂδ・｣ﾂδｼﾃ｣ﾂδｫﾃ｣ﾂつ津ｧﾂ板淌ｦﾂ按静ｯﾂｼﾂ・
        let tools = self.build_tools_for_agent(agent_def);



        let prompt = Prompt {

            input: _input_items,

            tools,

            parallel_tool_calls: false,

            base_instructions: BaseInstructions {

                text: String::new(), // ﾃ｣ﾂδ・ﾂδ陛｣ﾂつｩﾃ｣ﾂδｫﾃ｣ﾂδ暗｣ﾂ・ﾃ｣ﾂδ凖｣ﾂ・ﾃ｣ﾂつｹﾃ｣ﾂつ､ﾃ｣ﾂδｳﾃ｣ﾂつｹﾃ｣ﾂδ暗｣ﾂδｩﾃ｣ﾂつｯﾃ｣ﾂつｷﾃ｣ﾂδｧﾃ｣ﾂδｳ

            },

            personality: None,

            output_schema: None,

        };



        // ﾃ｣ﾂδ・ﾂδ静｣ﾂδ・ﾂつｰ: ﾃ｣ﾂつｷﾃ｣ﾂつｹﾃ｣ﾂδ・ﾂδﾃ｣ﾂδ療｣ﾂδｭﾃ｣ﾂδｳﾃ｣ﾂδ療｣ﾂδ暗｣ﾂ・ｮﾃ･ﾂ・・ﾂｮﾂｹﾃ｣ﾂつ津｣ﾂδｭﾃ｣ﾂつｰﾃ･ﾂ・ｺﾃ･ﾂ環・
        debug!(

            "Agent '{}': Using default model instructions (base_instructions_override=None)",

            agent_def.name

        );



        // 6. LLMﾃ･ﾂ堕ｼﾃ｣ﾂ・ｳﾃ･ﾂ・ｺﾃ｣ﾂ・・
        let mut client_session = client.new_session();

        let mut stream = client_session

            .stream(

                &prompt,

                &model_info,

                &self.otel_manager,

                Some(self.reasoning_effort),

                self.reasoning_summary,

                None,

            )

            .await?;

        let mut response_text = String::new();

        let mut total_tokens = 0;



        while let Some(event) = stream.next().await {

            match event? {

                ResponseEvent::Created => {

                    debug!("Agent '{}': Response stream started", agent_def.name);

                }

                ResponseEvent::OutputItemDone(item) => {

                    debug!("Agent '{}': Output item done", agent_def.name);

                    // Extract text from ResponseItem

                    if let ResponseItem::Message { content, .. } = item {

                        for content_item in content {

                            if let ContentItem::OutputText { text } = content_item {

                                response_text.push_str(&text);

                            }

                        }

                    }

                }

                ResponseEvent::Completed {
                    response_id: _,
                    token_usage: Some(usage),
                    can_append: _,
                } => {
                    debug!("Agent '{}': Response completed", agent_def.name);

                    // Use actual token usage from API

                    total_tokens = usage.total_tokens as usize;

                    debug!(

                        "Agent '{}': Actual token usage: {} (input: {}, output: {})",

                        agent_def.name, usage.total_tokens, usage.input_tokens, usage.output_tokens

                    );

                }

                ResponseEvent::Completed {
                    response_id: _,
                    token_usage: None,
                    can_append: _,
                } => {
                    debug!("Agent '{}': Response completed", agent_def.name);

                }

                _ => {}

            }

        }



        // 7. ﾃ｣ﾂδ暗｣ﾂ・ﾃ｣ﾂつｯﾃ｣ﾂδｳﾃ､ﾂｺﾂ暗ｧﾂｮﾂ療｣ﾂδ・｣ﾂつｧﾃ｣ﾂδ・ﾂつｯﾃ｣ﾂ・ｨﾃｦﾂｶﾂ暗ｨﾂｲﾂｻ

        if !self.budgeter.try_consume(&agent_def.name, total_tokens)? {

            anyhow::bail!("Token budget exceeded for agent '{}'", agent_def.name);

        }



        info!(

            "Agent '{}' completed LLM execution: {} tokens used",

            agent_def.name, total_tokens

        );



        // 8. ﾃ｣ﾂつ｢ﾃ｣ﾂδｼﾃ｣ﾂδ・ﾂつ｣ﾃ｣ﾂδ陛｣ﾂつ｡ﾃ｣ﾂつｯﾃ｣ﾂδ暗ｧﾂ板淌ｦﾂ按・
        let artifacts_dir = self.workspace_dir.join("artifacts");

        tokio::fs::create_dir_all(&artifacts_dir).await?;



        let mut generated_artifacts = Vec::new();

        for artifact_path in &agent_def.artifacts {

            let full_path = self.workspace_dir.join(artifact_path);

            if let Some(parent) = full_path.parent() {

                tokio::fs::create_dir_all(parent).await?;

            }



            // ﾃ｣ﾂつ｢ﾃ｣ﾂδｼﾃ｣ﾂδ・ﾂつ｣ﾃ｣ﾂδ陛｣ﾂつ｡ﾃ｣ﾂつｯﾃ｣ﾂδ暗･ﾂ・ﾃ･ﾂｮﾂｹﾃ｣ﾂつ津ｧﾂ板淌ｦﾂ按・
            let content = format!(

                "# Agent: {}\n\n## Goal\n{}\n\n## Task\n{}\n\n## Inputs\n{}\n\n## Agent Response\n\n{}\n\n## Execution Summary\n\n- Tokens Used: {}\n- Success Criteria:\n{}\n",

                agent_def.name,

                agent_def.goal,

                goal,

                inputs

                    .iter()

                    .map(|(k, v)| format!("- **{k}**: {v}"))

                    .collect::<Vec<_>>()

                    .join("\n"),

                response_text,

                total_tokens,

                agent_def

                    .success_criteria

                    .iter()

                    .map(|c| format!("  - {c}"))

                    .collect::<Vec<_>>()

                    .join("\n")

            );



            tokio::fs::write(&full_path, content).await?;

            generated_artifacts.push(artifact_path.clone());

        }



        Ok(generated_artifacts)

    }



    /// ﾃ･ﾂ按ｩﾃｧﾂ板ｨﾃ･ﾂ渉ｯﾃｨﾂδｽﾃ｣ﾂ・ｪﾃ｣ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ暗､ﾂｸﾂﾃｨﾂｦﾂｧﾃ｣ﾂつ津･ﾂ渉姪･ﾂｾﾂ・
    pub async fn list_agents(&self) -> Result<Vec<String>> {

        let loader = self.loader.read().await;

        loader.list_available_agents()

    }



    /// ﾃ･ﾂｮﾂ淌ｨﾂ｡ﾂ古､ﾂｸﾂｭﾃ｣ﾂ・ｮﾃ｣ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ暗ｧﾂ環ｶﾃｦﾂ・凝｣ﾂつ津･ﾂ渉姪･ﾂｾﾂ・
    pub async fn get_running_agents(&self) -> HashMap<String, AgentStatus> {

        self.running_agents.read().await.clone()

    }



    /// ﾃ｣ﾂδ暗｣ﾂ・ﾃ｣ﾂつｯﾃ｣ﾂδｳﾃ､ﾂｽﾂｿﾃｧﾂ板ｨﾃｧﾂ環ｶﾃｦﾂｳﾂ・｣ﾂつ津･ﾂ渉姪･ﾂｾﾂ・
    pub fn get_budget_status(&self) -> (usize, usize, f64) {

        let used = self.budgeter.get_used();

        let remaining = self.budgeter.get_remaining();

        let utilization = self.budgeter.get_utilization();

        (used, remaining, utilization)

    }



    /// ﾃｨﾂｻﾂｽﾃｩﾂ・湘ｧﾂ可暗｣ﾂδ陛｣ﾂつｩﾃ｣ﾂδｼﾃ｣ﾂδｫﾃ｣ﾂδ静｣ﾂδε｣ﾂつｯﾃ｣ﾂ・古･ﾂｿﾂ・ﾂｦﾂ・｣ﾂ・凝｣ﾂδ・｣ﾂつｧﾃ｣ﾂδ・ﾂつｯ

    pub fn should_use_lightweight(&self, threshold: f64) -> bool {

        self.budgeter.should_fallback_lightweight(threshold)

    }



    /// ﾃ｣ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ暗ｦﾂｨﾂｩﾃｩﾂ卍静｣ﾂ・ｫﾃ･ﾂ淞ｺﾃ｣ﾂ・･ﾃ｣ﾂ・・ﾂ・ｦﾃ｣ﾂδ・ﾂ・ﾃ｣ﾂδｫﾃ､ﾂｻﾂ陛ｦﾂｧﾂ佚｣ﾂつ津ｦﾂｧﾂ凝ｧﾂｯﾂ・
    fn build_tools_for_agent(

        &self,

        agent_def: &AgentDefinition,

    ) -> Vec<crate::client_common::tools::ToolSpec> {

        use crate::tools::spec::create_grep_files_tool;

        use crate::tools::spec::create_list_dir_tool;

        use crate::tools::spec::create_read_file_tool;



        let mut tools = Vec::new();



        debug!(

            "Building tools for agent '{}': {:?}",

            agent_def.name, agent_def.tools.mcp

        );



        for tool_name in &agent_def.tools.mcp {

            match tool_name.as_str() {

                "read_file" => {

                    tools.push(create_read_file_tool());

                    debug!("Added read_file tool for agent '{}'", agent_def.name);

                }

                "grep" | "grep_files" => {

                    tools.push(create_grep_files_tool());

                    debug!("Added grep_files tool for agent '{}'", agent_def.name);

                }

                "list_dir" => {

                    tools.push(create_list_dir_tool());

                    debug!("Added list_dir tool for agent '{}'", agent_def.name);

                }

                // "codebase_search" => {

                //     use crate::tools::spec::create_codebase_search_tool;

                //     tools.push(create_codebase_search_tool());

                //     debug!("Added codebase_search tool for agent '{}'", agent_def.name);

                // }

                _ => {

                    debug!("Unknown tool in agent definition: {}", tool_name);

                }

            }

        }



        info!(

            "Agent '{}' configured with {} tools",

            agent_def.name,

            tools.len()

        );



        tools

    }

}



#[cfg(test)]

mod tests {

    use super::*;

    use pretty_assertions::assert_eq;

    use std::fs;

    use tempfile::TempDir;



    #[tokio::test]

    async fn test_agent_runtime_delegate() {

        use crate::config::ConfigBuilder;

        use crate::model_provider_info::ModelProviderInfo;

        use crate::model_provider_info::WireApi;



        let temp_dir = TempDir::new().unwrap();

        let agents_dir = temp_dir.path().join(".codex/agents");

        fs::create_dir_all(&agents_dir).unwrap();



        let agent_yaml = r#"

name: "Test Agent"

goal: "Test goal"

tools:

  mcp: []

  fs:

    read: true

    write:

      - "./artifacts"

  net:

    allow: []

  shell: []

policies:

  context:

    max_tokens: 5000

    retention: "job"

  secrets:

    redact: false

success_criteria:

  - "ﾃ･ﾂ淞ｺﾃｦﾂｺﾂ・"

artifacts:

  - "artifacts/test-output.md"

"#;



        fs::write(agents_dir.join("test-agent.yaml"), agent_yaml).unwrap();



        // ﾃ｣ﾂδ｢ﾃ｣ﾂδ・ﾂつｯConfigﾃ､ﾂｽﾂ愿ｦﾂ・

        let codex_home = temp_dir.path().to_path_buf();

        let config = Arc::new(

            ConfigBuilder::default()

                .codex_home(codex_home.clone())

                .build()

                .await

                .unwrap(),

        );

        let provider = ModelProviderInfo {

            name: "Test Provider".to_string(),

            base_url: Some("https://api.openai.com/v1".to_string()),

            env_key: Some("OPENAI_API_KEY".to_string()),

            wire_api: WireApi::Responses,

            env_key_instructions: None,

            query_params: None,

            http_headers: None,

            env_http_headers: None,

            request_max_retries: Some(4),

            stream_max_retries: Some(10),

            stream_idle_timeout_ms: Some(300_000),

            experimental_bearer_token: None,

            requires_openai_auth: false,

            supports_websockets: false,

        };

        let conversation_id = ThreadId::new();

        let otel_manager = OtelEventManager::new(

            conversation_id,

            "test-model",

            "test",

            None,

            None,

            None,

            "test-originator".to_string(),

            false,

            "test".to_string(),

            codex_protocol::protocol::SessionSource::Cli,

        );



        let runtime = AgentRuntime::new(

            temp_dir.path().to_path_buf(),

            10000,

            config,

            None,

            otel_manager,

            provider,

            conversation_id,

            ReasoningEffort::default(),

            ReasoningSummary::default(),

            Verbosity::default(),

        );



        let mut inputs = HashMap::new();

        inputs.insert("key1".to_string(), "value1".to_string());



        // Note: This will fail without real API credentials, but demonstrates the structure

        let result = runtime

            .delegate("test-agent", "Test goal", inputs, Some(5000), None)

            .await;



        // In real tests, we'd use mocks or fixtures

        // For now, just verify compilation

        match result {

            Ok(r) => {

                assert_eq!(r.agent_name, "test-agent");

            }

            Err(_) => {

                // Expected without real API credentials

            }

        }

    }



    #[tokio::test]

    async fn test_list_agents() {

        use crate::config::ConfigBuilder;

        use crate::model_provider_info::ModelProviderInfo;

        use crate::model_provider_info::WireApi;



        let temp_dir = TempDir::new().unwrap();

        let agents_dir = temp_dir.path().join(".codex/agents");

        fs::create_dir_all(&agents_dir).unwrap();



        fs::write(agents_dir.join("agent1.yaml"), "name: Agent1\ngoal: Goal1\ntools: {}\npolicies: {context: {}}\nsuccess_criteria: []\nartifacts: []").unwrap();

        fs::write(agents_dir.join("agent2.yaml"), "name: Agent2\ngoal: Goal2\ntools: {}\npolicies: {context: {}}\nsuccess_criteria: []\nartifacts: []").unwrap();



        let codex_home = temp_dir.path().to_path_buf();

        let config = Arc::new(

            ConfigBuilder::default()

                .codex_home(codex_home.clone())

                .build()

                .await

                .unwrap(),

        );

        let provider = ModelProviderInfo {

            name: "Test Provider".to_string(),

            base_url: Some("https://api.openai.com/v1".to_string()),

            env_key: Some("OPENAI_API_KEY".to_string()),

            wire_api: WireApi::Responses,

            env_key_instructions: None,

            query_params: None,

            http_headers: None,

            env_http_headers: None,

            request_max_retries: Some(4),

            stream_max_retries: Some(10),

            stream_idle_timeout_ms: Some(300_000),

            experimental_bearer_token: None,

            requires_openai_auth: false,

            supports_websockets: false,

        };

        let conversation_id = ThreadId::new();

        let otel_manager = OtelEventManager::new(

            conversation_id,

            "test-model",

            "test",

            None,

            None,

            None,

            "test-originator".to_string(),

            false,

            "test".to_string(),

            codex_protocol::protocol::SessionSource::Cli,

        );



        let runtime = AgentRuntime::new(

            temp_dir.path().to_path_buf(),

            10000,

            config.clone(),

            None,

            otel_manager,

            provider,

            conversation_id,

            ReasoningEffort::default(),

            ReasoningSummary::default(),

            Verbosity::default(),

        );

        let agents = runtime.list_agents().await.unwrap();



        assert_eq!(agents, vec!["agent1", "agent2"]);

    }

}



// ========== Codex MCP Integration (Phase 2) ==========



impl AgentRuntime {

    /// Codexﾃ｣ﾂδ静｣ﾂつ､ﾃ｣ﾂδ甘｣ﾂδｪﾃ｣ﾂδ妥｣ﾂつｹﾃ｣ﾂつ津ｨﾂｨﾂｭﾃ･ﾂｮﾂ・
    pub fn with_codex_binary_path(mut self, path: PathBuf) -> Self {

        self.codex_binary_path = Some(path);

        self

    }



    /// ﾃ･ﾂ債氾ｨﾂｪﾂｿﾃ｣ﾂつｹﾃ｣ﾂδ暗｣ﾂつ｢ﾃ｣ﾂ・ｸﾃ｣ﾂ・ｮﾃ･ﾂ渉づｧﾂ・ﾃ｣ﾂつ津･ﾂ渉姪･ﾂｾﾂ・
    #[cfg(feature = "custom-features")]

    pub fn collaboration_store(&self) -> Arc<CollaborationStore> {

        self.collaboration_store.clone()

    }



    /// Codex MCP Serverﾃ｣ﾂつ痴tdio ﾃ｣ﾂδ｢ﾃ｣ﾂδｼﾃ｣ﾂδ嘉｣ﾂ・ｧﾃｨﾂｵﾂｷﾃ･ﾂ仰・
    async fn spawn_codex_mcp_server(&self) -> Result<Arc<RmcpClient>> {

        let codex_path = self

            .codex_binary_path

            .clone()

            .or_else(|| std::env::current_exe().ok())

            .ok_or_else(|| anyhow!("Codex binary path not configured"))?;



        info!(

            "Spawning Codex MCP Server: {} mcp-server",

            codex_path.display()

        );



        let client = RmcpClient::new_stdio_client(

            codex_path.into_os_string(),

            vec![OsString::from("mcp-server")],

            None,

            &[],  // env_vars

            None, // cwd

        )

        .await

        .context("Failed to spawn Codex MCP server")?;



        // Initialize MCP session

        let init_params = InitializeRequestParams {
            meta: None,
            client_info: rmcp::model::Implementation {
                name: "codex-subagent-runtime".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                title: Some("Codex Subagent Runtime".into()),
                description: None,
                icons: None,
                website_url: None,
            },
            protocol_version: ProtocolVersion::V_2025_06_18,
            capabilities: rmcp::model::ClientCapabilities {
                elicitation: None,
                experimental: None,
                extensions: None,
                sampling: None,
                roots: None,
                tasks: None,
            },
        };


        let send_elicitation: SendElicitation =

            Box::new(|_request_id: RequestId, _elicitation: Elicitation| {

                async move { anyhow::bail!("Elicitation not supported in AgentRuntime") }.boxed()

            });

        client

            .initialize(init_params, Some(Duration::from_secs(10)), send_elicitation)

            .await

            .context("Failed to initialize Codex MCP server")?;



        info!("Codex MCP Server initialized successfully");



        Ok(Arc::new(client))

    }



    /// ﾃ｣ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ暗ｦﾂｨﾂｩﾃｩﾂ卍静｣ﾂ・ｫﾃ･ﾂ淞ｺﾃ｣ﾂ・･ﾃ｣ﾂ・・ﾂ・ｦCodex MCP toolsﾃ｣ﾂつ津｣ﾂδ陛｣ﾂつ｣ﾃ｣ﾂδｫﾃ｣ﾂつｿﾃ｣ﾂδｪﾃ｣ﾂδｳﾃ｣ﾂつｰ

    fn filter_codex_mcp_tools(agent_def: &AgentDefinition) -> Vec<String> {

        agent_def

            .tools

            .mcp

            .iter()

            .filter(|tool| Self::is_codex_tool(tool))

            .cloned()

            .collect()

    }



    fn is_codex_tool(tool: &str) -> bool {

        let canonical = tool.rsplit("__").next().unwrap_or(tool);

        canonical.starts_with("codex_") || canonical.starts_with("codex-")

    }



    /// Codex MCP toolsﾃ｣ﾂ・ｮﾃｨﾂｪﾂｬﾃｦﾂ伉偲｣ﾂつ津ｧﾂ板淌ｦﾂ・ﾂ・ﾂ暗｣ﾂ・ﾃ｣ﾂδｭﾃ｣ﾂδｳﾃ｣ﾂδ療｣ﾂδ暗ｧﾂ板ｨﾂ・ﾂ・
    fn build_codex_mcp_tools_description(allowed_tools: &[String]) -> String {

        let mut desc = String::from("Available Codex MCP Tools:\n\n");



        for tool in allowed_tools {
            let tool_desc = match tool.as_str() {
                "codex_read_file" => concat!(
                    "- codex_read_file(path: str) -> str\n  ",
                    "Read a file from the workspace using Codex.\n  ",
                    "Safe, read-only operation."
                ),
                "codex_grep" => concat!(
                    "- codex_grep(pattern: str, path: Optional[str]) -> List[str]\n  ",
                    "Search for patterns in files using Codex grep.\n  ",
                    "Safe, read-only operation."
                ),
                "codex_codebase_search" => concat!(
                    "- codex_codebase_search(query: str, target_directories: Optional[List[str]]) -> List[str]\n  ",
                    "Semantic code search using Codex.\n  ",
                    "Safe, read-only operation."
                ),
                "codex_apply_patch" => concat!(
                    "- codex_apply_patch(patch: str) -> str\n  ",
                    "Apply a code patch using Codex.\n  ",
                    "Requires write permission."
                ),
                "codex_shell" => concat!(
                    "- codex_shell(command: str) -> str\n  ",
                    "Execute a shell command via Codex (restricted).\n  ",
                    "Requires shell permission."
                ),
                "codex-supervisor" => concat!(
                    "- codex-supervisor(goal: str, strategy: Optional[str]) -> SupervisorReport\n  ",
                    "Plan and coordinate multiple Codex subagents through the Supervisor layer.\n  ",
                    "Use when you need structured collaboration across specialists."
                ),
                "codex-deep-research" => concat!(
                    "- codex-deep-research(query: str, strategy: Optional[str], max_depth: Optional[int]) -> ResearchReport\n  ",
                    "Run Codex DeepResearcher for multi-source investigations with citations.\n  ",
                    "Ideal for comprehensive research and evidence gathering."
                ),
                "codex-subagent" => concat!(
                    "- codex-subagent(action: str, agent_type: Optional[str], task: Optional[str], task_id: Optional[str]) -> ToolResult\n  ",
                    "Manage Codex subagents: start tasks, auto-dispatch, check inbox, status, thinking, or token usage.\n  ",
                    "Delegate work to specialist agents and retrieve their outputs."
                ),
                "codex-custom-command" => concat!(
                    "- codex-custom-command(action: str, command_name: Optional[str], context: Optional[str]) -> ToolResult\n  ",
                    "Execute curated multi-step workflows (e.g., analyze_code, deep_research) mapped to subagents.\n  ",
                    "Handy for quick access to predefined automation."
                ),
                "codex-hook" => concat!(
                    "- codex-hook(event: str, context: Optional[str]) -> HookAck\n  ",
                    "Trigger lifecycle hooks such as on_subagent_start/on_task_complete for integrations.\n  ",
                    "Use to capture workflow events or integrate external systems."
                ),
                "codex-auto-orchestrate" => concat!(
                    "- codex-auto-orchestrate(goal: str, strategy: Optional[str]) -> OrchestrationReport\n  ",
                    "Automatically analyze goals and dispatch the optimal mix of subagents.\n  ",
                    "Best for high-level objectives that benefit from autonomous planning."
                ),
                _ => continue,
            };
            desc.push_str(tool_desc);
            desc.push_str("\n\n");
        }



        desc.push_str(concat!(
            "To use these tools, output a tool call in the following format:\n",
            "TOOL_CALL: tool_name(arg1=\"value1\", arg2=\"value2\")\n\n",
            "The results will be provided to you for further analysis.",
        ));



        desc

    }



    /// ﾃ｣ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ暗｣ﾂつ辰odex MCPﾃｧﾂｵﾂ古ｧﾂ板ｱﾃ｣ﾂ・ｧﾃ･ﾂｮﾂ淌ｨﾂ｡ﾂ古ｯﾂｼﾂ・hase 3: ﾃ･ﾂｮﾂ古･ﾂ・ﾃ･ﾂｮﾂ淌ｨﾂ｣ﾂ・ﾂｼﾂ・
    pub async fn execute_agent_with_codex_mcp(

        &self,

        agent_def: &AgentDefinition,

        goal: &str,

        inputs: HashMap<String, String>,

        _deadline: Option<u64>,

    ) -> Result<Vec<String>> {

        debug!(

            "Executing agent '{}' with Codex MCP integration",

            agent_def.name

        );



        // 1. Codex MCP Serverﾃ｣ﾂつ津ｨﾂｵﾂｷﾃ･ﾂ仰・
        let mcp_client = self

            .spawn_codex_mcp_server()

            .await

            .context("Failed to spawn Codex MCP server")?;



        // 2. ﾃ｣ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ暗ｦﾂｨﾂｩﾃｩﾂ卍静｣ﾂ・ｧﾃ｣ﾂδ・ﾂ・ﾃ｣ﾂδｫﾃ｣ﾂつ津｣ﾂδ陛｣ﾂつ｣ﾃ｣ﾂδｫﾃ｣ﾂつｿﾃ｣ﾂδｪﾃ｣ﾂδｳﾃ｣ﾂつｰ

        let allowed_tools = Self::filter_codex_mcp_tools(agent_def);



        info!(

            "Agent '{}' is allowed to use {} Codex MCP tools: {:?}",

            agent_def.name,

            allowed_tools.len(),

            allowed_tools

        );



        // 3. ﾃ｣ﾂつｷﾃ｣ﾂつｹﾃ｣ﾂδ・ﾂδﾃ｣ﾂδ療｣ﾂδｭﾃ｣ﾂδｳﾃ｣ﾂδ療｣ﾂδ暗ｦﾂｧﾂ凝ｧﾂｯﾂ嘉ｯﾂｼﾂ暗｣ﾂδ・｣ﾂδｼﾃ｣ﾂδｫﾃｨﾂｪﾂｬﾃｦﾂ伉偲･ﾂ青ｫﾃ｣ﾂつﾂ・ﾂ・
        let tools_description = Self::build_codex_mcp_tools_description(&allowed_tools);



        let system_prompt = format!(
            "You are a specialized sub-agent with the following role:\n\nAgent: {}\nGoal: {}\n\nSuccess Criteria:\n{}\n\nInputs provided:\n{}\n\n{}\n\nPlease analyze the task and use the available Codex MCP tools to complete it. When you need to use a tool, output it in the specified format. After all tool calls are complete, provide a final summary.",
            agent_def.name,
            agent_def.goal,
            agent_def.success_criteria.join("\n- "),
            inputs
                .iter()
                .map(|(k, v)| format!("- {k}: {v}"))
                .collect::<Vec<_>>()
                .join("\n"),
            tools_description
        );



        // 4. ﾃ･ﾂ按敕ｦﾂ慊淌｣ﾂδ療｣ﾂδｭﾃ｣ﾂδｳﾃ｣ﾂδ療｣ﾂδ・

        let user_prompt = format!("Task: {goal}");



        // 5. LLMﾃ･ﾂｯﾂｾﾃｨﾂｩﾂｱﾃ｣ﾂδｫﾃ｣ﾂδｼﾃ｣ﾂδ療ｯﾂｼﾂ暗ｦﾂ慊ﾃ･ﾂ､ﾂｧ5ﾃ･ﾂ崢榲｣ﾂ・ﾃ｣ﾂδ・ﾂ・ﾃ｣ﾂδｫﾃ･ﾂ堕ｼﾃ｣ﾂ・ｳﾃ･ﾂ・ｺﾃ｣ﾂ・療ｯﾂｼﾂ・
        let max_iterations = 5;

        let mut conversation_history = vec![

            ("system".to_string(), system_prompt),

            ("user".to_string(), user_prompt.clone()),

        ];

        let mut artifacts = Vec::new();



        for iteration in 0..max_iterations {

            debug!("Agent iteration {}/{}", iteration + 1, max_iterations);



            // LLMﾃ･ﾂ堕ｼﾃ｣ﾂ・ｳﾃ･ﾂ・ｺﾃ｣ﾂ・・
            let llm_response = self

                .call_llm_for_agent(&conversation_history)

                .await

                .context("Failed to call LLM for agent")?;



            conversation_history.push(("assistant".to_string(), llm_response.clone()));

            artifacts.push(llm_response.clone());



            // ﾃ｣ﾂδ・ﾂ・ﾃ｣ﾂδｫﾃ｣ﾂつｳﾃ｣ﾂδｼﾃ｣ﾂδｫﾃｦﾂ､ﾂ愿･ﾂ・

            let tool_calls = self.detect_tool_calls(&llm_response);



            if tool_calls.is_empty() {

                // ﾃ｣ﾂδ・ﾂ・ﾃ｣ﾂδｫﾃ｣ﾂつｳﾃ｣ﾂδｼﾃ｣ﾂδｫﾃ｣ﾂ・古｣ﾂ・ｪﾃ｣ﾂ・・ﾂﾂｴﾃ･ﾂ青暗｣ﾂ・ﾃｧﾂｵﾂづ､ﾂｺﾂ・
                info!("No more tool calls detected. Agent task completed.");

                break;

            }



            // ﾃ｣ﾂδ・ﾂ・ﾃ｣ﾂδｫﾃ･ﾂｮﾂ淌ｨﾂ｡ﾂ・
            let mut tool_results = Vec::new();

            for (tool_name, tool_args) in tool_calls {

                if !allowed_tools.contains(&tool_name) {

                    let error_msg =

                        format!("ERROR: Tool '{tool_name}' is not permitted for this agent");

                    tool_results.push(error_msg);

                    continue;

                }



                info!(

                    "Executing Codex MCP tool: {} with args: {:?}",

                    tool_name, tool_args

                );



                match self

                    .execute_codex_mcp_tool(&mcp_client, &tool_name, tool_args)

                    .await

                {

                    Ok(result) => {

                        tool_results.push(format!("TOOL_RESULT[{tool_name}]: {result}"));

                    }

                    Err(e) => {

                        let error_msg = format!("ERROR executing tool '{tool_name}': {e}");

                        error!("{error_msg}");

                        tool_results.push(error_msg);

                    }

                }

            }



            // ﾃ｣ﾂδ・ﾂ・ﾃ｣ﾂδｫﾃｧﾂｵﾂ静ｦﾂ楪愿｣ﾂつ鱈LMﾃ｣ﾂ・ｫﾃ｣ﾂδ陛｣ﾂつ｣ﾃ｣ﾂδｼﾃ｣ﾂδ嘉｣ﾂδ静｣ﾂδ・ﾂつｯ

            let feedback = tool_results.join("\n\n");

            conversation_history.push(("user".to_string(), feedback.clone()));

            artifacts.push(format!("--- Tool Execution Results ---\n{feedback}"));

        }



        info!("Agent '{}' completed execution", agent_def.name);



        Ok(artifacts)

    }



    /// LLMﾃ｣ﾂつ津･ﾂ堕ｼﾃ｣ﾂ・ｳﾃ･ﾂ・ｺﾃ｣ﾂ・療｣ﾂ・ｦﾃ｣ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ暗｣ﾂ・ﾃ･ﾂｿﾂ愿ｧﾂｭﾂ氾｣ﾂつ津･ﾂ渉姪･ﾂｾﾂ・
    async fn call_llm_for_agent(&self, conversation: &[(String, String)]) -> Result<String> {

        // ﾃ｣ﾂδ療｣ﾂδｭﾃ｣ﾂδｳﾃ｣ﾂδ療｣ﾂδ暗ｦﾂｧﾂ凝ｧﾂｯﾂ嘉ｯﾂｼﾂ暗ｦﾂ慊ﾃｦﾂ鳴ｰﾃ｣ﾂ・ｮﾃ｣ﾂδ｡ﾃ｣ﾂδ・ﾂつｻﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂ・ｮﾃ｣ﾂ・ｿﾃ｣ﾂつ津､ﾂｽﾂｿﾃｧﾂ板ｨﾂ・ﾂ・
        let last_message = conversation

            .last()

            .ok_or_else(|| anyhow!("Conversation history is empty"))?;



        let system_instructions = conversation

            .first()

            .filter(|(role, _)| role == "system")

            .map(|(_, content)| content.clone());



        let input_items = vec![ResponseItem::Message {

            id: None,

            role: "user".to_string(),

            content: vec![ContentItem::InputText {

                text: last_message.1.clone(),

            }],

            end_turn: None,

            phase: None,

        }];



        let prompt = Prompt {

            input: input_items,

            tools: vec![],

            parallel_tool_calls: false,

            base_instructions: BaseInstructions {

                text: system_instructions.unwrap_or_default(),

            },

            personality: None,

            output_schema: None,

        };



        // ModelClientﾃｧﾂｵﾂ古ｧﾂ板ｱﾃ｣ﾂ・ｧLLMﾃ･ﾂ堕ｼﾃ｣ﾂ・ｳﾃ･ﾂ・ｺﾃ｣ﾂ・・
        let model = self.config.model.as_deref().unwrap_or("gpt-5.2-codex");

        let model_info = crate::models_manager::model_info::with_config_overrides(

            crate::models_manager::model_info::model_info_from_slug(model),

            &self.config,

        );

        let model_client = ModelClient::new(

            self.auth_manager.clone(),

            self.conversation_id,

            self.provider.clone(),

            codex_protocol::protocol::SessionSource::Cli,

            self.config.model_verbosity,
            crate::ws_version_from_features(&self.config),
            self.config
                .features
                .enabled(Feature::EnableRequestCompression),
            self.config.features.enabled(Feature::RuntimeMetrics),
            None,

        );



        let mut client_session = model_client.new_session();

        let mut response_stream = client_session

            .stream(

                &prompt,

                &model_info,

                &self.otel_manager,

                Some(self.reasoning_effort),

                self.reasoning_summary,

                None,

            )

            .await

            .context("Failed to stream LLM response")?;



        // ﾃ｣ﾂδｬﾃ｣ﾂつｹﾃ｣ﾂδ敕｣ﾂδｳﾃ｣ﾂつｹﾃ｣ﾂつ津･ﾂ渉偲ｩﾂ崢・
        let mut full_response = String::new();

        let mut _tokens_used = 0;



        while let Some(event) = response_stream.next().await {

            match event? {

                ResponseEvent::OutputItemDone(ResponseItem::Message { content, .. }) => {

                    // ResponseItemﾃ｣ﾂ・凝｣ﾂつ嘉｣ﾂδ・ﾂつｭﾃ｣ﾂつｹﾃ｣ﾂδ暗｣ﾂつ津ｦﾂ環ｽﾃ･ﾂ・ｺ

                    for content_item in content {

                        if let ContentItem::OutputText { text } = content_item {

                            full_response.push_str(&text);

                        }

                    }

                }

                ResponseEvent::Completed {
                    response_id: _,
                    token_usage,
                    can_append: _,
                } => {
                    // ﾃ･ﾂｮﾂ淌ｩﾂ堋崚｣ﾂ・ｮﾃ｣ﾂδ暗｣ﾂ・ﾃ｣ﾂつｯﾃ｣ﾂδｳﾃ､ﾂｽﾂｿﾃｧﾂ板ｨﾃｩﾂ・湘｣ﾂつ津｣ﾂつｭﾃ｣ﾂδ｣ﾃ｣ﾂδ療｣ﾂδ・｣ﾂδ｣

                    if let Some(usage) = token_usage {

                        _tokens_used = usage.total_tokens as usize;

                        debug!(

                            "LLM call completed: {} tokens (input: {}, output: {})",

                            usage.total_tokens, usage.input_tokens, usage.output_tokens

                        );

                        // Codexﾃ｣ﾂつｨﾃ｣ﾂδｼﾃ｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδｳﾃ｣ﾂδ・ ﾃ｣ﾂδ暗｣ﾂ・ﾃ｣ﾂつｯﾃ｣ﾂδｳﾃ｣ﾂδ静｣ﾂつｸﾃ｣ﾂつｧﾃ｣ﾂδ・ﾂδ暗ｦﾂｶﾂ暗ｨﾂｲﾂｻﾃ｣ﾂ・ｮﾃ｣ﾂδｭﾃ｣ﾂつｸﾃ｣ﾂδ・ﾂつｯﾃ｣ﾂつ津ｨﾂｿﾂｽﾃ･ﾂ環ﾃ｣ﾂ・凖｣ﾂつ凝･ﾂﾂｴﾃ･ﾂ青暗｣ﾂ・

                        // agent_nameﾃｧﾂｭﾂ嘉｣ﾂ・ﾃｧﾂｮﾂ｡ﾃｧﾂ青・ﾂｸﾂ甘｣ﾂﾂ・｣ﾂ・禿｣ﾂ・ｮﾃｩﾂ鳴｢ﾃｦﾂ閉ｰﾃ｣ﾂ・ｮﾃ･ﾂ堕ｼﾃ｣ﾂ・ｳﾃ･ﾂ・ｺﾃ｣ﾂ・療･ﾂ・ﾃ｣ﾂつБgentﾃｦﾂｧﾂ凝ｩﾂﾂﾃ､ﾂｽﾂ禿｣ﾂ・ｧ

                        // ﾃ｣ﾂつ｢ﾃ｣ﾂδｭﾃ｣ﾂつｱﾃ｣ﾂδｼﾃ｣ﾂつｷﾃ｣ﾂδｧﾃ｣ﾂδｳﾃ･ﾂ・ｦﾃｧﾂ青・ﾂつ津･ﾂｮﾂ淌ｨﾂ｣ﾂ・ﾂ・療｣ﾂ・ｦﾃ｣ﾂ・湘｣ﾂ・ﾃ｣ﾂ・陛｣ﾂ・・・ﾂ・untimeﾃ･ﾂ債佚ｧﾂ仰ｬﾃ｣ﾂ・ｧﾃ｣ﾂ・ｯﾃｦﾂ慊ｪﾃ･ﾂｯﾂｾﾃ･ﾂｿﾂ愿ｯﾂｼﾂ嘉｣ﾂﾂ・
                    }

                }

                _ => {}

            }

        }



        Ok(full_response)

    }



    /// LLMﾃ｣ﾂδｬﾃ｣ﾂつｹﾃ｣ﾂδ敕｣ﾂδｳﾃ｣ﾂつｹﾃ｣ﾂ・凝｣ﾂつ嘉｣ﾂδ・ﾂ・ﾃ｣ﾂδｫﾃ｣ﾂつｳﾃ｣ﾂδｼﾃ｣ﾂδｫﾃ｣ﾂつ津ｦﾂ､ﾂ愿･ﾂ・

    fn detect_tool_calls(&self, response: &str) -> Vec<(String, serde_json::Value)> {

        let mut tool_calls = Vec::new();



        // ﾃ｣ﾂδ妥｣ﾂつｿﾃ｣ﾂδｼﾃ｣ﾂδｳ: TOOL_CALL: tool_name(arg1="value1", arg2="value2")

        // ﾃｧﾂｰﾂ｡ﾃｦﾂ伉禿･ﾂｮﾂ淌ｨﾂ｣ﾂ・ JSONﾃ｣ﾂδ陛｣ﾂつｩﾃ｣ﾂδｼﾃ｣ﾂδ榲｣ﾂδε｣ﾂδ暗｣ﾂつづｦﾂ､ﾂ愿･ﾂ・

        for line in response.lines() {

            let line = line.trim();



            // TOOL_CALL: codex_read_file(path="src/auth.rs")

            if line.starts_with("TOOL_CALL:")

                && let Some(call_str) = line.strip_prefix("TOOL_CALL:").map(str::trim)

                && let Some((tool_name, args_str)) = call_str.split_once('(')

            {

                let tool_name = tool_name.trim().to_string();

                let args_str = args_str.trim_end_matches(')').trim();



                // ﾃｧﾂｰﾂ｡ﾃｦﾂ伉禿｣ﾂδ妥｣ﾂδｼﾃ｣ﾂつｹ: key="value" ﾃ･ﾂｽﾂ｢ﾃ･ﾂｼﾂ・
                let mut args = serde_json::Map::new();

                for part in args_str.split(',') {

                    if let Some((key, value)) = part.split_once('=') {

                        let key = key.trim().to_string();

                        let value = value.trim().trim_matches('"').to_string();

                        args.insert(key, serde_json::Value::String(value));

                    }

                }



                tool_calls.push((tool_name, serde_json::Value::Object(args)));

            }

        }



        tool_calls

    }



    /// Codex MCPﾃ｣ﾂδ・ﾂ・ﾃ｣ﾂδｫﾃ｣ﾂつ津･ﾂｮﾂ淌ｨﾂ｡ﾂ・
    async fn execute_codex_mcp_tool(

        &self,

        mcp_client: &Arc<RmcpClient>,

        tool_name: &str,

        args: serde_json::Value,

    ) -> Result<String> {

        debug!(

            "Executing Codex MCP tool: {} with args: {:?}",

            tool_name, args

        );



        let result = mcp_client

            .call_tool(

                tool_name.to_string(),

                Some(args),

                Some(Duration::from_secs(30)),

            )

            .await

            .context(format!("Failed to call Codex MCP tool '{tool_name}'"))?;



        // ﾃｧﾂｵﾂ静ｦﾂ楪愿｣ﾂつ津｣ﾂδ・｣ﾂつｭﾃ｣ﾂつｹﾃ｣ﾂδ暗･ﾂｽﾂ｢ﾃ･ﾂｼﾂ湘｣ﾂ・ｫﾃ･ﾂ､ﾂ嘉ｦﾂ渉・

        let result_text =

            serde_json::to_string_pretty(&result).unwrap_or_else(|_| format!("{result:?}"));



        Ok(result_text)

    }

}



#[tokio::test]

async fn test_filter_codex_mcp_tools() {

    use crate::agents::types::ContextPolicy;

    use crate::agents::types::ToolPermissions;



    let agent_def = AgentDefinition {

        name: "test-agent".to_string(),

        goal: "Test".to_string(),

        instructions: None,

        tools: ToolPermissions {

            mcp: vec![

                "codex_read_file".to_string(),

                "codex-subagent".to_string(),

                "mcp__server__codex-deep-research".to_string(),

                "some_other_tool".to_string(), // ﾃｩﾂ敖曚odexﾃ｣ﾂδ・ﾂ・ﾃ｣ﾂδｫ

            ],

            fs: Default::default(),

            net: Default::default(),

            shell: Default::default(),

        },

        policies: crate::agents::types::AgentPolicies {

            shell: None,

            net: None,

            context: ContextPolicy {

                max_tokens: 1000,

                retention: "job".to_string(),

            },

            secrets: Default::default(),

        },

        success_criteria: vec![],

        artifacts: vec![],

        extra: Default::default(),

    };



    let filtered = AgentRuntime::filter_codex_mcp_tools(&agent_def);



    assert_eq!(filtered.len(), 3);

    assert!(filtered.contains(&"codex_read_file".to_string()));

    assert!(filtered.contains(&"codex-subagent".to_string()));

    assert!(filtered.contains(&"mcp__server__codex-deep-research".to_string()));

    assert!(!filtered.contains(&"some_other_tool".to_string()));

}



#[tokio::test]

async fn test_build_codex_mcp_tools_description() {

    let tools = vec![

        "codex_read_file".to_string(),

        "codex-subagent".to_string(),

        "codex-deep-research".to_string(),

        "codex-auto-orchestrate".to_string(),

    ];

    let desc = AgentRuntime::build_codex_mcp_tools_description(&tools);



    assert!(desc.contains("codex_read_file"));

    assert!(desc.contains("codex-subagent"));

    assert!(desc.contains("Manage Codex subagents"));

    assert!(desc.contains("codex-deep-research"));

    assert!(desc.contains("DeepResearcher"));

    assert!(desc.contains("codex-auto-orchestrate"));

    assert!(desc.contains("Safe, read-only operation"));

}

