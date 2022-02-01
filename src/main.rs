mod app;
mod cli;
mod error;
mod metrics;
mod report;

use structopt::StructOpt;

use crate::app::App;
use crate::cli::Command;

#[tokio::main]
async fn main() {
    let command = Command::from_args();
    let app = App::new();
    app.run(&command).await;
}
