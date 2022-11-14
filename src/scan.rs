use walkdir::{DirEntry, WalkDir};
use std::path::Path;
use crate::musicfile::MusicFile;

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
