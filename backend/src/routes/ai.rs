use actix_web::web;
use crate::controllers::ai_ctrl;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/ai")
            .route("/chat", web::post().to(ai_ctrl::chat_completion))
            .route("/analyze", web::post().to(ai_ctrl::analyze_code))
            .route("/embeddings", web::post().to(ai_ctrl::generate_embeddings))
            .route("/models", web::get().to(ai_ctrl::get_models))
            .route("/health", web::get().to(ai_ctrl::health_check))
    );
}
