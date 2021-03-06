use crate::cli::Command;
use crate::report::K6Report;

pub struct App;

impl App {
    pub fn new() -> Self {
        App {}
    }

    pub async fn run(&self, command: &Command) {
        match command {
            Command::Export {
                host,
                port,
                database,
                username,
                password,
                https,
                from,
                exclude_setup_steps,
                exclude_teardown_steps,
                output_directory,
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
                    output_directory,
                );
                match k6_report.extract_metrics().await {
                    Ok(_) => println!("Export finished successfully"),
                    Err(error) => println!("Export can't be executed. Reason: {:?}", error),
                }
            }
        }
    }
}
