use clap::Parser;
use projet_l3_2::cli::Cli;
use projet_l3_2::scan::scan;
// fichier principal

fn main() {
    // récupération des entrées de l'user avec module cli
    let args = Cli::parse();
    println!("{:?}", args); // checkprint
    // gestion du cas scan
    if args.command == "scan" {
        // création du vect contenant tout les fichiers supportés
        let music_files = scan(&args.path);
        // checkprints
        for elm in music_files {
            println!("----------------------------");
            println!("{:?}", elm);
            println!("----------------------------");
        }
    }
}
