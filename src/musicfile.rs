use std::path::{Path, PathBuf};
use id3::{Tag, TagLike};
// use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde_json;

// structure de base
#[derive(Debug)]
pub struct MusicFile {
    path: PathBuf,
    metadata: Tag,
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
        std::fs::write(&newpath, Self::generate_str(&path));
        Self::read_tag(&tag);

        // retourne le constructeur
        MusicFile {
            path: path.to_path_buf(),
            metadata: tag,
        }
    }

    pub fn generate_str(path: &Path) -> String {
        let mut mystring:String = String::new();
        mystring.push_str("{\"path\": ");
        mystring.push_str(&serde_json::to_string(path).unwrap());
        mystring.push_str("}");
        mystring
    }

    pub fn read_tag(tag:&Tag) {
        println!("==================================");
        println!("==================================");
        println!("{}", tag.duration().unwrap());
        println!("==================================");
        println!("==================================");
    }
}
