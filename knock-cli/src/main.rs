use clap::Parser;
use log::{error, LevelFilter};
use pretty_env_logger::env_logger::Builder;

mod config;
mod rule;

#[derive(Parser, Debug)]
#[command(version = env!("VERSION"), about, long_about = "A port knocking console application written in Rust")]
struct Args {
    /// Path to the configuration file
    #[arg(short, long, default_value = "config.yaml")]
    config: String,
    /// The port knocking rule to execute
    #[arg(short, long)]
    rule: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    Builder::new()
        .filter_level(LevelFilter::Info) // Set default log level to Info
        .init();

    let rule = match args.rule {
        Some(rule) => rule,
        None => {
            error!("No rule specified.");
            return Ok(());
        }
    };

    let config = config::load_config(&args.config)?;
    let executor = rule::RuleExecutor::new(config);
    executor.run(&rule)?;

    Ok(())
}
