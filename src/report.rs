use std::fmt::Write;

use chrono::{DateTime, Duration, SecondsFormat, Utc};
use influxdb::{Client, ReadQuery};

use crate::error::Result;
use crate::metrics::HttpReqDurationMetric;

pub struct K6Report {
    invoked_at: DateTime<Utc>,
    output_directory: String,
    db_client: Client,
    retention_policy_name: String,
    from: Option<u64>,
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
        from: &Option<u64>,
        exclude_setup_steps: &bool,
        exclude_teardown_steps: &bool,
        output_directory: &String,
    ) -> Self {
        let invoked_at = Utc::now();
        let connection_url = match https {
            true => format! {"https://{host}:{port}", host = host, port = port},
            false => format!("http://{host}:{port}", host = host, port = port),
        };
        let db_client = match username.is_some() && password.is_some() {
            true => {
                let auth_username = username.clone().unwrap();
                let auth_password = password.clone().unwrap();
                Client::new(connection_url, database).with_auth(auth_username, auth_password)
            }
            false => Client::new(connection_url, database),
        };

        K6Report {
            invoked_at,
            output_directory: output_directory.to_owned(),
            db_client,
            retention_policy_name: String::from("autogen"),
            from: from.to_owned(),
            exclude_setup_steps: exclude_setup_steps.to_owned(),
            exclude_teardown_steps: exclude_teardown_steps.to_owned(),
        }
    }

    pub async fn extract_metrics(&self) {
        let export_requests = ["http_req_duration"];

        for table_name in export_requests {
            println!("Export for the `{0}` has completed", table_name);

            match self.export_metric(table_name).await {
                Ok(_) => println!("Export for the `{0}` has completed", table_name),
                Err(error) => {
                    println!(
                        "Data for `{0}` can't be exported for. Details: {1}",
                        table_name, error
                    );
                }
            };
        }
    }

    pub async fn export_metric(&self, table_name: &str) -> Result<()> {
        let query = self.build_query(table_name)?;
        let mut db_result = self.db_client.json_query(query).await?;

        // TODO: Deserialize + write to a file
        let result = db_result.deserialize_next::<HttpReqDurationMetric>()?;
        //.series
        //.into_iter()
        //.map(|mut x| x)
        //.collect::<Vec<HttpReqDurationMetric>>();

        println!("{:?}", result.series);

        Ok(())
    }

    fn build_query(&self, metric_name: &str) -> Result<ReadQuery> {
        let mut raw_query = String::from("SELECT ");
        let selected_fields = [
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
        .join(", ");
        write!(&mut raw_query, "{}", selected_fields)?;

        let from_statement = format!(
            " FROM {0}.{1}.{2}",
            self.db_client.database_name(),
            self.retention_policy_name,
            metric_name
        );
        write!(&mut raw_query, "{}", from_statement)?;

        let mut filters: Vec<String> = vec![];

        if self.from.is_some() {
            let minutes_offset = self.from.unwrap_or(0);
            let start_timestamp = self.invoked_at - Duration::minutes(minutes_offset as i64);
            let filter_clause = format!(
                "time > '{}'",
                start_timestamp.to_rfc3339_opts(SecondsFormat::AutoSi, true),
            );
            filters.push(filter_clause);
        }

        if self.exclude_setup_steps {
            filters.push(r#""group"!='::setup'"#.to_string());
        }

        if self.exclude_teardown_steps {
            filters.push(r#""group"!='::teardown'"#.to_string());
        }

        if !filters.is_empty() {
            let where_clause = filters.join(" AND ");
            write!(&mut raw_query, "{}", format!(" WHERE {0}", where_clause))?;
        }

        Ok(ReadQuery::new(raw_query))
    }
}
