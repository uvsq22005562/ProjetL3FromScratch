

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::musicfile::MusicFile;

    #[test]
    fn test_create_musicfile() {
        let path:PathBuf = PathBuf::from("src/test/Future - IM DAT N____ (Official Music Video).mp3");
        let musicfile:MusicFile = MusicFile::new(&path);
        assert_eq!(MusicFile{
            title: "imdat..".to_string(),
            artist: "que/future".to_string(),
            album: "jsp album".to_string(),
            year: "2022".to_string(),
            numero: "12".to_string(),
            path: path
        }, musicfile)
    }
}