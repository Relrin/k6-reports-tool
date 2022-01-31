mod cli;
mod client;

use structopt::StructOpt;

use crate::cli::Command;
use crate::client::K6client;

#[tokio::main]
async fn main() {
    let command = Command::from_args();
    let client = K6client::new();
    client.run(&command).await;
}
