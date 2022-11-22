use std::path::{Path, PathBuf};
use id3::{Tag, TagLike};
use serde_json;

// structure de base
#[derive(Debug)]
pub struct MusicFile {
    path: PathBuf,
    album: String,
    artist: String,
    title: String,
    year: String,
    numero: String,

}

// implémentation du constructeur
impl MusicFile {
    pub fn new(path: &Path) -> MusicFile {
        // création d'un tag ID3 -> contient l'ensemble des metadatas spécifique aux fichiers mp3
        // !!! on obtient pas les metadata classiques -> a ajouter plus tard.
        let tag = Tag::read_from_path(path).unwrap();

        // construit l'instance de musicfile
        let result: MusicFile = MusicFile {
            path: path.to_path_buf(),
            artist : if tag.clone().get("TPE1").is_some() {
                tag.clone().get("TPE1").unwrap().content().
                    text().unwrap().to_string()
            } else {"inconnu".to_string()},
            album : if tag.clone().get("TALB").is_some() {
                tag.clone().get("TALB").unwrap().content().
                    text().unwrap().to_string()
            } else {"inconnu".to_string()},
            title : if tag.clone().get("TIT2").is_some() {
                tag.clone().get("TIT2").unwrap().content().
                    text().unwrap().to_string()
            } else {"inconnu".to_string()},
            year : if tag.clone().get("TDRC").is_some() {
                tag.clone().get("TDRC").unwrap().content().
                    text().unwrap().to_string()
            } else {"inconnu".to_string()},
            numero : if tag.clone().get("TRCK").is_some() {
                tag.clone().get("TRCK").unwrap().content().
                    text().unwrap().to_string()
            } else {"inconnu".to_string()},
        };
        result
    }

    pub fn generate_str(item: &MusicFile) -> String {
        let mut mystring:String = String::new();
        mystring.push_str("    {\n        \"path\": ");
        mystring.push_str(&serde_json::to_string(&item.path).unwrap());
        mystring.push_str(",\n        \"album\": ");
        mystring.push_str(&serde_json::to_string(&item.album).unwrap());
        mystring.push_str(",\n        \"artist\": ");
        mystring.push_str(&serde_json::to_string(&item.artist).unwrap());
        mystring.push_str(",\n        \"title\": ");
        mystring.push_str(&serde_json::to_string(&item.title).unwrap());
        mystring.push_str(",\n        \"year\": ");
        mystring.push_str(&serde_json::to_string(&item.year).unwrap());
        mystring.push_str(",\n        \"numero\": ");
        mystring.push_str(&serde_json::to_string(&item.numero).unwrap());
        mystring.push_str("\n    }");
        mystring
    }
}
