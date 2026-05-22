use clap::Parser;

mod scheduler;

#[derive(Parser, Debug)]
#[command(name = "joe-rnal-webhook")]
#[command(author = "Joe-rnal Team")]
#[command(version = "0.1.0")]
#[command(about = "Rust port of Joe-rnal webhook reminders.", long_about = None)]
struct Args {
    /// Select the scheduler execution mode
    #[arg(short, long, value_enum, default_value_t = scheduler::RunMode::Unified)]
    mode: scheduler::RunMode,

    /// Run as a dry-run test mode to display output details without sending webhooks
    #[arg(short, long)]
    dry_run: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    scheduler::start_scheduler(args.mode, args.dry_run).await?;
    Ok(())
}
