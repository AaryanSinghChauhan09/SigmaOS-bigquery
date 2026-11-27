#![forbid(unsafe_code)]
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "sigmaos-bigquery", about = "BigQuery serverless data warehouse client for SigmaOS")]
struct Cli { #[command(subcommand)] cmd: Commands }
#[derive(Subcommand)]
enum Commands { List, Status }
fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::List   => println!("sigmaos-bigquery: no entries configured."),
        Commands::Status => println!("sigmaos-bigquery: ready."),
    }
}
