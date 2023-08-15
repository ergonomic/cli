use clap::Parser;
use ergo_lib_envrc as ergo;
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

    let result = render(text);
    io::stdout().write_all(result.as_bytes())?;

    Ok(())
}

fn render(text: String) -> String {
    return ergo::render(text);
}
