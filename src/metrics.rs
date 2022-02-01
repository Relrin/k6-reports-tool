use chrono::{DateTime, Utc};

pub trait K6Metric {
    fn get_metric_name() -> &'static str;
}

pub struct HttpReqDuration {
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

impl K6Metric for HttpReqDuration {
    fn get_metric_name() -> &'static str {
        return "http_req_duration";
    }
}
