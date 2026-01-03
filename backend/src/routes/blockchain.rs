use actix_web::web;
use crate::controllers::blockchain_ctrl;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/blockchain")
            .route("/nonce", web::post().to(blockchain_ctrl::get_nonce))
            .route("/verify-signature", web::post().to(blockchain_ctrl::verify_signature))
            .route("/link-wallet", web::post().to(blockchain_ctrl::link_wallet))
            .route("/transactions", web::get().to(blockchain_ctrl::get_transactions))
            .route("/payment", web::post().to(blockchain_ctrl::create_payment))
            .route("/verify-tx/{tx_hash}", web::get().to(blockchain_ctrl::verify_transaction))
            .route("/balance", web::get().to(blockchain_ctrl::get_balance))
            .route("/health", web::get().to(blockchain_ctrl::health_check))
    );
}
