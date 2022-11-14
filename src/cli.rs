use clap::Parser;

#[derive(Parser)]  // offre a la struct le rôle de parser
#[derive(Debug)]  // permet l'affichage
pub struct Cli {
    // premier argument
    pub command: String,
    // second argument
    pub path: std::path::PathBuf,
}