use metaflac::Tag;
use std::path::{Path, PathBuf};
use std::io;

#[derive(Clone, Debug)]
struct Song {
    path: PathBuf,
    track: u32,
    title: String,
    artist: String,
    album: String,
}

fn read_from_metaflac(path: PathBuf) -> Option<Song> {
    let tag = Tag::read_from_path(&path).unwrap();
    let comments = tag.vorbis_comments().unwrap();
    let song = Song {
        path,
        track: comments.track().unwrap(),
        title: comments.title().unwrap().join(" "),
        artist: comments.artist().unwrap().join(" "),
        album: comments.album().unwrap().join(" "),
    };
    Some(song)
}

fn read_from_id3(_path: PathBuf) -> Option<Song> {
    unimplemented!()
}

fn collect_songs(dir: &Path) -> io::Result<Vec<Song>> {
    if !dir.is_dir() {
        return Err(io::Error::from(io::ErrorKind::InvalidInput));
    }

    let mut result = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        // If this is a directory, search for songs within it
        if path.is_dir() {
            result.append(&mut collect_songs(&path)?);
            continue;
        }

        // Figure out the file type of this file
        match tree_magic::from_filepath(&path).as_str() {
            "audio/mp3" => result.push(read_from_id3(path).unwrap()),
            "audio/aiff" => result.push(read_from_id3(path).unwrap()),
            "audio/flac" => result.push(read_from_metaflac(path).unwrap()),
            _ => continue,
        }
    }

    Ok(result)
}

fn main() {
    // Scan current directory
    let songs = collect_songs(Path::new("C:/Users/will/Downloads/Anjunabeats"));
    for song in songs.unwrap() {
        println!("{:?}", song);
    }
}
