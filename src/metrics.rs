﻿use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct HttpReqDurationMetric {
    time: DateTime<Utc>,
    error: String,
    error_code: u16,
    expected_response: bool,
    group: String,
    method: String,
    name: String,
    proto: String,
    scenario: String,
    status: u16,
    tls_version: String,
    url: String,
    value: f64, // duration
}