mod commands;
use crate::commands::cli::CLI;
use clap::Parser;

fn main() {
    let cli = CLI::parse();
    cli.exec();
}
