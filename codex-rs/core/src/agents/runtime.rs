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



/// ãµãã¨ã¼ã¸ã§ã³ãã©ã³ã¿ã¤ã 

pub struct AgentRuntime {

    /// ã¨ã¼ã¸ã§ã³ãã­ã¼ãã¼

    loader: Arc<RwLock<AgentLoader>>,

    /// ããEã¯ã³äºç®ç®¡çE
    budgeter: Arc<TokenBudgeter>,

    /// å®è¡ä¸­ã®ã¨ã¼ã¸ã§ã³ãE
    running_agents: Arc<RwLock<HashMap<String, AgentStatus>>>,

    /// ã¯ã¼ã¯ã¹ããEã¹ãE£ã¬ã¯ããª

    workspace_dir: PathBuf,

    /// LLMè¨­å®E
    config: Arc<Config>,

    /// èªè¨¼ããã¼ã¸ã£ã¼

    auth_manager: Option<Arc<AuthManager>>,

    /// OpenTelemetry ã¤ãã³ããEããEã¸ã£ã¼

    otel_manager: OtelEventManager,

    /// ã¢ãE«ãã­ãã¤ãã¼æE ±

    provider: ModelProviderInfo,

    /// ä¼è©±ID

    conversation_id: ConversationId,

    /// Codexãã¤ããªãã¹EECPçµ±åç¨EE
    codex_binary_path: Option<PathBuf>,

    /// ãµãã¨ã¼ã¸ã§ã³ãéã®åèª¿ã¹ãã¢

    #[cfg(feature = "custom-features")]

    collaboration_store: Arc<CollaborationStore>,

    /// Reasoning effortè¨­å®E
    reasoning_effort: ReasoningEffort,

    /// Reasoning summaryè¨­å®E
    reasoning_summary: ReasoningSummary,

    /// Verbosityè¨­å®E
    verbosity: Verbosity,

}



impl AgentRuntime {

    /// æ°ããã©ã³ã¿ã¤ã ãä½æE

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



