use crate::cli::Command;

pub struct K6client;

impl K6client {
    pub fn new() -> Self {
        K6client {}
    }

    pub async fn run(&self, command: &Command) {}
}
