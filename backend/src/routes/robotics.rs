use actix_web::web;
use crate::controllers::robotics_ctrl;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/robotics")
            .route("/devices", web::get().to(robotics_ctrl::get_devices))
            .route("/devices", web::post().to(robotics_ctrl::register_device))
            .route("/devices/{device_id}", web::get().to(robotics_ctrl::get_device))
            .route("/devices/{device_id}", web::delete().to(robotics_ctrl::delete_device))
            .route("/devices/{device_id}/command", web::post().to(robotics_ctrl::send_command))
            .route("/devices/{device_id}/status", web::patch().to(robotics_ctrl::update_status))
            .route("/devices/{device_id}/telemetry", web::get().to(robotics_ctrl::get_telemetry))
            .route("/health", web::get().to(robotics_ctrl::health_check))
    );
}
