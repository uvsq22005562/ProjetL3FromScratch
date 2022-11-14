use std::path::{Path, PathBuf};
use id3::{Tag, TagLike};
use serde::ser::{Serialize, Serializer, SerializeStruct};
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
        std::fs::write(newpath,serde_json::to_string(path).unwrap());
        // retourne le constructeur
        MusicFile {
            path: path.to_path_buf(),
            metadata: tag,
        }

    }
}

// implémentation de la fonction serialize (pas nécessaire au final, surement supprimé dans la prochaine version)
impl Serialize for MusicFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer, {
        let mut state = serializer.serialize_struct("MusicFile", 2)?;
        state.serialize_field("path", &self.path)?;
        //state.serialize_field("metadata", &self.metadata)?;
        state.end()
    }
}