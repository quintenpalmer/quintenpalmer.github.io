use std::collections::BTreeMap;

use crate::model;

pub fn organize_tracks(
    tracks: Vec<model::AudioFileTrackMetadata>,
) -> Result<model::Library, model::Error> {
    let mut library = model::Library {
        artists: BTreeMap::new(),
    };

    for track in tracks.into_iter() {
        let artist_entry = library
            .artists
            .entry(track.resolve_album_artist())
            .or_insert(model::Artist {
                name: track.resolve_album_artist(),
                albums: BTreeMap::new(),
            });

        let album_entry =
            artist_entry
                .albums
                .entry(track.resolve_album())
                .or_insert(model::Album {
                    name: track.resolve_album(),
                    discs: BTreeMap::new(),
                });

        let disc_entry = album_entry
            .discs
            .entry(track.resolve_disc_number())
            .or_insert(model::Disc {
                number: track.resolve_disc_number(),
                tracks: BTreeMap::new(),
            });

        let conflict = disc_entry
            .tracks
            .insert(track.resolve_track_number(), track);

        match conflict {
            Some(c) => {
                return Err(model::Error::ConflictingTrack(
                    c.resolve_album_artist(),
                    c.resolve_album(),
                    c.resolve_disc_number(),
                    c.resolve_track_number(),
                    c.track_title,
                ))
            }
            None => (),
        };
    }

    Ok(library)
}
