use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::errors::{ApiError, ApiResult};

/// Robotics service for managing devices and commands
pub struct RoboticsService;

impl RoboticsService {
    pub fn new() -> Self {
        Self
    }

    /// Validate device command
    pub fn validate_command(&self, device_type: &str, command: &str) -> ApiResult<bool> {
        let valid_commands: &[&str] = match device_type {
            "drone" => &["takeoff", "land", "hover", "move", "rotate", "return_home", "emergency_stop"],
            "robot" => &["move_forward", "move_backward", "turn_left", "turn_right", "stop", "grab", "release"],
            "rover" => &["drive", "stop", "turn", "scan", "deploy_sensor", "retract_sensor"],
            _ => return Err(ApiError::ValidationError(format!("Unknown device type: {}", device_type))),
        };

        if valid_commands.contains(&command) {
            Ok(true)
        } else {
            Err(ApiError::ValidationError(format!(
                "Invalid command '{}' for device type '{}'. Valid commands: {:?}",
                command, device_type, valid_commands
            )))
        }
    }

    /// Parse and validate command parameters
    pub fn parse_command_params(&self, command: &str, params: &serde_json::Value) -> ApiResult<CommandParams> {
        match command {
            "move" | "drive" => {
                let speed = params.get("speed")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.5);
                let direction = params.get("direction")
                    .and_then(|v| v.as_str())
                    .unwrap_or("forward");
                let duration_ms = params.get("duration_ms")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(1000);

                if speed < 0.0 || speed > 1.0 {
                    return Err(ApiError::ValidationError("Speed must be between 0.0 and 1.0".to_string()));
                }

                Ok(CommandParams::Movement {
                    speed: speed as f32,
                    direction: direction.to_string(),
                    duration_ms,
                })
            }
            "rotate" | "turn" | "turn_left" | "turn_right" => {
                let degrees = params.get("degrees")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(90.0);
                let speed = params.get("speed")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.3);

                Ok(CommandParams::Rotation {
                    degrees: degrees as f32,
                    speed: speed as f32,
                })
            }
            "hover" => {
                let altitude = params.get("altitude")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(1.0);

                Ok(CommandParams::Hover {
                    altitude: altitude as f32,
                })
            }
            _ => Ok(CommandParams::Simple),
        }
    }

    /// Generate telemetry data (simulated)
    pub fn generate_telemetry(&self, device_type: &str) -> DeviceTelemetry {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        DeviceTelemetry {
            timestamp: Utc::now(),
            battery_level: rng.gen_range(20..100),
            cpu_temp: rng.gen_range(35.0..75.0),
            signal_strength: rng.gen_range(-80..-30),
            position: Position {
                latitude: rng.gen_range(-90.0..90.0),
                longitude: rng.gen_range(-180.0..180.0),
                altitude: if device_type == "drone" { Some(rng.gen_range(0.0..100.0)) } else { None },
            },
            velocity: Velocity {
                x: rng.gen_range(-5.0..5.0),
                y: rng.gen_range(-5.0..5.0),
                z: if device_type == "drone" { Some(rng.gen_range(-2.0..2.0)) } else { None },
            },
            sensors: vec![
                SensorReading {
                    sensor_type: "temperature".to_string(),
                    value: rng.gen_range(15.0..35.0),
                    unit: "°C".to_string(),
                },
                SensorReading {
                    sensor_type: "humidity".to_string(),
                    value: rng.gen_range(30.0..80.0),
                    unit: "%".to_string(),
                },
            ],
        }
    }

    /// Calculate estimated battery drain for command
    pub fn estimate_battery_drain(&self, command: &str, params: &CommandParams) -> f32 {
        match params {
            CommandParams::Movement { speed, duration_ms, .. } => {
                let base_drain = 0.1;
                base_drain * speed * (*duration_ms as f32 / 1000.0)
            }
            CommandParams::Rotation { degrees, speed } => {
                let base_drain = 0.05;
                base_drain * (degrees.abs() / 360.0) * speed
            }
            CommandParams::Hover { altitude } => {
                0.2 * altitude // Higher altitude = more drain
            }
            CommandParams::Simple => 0.01,
        }
    }
}

impl Default for RoboticsService {
    fn default() -> Self {
        Self::new()
    }
}

// Data structures
#[derive(Debug, Serialize, Deserialize)]
pub enum CommandParams {
    Movement {
        speed: f32,
        direction: String,
        duration_ms: u64,
    },
    Rotation {
        degrees: f32,
        speed: f32,
    },
    Hover {
        altitude: f32,
    },
    Simple,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceTelemetry {
    pub timestamp: DateTime<Utc>,
    pub battery_level: u8,
    pub cpu_temp: f64,
    pub signal_strength: i32,
    pub position: Position,
    pub velocity: Velocity,
    pub sensors: Vec<SensorReading>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorReading {
    pub sensor_type: String,
    pub value: f64,
    pub unit: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResult {
    pub command_id: Uuid,
    pub status: String,
    pub executed_at: DateTime<Utc>,
    pub estimated_duration_ms: u64,
    pub estimated_battery_drain: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceStats {
    pub total_commands_executed: u64,
    pub total_runtime_hours: f64,
    pub average_battery_usage: f64,
    pub last_maintenance: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_command() {
        let service = RoboticsService::new();
        
        assert!(service.validate_command("drone", "takeoff").is_ok());
        assert!(service.validate_command("drone", "land").is_ok());
        assert!(service.validate_command("drone", "invalid").is_err());
        
        assert!(service.validate_command("robot", "move_forward").is_ok());
        assert!(service.validate_command("robot", "grab").is_ok());
        
        assert!(service.validate_command("rover", "drive").is_ok());
        assert!(service.validate_command("rover", "scan").is_ok());
        
        assert!(service.validate_command("unknown", "any").is_err());
    }

    #[test]
    fn test_parse_command_params() {
        let service = RoboticsService::new();
        
        let params = serde_json::json!({
            "speed": 0.5,
            "direction": "forward",
            "duration_ms": 2000
        });
        
        let result = service.parse_command_params("move", &params);
        assert!(result.is_ok());
        
        if let Ok(CommandParams::Movement { speed, direction, duration_ms }) = result {
            assert_eq!(speed, 0.5);
            assert_eq!(direction, "forward");
            assert_eq!(duration_ms, 2000);
        }
    }

    #[test]
    fn test_generate_telemetry() {
        let service = RoboticsService::new();
        
        let telemetry = service.generate_telemetry("drone");
        assert!(telemetry.battery_level <= 100);
        assert!(telemetry.position.altitude.is_some());
        
        let telemetry = service.generate_telemetry("rover");
        assert!(telemetry.position.altitude.is_none());
    }
}
