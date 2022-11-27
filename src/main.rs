use clap::Parser;
use projet_l3_2::cli::Cli;
use projet_l3_2::scan::{scan, write2json};
use projet_l3_2::search::search;
use projet_l3_2::playlist;
use std::path::PathBuf;
use std::panic;


fn main() {
    // récupération des entrées de l'user avec module cli
    let args = Cli::parse();
    // gestion du cas scan
    if args.command == "scan" {
        // conversion du 2nd argument en path
        let mut temp_path = PathBuf::new();
        temp_path.push(args.arg2.unwrap());
        // création du vect contenant tout les fichiers supportés
        match &args.arg3 {
            Some(s) => { match s as &str {
                "w" => {let x = scan(&temp_path, true);
                write2json(x);},
                _ => panic!("invalide 3rd argument -> type w to write the request into md"),
            }
            },
            None => {let x = scan(&temp_path, false);
                write2json(x);}
        }
    } else if args.command == "search" {
        match &args.arg3 {
            Some(s) => { match s as &str {
                "w" => {search(args.arg2.unwrap(), true);},
                _ => panic!("invalide 3rd argument -> type w to write the request into md"),
            }
                },
            None => {search(args.arg2.unwrap(), false);}
        }
    } else if args.command == "playlist" {
        let mut temp_path = PathBuf::new();
        temp_path.push(args.arg2.unwrap());
        match &args.arg4 {
            Some(s) => { match s as &str {
                "w" => {playlist::playlist(&temp_path, args.arg3.unwrap(), true);},
                _ => panic!("invalide 3rd argument -> type w to write the request into md"),
            }
            },
            None => {playlist::playlist(&temp_path, args.arg3.unwrap(), false);}
        }
    }
}
