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

        // tout une série de test pour arriver a écrire dans un json
        let mut newpath: PathBuf = PathBuf::new();
        newpath.push("src/test/testjson.json");
        match std::fs::write(&newpath, Self::generate_str(&path)) {
            Err(e) => println!("{:?}", e),
            Ok(_o) => println!("succed at writting into json")
        }
        println!("{:?}", tag.frames().count());
        // gros print bien moche qui sert pour observer la construction de tag
        // println!("{:?}", tag);

        // retourne le constructeur
        MusicFile {
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
        }
    }

    pub fn generate_str(path: &Path) -> String {
        let mut mystring:String = String::new();
        mystring.push_str("{\"path\": ");
        mystring.push_str(&serde_json::to_string(path).unwrap());
        mystring.push_str("}");
        mystring
    }
}
