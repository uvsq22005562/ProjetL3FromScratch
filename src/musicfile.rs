use std::path::{Path, PathBuf};
use id3::{Tag, TagLike};


// structure de base
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MusicFile {
    pub path: PathBuf,
    pub album: String,
    pub artist: String,
    pub title: String,
    pub year: String,
    pub numero: String,

}

impl MusicFile {
    pub fn new(path: &Path) -> MusicFile {
        // création d'un tag ID3 -> contient l'ensemble des metadatas spécifique aux fichiers mp3
        // !!! on obtient pas les metadata classiques -> a ajouter plus tard.
        let tag = Tag::read_from_path(path).unwrap();

        // construit l'instance de musicfile
        let result: MusicFile = MusicFile {
            path: path.to_path_buf(),
            artist: if tag.clone().get("TPE1").is_some() {
                tag.clone().get("TPE1").unwrap().content().
                    text().unwrap().to_string()
            } else { "inconnu".to_string() },
            album: if tag.clone().get("TALB").is_some() {
                tag.clone().get("TALB").unwrap().content().
                    text().unwrap().to_string()
            } else { "inconnu".to_string() },
            title: if tag.clone().get("TIT2").is_some() {
                tag.clone().get("TIT2").unwrap().content().
                    text().unwrap().to_string()
            } else { "inconnu".to_string() },
            year: if tag.clone().get("TDRC").is_some() {
                tag.clone().get("TDRC").unwrap().content().
                    text().unwrap().to_string()
            } else { "inconnu".to_string() },
            numero: if tag.clone().get("TRCK").is_some() {
                tag.clone().get("TRCK").unwrap().content().
                    text().unwrap().to_string()
            } else { "inconnu".to_string() },
        };
        result
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MFContainer {
    pub file: Vec<MusicFile>,
}

impl MFContainer {
    pub fn new() -> MFContainer {
        MFContainer {file:Vec::new()}
    }
    pub fn add(self: &mut Self, data: MusicFile) {
        self.file.push(data);
    }
}