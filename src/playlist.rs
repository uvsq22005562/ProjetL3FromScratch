/// all the functions used to create playlist and write it into a m3u file
use m3u;
use crate::musicfile::MFContainer;
use std::fs::File;
use std::path::Path;
use crate::m2d::{playlist_to_md2, playlist_to_md22};
use crate::scan::scan;
use crate::search::search;


/// path - will be changed to the name of the playlist
/// contraint - todo("changer en option") conditions needed to add a song to the playlist
/// write - if true will write summary of query execution
///
/// generate the MFcontainer corresponding to the asked playlist using the scan and search functions.
/// then the function display and write the playlist using create() and display_result()
pub fn playlist(path: &Path, contraint: String, write:bool) {
    scan(path, false);
    let search_res = search(contraint.clone(), false);
    if write {playlist_to_md22(String::from(path.to_str().unwrap()),
                              contraint,
                              &search_res,
                              search_res.file.len() as u32)}
    display_result(&search_res);
    create(search_res);
}


/// todo("ajouter le nom de la playlist afin de nommer le fichier créer")
/// function that will write the content of the playlist to an m3u file that can be read by vlc
pub fn create(data: MFContainer) {
    let mut playlist = Vec::new();
    for elm in data.file {
        playlist.push(m3u::path_entry(elm.path))
    }
    let mut file:File = File::create("src/output/playlist.m3u").unwrap();
    let mut writer = m3u::Writer::new(&mut file);
    for entry in playlist {
        writer.write_entry(&entry).unwrap();
    }
}


/// writes for the user a summary of the execution of the query in the command interface
pub fn display_result(data: &MFContainer) {
    println!(">>> {} fichier(s) ajouté(s) a la playlist", data.file.len());
    for elm in &data.file {
        println!(">>> >>> {}", elm.title);
    }
}