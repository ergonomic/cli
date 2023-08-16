use clap::Args;
use ergo_lib_envrc as ergo;
use std::io::{self, Write};

#[derive(Debug, Args)]
#[command()]
pub struct CLI {
    #[arg(required = true)]
    path: String,
}

impl CLI {
    pub fn exec(&self) {
        let path = &self.path;
        let text = std::fs::read_to_string(path).expect("failed reading path");
        let result = ergo::render(text);
        io::stdout()
            .write_all(result.as_bytes())
            .expect("failed to write to stdout");
    }
}
