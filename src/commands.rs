// pub mod commands;

// use clap::{Args, Parser, Subcommand};
// use ergo_lib_envrc as ergo;
// use std::io::{self, Write};
// use std::path::PathBuf;

// #[derive(Debug, Parser)]
// #[command(author, version)]
// #[command(about = "ergo - a CLI for developer & workspace ergonomics")]
// pub struct CLI {
//     #[clap(subcommand)]
//     pub command: Commands,
// }

// #[derive(Debug, Subcommand)]
// pub enum Commands {
//     #[clap(arg_required_else_help = true)]
//     /// Does stuff w/ `envrc`
//     ENVRC(ENVRC),
// }

// #[derive(Args, Debug)]
// struct ENVRC {
//     /// A path to a markdown template
//     path: PathBuf,
// }

// pub fn render(text: String) -> String {
//     return ergo::render(text);
// }

// fn main() -> std::io::Result<()> {
//     let cli = CLI::parse();
//     let path = &cli.path;
//     let text = match std::fs::read_to_string(path) {
//         Ok(text) => text,
//         Err(e) => return Err(e),
//     };

//     let result = render(text);
//     io::stdout().write_all(result.as_bytes())?;

//     Ok(())
// }

// fn render(text: String) -> String {
//     return ergo::render(text);
// }

// use super::*;

// #[derive(Debug, Parser)]
// #[clap(
//     name = "Cryptifer",
//     about = "Cryptifer is a CLI Application to Encrypt and Decrypt the file",
//     version = "0.0.1"
// )]
// pub struct Cryptifer {
//     #[clap(subcommand)]
//     pub command: Commands,
// }

// #[derive(Debug, Subcommand)]
// pub enum Commands {
//     /// Generates Keystore to out file given
//     #[clap(arg_required_else_help = true)]
//     Generate {
//         #[clap(short = 'o', long)]
//         output_path: String,
//     },
//     /// Encrypts file specified using keypath
//     #[clap(arg_required_else_help = true)]
//     Encrypt {
//         #[clap(short = 'f', long)]
//         file_path: String,
//         #[clap(short = 'k', long)]
//         key_path: String,
//     },
//     /// Decrypts file specified using keypath
//     #[clap(arg_required_else_help = true)]
//     Decrypt {
//         #[clap(short = 'f', long)]
//         encrypted_file: String,
//         #[clap(short = 'k', long)]
//         key_path: String,
//     },
// }
