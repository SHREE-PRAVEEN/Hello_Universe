use actix_web::web;
use crate::controllers::dashboard_ctrl;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/dashboard")
            .route("/overview", web::get().to(dashboard_ctrl::get_overview))
            .route("/activity", web::get().to(dashboard_ctrl::get_activity))
            .route("/quick-stats", web::get().to(dashboard_ctrl::get_quick_stats))
            .route("/public-stats", web::get().to(dashboard_ctrl::get_public_stats))
    );
}
