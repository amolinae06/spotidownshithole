use std::path::{Path, PathBuf};
use std::fs::File;
use chrono::{Datelike, NaiveDate};
use oggvorbismeta::{CommentHeader, VorbisComments, read_comment_header, replace_comment_header};

use crate::error::SpotifyError;
use super::Field;

pub struct OGGTag {
    path: PathBuf,
    tag: CommentHeader
}

impl OGGTag {
    /// Load tag from file
    pub fn open(path: impl AsRef<Path>) -> Result<OGGTag, SpotifyError> {
        let mut file = File::open(&path)?;
        let tag = read_comment_header(&mut file);
    
        Ok(OGGTag {
            path: path.as_ref().to_owned(),
            tag
        })
    }
}

impl super::Tag for OGGTag {
    fn set_separator(&mut self, _separator: &str) {}

    fn set_field(&mut self, field: Field, value: Vec<String>) {
        let tag = match field {
            Field::Title => "TITLE",
            Field::Artist => "ARTIST",
            Field::Album => "ALBUM",
            Field::TrackNumber => "TRACKNUMBER",
            Field::DiscNumber => "DISCNUMBER",
            Field::Genre => "GENRE",
            Field::Label => "LABEL",
            Field::AlbumArtist => "ALBUMARTIST",
        };
        self.set_raw(tag, value);
    }

    fn add_cover(&mut self, _mime: &str, _data: Vec<u8>) {
        error!("ALBUM ART IN OGG NOT SUPPORTED!");
    }

    fn set_raw(&mut self, tag: &str, value: Vec<String>) {
        self.tag.add_tag_multi(tag, &value.iter().map(|v| v.as_str()).collect::<Vec<&str>>());
    }

    fn save(&mut self) -> Result<(), SpotifyError> {
        let file = File::open(&self.path)?;
        let mut out = replace_comment_header(file, self.tag.clone());
        let mut file = File::create(&self.path)?;
        std::io::copy(&mut out, &mut file)?;
        Ok(())
    }

    fn set_release_date(&mut self, date: NaiveDate) {
        self.tag.add_tag_single("DATE", &format!("{}-{:02}-{:02}", date.year(), date.month(), date.day()))
    }
}