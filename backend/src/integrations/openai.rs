use crate::{
    models::ai::AiTagItem,
    utils::errors::{AppError, AppResult},
};
use serde::{Deserialize, Serialize};

pub struct OpenAiClient {
    api_key: String,
    model: String,
    http: reqwest::Client,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
    response_format: ResponseFormat,
}

#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ResponseFormat {
    #[serde(rename = "type")]
    kind: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ChoiceMessage,
}

#[derive(Deserialize)]
struct ChoiceMessage {
    content: String,
}

#[derive(Deserialize)]
struct TagResponse {
    tags: Vec<TagItem>,
}

#[derive(Deserialize)]
struct TagItem {
    tag: String,
    confidence: f64,
}

impl OpenAiClient {
    pub fn new(api_key: &str, model: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            model: model.to_string(),
            http: reqwest::Client::new(),
        }
    }

    /// Extract tags from content using GPT
    pub async fn extract_tags(&self, content: &str) -> AppResult<Vec<AiTagItem>> {
        let system = "You are an expert technical content tagger for a robotics, AI, and engineering platform. \
                      Extract relevant tags from the given content. Return ONLY a JSON object with a 'tags' array. \
                      Each tag has 'tag' (string, lowercase, max 3 words) and 'confidence' (0.0-1.0). \
                      Return 5-15 tags. No markdown, no code blocks, just raw JSON.";

        let user_prompt = format!(
            "Extract technical tags from this content:\n\n{}",
            &content[..content.len().min(2000)]
        );

        let body = ChatRequest {
            model: self.model.clone(),
            messages: vec![
                ChatMessage { role: "system".into(), content: system.into() },
                ChatMessage { role: "user".into(), content: user_prompt },
            ],
            temperature: 0.2,
            response_format: ResponseFormat { kind: "json_object".into() },
        };

        let resp = self.http
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("OpenAI request failed: {}", e)))?;

        if !resp.status().is_success() {
            let err = resp.text().await.unwrap_or_default();
            return Err(AppError::Internal(anyhow::anyhow!("OpenAI error: {}", err)));
        }

        let chat_resp: ChatResponse = resp.json().await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("OpenAI parse error: {}", e)))?;

        let raw_content = chat_resp.choices
            .first()
            .map(|c| c.message.content.as_str())
            .unwrap_or("{}");

        let tag_resp: TagResponse = serde_json::from_str(raw_content)
            .unwrap_or(TagResponse { tags: vec![] });

        Ok(tag_resp.tags.into_iter()
            .map(|t| AiTagItem { tag: t.tag, confidence: t.confidence })
            .collect())
    }

    /// Generate embeddings for semantic search
    pub async fn embed(&self, text: &str, model: &str) -> AppResult<Vec<f32>> {
        #[derive(Serialize)]
        struct EmbedRequest { model: String, input: String }
        #[derive(Deserialize)]
        struct EmbedResponse { data: Vec<EmbedData> }
        #[derive(Deserialize)]
        struct EmbedData { embedding: Vec<f32> }

        let body = EmbedRequest { model: model.to_string(), input: text.to_string() };
        let resp = self.http
            .post("https://api.openai.com/v1/embeddings")
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("OpenAI embed failed: {}", e)))?;

        let embed_resp: EmbedResponse = resp.json().await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("OpenAI embed parse: {}", e)))?;

        Ok(embed_resp.data.into_iter()
            .next()
            .map(|d| d.embedding)
            .unwrap_or_default())
    }
}
