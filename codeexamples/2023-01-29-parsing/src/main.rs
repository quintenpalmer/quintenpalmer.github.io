use std::env;

mod util;
mod parse;
mod scan;
mod organize;
mod model;
pub mod impls;

fn main() {
    println!("Let's read some audio metadata");
    let args: Vec<String> = env::args().collect();

    println!("Let's build the library by parsing all of the files");
    let library = model::Library::from_library_directory(args[1].clone()).unwrap();

    println!("Now let's print all of the tracks we found");
    for artist in library.artists.values() {
        println!("\tArtist: {}", artist.name);
        for album in artist.albums.values() {
            println!("\t\tAlbum: {}", album.name);
            for disc in album.discs.values() {
                println!("\t\t\tDisc: {}", disc.number);
                for track in disc.tracks.values() {
                    println!("\t\t\t\tTrack: {: >3} - {}", track.resolve_track_number(), track.track_title);
                }
            }
        }
    }
}
