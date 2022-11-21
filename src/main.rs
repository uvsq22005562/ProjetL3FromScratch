use clap::Parser;
use projet_l3_2::cli::Cli;
// use projet_l3_2::musicfile::MusicFile; // osef
//use std::path::{Path, PathBuf};  // osef
use clap::builder::Str;
use projet_l3_2::scan::{scan, write2json};
// use std::fs::OpenOptions;  // osef
use std::io::Write;
// fichier principal

fn main() {
    // récupération des entrées de l'user avec module cli
    let args = Cli::parse();
    println!("{:?}", args); // checkprint
    // gestion du cas scan
    if args.command == "scan" {
        // création du vect contenant tout les fichiers supportés
        let music_files = scan(&args.path);
        write2json(&music_files);
    }
}
