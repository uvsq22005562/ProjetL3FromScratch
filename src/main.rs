use clap::Parser;
use projet_l3_2::cli::Cli;
use projet_l3_2::scan::{scan, write2json};
use projet_l3_2::search::get_stored_metadata;
use std::path::PathBuf;


fn main() {
    // récupération des entrées de l'user avec module cli
    let args = Cli::parse();
    // gestion du cas scan
    if args.command == "scan" {
        // conversion du 2nd argument en path
        let mut temp_path = PathBuf::new();
        temp_path.push(args.arg2.unwrap());
        // création du vect contenant tout les fichiers supportés
        let music_files = scan(&temp_path);
        write2json(&music_files);
    } else if args.command == "search" {
        get_stored_metadata()
    }
}
