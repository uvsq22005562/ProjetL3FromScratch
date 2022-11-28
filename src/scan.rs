/// all the functions needed to execute the scan command and to write the result into a json file.
use walkdir::{DirEntry, WalkDir};
use crate::musicfile::{MFContainer, MusicFile};
use std::path::{Path, PathBuf};
use std::io::Write;
use std::fs::File;
use crate::m2d::scan_to_md2;


const SUPPORTED_EXT: [&str; 1] = ["mp3"];


/// returns true if the file format is supported
fn is_supported(entry: &DirEntry) -> bool {
    entry.path().is_file() && SUPPORTED_EXT.contains(&entry.path().extension().unwrap().to_str().unwrap())
}


/// path - starting path of the recursion
/// write - if true will write summary of query execution
///
/// recursively traverse the given tree, filling an MFcontainer with all supported files encountered (counting them).
// then writes a summary of the execution to the command shell.
// returns an MFcontainer containing all the files found and saves it with write2json()
pub fn scan(path: &Path, write:bool) -> MFContainer {
    let mut count:u32 = 0;
    let mut container:MFContainer = MFContainer::new();
    for entry in WalkDir::new(path) {
        let temp:&DirEntry = entry.as_ref().unwrap();
        if is_supported(&temp) {
            let y = MusicFile::new(temp.path());
            container.add(y);
            count += 1;
        }
    }
    println!(">>> {} fichier(s) trouvé(s)", count);
    if write {scan_to_md2(String::from(path.to_str().unwrap()),&container, count)}
    container
}


/// stores metadata of all files found when running scan() in the src/output/last_request.json file
pub fn write2json(data: MFContainer) {
    let mut new_path: PathBuf = PathBuf::new();
    new_path.push("src/output/last_request.json");
    let mut file:File = File::create(new_path).expect("Cannot create json");
    match file.write_all(serde_json::to_string(&data).unwrap().as_bytes()) {
        Err(e) => println!("{:?}", e),
        _=> ()
    }
}