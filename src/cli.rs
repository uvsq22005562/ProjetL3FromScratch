use clap::Parser;

#[derive(Parser)]  // offre a la struct le rôle de parser
#[derive(Debug)]
pub struct Cli {
    pub command: String,
    pub path: std::path::PathBuf,
}