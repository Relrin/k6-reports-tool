use chrono::{DateTime, FixedOffset};
use influxdb::Client;

pub struct K6Report {
    db_client: Client,
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
            from: from.to_owned(),
            exclude_setup_steps: exclude_setup_steps.to_owned(),
            exclude_teardown_steps: exclude_teardown_steps.to_owned(),
        }
    }

    pub async fn extract_metrics(&self) {}
}
