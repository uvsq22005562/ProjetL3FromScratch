use walkdir::{DirEntry, WalkDir};
use crate::musicfile::MusicFile;
use std::path::{Path, PathBuf};
use std::fs::OpenOptions;
use std::io::Write;

//liste des formats supportés (voir possibilité de séléction avec arg
const SUPPORTED_EXT: [&str; 1] = ["mp3"];

// fonction vérifiant que le fichier passé en argument est supporté
fn is_supported(entry: &DirEntry) -> bool {
    // Direntry représente une entry sur un directory ouvert (un élément présent dans un dossier)
    entry.path().is_file() && SUPPORTED_EXT.contains(&entry.path().extension().unwrap().to_str().unwrap())
}

// fonction de scan - retourne directement le vecteur utilisé dans le main
pub fn scan(path: &Path) -> Vec<MusicFile> {
    // création du vecteur
    let mut files = Vec::new();
    // énumération sur la récurcion de walkdir (passage récursice sur tout les elm contenu dans le path)
    for entry in WalkDir::new(path) {
        let temp = entry.as_ref().unwrap();
        // vérification du support du fichier
        if is_supported(&temp) {
            // création d'une musicfile avec le path en argument -> ajout dans le vecteur
            files.push(MusicFile::new(temp.path()));
        }
    }
    files // retourne le vecteur
}

pub fn write2json(data: &Vec<MusicFile>){
    // création du chemin vers le fichier cible json (!!! rajouter création si fichier inexistant)
    let mut newpath: PathBuf = PathBuf::new();
    newpath.push("src/test/stored_metadata.json");
    let mut metadata_vec:Vec<String> = Vec::new();
    // génération des strings pour chaque fichier mp3 trouvé
    for elm in data {
        metadata_vec.push(MusicFile::generate_str(&elm));
    }
    // écriture
    let mut file = OpenOptions::new().write(true).open(newpath).expect("Cannot Open");
    // ouverture de la syntaxe json
    file.write_all("{\n    \"mp3 files\" :[\n".as_bytes());
    // écriture des metadata
    while metadata_vec.len() != 1 {
        file.write_all(metadata_vec.pop().unwrap().as_bytes());
        file.write_all(",\n".as_bytes());
    }
    file.write_all(metadata_vec.pop().unwrap().as_bytes());
    // fermeture de la syntaxe json
    file.write_all("\n    ]\n}".as_bytes());
}