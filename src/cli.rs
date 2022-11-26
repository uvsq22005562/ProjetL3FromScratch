use clap::Parser;

#[derive(Parser)]  // offre a la struct le rôle de parser
#[derive(Debug)]  // permet l'affichage
pub struct Cli {
    // premier argument
    pub command: String,
    // second argument (scan -> path) (search -> filter)
    pub arg2: Option<String>,
    pub arg3: Option<String>
}