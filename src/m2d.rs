/// functions needed to store the request result into a markdown file
/// todo("rassembler les 3 fontions en une")
use std::fs::File;
use std::fs;
use markdown_gen;
use crate::musicfile::MFContainer;


/// search - user input (to store the original request)
/// found - files found while executing scan()
/// nb - amount of file that as been found
///
/// converted into string all the arguments and results of functions concerning the execution of scan.
/// Write each of the strings to the file src/output/request_history.md
/// Always write at the end of the file.
pub fn scan_to_md2(search: String, found: &MFContainer, nb:u32) {
    let old_content = fs::read_to_string("src/output/request_history.md").unwrap();
    let file = File::create("src/output/request_history.md").unwrap();
    let mut md = markdown_gen::markdown::Markdown::new(file);
    let line1 = "scan ".to_string() + &search + &" w".to_string();
    let line2 = nb.to_string() + " fichier(s) supporté(s)";
    md.write(&*old_content).unwrap();
    md.write(&*line1).unwrap();
    md.write(&*line2).unwrap();
    for elm in &found.file {
        md.write(elm.path.to_str().unwrap()).expect("failed to write into md");
    }
}


pub fn search_to_md2(search: String, found: &MFContainer, nb:u32) {
    let old_content = fs::read_to_string("src/output/request_history.md").unwrap();
    let file = File::create("src/output/request_history.md").unwrap();
    let mut md = markdown_gen::markdown::Markdown::new(file);
    let line1 = "search ".to_string() + &search + &" w".to_string();
    let line2 = nb.to_string() + " correspondance(s)";
    md.write(&*old_content).unwrap();
    md.write(&*line1).unwrap();
    md.write(&*line2).unwrap();
    for elm in &found.file {
        md.write(elm.path.to_str().unwrap()).expect("failed to write into md");
    }
}


pub fn playlist_to_md2(search: String, found:&MFContainer, nb:u32) {
    let old_content = fs::read_to_string("src/output/request_history.md").unwrap();
    let file = File::create("src/output/request_history.md").unwrap();
    let mut md = markdown_gen::markdown::Markdown::new(file);
    let line1 = "playlist ".to_string() + &search + &" w".to_string();
    let line2 = nb.to_string() + " musique(s) ajoutée(s) a la playlist";
    md.write(&*old_content).unwrap();
    md.write(&*line1).unwrap();
    md.write(&*line2).unwrap();
    for elm in &found.file {
        md.write(elm.title.as_str()).expect("failed to write into md");
    }
}