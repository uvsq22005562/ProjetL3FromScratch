use id3::{Tag, TagLike, Version};
use crate::musicfile::{MusicFile, MFContainer};

pub fn add_tag(data: MFContainer) {
    let mut tag = Tag::new();
    tag.set_album("nouveau nom");
    for elm in data.file {
        tag.write_to_path(elm.path, Version::Id3v22);
    }
}