use codex_protocol::models::ResponseItem;
use serde_json::Value;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Compression {
    #[default]
    None,
    Zstd,
}

pub(crate) fn attach_item_ids(payload_json: &mut Value, original_items: &[ResponseItem]) {

// --- Fork custom additions ---
pub struct ResponsesRequest {
    pub body: Value,
    pub headers: HeaderMap,
    pub compression: Compression,
#[derive(Default)]
pub struct ResponsesRequestBuilder<'a> {
    model: Option<&'a str>,
    instructions: Option<&'a str>,
    input: Option<&'a [ResponseItem]>,
    tools: Option<&'a [Value]>,
    parallel_tool_calls: bool,
    reasoning: Option<Reasoning>,
    include: Vec<String>,
    prompt_cache_key: Option<String>,
    text: Option<TextControls>,
    conversation_id: Option<String>,
    session_source: Option<SessionSource>,
    store_override: Option<bool>,
    headers: HeaderMap,
    compression: Compression,
impl<'a> ResponsesRequestBuilder<'a> {
    pub fn new(model: &'a str, instructions: &'a str, input: &'a [ResponseItem]) -> Self {
        Self {
            model: Some(model),
            instructions: Some(instructions),
            input: Some(input),
            ..Default::default()
    pub fn tools(mut self, tools: &'a [Value]) -> Self {
        self.tools = Some(tools);
        self
    pub fn parallel_tool_calls(mut self, enabled: bool) -> Self {
        self.parallel_tool_calls = enabled;
        self
    pub fn reasoning(mut self, reasoning: Option<Reasoning>) -> Self {
        self.reasoning = reasoning;
        self
    pub fn include(mut self, include: Vec<String>) -> Self {
        self.include = include;
        self
    pub fn prompt_cache_key(mut self, key: Option<String>) -> Self {
        self.prompt_cache_key = key;
        self
    pub fn text(mut self, text: Option<TextControls>) -> Self {
        self.text = text;
        self
    pub fn conversation(mut self, conversation_id: Option<String>) -> Self {
        self.conversation_id = conversation_id;
        self
    pub fn session_source(mut self, source: Option<SessionSource>) -> Self {
        self.session_source = source;
        self
    pub fn store_override(mut self, store: Option<bool>) -> Self {
        self.store_override = store;
        self
    pub fn extra_headers(mut self, headers: HeaderMap) -> Self {
        self.headers = headers;
        self
    pub fn compression(mut self, compression: Compression) -> Self {
        self.compression = compression;
        self
    #[allow(clippy::result_large_err)]
    pub fn build(self, provider: &Provider) -> Result<ResponsesRequest, ApiError> {
        let model = self
            .model
            .ok_or_else(|| ApiError::Stream("missing model for responses request".into()))?;
        let instructions = self
            .instructions
            .ok_or_else(|| ApiError::Stream("missing instructions for responses request".into()))?;
        let input = self
            .input
            .ok_or_else(|| ApiError::Stream("missing input for responses request".into()))?;
        let tools = self.tools.unwrap_or_default();
        let store = self
            .store_override
            .unwrap_or_else(|| provider.is_azure_responses_endpoint());
        let req = ResponsesApiRequest {
            model,
            instructions,
            input,
            tools,
            tool_choice: "auto",
            parallel_tool_calls: self.parallel_tool_calls,
            reasoning: self.reasoning,
            store,
            stream: true,
            include: self.include,
            prompt_cache_key: self.prompt_cache_key,
            text: self.text,
        let mut body = serde_json::to_value(&req)
            .map_err(|e| ApiError::Stream(format!("failed to encode responses request: {e}")))?;
        if store && provider.is_azure_responses_endpoint() {
            attach_item_ids(&mut body, input);
        let mut headers = self.headers;
        headers.extend(build_conversation_headers(self.conversation_id));
        if let Some(subagent) = subagent_header(&self.session_source) {
            insert_header(&mut headers, "x-openai-subagent", &subagent);
        Ok(ResponsesRequest {
            body,
            headers,
            compression: self.compression,
fn attach_item_ids(payload_json: &mut Value, original_items: &[ResponseItem]) {
// --- End fork additions ---
    let Some(input_value) = payload_json.get_mut("input") else {
        return;
    };
    let Value::Array(items) = input_value else {
        return;
    };

    for (value, item) in items.iter_mut().zip(original_items.iter()) {
        if let ResponseItem::Reasoning { id, .. }
        | ResponseItem::Message { id: Some(id), .. }
        | ResponseItem::WebSearchCall { id: Some(id), .. }
        | ResponseItem::FunctionCall { id: Some(id), .. }
        | ResponseItem::LocalShellCall { id: Some(id), .. }
        | ResponseItem::CustomToolCall { id: Some(id), .. } = item
        {
            if id.is_empty() {
                continue;
            }

            if let Some(obj) = value.as_object_mut() {
                obj.insert("id".to_string(), Value::String(id.clone()));
            }
        }
    }
}
