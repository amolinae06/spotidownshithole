use std::path::{Path, PathBuf};
use std::convert::TryInto;
use mp4ameta::{Tag, Data, Img};
use mp4ameta::ident::DataIdent;
use chrono::{DateTime, NaiveDate, Utc};

use crate::error::SpotifyError;

use super::Field;

pub struct MP4Tag {
    tag: Tag,
    path: PathBuf,
    separator: String
}

impl MP4Tag {
    /// Read tag from file
    pub fn open(path: impl AsRef<Path>) -> Result<MP4Tag, SpotifyError> {
        let tag = Tag::read_from_path(&path)?;
        Ok(MP4Tag {
            tag,
            path: path.as_ref().to_owned(),
            separator: ", ".to_owned()
        })
    }
}

impl super::Tag for MP4Tag {
    fn set_separator(&mut self, separator: &str) {
        self.separator = separator.to_string();
    }

    fn set_raw(&mut self, tag: &str, value: Vec<String>) {
        let mut bytes = tag.as_bytes().to_owned();
        // Replace UTF-8 © with the proper character
        if bytes.len() == 5 && bytes[0..2] == [194, 169] {
            bytes = vec![0xa9, bytes[2], bytes[3], bytes[4]];
        }

        let data: Vec<_> = value.into_iter().map(|v| Data::Utf8(v)).collect();

        // Fourcc
        if bytes.len() == 4 {
            let ident = DataIdent::fourcc(bytes.try_into().unwrap());
            self.tag.set_all_data(ident, data);
            return;
        }

        // Freeform
        let ident = DataIdent::freeform("com.apple.iTunes", tag);
        self.tag.set_all_data(ident, data);
    }

    fn set_field(&mut self, field: Field, value: Vec<String>) {
        let ident = match field {
            Field::Title => DataIdent::fourcc(*b"\xa9nam"),
            Field::Artist => DataIdent::fourcc(*b"\xa9ART"),
            Field::Album => DataIdent::fourcc(*b"\xa9alb"),
            Field::TrackNumber => DataIdent::fourcc(*b"trkn"),
            Field::DiscNumber => DataIdent::fourcc(*b"disk"),
            Field::AlbumArtist => DataIdent::fourcc(*b"aART"),
            Field::Genre => DataIdent::fourcc(*b"\xa9gen"),
            Field::Label => DataIdent::freeform("com.apple.iTunes", "LABEL"),
        };
        self.tag.set_data(ident, Data::Utf8(value.join(&self.separator)));
    }

    fn set_release_date(&mut self, date: NaiveDate) {
        let ident = DataIdent::fourcc(*b"\xa9day");
        // Convert NaiveDate to ISO timestamp
        let date = date.and_hms(0, 0, 0);
        let date: DateTime<Utc> = DateTime::from_utc(date, Utc);
        self.tag.set_data(ident, Data::Utf8(format!("{}", date.format("%+"))));
    }

    fn add_cover(&mut self, mime: &str, data: Vec<u8>) {
        let mime = mime.trim();
        match mime {
            "image/jpeg" | "image/jpg" => self.tag.add_artwork(Img::jpeg(data)),
            "image/png" => self.tag.add_artwork(Img::png(data)),
            _ => warn!("Invalid mime: {}, skipping album art!", mime)
        };
    }

    fn save(&mut self) -> Result<(), SpotifyError> {
        self.tag.write_to_path(&self.path)?;
        Ok(())
    }
}