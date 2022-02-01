use crate::cli::Command;
use crate::report::K6Report;

pub struct App;

impl App {
    pub fn new() -> Self {
        App {}
    }

    pub async fn run(&self, command: &Command) {
        match command {
            Command::Extract {
                host,
                port,
                database,
                username,
                password,
                https,
                from,
                exclude_setup_steps,
                exclude_teardown_steps,
            } => {
                let k6_report = K6Report::new(
                    host,
                    port,
                    database,
                    username,
                    password,
                    https,
                    from,
                    exclude_setup_steps,
                    exclude_teardown_steps,
                );
                k6_report.extract_metrics().await;
            }
        }
    }
}
