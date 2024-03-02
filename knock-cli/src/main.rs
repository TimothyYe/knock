use clap::Parser;

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
    let rule = match args.rule {
        Some(rule) => rule,
        None => {
            println!("No rule specified.");
            return Ok(());
        }
    };

    let config = config::load_config(&args.config)?;
    let executor = rule::RuleExecutor::new(config);
    executor.run(&rule)?;

    Ok(())
}
