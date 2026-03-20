use dioxus::prelude::*;
use chrono::{Datelike, Utc};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct HealthCheckDto {
    pub ok: bool,
}

#[get("/api/health")]
pub async fn health_check() -> ServerFnResult<HealthCheckDto> {
    Ok(HealthCheckDto { ok: true })
}

#[get("/api/today")]
pub async fn today() -> ServerFnResult<u64> {
    Ok(Utc::now().date_naive().num_days_from_ce() as u64)
}
