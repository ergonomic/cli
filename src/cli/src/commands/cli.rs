use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(bin_name = "ergo", version)]
pub struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    ENVRC(super::envrc::CLI),
}

impl CLI {
    pub fn exec(&self) {
        match &self.command {
            Commands::ENVRC(cli) => cli.exec(),
        }
    }
}
