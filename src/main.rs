/// Projet rust L3.S5 informatique - Jules Marty - 22005562
/// todo : pour la fonction playlist, faire en sorte que le chemin relatif soit (fichier par fichier transfomrmé en absolut)
/// todo : revoir la syntaxe du fichier markdown
use clap::Parser;
use projet_l3_2::cli::Cli;
use projet_l3_2::interface::start;
use projet_l3_2::scan::{scan, write2json};
use projet_l3_2::search::search;
use projet_l3_2::playlist;
use std::path::PathBuf;
use std::panic;
use projet_l3_2::musicfile::MFContainer;


/// read the arguments and run the corresponding function (scan/search/playlist)
fn main() {
    let args = Cli::parse();
    // scan command management
    if args.command == None {
        start();
    } else if args.command == Some("scan".to_string()) {
        let mut temp_path:PathBuf = PathBuf::new();
        temp_path.push(args.arg2.unwrap());
        match &args.arg3 {
            Some(s) => { match s as &str {
                "w" => {let x:MFContainer = scan(&temp_path, true);
                write2json(x);},
                _ => panic!("invalide 3rd argument -> type w to write the request into md"),
            }
            },
            None => {let x:MFContainer = scan(&temp_path, false);
                write2json(x);}
        }
        // search command management
    } else if args.command == Some("search".to_string()) {
        match &args.arg3 {
            Some(s) => { match s as &str {
                "w" => {
                    search(args.arg2.unwrap(), true);
                },
                _ => panic!("invalide 3rd argument -> type w to write the request into md"),
                            }
            },
            None => {
                search(args.arg2.unwrap(), false);
            }
        }
        // playlist command management
    } else if args.command == Some("playlist".to_string()) {
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
