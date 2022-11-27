use walkdir::{DirEntry, WalkDir};
use crate::musicfile::{MFContainer, MusicFile};
use std::path::{Path, PathBuf};
use std::fs::OpenOptions;
use std::io::Write;
use crate::m2d::scan_to_md2;

//liste des formats supportés (voir possibilité de séléction avec arg
const SUPPORTED_EXT: [&str; 1] = ["mp3"];

// fonction vérifiant que le fichier passé en argument est supporté
fn is_supported(entry: &DirEntry) -> bool {
    // Direntry représente une entry sur un directory ouvert (un élément présent dans un dossier)
    entry.path().is_file() && SUPPORTED_EXT.contains(&entry.path().extension().unwrap().to_str().unwrap())
}

// fonction de scan - retourne directement le vecteur utilisé dans le main
pub fn scan(path: &Path, write:bool) -> MFContainer {
    // création du vecteur
    let mut count:u32 = 0;
    let mut container:MFContainer = MFContainer::new();
    // énumération sur la recursion de walkdir (passage recursive sur tout les elm contenu dans le path)
    for entry in WalkDir::new(path) {
        let temp = entry.as_ref().unwrap();
        // vérification du support du fichier
        if is_supported(&temp) {
            // création d'une musicfile avec le path en argument -> ajout dans le vecteur
            let y = MusicFile::new(temp.path());
            container.add(y);
            count += 1;
        }
    }
    if write {scan_to_md2(String::from(path.to_str().unwrap()),&container, count)}
    container // retourne le vecteur
}

pub fn write2json(data: MFContainer) {
    let mut new_path: PathBuf = PathBuf::new();
    new_path.push("src/test/stored_metadata.json");
    let mut file = OpenOptions::new().write(true).open(new_path).expect("Cannot Open");
    match file.write_all(serde_json::to_string(&data).unwrap().as_bytes()) {
        Err(e) => println!("{:?}", e),
        _=> ()
    }
    // let deserialized_mf:Result<MusicFile, serde_json::Error> = serde_json::from_str(&serialized_mf);
}