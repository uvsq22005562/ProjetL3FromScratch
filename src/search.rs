use std::path::{Path, PathBuf};
use std::{fs, vec};  // file system btw
use std::str::Split;


pub fn get_stored_metadata() {
    let mut newpath: PathBuf = PathBuf::new();
    newpath.push("src/test/stored_metadata.json");
    let content:String = fs::read_to_string(newpath).unwrap();
    let c = separate_content(content);
    for i in c {
        for x in &i {
            println!("{:?}", x);
        }
    }
}

pub fn separate_content(ct: String) -> Vec<Vec<String>> {
    let iterator = ct.split("\n");
    let mut result = Vec::new();
    let mut temp = Vec::new();
    for x in iterator {
        if x.contains("\",") {
            temp.push(x.clone().to_string());
        }
        if x.contains("numero") {
            result.push(temp.clone());
            temp = Vec::new();
        }
    }
    result
}

pub fn search(arg: String) {
    println!("todo");
}