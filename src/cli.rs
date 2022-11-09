use clap::Parser;

#[derive(Parser)]  // offre a la struct le rôle de parser
pub struct Cli {
    command: String,
    path: std::path::PathBuf,
}