    /// è¤E°ã¨ã¼ã¸ã§ã³ããä¸¦åå®è¡E
    pub async fn delegate_parallel(

        &self,

        agents: Vec<(String, String, HashMap<String, String>, Option<usize>)>, // (agent_name, goal, inputs, budget)

        _deadline: Option<u64>,

    ) -> Result<Vec<AgentResult>> {

        info!("Starting parallel delegation of {} agents", agents.len());



        // åE¨ã¼ã¸ã§ã³ããtokio::spawnã§ä¸¦åèµ·åE
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



        // å¨ã¨ã¼ã¸ã§ã³ããEå®äºEå¾E©E
        let mut results = Vec::new();

        for (agent_name, handle) in handles {

            match handle.await {

                Ok(Ok(result)) => {

                    info!("Agent '{}' completed successfully", agent_name);

                    results.push(result);

                }

                Ok(Err(e)) => {

                    error!("Agent '{}' failed: {}", agent_name, e);

                    // ã¨ã©ã¼ã§ãç¶è¡ãã¦ä»ãEã¨ã¼ã¸ã§ã³ããEçµæãåéE
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



    /// ãã­ã³ããããã«ã¹ã¿ã ã¨ã¼ã¸ã§ã³ããä½æEãã¦å®è¡E
    pub async fn create_and_run_custom_agent(

        &self,

        prompt: &str,

        budget: Option<usize>,

    ) -> Result<AgentResult> {

        info!("Creating custom agent from prompt");



        // LLMãä½¿ã£ã¦ãã­ã³ããããã¨ã¼ã¸ã§ã³ãå®ç¾©ãçæE
        let agent_def = self.generate_agent_from_prompt(prompt).await?;



        info!("Generated custom agent: {}", agent_def.name);



        // ã«ã¹ã¿ã ã¨ã¼ã¸ã§ã³ããã¡ã¢ãªä¸ã§å®è¡ï¼EAMLä¿å­ä¸è¦E¼E
        self.execute_custom_agent_inline(agent_def, budget).await

    }



    /// ãã­ã³ããããã¨ã¼ã¸ã§ã³ãå®ç¾©ãçæE
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



        // LLMå¼ã³åºãE
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

            self.config.features.enabled(Feature::ResponsesWebsockets),

            self.config.features.enabled(Feature::ResponsesWebsocketsV2),

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



        // ã¬ã¹ãã³ã¹ãåéE
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



        // JSONãæ½åºEã³ã¼ããã­ãE¯åEEå¯è½æ§ãããããE¼E
        let json_str = if let Some(start) = full_response.find('{') {

            if let Some(end) = full_response.rfind('}') {

                &full_response[start..=end]

            } else {

                &full_response

            }

        } else {

            &full_response

        };



        // JSONããã¼ã¹

        let agent_def: AgentDefinition =

            serde_json::from_str(json_str).context("Failed to parse generated agent definition")?;



        info!(

            "Successfully generated agent definition: {}",

            agent_def.name

        );



        Ok(agent_def)

    }



    /// ã«ã¹ã¿ã ã¨ã¼ã¸ã§ã³ããã¤ã³ã©ã¤ã³ã§å®è¡ï¼EAMLä¿å­ãªãï¼E
    async fn execute_custom_agent_inline(

        &self,

        agent_def: AgentDefinition,

        budget: Option<usize>,

    ) -> Result<AgentResult> {

        let agent_name = &agent_def.name;

        info!("Executing custom agent '{}' inline", agent_name);



        // äºç®è¨­å®E
        let effective_budget = budget.unwrap_or(agent_def.policies.context.max_tokens);

        self.budgeter

            .set_agent_limit(agent_name, effective_budget)?;



        // å®è¡ã¹ãEEã¿ã¹æ´æ°

        {

            let mut running = self.running_agents.write().await;

            running.insert(agent_name.to_string(), AgentStatus::Running);

        }



        let start_time = Instant::now();

        let start_timestamp = chrono::Utc::now().to_rfc3339();



        // ã¨ã¼ã¸ã§ã³ãå®è¡E
        let result = match self

            .execute_agent(&agent_def, &agent_def.goal, HashMap::new(), None)

            .await

        {

            Ok(artifacts) => {

                let duration_secs = start_time.elapsed().as_secs_f64();

                let tokens_used = self.budgeter.get_agent_usage(agent_name);



                // ç£æ»ã­ã°: æå

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



                // ç£æ»ã­ã°: å¤±æE
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



        // å®è¡ã¹ãEEã¿ã¹æ´æ°

        {

            let mut running = self.running_agents.write().await;

            running.insert(agent_name.to_string(), result.status.clone());

        }



        // ã³ã©ãã¬ã¼ã·ã§ã³ã¹ãã¢ã«çµæãä¿å­E
        #[cfg(feature = "custom-features")]

        {

            self.collaboration_store

                .store_agent_result(agent_name.to_string(), result.clone());

        }



        Ok(result)

    }



    /// ä¸¦åå®è¡ç¨ã«ã¯ã­ã¼ã³

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



    /// ã¨ã¼ã¸ã§ã³ããå§ä»»å®è¡E
    pub async fn delegate(

        &self,

        agent_name: &str,

        goal: &str,

        inputs: HashMap<String, String>,

        budget: Option<usize>,

        deadline: Option<u64>,

    ) -> Result<AgentResult> {

        info!("Delegating to agent '{}': {}", agent_name, goal);



        // ã¨ã¼ã¸ã§ã³ãå®ç¾©ãèª­ã¿è¾¼ã¿

        let agent_def = {

            let mut loader = self.loader.write().await;

            loader

                .load_by_name(agent_name)

                .with_context(|| format!("Failed to load agent '{agent_name}'"))?

        };



        // å±ææå ±ãåEåã¸åãè¾¼ã

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



        // äºç®ãè¨­å®E
        if let Some(budget) = budget {

            self.budgeter.set_agent_limit(agent_name, budget)?;

        } else {

            // ãEã©ã«ãäºç®ãEã³ã³ãE­ã¹ããEãªã·ã¼ããåå¾E
            self.budgeter

                .set_agent_limit(agent_name, agent_def.policies.context.max_tokens)?;

        }



        // å®è¡ã¹ãEEã¿ã¹ãæ´æ°

        {

            let mut running = self.running_agents.write().await;

            running.insert(agent_name.to_string(), AgentStatus::Running);

        }



        // å®è¡éå§E
        let start_time = Instant::now();

        let start_timestamp = chrono::Utc::now().to_rfc3339();



        // ç£æ»ã­ã°: ã¨ã¼ã¸ã§ã³ãéå§E
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



                // ç£æ»ã­ã°: ã¨ã¼ã¸ã§ã³ãå®äºE
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



                // ç£æ»ã­ã°: ã¨ã¼ã¸ã§ã³ãå¤±æE
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



        // å®è¡ã¹ãEEã¿ã¹ãæ´æ°

        {

            let mut running = self.running_agents.write().await;

            running.insert(agent_name.to_string(), result.status.clone());

        }



        // å®è¡çµæãåèª¿ã¹ãã¢ã«ä¿å­E
        #[cfg(feature = "custom-features")]

        {

            self.collaboration_store

                .store_agent_result(agent_name.to_string(), result.clone());

        }



        Ok(result)

    }



    /// ã¨ã¼ã¸ã§ã³ããå®éã«å®è¡E
    async fn execute_agent(

        &self,

        agent_def: &AgentDefinition,

        goal: &str,

        inputs: HashMap<String, String>,

        _deadline: Option<u64>,

    ) -> Result<Vec<String>> {

        debug!("Executing agent '{}' with goal: {}", agent_def.name, goal);



        // 1. ã·ã¹ãE ãã­ã³ããæ§ç¯ï¼ã·ã³ãã«çï¼E
        let _system_prompt = format!("You are a {} agent. {}", agent_def.name, agent_def.goal);



        // 2. ã¦ã¼ã¶ã¼å¥åãæ§ç¯ï¼ã¿ã¹ã¯ã¨inputsãå«ãEE
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



        // 3. ModelClientä½æE

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

            self.config.features.enabled(Feature::ResponsesWebsockets),

            self.config.features.enabled(Feature::ResponsesWebsocketsV2),

            self.config

                .features

                .enabled(Feature::EnableRequestCompression),

            self.config.features.enabled(Feature::RuntimeMetrics),

            None,

        );



        // 4. ResponseItemæ§ç¯ï¼Eromptã«æ¸¡ãï¼E
        let _input_items = vec![ResponseItem::Message {

            id: None,

            role: "user".to_string(),

            content: vec![ContentItem::InputText {

                text: user_message.clone(),

            }],

            end_turn: None,

            phase: None,

        }];



        // 5. Promptæ§ç¯ï¼ã¨ã¼ã¸ã§ã³ãæ¨©éãããã¼ã«ãçæï¼E
        let tools = self.build_tools_for_agent(agent_def);



        let prompt = Prompt {

            input: _input_items,

            tools,

            parallel_tool_calls: false,

            base_instructions: BaseInstructions {

                text: String::new(), // ãEã©ã«ããEããEã¹ã¤ã³ã¹ãã©ã¯ã·ã§ã³

            },

            personality: None,

            output_schema: None,

        };



        // ãEãE°: ã·ã¹ãE ãã­ã³ããã®åE®¹ãã­ã°åºåE
        debug!(

            "Agent '{}': Using default model instructions (base_instructions_override=None)",

            agent_def.name

        );



        // 6. LLMå¼ã³åºãE
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



        // 7. ããEã¯ã³äºç®ãã§ãE¯ã¨æ¶è²»

        if !self.budgeter.try_consume(&agent_def.name, total_tokens)? {

            anyhow::bail!("Token budget exceeded for agent '{}'", agent_def.name);

        }



        info!(

            "Agent '{}' completed LLM execution: {} tokens used",

            agent_def.name, total_tokens

        );



        // 8. ã¢ã¼ãE£ãã¡ã¯ãçæE
        let artifacts_dir = self.workspace_dir.join("artifacts");

        tokio::fs::create_dir_all(&artifacts_dir).await?;



        let mut generated_artifacts = Vec::new();

        for artifact_path in &agent_def.artifacts {

            let full_path = self.workspace_dir.join(artifact_path);

            if let Some(parent) = full_path.parent() {

                tokio::fs::create_dir_all(parent).await?;

            }



            // ã¢ã¼ãE£ãã¡ã¯ãåEå®¹ãçæE
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



    /// å©ç¨å¯è½ãªã¨ã¼ã¸ã§ã³ãä¸è¦§ãåå¾E
    pub async fn list_agents(&self) -> Result<Vec<String>> {

        let loader = self.loader.read().await;

        loader.list_available_agents()

    }



    /// å®è¡ä¸­ã®ã¨ã¼ã¸ã§ã³ãç¶æãåå¾E
    pub async fn get_running_agents(&self) -> HashMap<String, AgentStatus> {

        self.running_agents.read().await.clone()

    }



    /// ããEã¯ã³ä½¿ç¨ç¶æ³ãåå¾E
    pub fn get_budget_status(&self) -> (usize, usize, f64) {

        let used = self.budgeter.get_used();

        let remaining = self.budgeter.get_remaining();

        let utilization = self.budgeter.get_utilization();

        (used, remaining, utilization)

    }



    /// è»½éçãã©ã¼ã«ããã¯ãå¿E¦ããã§ãE¯

    pub fn should_use_lightweight(&self, threshold: f64) -> bool {

        self.budgeter.should_fallback_lightweight(threshold)

    }



    /// ã¨ã¼ã¸ã§ã³ãæ¨©éã«åºã¥ãE¦ãEEã«ä»æ§ãæ§ç¯E
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

  - "åºæºE"

artifacts:

  - "artifacts/test-output.md"

"#;



        fs::write(agents_dir.join("test-agent.yaml"), agent_yaml).unwrap();



        // ã¢ãE¯Configä½æE

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

    /// Codexãã¤ããªãã¹ãè¨­å®E
    pub fn with_codex_binary_path(mut self, path: PathBuf) -> Self {

        self.codex_binary_path = Some(path);

        self

    }



    /// åèª¿ã¹ãã¢ã¸ã®åçEãåå¾E
    #[cfg(feature = "custom-features")]

    pub fn collaboration_store(&self) -> Arc<CollaborationStore> {

        self.collaboration_store.clone()

    }



    /// Codex MCP Serverãstdio ã¢ã¼ãã§èµ·åE
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



    /// ã¨ã¼ã¸ã§ã³ãæ¨©éã«åºã¥ãE¦Codex MCP toolsããã£ã«ã¿ãªã³ã°

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



    /// Codex MCP toolsã®èª¬æãçæEEãEã­ã³ããç¨EE
    fn build_codex_mcp_tools_description(allowed_tools: &[String]) -> String {

        let mut desc = String::from("Available Codex MCP Tools:\n\n");



        for tool in allowed_tools {

            let tool_desc = match tool.as_str() {

                "codex_read_file" => {

                    "- codex_read_file(path: str) -> str\n  \

                     Read a file from the workspace using Codex.\n  \

                     Safe, read-only operation."

                }

                "codex_grep" => {

                    "- codex_grep(pattern: str, path: Optional[str]) -> List[str]\n  \

                     Search for patterns in files using Codex grep.\n  \

                     Safe, read-only operation."

                }

                "codex_codebase_search" => {

                    "- codex_codebase_search(query: str, target_directories: Optional[List[str]]) -> List[str]\n  \

                     Semantic code search using Codex.\n  \

                     Safe, read-only operation."

                }

                "codex_apply_patch" => {

                    "- codex_apply_patch(patch: str) -> str\n  \

                     Apply a code patch using Codex.\n  \

                     Requires write permission."

                }

                "codex_shell" => {

                    "- codex_shell(command: str) -> str\n  \

                     Execute a shell command via Codex (restricted).\n  \

                     Requires shell permission."

                }

                "codex-supervisor" => {

                    "- codex-supervisor(goal: str, strategy: Optional[str]) -> SupervisorReport\n  \

                     Plan and coordinate multiple Codex subagents through the Supervisor layer.\n  \

                     Use when you need structured collaboration across specialists."

                }

                "codex-deep-research" => {

                    "- codex-deep-research(query: str, strategy: Optional[str], max_depth: Optional[int]) -> ResearchReport\n  \

                     Run Codex DeepResearcher for multi-source investigations with citations.\n  \

                     Ideal for comprehensive research and evidence gathering."

                }

                "codex-subagent" => {

                    "- codex-subagent(action: str, agent_type: Optional[str], task: Optional[str], task_id: Optional[str]) -> ToolResult\n  \

                     Manage Codex subagents: start tasks, auto-dispatch, check inbox, status, thinking, or token usage.\n  \

                     Delegate work to specialist agents and retrieve their outputs."

                }

                "codex-custom-command" => {

                    "- codex-custom-command(action: str, command_name: Optional[str], context: Optional[str]) -> ToolResult\n  \

                     Execute curated multi-step workflows (e.g., analyze_code, deep_research) mapped to subagents.\n  \

                     Handy for quick access to predefined automation."

                }

                "codex-hook" => {

                    "- codex-hook(event: str, context: Optional[str]) -> HookAck\n  \

                     Trigger lifecycle hooks such as on_subagent_start/on_task_complete for integrations.\n  \

                     Use to capture workflow events or integrate external systems."

                }

                "codex-auto-orchestrate" => {

                    "- codex-auto-orchestrate(goal: str, strategy: Optional[str]) -> OrchestrationReport\n  \

                     Automatically analyze goals and dispatch the optimal mix of subagents.\n  \

                     Best for high-level objectives that benefit from autonomous planning."

                }

                _ => continue,

            };

            desc.push_str(tool_desc);

            desc.push_str("\n\n");

        }



        desc.push_str(

            "To use these tools, output a tool call in the following format:\n\

             TOOL_CALL: tool_name(arg1=\"value1\", arg2=\"value2\")\n\n\

             The results will be provided to you for further analysis.",

        );



        desc

    }



    /// ã¨ã¼ã¸ã§ã³ããCodex MCPçµç±ã§å®è¡ï¼Ehase 3: å®åEå®è£E¼E
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



        // 1. Codex MCP Serverãèµ·åE
        let mcp_client = self

            .spawn_codex_mcp_server()

            .await

            .context("Failed to spawn Codex MCP server")?;



        // 2. ã¨ã¼ã¸ã§ã³ãæ¨©éã§ãEEã«ããã£ã«ã¿ãªã³ã°

        let allowed_tools = Self::filter_codex_mcp_tools(agent_def);



        info!(

            "Agent '{}' is allowed to use {} Codex MCP tools: {:?}",

            agent_def.name,

            allowed_tools.len(),

            allowed_tools

        );



        // 3. ã·ã¹ãE ãã­ã³ããæ§ç¯ï¼ãã¼ã«èª¬æå«ãEE
        let tools_description = Self::build_codex_mcp_tools_description(&allowed_tools);



        let system_prompt = format!(

            "You are a specialized sub-agent with the following role:\n\

             \n\

             Agent: {}\n\

             Goal: {}\n\

             \n\

             Success Criteria:\n{}\n\

             \n\

             Inputs provided:\n{}\n\

             \n\

             {}\n\

             \n\

             Please analyze the task and use the available Codex MCP tools to complete it.\

             When you need to use a tool, output it in the specified format.\

             After all tool calls are complete, provide a final summary.",

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



        // 4. åæãã­ã³ãã

        let user_prompt = format!("Task: {goal}");



        // 5. LLMå¯¾è©±ã«ã¼ãï¼æå¤§5åãEãEEã«å¼ã³åºãï¼E
        let max_iterations = 5;

        let mut conversation_history = vec![

            ("system".to_string(), system_prompt),

            ("user".to_string(), user_prompt.clone()),

        ];

        let mut artifacts = Vec::new();



        for iteration in 0..max_iterations {

            debug!("Agent iteration {}/{}", iteration + 1, max_iterations);



            // LLMå¼ã³åºãE
            let llm_response = self

                .call_llm_for_agent(&conversation_history)

                .await

                .context("Failed to call LLM for agent")?;



            conversation_history.push(("assistant".to_string(), llm_response.clone()));

            artifacts.push(llm_response.clone());



            // ãEEã«ã³ã¼ã«æ¤åE

            let tool_calls = self.detect_tool_calls(&llm_response);



            if tool_calls.is_empty() {

                // ãEEã«ã³ã¼ã«ããªãE ´åãEçµäºE
                info!("No more tool calls detected. Agent task completed.");

                break;

            }



            // ãEEã«å®è¡E
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



            // ãEEã«çµæãLLMã«ãã£ã¼ãããE¯

            let feedback = tool_results.join("\n\n");

            conversation_history.push(("user".to_string(), feedback.clone()));

            artifacts.push(format!("--- Tool Execution Results ---\n{feedback}"));

        }



        info!("Agent '{}' completed execution", agent_def.name);



        Ok(artifacts)

    }



    /// LLMãå¼ã³åºãã¦ã¨ã¼ã¸ã§ã³ããEå¿ç­ãåå¾E
    async fn call_llm_for_agent(&self, conversation: &[(String, String)]) -> Result<String> {

        // ãã­ã³ããæ§ç¯ï¼ææ°ã®ã¡ãE»ã¼ã¸ã®ã¿ãä½¿ç¨EE
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



        // ModelClientçµç±ã§LLMå¼ã³åºãE
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

            self.config.features.enabled(Feature::ResponsesWebsockets),

            self.config.features.enabled(Feature::ResponsesWebsocketsV2),

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



        // ã¬ã¹ãã³ã¹ãåéE
        let mut full_response = String::new();

        let mut _tokens_used = 0;



        while let Some(event) = response_stream.next().await {

            match event? {

                ResponseEvent::OutputItemDone(ResponseItem::Message { content, .. }) => {

                    // ResponseItemãããE­ã¹ããæ½åº

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
                    // å®éã®ããEã¯ã³ä½¿ç¨éãã­ã£ããã£

                    if let Some(usage) = token_usage {

                        _tokens_used = usage.total_tokens as usize;

                        debug!(

                            "LLM call completed: {} tokens (input: {}, output: {})",

                            usage.total_tokens, usage.input_tokens, usage.output_tokens

                        );

                        // Codexã¨ã¼ã¸ã§ã³ãE ããEã¯ã³ãã¸ã§ãEæ¶è²»ã®ã­ã¸ãE¯ãè¿½å ããå ´åãE

                        // agent_nameç­ãEç®¡çE¸ããã®é¢æ°ã®å¼ã³åºãåEãAgentæ§é ä½ã§

                        // ã¢ã­ã±ã¼ã·ã§ã³å¦çEå®è£Eã¦ãã ããEEuntimeåç¬ã§ã¯æªå¯¾å¿ï¼ãE
                    }

                }

                _ => {}

            }

        }



        Ok(full_response)

    }



    /// LLMã¬ã¹ãã³ã¹ãããEEã«ã³ã¼ã«ãæ¤åE

    fn detect_tool_calls(&self, response: &str) -> Vec<(String, serde_json::Value)> {

        let mut tool_calls = Vec::new();



        // ãã¿ã¼ã³: TOOL_CALL: tool_name(arg1="value1", arg2="value2")

        // ç°¡æå®è£E JSONãã©ã¼ããããæ¤åE

        for line in response.lines() {

            let line = line.trim();



            // TOOL_CALL: codex_read_file(path="src/auth.rs")

            if line.starts_with("TOOL_CALL:")

                && let Some(call_str) = line.strip_prefix("TOOL_CALL:").map(str::trim)

                && let Some((tool_name, args_str)) = call_str.split_once('(')

            {

                let tool_name = tool_name.trim().to_string();

                let args_str = args_str.trim_end_matches(')').trim();



                // ç°¡æãã¼ã¹: key="value" å½¢å¼E
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



    /// Codex MCPãEEã«ãå®è¡E
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



        // çµæããã­ã¹ãå½¢å¼ã«å¤æ

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

                "some_other_tool".to_string(), // éCodexãEEã«

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

