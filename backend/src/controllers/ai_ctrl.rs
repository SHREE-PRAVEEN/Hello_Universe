use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::errors::{ApiError, ApiResponse};
use crate::middleware::AuthenticatedUser;
use crate::services::ai_services::{AIService, ChatMessage, ChatRequest as ServiceChatRequest};

/// Chat completion request
#[derive(Debug, Deserialize)]
pub struct ChatCompletionRequest {
    pub messages: Vec<MessageInput>,
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct MessageInput {
    pub role: String,
    pub content: String,
}

/// Code analysis request
#[derive(Debug, Deserialize)]
pub struct CodeAnalysisRequest {
    pub code: String,
    pub language: String,
}

/// Embedding request
#[derive(Debug, Deserialize)]
pub struct EmbeddingRequest {
    pub text: String,
}

/// AI assistant chat response
#[derive(Debug, Serialize)]
pub struct AssistantResponse {
    pub message: String,
    pub model: String,
    pub tokens_used: Option<u32>,
}

/// Chat completion endpoint
pub async fn chat_completion(
    _user: AuthenticatedUser,
    body: web::Json<ChatCompletionRequest>,
) -> Result<HttpResponse, ApiError> {
    let ai_service = AIService::new();
    
    if !ai_service.is_configured() {
        return Err(ApiError::ServiceUnavailable("AI service not configured".to_string()));
    }

    let messages: Vec<ChatMessage> = body.messages.iter()
        .map(|m| ChatMessage {
            role: m.role.clone(),
            content: m.content.clone(),
        })
        .collect();

    let request = ServiceChatRequest {
        messages,
        model: body.model.clone(),
        temperature: body.temperature,
        max_tokens: body.max_tokens,
    };

    let response = ai_service.chat_completion(&request).await?;

    Ok(ApiResponse::success(AssistantResponse {
        message: response.message,
        model: response.model,
        tokens_used: response.usage.map(|u| u.total_tokens),
    }))
}

/// Analyze robotics code
pub async fn analyze_code(
    _user: AuthenticatedUser,
    body: web::Json<CodeAnalysisRequest>,
) -> Result<HttpResponse, ApiError> {
    let ai_service = AIService::new();
    
    if !ai_service.is_configured() {
        return Err(ApiError::ServiceUnavailable("AI service not configured".to_string()));
    }

    let analysis = ai_service.analyze_robotics_code(&body.code, &body.language).await?;

    Ok(ApiResponse::success(analysis))
}

/// Generate embeddings for text
pub async fn generate_embeddings(
    _user: AuthenticatedUser,
    body: web::Json<EmbeddingRequest>,
) -> Result<HttpResponse, ApiError> {
    let ai_service = AIService::new();
    
    if !ai_service.is_configured() {
        return Err(ApiError::ServiceUnavailable("AI service not configured".to_string()));
    }

    let embeddings = ai_service.generate_embeddings(&body.text).await?;

    Ok(ApiResponse::success(serde_json::json!({
        "embeddings": embeddings,
        "dimensions": embeddings.len()
    })))
}

/// Get available AI models
pub async fn get_models() -> Result<HttpResponse, ApiError> {
    let ai_service = AIService::new();
    
    Ok(ApiResponse::success(serde_json::json!({
        "available": ai_service.is_configured(),
        "models": [
            {
                "id": "gpt-3.5-turbo",
                "name": "GPT-3.5 Turbo",
                "description": "Fast and efficient for most tasks"
            },
            {
                "id": "gpt-4",
                "name": "GPT-4",
                "description": "Most capable model for complex tasks"
            },
            {
                "id": "text-embedding-ada-002",
                "name": "Embedding Ada",
                "description": "Text embedding model"
            }
        ]
    })))
}

/// AI service health check
pub async fn health_check() -> HttpResponse {
    let ai_service = AIService::new();
    
    HttpResponse::Ok().json(serde_json::json!({
        "service": "ai",
        "status": if ai_service.is_configured() { "available" } else { "not_configured" },
        "features": {
            "chat": ai_service.is_configured(),
            "embeddings": ai_service.is_configured(),
            "code_analysis": ai_service.is_configured()
        }
    }))
}
