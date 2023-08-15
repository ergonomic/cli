use clap::Parser;
use ergo_envrc_lib as ergo;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Parser)]
struct CLI {
    path: PathBuf,
}

fn main() -> std::io::Result<()> {
    let cli = CLI::parse();
    let path = &cli.path;
    let text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => return Err(e),
    };

    let result = parse(text);
    io::stdout().write_all(result.as_bytes())?;

    Ok(())
}

fn parse(text: String) -> String {
    let result = ergo::parse(text);
    return result;
}
