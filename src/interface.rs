/// functions used to run the programme without any arg
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;
use crate::musicfile::MFContainer;
use crate::playlist::playlist;
use crate::scan::{scan, write2json};
use crate::search::search;
use crate::tag::add_tag;


/// launch the interface by printing question, parsing the result and reacting
pub fn start() {
    print!("1 - scan / 2 - search / 3 - playlist  / 4 - tag >>>   ");
    let command:u32 = read_user_input().parse().unwrap();
    print!("souhaitez vous conserver le résultat de la requête dans un fichier markdown ?  1 - oui / 2 - non  >>>  ");
    let write:bool;
    match read_user_input().parse().unwrap() {
        1 => write = true,
        2 => write = false,
        _ => panic!("veuillez saisir une valeur correcte parmi (1, 2)")
    }
    // scan
    if command == 1 {
        print!("enter a correct path :  >>>  ");
        let mut path:PathBuf = PathBuf::new();
        path.push(read_user_input());
        let data:MFContainer = scan(&path, write);
        write2json(data);

    // search
    } else if command == 2 {
        print!("write a filter >>>  ");
        let  filter:String = read_user_input();
        let _data:MFContainer = search(filter, write);

    // playlist
    } else if command == 3 {
        print!("write a correct path >>> ");
        let mut path:PathBuf = PathBuf::new();
        path.push(read_user_input());
        print!("write a correct filter >>> ");
        let filter:String = read_user_input();
        playlist(&path, filter, write);
    } else if command == 4 {
        print!("write a filter  >>>  ");
        let filter:String = read_user_input();
        print!("write a replacement line  >>> ");
        let replace:String = read_user_input();
        add_tag(search(filter, false), replace);
    }
}


/// read the user input and returns it
fn read_user_input() -> String {
    let mut res = String::new();
    let _= stdout().flush();
    stdin().read_line(&mut res).expect("Did not enter a correct string");
    if let Some('\n') = res.chars().next_back() {
        res.pop();
    }
    if let Some('\r')= res.chars().next_back() {
        res.pop();
    }
    res
}