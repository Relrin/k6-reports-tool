use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "k6-reports")]
pub enum Command {
    /// Extracts test results in the CSV format from InfluxDB
    #[structopt(name = "extract")]
    Extract {
        #[structopt(
            short = "h",
            long = "host",
            default_value = "localhost",
            help = " Host to connect to"
        )]
        host: String,

        #[structopt(
            short = "p",
            long = "port",
            default_value = "8086",
            help = "Port to connect to"
        )]
        port: String,

        #[structopt(
            short = "d",
            long = "database",
            default_value = "k6",
            help = "Database to connect to the server"
        )]
        database: String,

        #[structopt(
            short = "u",
            long = "username",
            help = "Username for a connection",
            env = "K6_REPORTS_DB_USERNAME",
            hide_env_values = true
        )]
        username: Option<String>,

        #[structopt(
            short = "s",
            long = "password",
            help = "Used password along with the username",
            env = "K6_REPORTS_DB_PASSWORD",
            hide_env_values = true
        )]
        password: Option<String>,

        #[structopt(long = "--https", help = "Connecting to the database with HTTPS")]
        https: bool,

        #[structopt(
            long = "--from",
            help = "How long to look into history by time (in minutes)"
        )]
        from: Option<u64>,

        #[structopt(long = "--exclude-setup", help = "Exclude setup steps from reports")]
        exclude_setup_steps: bool,

        #[structopt(
            long = "--exclude-teardown",
            help = "Exclude teardown steps from reports"
        )]
        exclude_teardown_steps: bool,

        #[structopt(
            long = "--output",
            help = "Output directory for extracted data",
            default_value = "./reports"
        )]
        output_directory: String,
    },
}
