use std::path::{Path, PathBuf};
use std::{fs, vec};  // file system btw
use std::str::Split;
use clap::builder::Str;


pub fn get_stored_metadata() -> String {
    let mut newpath: PathBuf = PathBuf::new();
    newpath.push("src/test/stored_metadata.json");
    let content:String = fs::read_to_string(newpath).unwrap();
    content
    /*for i in c {
        for x in &i {
            println!("{:?}", x);
        }
    }*/
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

pub fn arg_read(arg: String) -> Vec<String> {
    let mut res = Vec::new();
    for values in arg.split("=") {
        res.push(values.to_string().clone());
    }
    println!("{:?}", res);
    res
}

pub fn search(arg: String) {
    let content = separate_content(get_stored_metadata());
    let argument = arg_read(arg);
    let mut temp_path:Vec<String> = Vec::new(); // buff all path while parsing
    let mut select:Vec<String> = Vec::new();
    for sub_content in content {
        if temp_path.len() != 0 {
            let _ = temp_path.pop();
        }
        for line in &sub_content {
            if line.contains("path") {
                //temp_path.push();
                println!("{:?}", line.split("\","));
            }
            if line.contains(&argument[0]) && line.contains(&argument[1]) {
                select = sub_content.clone();
                println!("{:?}", select);
            }
        }
    }

}