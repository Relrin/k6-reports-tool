use chrono::{DateTime, FixedOffset};
use influxdb::{Client, ReadQuery};

use crate::metrics::{HttpReqDuration, K6Metric};

pub struct K6Report {
    db_client: Client,
    retention_policy_name: String,
    from: Option<DateTime<FixedOffset>>,
    exclude_setup_steps: bool,
    exclude_teardown_steps: bool,
}

impl K6Report {
    pub fn new(
        host: &String,
        port: &String,
        database: &String,
        username: &Option<String>,
        password: &Option<String>,
        https: &bool,
        from: &Option<DateTime<FixedOffset>>,
        exclude_setup_steps: &bool,
        exclude_teardown_steps: &bool,
    ) -> K6Report {
        let connection_url = match https {
            true => format!("http://{host}:{port}", host = host, port = port),
            false => format! {"https://{host}:{port}", host = host, port = port},
        };
        let mut db_client = match username.is_some() && password.is_some() {
            true => {
                let auth_username = username.clone().unwrap();
                let auth_password = password.clone().unwrap();
                Client::new(connection_url, database).with_auth(auth_username, auth_password)
            }
            false => Client::new(connection_url, database),
        };

        K6Report {
            db_client,
            retention_policy_name: String::from("autogen"),
            from: from.to_owned(),
            exclude_setup_steps: exclude_setup_steps.to_owned(),
            exclude_teardown_steps: exclude_teardown_steps.to_owned(),
        }
    }

    pub async fn extract_metrics(&self) {
        let metrics = [HttpReqDuration::get_metric_name()];

        //for metric in metrics {
        //    let query = ReadQuery::new("SELECT * FROM weather");
        //}
    }

    // pub async fn read_metric_from_db<T>(&self, metric: T)
    // where
    //     T: K6Metric,
    // {
    //     let query
    // }
}
