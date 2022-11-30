use id3::{Tag, TagLike, Version, Frame};
use crate::musicfile::MFContainer;


/// function used to modify a precise tag from a mp3 file
pub fn add_tag(data: MFContainer, to_change:String) {
    let tag_id:&str;
    let mut readed_arg:Vec<&str> = Vec::new();
    for elm in to_change.split("=") {
        readed_arg.push(elm);
    }
    match readed_arg[0] {
        "artist" => tag_id = "TPE1",
        "album" => tag_id = "TALB",
        "title" => tag_id = "TIT2",
        "year" => tag_id = "TDRC",
        "numero" => tag_id = "TRCK",
        _ => panic!("le tag modifié n'est pas disponible ou pas correct")
    }
    for elm in data.file {
        let mut tag = Tag::read_from_path(&elm.path).unwrap();
        tag.add_frame(Frame::text(tag_id, readed_arg[1]));
        tag.write_to_path(elm.path, Version::Id3v24).unwrap();
    }
}