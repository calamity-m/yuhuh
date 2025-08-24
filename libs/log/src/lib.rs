use std::error::Error;

use tracing::{Level, Span, span};
use tracing_subscriber::EnvFilter;

#[derive(clap::ValueEnum, Clone, Debug, Default, PartialEq)]
pub enum LoggingFormat {
    /// JSON format - structured logging suitable for log aggregation systems
    ///
    /// Produces compact, machine-readable JSON output ideal for production
    /// environments and log processing pipelines like ELK stack, Fluentd, etc.
    ///
    /// Default.
    #[default]
    Json,
    /// Pretty format - human-readable output for development
    ///
    /// Produces colorized, indented output that's easier to read during
    /// development and debugging. Not recommended for production use.
    Pretty,
}

#[derive(clap::Parser, Debug, Clone)]
pub struct LoggingConfig {
    // Output format for log messages
    #[clap(short, long, default_value_t, value_enum, env)]
    pub format: LoggingFormat,

    /// Global log level for the application
    #[clap(long, env, default_value_t = Level::INFO)]
    pub level: tracing::Level,

    /// Log level for Axum web framework
    #[clap(long, env, default_value_t = Level::INFO)]
    pub axum_level: tracing::Level,

    /// Log level for SQLx database operations
    #[clap(long, env, default_value_t = Level::INFO)]
    pub sqlx_level: tracing::Level,
}

/// Common fields included in all log entries and spans.
///
/// This struct contains metadata that should be present across all logging
/// output to provide consistent context about the running application.
/// The lifetime parameter allows for zero-copy string references.
pub struct CommonFields<'a> {
    /// Name of the system or service
    ///
    /// A short, descriptive name for the application or service. This helps
    /// identify log entries when aggregating logs from multiple services.
    /// Should be consistent across all instances of the same service.
    pub system: &'a str,
    /// Version of the running application
    ///
    /// The version string of the currently running application. Typically
    /// follows semantic versioning (e.g., "1.2.3"). Useful for correlating
    /// logs with specific releases and debugging version-specific issues.
    pub version: &'a str,
}

/// Initializes the global tracing subscriber and creates a root span for the application.
///
/// This function sets up the tracing infrastructure using `tracing-subscriber` and creates
/// a global span that can be used throughout the application lifecycle. The span includes
/// common fields like system name and version for consistent logging context.
///
/// # Arguments
///
/// * `common` - Common fields to be included in all spans and events, such as system name and version
/// * `config` - Logging configuration that controls subscriber behavior (log levels, formatting, etc.)
///
/// # Returns
///
/// * `Ok(Span)` - A root span that should be entered and held for the application lifetime
/// * `Err(Box<dyn Error>)` - If tracing subscriber initialization fails
///
/// # Errors
///
/// This function will return an error if:
/// - The tracing subscriber cannot be initialized (e.g., already initialized)
/// - Invalid logging configuration is provided
/// - System resources for logging are unavailable
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// use your_crate::log::{self, CommonFields};
/// use your_crate::config::LoggingConfig;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = your_crate::config::Config::parse();
///
///     let global_span = log::init(
///         &CommonFields {
///             system: "my-service",
///             version: "1.0.0",
///         },
///         &config.log,
///     )
///     .expect("logging initialisation failed");
///
///     let _guard = global_span.enter();
///
///     tracing::info!("Application started");
///     
///     run_application()?;
///     
///     tracing::info!("Application shutting down");
///     Ok(())
/// }
/// ```
///
/// ## Creating Child Spans
///
/// ```rust
/// use tracing::{info_span, info};
///
/// fn some_operation() {
///     let span = info_span!("operation", task = "data_processing");
///     let _enter = span.enter();
///     
///     info!("Starting data processing");
///     info!("Data processing completed");
/// }
/// ```
///
/// # Notes
///
/// - This function should only be called once at application startup
/// - The returned span should be entered immediately and the guard held for the application lifetime
/// - All subsequent tracing events will inherit the context from this global span
/// - The span guard should be kept in scope until the application terminates
///
/// # See Also
///
/// - [`tracing::Span`] - For more information about spans
/// - [`tracing_subscriber`] - For subscriber configuration options
/// - [`CommonFields`] - For available common field options
pub fn init(common: &CommonFields, config: &LoggingConfig) -> Result<Span, Box<dyn Error>> {
    // Create the level filter
    let filter = EnvFilter::builder().parse(format!(
        "{},axum={},sqlx={}",
        config.level.as_str(),
        config.axum_level.as_str(),
        config.sqlx_level.as_str()
    ))?;

    let instrument = tracing_subscriber::fmt().with_env_filter(filter);

    let instrument = instrument.with_thread_ids(true);

    // try_init returns an error if already initialized, but doesn't panic.
    //
    // We don't care if it failed due to already being initialized
    // Just proceed to create the span
    match config.format {
        LoggingFormat::Json => instrument.json().try_init().ok(),
        LoggingFormat::Pretty => instrument.pretty().try_init().ok(),
    };

    let global_span = span!(
        Level::INFO,
        "yuhuh",
        system = common.system,
        version = common.version
    );

    Ok(global_span)
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use tracing::{error, info, warn};
    use tracing_test::traced_test;

    #[test]
    fn test_logging_config_defaults() {
        let config = LoggingConfig::try_parse_from(["test"]).unwrap();

        assert!(matches!(config.format, LoggingFormat::Json));
        assert_eq!(config.level, tracing::Level::INFO);
        assert_eq!(config.axum_level, tracing::Level::INFO);
        assert_eq!(config.sqlx_level, tracing::Level::INFO);
    }

    #[test]
    fn test_logging_config_command_line() {
        let config = LoggingConfig::try_parse_from([
            "test",
            "--format",
            "pretty",
            "--level",
            "debug",
            "--axum-level",
            "warn",
            "--sqlx-level",
            "error",
        ])
        .unwrap();

        assert!(matches!(config.format, LoggingFormat::Pretty));
        assert_eq!(config.level, tracing::Level::DEBUG);
        assert_eq!(config.axum_level, tracing::Level::WARN);
        assert_eq!(config.sqlx_level, tracing::Level::ERROR);
    }

    #[test]
    fn test_logging_config_env_vars() {
        // Hah HA HA. This might not be the best way to handle this? but
        // I can't really be bothered finding a crate just to set env vars
        // to test fkn clap works.
        unsafe { std::env::set_var("FORMAT", "pretty") };
        unsafe { std::env::set_var("LEVEL", "debug") };

        let config = LoggingConfig::try_parse_from(["test"]).unwrap();

        assert!(matches!(config.format, LoggingFormat::Pretty));
        assert_eq!(config.level, tracing::Level::DEBUG);

        // Clean up
        unsafe { std::env::remove_var("FORMAT") };
        unsafe { std::env::remove_var("LEVEL") };
    }

    #[traced_test]
    #[test]
    fn test_logging_output_with_spans() {
        let common = CommonFields {
            system: "test-system",
            version: "1.0.0",
        };

        let config = LoggingConfig {
            format: LoggingFormat::Json,
            level: tracing::Level::TRACE,
            axum_level: tracing::Level::TRACE,
            sqlx_level: tracing::Level::TRACE,
        };

        // Initialize with our function
        let span = init(&common, &config).expect("Init should work");
        let _guard = span.enter();

        info!("Test info message");
        warn!("Test warning message");
        error!("Test error message");

        // Check that our common fields actually showed up
        assert!(logs_contain("system=\"test-system\" version=\"1.0.0"));
    }
}
