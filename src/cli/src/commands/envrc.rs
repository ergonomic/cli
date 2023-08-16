use clap::Args;

#[derive(Debug, Args)]
#[command()]
pub struct CLI {
    #[arg(required = true)]
    path: String,
}

impl CLI {
    pub fn exec(&self) {
        println!("envrc...exec")
    }
}
