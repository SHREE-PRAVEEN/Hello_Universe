use serde::{Deserialize, Serialize};
use crate::errors::{ApiError, ApiResult};

/// AI Service for handling AI-related operations
pub struct AIService {
    api_key: Option<String>,
    base_url: String,
}

impl AIService {
    pub fn new() -> Self {
        Self {
            api_key: std::env::var("AI_API_KEY").ok(),
            base_url: std::env::var("AI_API_URL")
                .unwrap_or_else(|_| "https://api.openai.com/v1".to_string()),
        }
    }

    /// Check if AI service is configured
    pub fn is_configured(&self) -> bool {
        self.api_key.is_some()
    }

    /// Generate chat completion
    pub async fn chat_completion(&self, request: &ChatRequest) -> ApiResult<ChatResponse> {
        let api_key = self.api_key.as_ref()
            .ok_or_else(|| ApiError::AIServiceError("AI service not configured".to_string()))?;

        let client = reqwest::Client::new();
        
        let payload = serde_json::json!({
            "model": request.model.as_deref().unwrap_or("gpt-3.5-turbo"),
            "messages": request.messages,
            "temperature": request.temperature.unwrap_or(0.7),
            "max_tokens": request.max_tokens.unwrap_or(1000),
        });

        let response = client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| ApiError::AIServiceError(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(ApiError::AIServiceError(format!("AI API error: {}", error_text)));
        }

        let api_response: OpenAIChatResponse = response.json().await
            .map_err(|e| ApiError::AIServiceError(format!("Failed to parse response: {}", e)))?;

        Ok(ChatResponse {
            id: api_response.id,
            message: api_response.choices.first()
                .map(|c| c.message.content.clone())
                .unwrap_or_default(),
            model: api_response.model,
            usage: api_response.usage.map(|u| TokenUsage {
                prompt_tokens: u.prompt_tokens,
                completion_tokens: u.completion_tokens,
                total_tokens: u.total_tokens,
            }),
        })
    }

    /// Generate text embeddings
    pub async fn generate_embeddings(&self, text: &str) -> ApiResult<Vec<f32>> {
        let api_key = self.api_key.as_ref()
            .ok_or_else(|| ApiError::AIServiceError("AI service not configured".to_string()))?;

        let client = reqwest::Client::new();
        
        let payload = serde_json::json!({
            "model": "text-embedding-ada-002",
            "input": text,
        });

        let response = client
            .post(format!("{}/embeddings", self.base_url))
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| ApiError::AIServiceError(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(ApiError::AIServiceError(format!("AI API error: {}", error_text)));
        }

        let api_response: EmbeddingResponse = response.json().await
            .map_err(|e| ApiError::AIServiceError(format!("Failed to parse response: {}", e)))?;

        api_response.data.first()
            .map(|d| d.embedding.clone())
            .ok_or_else(|| ApiError::AIServiceError("No embedding returned".to_string()))
    }

    /// Analyze code for robotics applications
    pub async fn analyze_robotics_code(&self, code: &str, language: &str) -> ApiResult<CodeAnalysis> {
        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: "You are an expert robotics and embedded systems engineer. Analyze the provided code for potential issues, optimizations, and safety concerns.".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: format!("Analyze this {} code for a robotics application:\n\n```{}\n{}\n```", language, language, code),
            },
        ];

        let request = ChatRequest {
            messages,
            model: Some("gpt-4".to_string()),
            temperature: Some(0.3),
            max_tokens: Some(2000),
        };

        let response = self.chat_completion(&request).await?;

        Ok(CodeAnalysis {
            analysis: response.message,
            suggestions: vec![],
            safety_concerns: vec![],
            optimization_tips: vec![],
        })
    }
}

impl Default for AIService {
    fn default() -> Self {
        Self::new()
    }
}

// Request/Response types
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub messages: Vec<ChatMessage>,
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub id: String,
    pub message: String,
    pub model: String,
    pub usage: Option<TokenUsage>,
}

#[derive(Debug, Serialize)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Serialize)]
pub struct CodeAnalysis {
    pub analysis: String,
    pub suggestions: Vec<String>,
    pub safety_concerns: Vec<String>,
    pub optimization_tips: Vec<String>,
}

// OpenAI API response structures
#[derive(Debug, Deserialize)]
struct OpenAIChatResponse {
    id: String,
    model: String,
    choices: Vec<OpenAIChoice>,
    usage: Option<OpenAIUsage>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
}

#[derive(Debug, Deserialize)]
struct OpenAIMessage {
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
}

#[derive(Debug, Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_service_creation() {
        let service = AIService::new();
        // Service should be created even without API key
        assert!(service.base_url.contains("openai"));
    }

    #[test]
    fn test_chat_message_serialization() {
        let msg = ChatMessage {
            role: "user".to_string(),
            content: "Hello".to_string(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("user"));
        assert!(json.contains("Hello"));
    }
}
