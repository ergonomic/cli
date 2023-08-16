use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version)]
#[command(bin_name = "ergo")]
#[command(about = "ergo - a CLI for developer & workspace ergonomics")]
pub struct CLI {
    #[command(subcommand)]
    pub command: Commands,
}

impl CLI {
    pub fn exec(&self) {
        println!("Hello, World!");
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    // #[clap(arg_required_else_help = true)]
    // /// Does stuff w/ `envrc`
    // ENVRC(ENVRC),
}

// use anyhow::Result;
// use clap::{Parser, Subcommand};

// /// Rusty example app
// #[derive(Parser, Debug)]
// #[command(version, bin_name = "rusty", disable_help_subcommand = true)]
// pub struct Cli {
//     #[command(subcommand)]
//     command: Commands,
// }

// #[derive(Subcommand, Debug)]
// enum Commands {
//     Exec(super::exec::Cli),
// }

// impl Cli {
//     pub fn exec(&self) -> Result<()> {
//         match &self.command {
//             Commands::Exec(cli) => cli.exec(),
//         }
//     }
// }
