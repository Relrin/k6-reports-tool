use std::fmt::Write;
use std::path::Path;

use chrono::{DateTime, Duration, SecondsFormat, Utc};
use csv_async::AsyncSerializer;
use influxdb::{Client, ReadQuery};
use tokio::fs::{create_dir_all, File};

use crate::error::Result;
use crate::metrics::{HttpReqDurationMetric, K6Metric};

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

    pub async fn extract_metrics(&self) -> Result<()> {
        create_dir_all(&self.output_directory).await?;
        let exported_metrics = [self.export_metric::<HttpReqDurationMetric>()];

        for metric_to_export in exported_metrics {
            metric_to_export.await?
        }

        Ok(())
    }

    pub async fn export_metric<T: K6Metric>(&self) -> Result<()> {
        let table_name = T::metric_table_name();
        println!("Exporting data for the `{0}` metrics", table_name);

        let query = self.build_query::<T>()?;
        let mut response = self.db_client.json_query(query).await?;
        let data = response.deserialize_next::<HttpReqDurationMetric>()?;

        let filename = format!("{0}.csv", table_name);
        let filepath = Path::new(&self.output_directory).join(&filename);
        let output_file = File::create(filepath).await?;
        let mut csv_writer = AsyncSerializer::from_writer(output_file);

        for series in data.series {
            for record in series.values {
                csv_writer.serialize(record).await?;
            }
        }

        println!("Export for the `{0}` metrics has completed", table_name);
        Ok(())
    }

    fn build_query<T: K6Metric>(&self) -> Result<ReadQuery> {
        let mut raw_query = String::from("SELECT ");
        let selected_fields = T::query_fields().join(", ");
        write!(&mut raw_query, "{}", selected_fields)?;

        let from_statement = format!(
            " FROM {0}.{1}.{2}",
            self.db_client.database_name(),
            self.retention_policy_name,
            T::metric_table_name()
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
