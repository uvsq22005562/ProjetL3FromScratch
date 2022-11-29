/// struct used to handle mp3 formats and associated functions
use std::path::{Path, PathBuf};
use id3::{Tag, TagLike};


/// represent a single mp3 file, with it's metadata
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MusicFile {
    pub path: PathBuf,
    pub album: String,
    pub artist: String,
    pub title: String,
    pub year: String,
    pub numero: String,
}


/// functions implemented for MusicFile
///     new - constructor
impl MusicFile {

    /// construct a MusicFile from a given path
    pub fn new(path: &Path) -> MusicFile {
        let tag:Tag = Tag::read_from_path(path).unwrap();
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


/// structure used to hold different instances of Musicfile
/// (initially thought to facilitate serialization)
#[derive(Serialize, Deserialize, Debug)]
pub struct MFContainer {
    pub file: Vec<MusicFile>,
}


/// functions implemented for MFContainer
impl MFContainer {

    /// constructor
    pub fn new() -> MFContainer {
        MFContainer {file:Vec::new()}
    }

    /// setter
    pub fn add(self: &mut Self, data: MusicFile) {
        self.file.push(data);
    }
}