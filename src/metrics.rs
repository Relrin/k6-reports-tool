use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use std::str::FromStr;

fn custom_deserialize_bool_from_str<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match bool::from_str(&s) {
        Ok(value) => Ok(value),
        _ => Ok(false),
    }
}

fn custom_deserialize_u16_from_str<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match u16::from_str(&s) {
        Ok(value) => Ok(value),
        _ => Ok(0),
    }
}

pub trait K6Metric {
    fn metric_table_name() -> &'static str;
    fn query_fields() -> &'static [&'static str];
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpReqDurationMetric {
    time: DateTime<Utc>,
    #[serde(deserialize_with = "custom_deserialize_bool_from_str")]
    expected_response: bool,
    group: Option<String>,
    method: String,
    name: String,
    proto: String,
    scenario: Option<String>,
    #[serde(deserialize_with = "custom_deserialize_u16_from_str")]
    status: u16,
    tls_version: String,
    url: String,
    value: f64, // duration
}

impl K6Metric for HttpReqDurationMetric {
    fn metric_table_name() -> &'static str {
        "http_req_duration"
    }

    fn query_fields() -> &'static [&'static str] {
        &[
            "time",
            r#""expected_response""#,
            r#""group""#,
            r#""method""#,
            r#""name""#,
            r#""proto""#,
            r#""scenario""#,
            r#""status""#,
            r#""tls_version""#,
            r#""url""#,
            "value",
        ]
    }
}
