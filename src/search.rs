use std::path::{PathBuf};
use std::{fs};  // file system btw
use crate::m2d::search_to_md2;
use crate::musicfile::MFContainer;


pub fn arg_read(arg: &String) -> Vec<String> {
    let mut res = Vec::new();
    for values in arg.split("=") {
        res.push(values.clone().to_string());
    }
    res
}

pub fn get_metadata() -> MFContainer {
    let mut new_path = PathBuf::new();
    new_path.push("src/test/stored_metadata.json");
    let data = fs::read_to_string(new_path).unwrap();
    let container:MFContainer = serde_json::from_str(&data).unwrap();
    container
}

pub fn search(arg: String, write:bool) -> MFContainer {
    let mut count:u32 = 0;
    let argument = arg_read(&arg);
    let data = get_metadata();
    let mut result:MFContainer = MFContainer::new();
    for elm in data.file {
        match argument[0].as_str() {
            "album" => if &elm.album == &argument[1]{result.add(elm); count +=1;},
            "artist" => if &elm.artist == &argument[1]{result.add(elm); count +=1;},
            "title" => if &elm.title == &argument[1]{result.add(elm); count +=1;},
            "year" => if &elm.year == &argument[1]{result.add(elm); count +=1;},
            "numero" => if &elm.numero == &argument[1]{result.add(elm); count +=1;},
            _ => continue
        }
    }
    if write {search_to_md2(arg, &result, count)}
    result

}