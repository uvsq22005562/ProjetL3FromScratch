/// all the functions used to execute the search command
use std::path::PathBuf;
use std::fs;
use crate::m2d::{search_to_md2};
use crate::musicfile::MFContainer;


/// arg - 2nd argument provided by the user, condition to select the mp3 files
/// write - if true will write summary of query execution
///
/// retrieves the metadata of all the files found by scan() and builds an MFcontainer according
/// to the conditions passed as arguments.
/// returns the MFcontainer and displays a summary of the execution on the shell.
pub fn search(arg: String, write:bool) -> MFContainer {
    let mut count:u32 = 0;
    let argument:Vec<String> = arg_read(&arg);
    let data:MFContainer = get_metadata();
    let mut result:MFContainer = MFContainer::new();
    for elm in data.file {
        match argument[0].as_str() {
            "album" => if elm.album == argument[1]{result.add(elm); count +=1;},
            "artist" => if elm.artist == argument[1]{result.add(elm); count +=1;},
            "title" => if elm.title == argument[1]{result.add(elm); count +=1;},
            "year" => if elm.year == argument[1]{result.add(elm); count +=1;},
            "numero" => if elm.numero == argument[1]{result.add(elm); count +=1;},
            _ => continue
        }
    }
    display_result(&result);
    if write {search_to_md2(arg, &result, count)}
    result
}


/// arg - in the form title="title of the music"
///
/// return a Vec containing split of the argument to make it easily readable
pub fn arg_read(arg: &str) -> Vec<String> {
    let mut res:Vec<String> = Vec::new();
    for values in arg.split('=') {
        res.push(values.to_string());
    }
    res
}


/// deserialize the MFcontainer which was written in the json by scan() and returns it
pub fn get_metadata() -> MFContainer {
    let mut new_path = PathBuf::new();
    new_path.push("src/output/last_request.json");
    match new_path.exists() {
        false => panic!("Veuillez procéder a un scan avant d'effectuer une recherche"),
        true => {
            let data = fs::read_to_string(new_path).unwrap();
            let container:MFContainer = serde_json::from_str(&data).unwrap();
            container
        }
    }
}


/// writes for the user a summary of the execution of the query in the command interface
pub fn display_result(data: &MFContainer) {
    println!(">>> {} fichier(s) correspondant(s) a votre recherche", data.file.len());
    for elm in &data.file {
        println!(">>> >>> {:?}", elm.title);
    }
}