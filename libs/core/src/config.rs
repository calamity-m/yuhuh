//! Configuration for the core application

#[derive(clap::Parser, Debug)]
pub struct Config {
    /// Port to serve core on.
    ///
    /// Defaults to 3000
    #[clap(long, env)]
    #[arg(default_value_t = 3000)]
    pub port: u16,
}
