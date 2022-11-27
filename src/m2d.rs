use std::fs::File;
use markdown_gen;
use crate::musicfile::MFContainer;


pub fn scan_to_md2(search: String, found: &MFContainer, nb:u32) {
    let file = File::create("src/test/test.md").unwrap();
    let mut md = markdown_gen::markdown::Markdown::new(file);
    let line1 = "scan ".to_string() + &search + &" w".to_string();
    let line2 = nb.to_string() + " fichier(s) supporté(s)";
    md.write(&*line1).unwrap();
    md.write(&*line2).unwrap();
    for elm in &found.file {
        md.write(elm.path.to_str().unwrap()).expect("failed to write into md");
    }
}

pub fn search_to_md2(search: String, found: &MFContainer, nb:u32) {
    let file = File::create("src/test/test.md").unwrap();
    let mut md = markdown_gen::markdown::Markdown::new(file);
    let line1 = "search ".to_string() + &search + &" w".to_string();
    let line2 = nb.to_string() + " correspondance(s)";
    md.write(&*line1).unwrap();
    md.write(&*line2).unwrap();
    for elm in &found.file {
        md.write(elm.path.to_str().unwrap()).expect("failed to write into md");
    }
}

pub fn playlist_to_md2(search: String, found:&MFContainer, nb:u32) {
    let file = File::create("src/test/test.md").unwrap();
    let mut md = markdown_gen::markdown::Markdown::new(file);
    let line1 = "playlist ".to_string() + &search + &" w".to_string();
    let line2 = nb.to_string() + " musique(s) ajoutée(s) a la playlist";
    md.write(&*line1).unwrap();
    md.write(&*line2).unwrap();
    for elm in &found.file {
        md.write(elm.title.as_str()).expect("failed to write into md");
    }
}