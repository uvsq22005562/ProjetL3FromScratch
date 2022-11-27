use m3u;
use crate::musicfile::MFContainer;
use std::fs::File;
use std::path::Path;
use crate::m2d::playlist_to_md2;
use crate::scan::scan;
use crate::search::search;

pub fn create(data: MFContainer) {
    let mut playlist = Vec::new();
    for elm in data.file {
        playlist.push(m3u::path_entry(elm.path))
    }
    let mut file = File::create("src/test/playlist.m3u").unwrap();
    let mut writer = m3u::Writer::new(&mut file);
    for entry in playlist {
        writer.write_entry(&entry).unwrap();
    }
}

pub fn playlist(path: &Path, contraint: String, write:bool) {
    scan(path, false);
    let search_res = search(contraint, false);
    if write {playlist_to_md2(String::from(path.to_str().unwrap()),&search_res, search_res.file.len() as u32)}
    create(search_res);
}