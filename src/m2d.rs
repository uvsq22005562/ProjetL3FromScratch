/// functions needed to store the request result into a markdown file
use std::fs::File;
use markdown_gen;
use markdown_gen::markdown::{AsMarkdown, List, Markdown};
use crate::musicfile::MFContainer;


/// search - user input (to store the original request)
/// found - files found while executing scan()
/// nb - amount of file that as been found
///
/// converted into string all the arguments and results of functions concerning the execution of scan.
/// Write each of the strings to the file src/output/request_history.md
/// the file contain only the last request result (missing time)
pub fn scan_to_md2(search: String, found: &MFContainer, nb:u32) {
    let file:File = File::create("src/output/request_history.md").unwrap();
    let mut md:Markdown<File> = markdown_gen::markdown::Markdown::new(file);

    let line1:String = "scan ".to_string() + &search + &" w".to_string();
    let line2:String = nb.to_string() + " fichier(s) supporté(s)";

    write2md(&mut md, vec![line1, line2], found);
}


/// same function for the display of search
/// should probably make a single function but i'm running out of time
pub fn search_to_md2(search: String, found: &MFContainer, nb:u32) {
    let file:File = File::create("src/output/request_history.md").unwrap();
    let mut md:Markdown<File> = markdown_gen::markdown::Markdown::new(file);

    let line1:String = "search ".to_string() + &search + &" w".to_string();
    let line2:String = nb.to_string() + " correspondance(s)";

    write2md(&mut md, vec![line1, line2], found);
}


/// same function for the display of playlist
pub fn playlist_to_md2(search: String, contraint:String, found:&MFContainer, nb:u32) {
    let file:File = File::create("src/output/request_history.md").unwrap();
    let mut md:Markdown<File> = markdown_gen::markdown::Markdown::new(file);

    let line1:String = "playlist ".to_string() + &search + &" ".to_string() + &contraint + &" w".to_string();
    let line2:String = nb.to_string() + " musique(s) ajoutée(s) a la playlist";
    write2md(&mut md, vec![line1, line2], found);
}


/// md - markdown file
/// headers - the 2 lines that change between the 3 functions
/// data - musicfiles returned inside a MFcontainer
///
/// function used to write into the markdown file.
pub fn write2md(md:&mut Markdown<File>, headers:Vec<String>, data:&MFContainer) {
    md.write(headers[0].heading(3)).unwrap();
    md.write(headers[1].heading(6)).unwrap();
    for elm in &data.file {
        let mut line3:String = String::new();
        line3 += &elm.title;
        line3 += &(" --> artist : ".to_string() + &elm.artist + &" album : " + &elm.album);
        md.write(List::new(false)
            .item(&*("titre -> ".to_string() + &elm.title))
            .item(&*("artiste -> ".to_string() + &elm.artist))
            .item(&*("album -> ".to_string() + &elm.album))
            .item(&*("année -> ".to_string() + &elm.year))
            .item(&*("numéro de track -> ".to_string() + &elm.numero))
            .item(*&elm.path.to_str().unwrap()).quote()).unwrap()
    }
}