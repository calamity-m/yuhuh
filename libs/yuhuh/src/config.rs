#[derive(clap::Parser, Debug, Clone, Default)]
pub struct Config {
    /// Port to serve core on.
    ///
    /// Defaults to 3000
    #[clap(long, env)]
    #[arg(default_value_t = 3000)]
    pub port: u16,

    #[clap(flatten)]
    pub log: log::LoggingConfig,

    #[clap(long, env)]
    pub database_url: String,
}
