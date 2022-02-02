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
pub struct ChecksMetric {
    time: DateTime<Utc>,
    check: String,
    group: Option<String>,
    method: String,
    scenario: Option<String>,
    value: f64, // The rate of successful checks
}

impl K6Metric for ChecksMetric {
    fn metric_table_name() -> &'static str {
        "checks"
    }

    fn query_fields() -> &'static [&'static str] {
        &[
            "time",
            r#""check""#,
            r#""group""#,
            r#""method""#,
            r#""scenario""#,
            "value",
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataReceivedMetric {
    time: DateTime<Utc>,
    group: Option<String>,
    scenario: Option<String>,
    value: f64, // The amount of received data
}

impl K6Metric for DataReceivedMetric {
    fn metric_table_name() -> &'static str {
        "data_received"
    }

    fn query_fields() -> &'static [&'static str] {
        &["time", r#""group""#, r#""scenario""#, "value"]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataSentMetric {
    time: DateTime<Utc>,
    group: Option<String>,
    scenario: Option<String>,
    value: f64, // The amount of data sent
}

impl K6Metric for DataSentMetric {
    fn metric_table_name() -> &'static str {
        "data_sent"
    }

    fn query_fields() -> &'static [&'static str] {
        &["time", r#""group""#, r#""scenario""#, "value"]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpReqConnectingMetric {
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
    value: f64, // Time spent establishing TCP connection to the remote host
}

impl K6Metric for HttpReqConnectingMetric {
    fn metric_table_name() -> &'static str {
        "http_req_connecting"
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpReqBlockedMetric {
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
    value: f64, // Time spent blocked (waiting for a free TCP connection slot) before initiating the request
}

impl K6Metric for HttpReqBlockedMetric {
    fn metric_table_name() -> &'static str {
        "http_req_blocked"
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpReqFailedMetric {
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
    value: f64, // The rate of failed requests according to setResponseCallback
}

impl K6Metric for HttpReqFailedMetric {
    fn metric_table_name() -> &'static str {
        "http_req_failed"
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpReqReceivingMetric {
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
    value: f64, // Time spent receiving response data from the remote host
}

impl K6Metric for HttpReqReceivingMetric {
    fn metric_table_name() -> &'static str {
        "http_req_receiving"
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpReqSendingMetric {
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
    value: f64, // Time spent sending data to the remote host
}

impl K6Metric for HttpReqSendingMetric {
    fn metric_table_name() -> &'static str {
        "http_req_sending"
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpReqTlsHandshakingMetric {
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
    value: f64, // Time spent handshaking TLS session with remote host
}

impl K6Metric for HttpReqTlsHandshakingMetric {
    fn metric_table_name() -> &'static str {
        "http_req_tls_handshaking"
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpReqWaitingMetric {
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
    value: f64, // Time spent waiting for response from remote host
}

impl K6Metric for HttpReqWaitingMetric {
    fn metric_table_name() -> &'static str {
        "http_req_waiting"
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IterationMetric {
    time: DateTime<Utc>,
    scenario: Option<String>,
    value: f64, // The aggregate number of times the VUs in the test have executed the JS script (the default function)
}

impl K6Metric for IterationMetric {
    fn metric_table_name() -> &'static str {
        "iterations"
    }

    fn query_fields() -> &'static [&'static str] {
        &["time", r#""scenario""#, "value"]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IterationsDurationMetric {
    time: DateTime<Utc>,
    scenario: Option<String>,
    value: f64, // The time it took to complete one full iteration. It includes the time spent in setup and teardown as well
}

impl K6Metric for IterationsDurationMetric {
    fn metric_table_name() -> &'static str {
        "iteration_duration"
    }

    fn query_fields() -> &'static [&'static str] {
        &["time", r#""group""#, r#""scenario""#, "value"]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VusMetric {
    time: DateTime<Utc>,
    value: f64, // Current number of active virtual users
}

impl K6Metric for VusMetric {
    fn metric_table_name() -> &'static str {
        "vus"
    }

    fn query_fields() -> &'static [&'static str] {
        &["time", "value"]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VusMaxMetric {
    time: DateTime<Utc>,
    value: f64, // Max possible number of virtual users (VU resources are pre-allocated, to ensure performance will not be affected when scaling up the load level)
}

impl K6Metric for VusMaxMetric {
    fn metric_table_name() -> &'static str {
        "vus_max"
    }

    fn query_fields() -> &'static [&'static str] {
        &["time", "value"]
    }
}
