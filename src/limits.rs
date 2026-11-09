//! What a file sent to the site may weigh, and what kind it may be
//! (docs/adr/0041): set in the administration, kept with the site's
//! settings, read at each request.
//!
//! Every limit has a default — the value the code had before it could be set
//! — and bounds an administrator cannot go past. The kinds are the ones the
//! archive recognises from a file's bytes (`documents::sniff`); an
//! administrator refuses some, and the rest are accepted. A kind a later
//! version learns to recognise is therefore accepted until refused.

use serde::{Deserialize, Serialize};

/// One limit, in megabytes: its default and its bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bound {
    /// Its name in the form and in the catalogue (`uploads-limit-<key>`).
    pub key: &'static str,
    pub default_mb: u32,
    pub min_mb: u32,
    pub max_mb: u32,
}

/// A file attached to a record: a document, a portrait, an emblem.
pub const DOCUMENT: Bound = Bound {
    key: "document",
    default_mb: (crate::documents::MAX_UPLOAD / MB) as u32,
    min_mb: 1,
    max_mb: 2048,
};
/// A picture pasted into a note.
pub const NOTE_IMAGE: Bound = Bound {
    key: "note-image",
    default_mb: (crate::notes::MAX_IMAGE / MB) as u32,
    min_mb: 1,
    max_mb: 200,
};
/// A file sent with a message to the personal assistant.
pub const ASSISTANT_FILE: Bound = Bound {
    key: "assistant-file",
    default_mb: (crate::assistant::FILE_MAX / MB) as u32,
    min_mb: 1,
    max_mb: 500,
};
/// All the files of one such message.
pub const ASSISTANT_MESSAGE: Bound = Bound {
    key: "assistant-message",
    default_mb: (crate::assistant::MESSAGE_FILES_MAX / MB) as u32,
    min_mb: 1,
    max_mb: 2000,
};
/// A GEDCOM file to convert or to start a family from.
pub const GEDCOM: Bound = Bound {
    key: "gedcom",
    default_mb: (crate::convert::MAX_UPLOAD / MB) as u32,
    min_mb: 1,
    max_mb: 500,
};
/// An archive restored or imported whole. A family's photographs make an
/// `.axgf` far larger than its GEDCOM — the operator's is 433 MB — and it is
/// streamed to disk, never held in memory, so this is about the disk.
pub const ARCHIVE: Bound = Bound {
    key: "archive",
    default_mb: 4096,
    min_mb: 16,
    max_mb: 65536,
};

/// Every limit, in the order the administration lists them.
pub const BOUNDS: [Bound; 6] = [
    DOCUMENT,
    NOTE_IMAGE,
    ASSISTANT_FILE,
    ASSISTANT_MESSAGE,
    GEDCOM,
    ARCHIVE,
];

const MB: usize = 1024 * 1024;

/// The kinds of file the archive recognises, by family, as the
/// administration lists them: their MIME type and the name a reader knows
/// them by.
pub const KINDS: &[(&str, &[(&str, &str)])] = &[
    (
        "image",
        &[
            ("image/jpeg", "JPEG"),
            ("image/png", "PNG"),
            ("image/gif", "GIF"),
            ("image/webp", "WebP"),
            ("image/bmp", "BMP"),
            ("image/tiff", "TIFF"),
        ],
    ),
    (
        "document",
        &[("application/pdf", "PDF"), ("text/plain", "TXT")],
    ),
    (
        "audio",
        &[
            ("audio/mpeg", "MP3"),
            ("audio/ogg", "Ogg"),
            ("audio/wav", "WAV"),
            ("audio/mp4", "M4A"),
            ("audio/flac", "FLAC"),
        ],
    ),
    (
        "video",
        &[
            ("video/mp4", "MP4"),
            ("video/webm", "WebM"),
            ("video/x-msvideo", "AVI"),
        ],
    ),
];

/// The name a reader knows a kind by: `PDF` for `application/pdf`.
pub fn kind_name(mime: &str) -> &str {
    KINDS
        .iter()
        .flat_map(|(_, kinds)| kinds.iter())
        .find(|(m, _)| *m == mime)
        .map(|(_, n)| *n)
        .unwrap_or(mime)
}

/// The limits and the kinds refused, as the site keeps them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Limits {
    pub document_mb: u32,
    pub note_image_mb: u32,
    pub assistant_file_mb: u32,
    pub assistant_message_mb: u32,
    pub gedcom_mb: u32,
    pub archive_mb: u32,
    /// The kinds of file refused, by MIME type: none unless an administrator
    /// refuses some.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub refused: Vec<String>,
}

impl Default for Limits {
    fn default() -> Self {
        Limits {
            document_mb: DOCUMENT.default_mb,
            note_image_mb: NOTE_IMAGE.default_mb,
            assistant_file_mb: ASSISTANT_FILE.default_mb,
            assistant_message_mb: ASSISTANT_MESSAGE.default_mb,
            gedcom_mb: GEDCOM.default_mb,
            archive_mb: ARCHIVE.default_mb,
            refused: Vec::new(),
        }
    }
}

fn bytes(mb: u32) -> usize {
    mb as usize * MB
}

impl Limits {
    /// Whether nothing was changed: the site's settings then say nothing.
    pub fn is_default(&self) -> bool {
        *self == Limits::default()
    }

    /// The value of a limit, by its bound.
    pub fn mb(&self, b: Bound) -> u32 {
        match b.key {
            "document" => self.document_mb,
            "note-image" => self.note_image_mb,
            "assistant-file" => self.assistant_file_mb,
            "assistant-message" => self.assistant_message_mb,
            "gedcom" => self.gedcom_mb,
            _ => self.archive_mb,
        }
    }

    /// Set a limit, by its bound, within the bound.
    pub fn set(&mut self, b: Bound, mb: u32) {
        let mb = mb.clamp(b.min_mb, b.max_mb);
        match b.key {
            "document" => self.document_mb = mb,
            "note-image" => self.note_image_mb = mb,
            "assistant-file" => self.assistant_file_mb = mb,
            "assistant-message" => self.assistant_message_mb = mb,
            "gedcom" => self.gedcom_mb = mb,
            _ => self.archive_mb = mb,
        }
    }

    /// Each limit brought within its bounds: what an edited settings file
    /// holds is not trusted further than the form.
    pub fn bounded(mut self) -> Self {
        for b in BOUNDS {
            let v = self.mb(b);
            self.set(b, v);
        }
        self
    }

    /// A file attached to a record, in bytes.
    pub fn document_bytes(&self) -> usize {
        bytes(self.document_mb)
    }

    /// A request carrying such a file and the form around it: four times
    /// the file, so that a file over the limit is read to its end and the
    /// page says so instead of the connection closing (docs/adr/0037).
    pub fn document_request(&self) -> usize {
        4 * self.document_bytes() + 64 * 1024
    }

    pub fn note_image_bytes(&self) -> usize {
        bytes(self.note_image_mb)
    }

    pub fn note_image_request(&self) -> usize {
        4 * self.note_image_bytes() + 64 * 1024
    }

    pub fn assistant_file_bytes(&self) -> usize {
        bytes(self.assistant_file_mb)
    }

    pub fn assistant_message_bytes(&self) -> usize {
        bytes(self.assistant_message_mb)
    }

    pub fn assistant_request(&self) -> usize {
        self.assistant_message_bytes() + MB
    }

    pub fn gedcom_bytes(&self) -> usize {
        bytes(self.gedcom_mb)
    }

    pub fn gedcom_request(&self) -> usize {
        self.gedcom_bytes() + 64 * 1024
    }

    pub fn archive_bytes(&self) -> usize {
        bytes(self.archive_mb)
    }

    /// A request carrying an archive: the archive and the form's framing, so
    /// that the page's own check says it is too large.
    pub fn archive_request(&self) -> usize {
        self.archive_bytes() + MB
    }

    /// Whether a file of this kind is accepted.
    pub fn allows(&self, mime: &str) -> bool {
        !self.refused.iter().any(|m| m == mime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_defaults_are_the_limits_the_code_had() {
        let l = Limits::default();
        assert_eq!(l.document_bytes(), crate::documents::MAX_UPLOAD);
        assert_eq!(
            l.document_request(),
            4 * crate::documents::MAX_UPLOAD + 64 * 1024
        );
        assert_eq!(l.note_image_bytes(), crate::notes::MAX_IMAGE);
        assert_eq!(l.assistant_file_bytes(), crate::assistant::FILE_MAX);
        assert_eq!(
            l.assistant_message_bytes(),
            crate::assistant::MESSAGE_FILES_MAX
        );
        assert_eq!(l.gedcom_bytes(), crate::convert::MAX_UPLOAD);
        assert_eq!(l.archive_bytes(), 4 * 1024 * 1024 * 1024);
        assert!(l.is_default());
        assert_eq!(
            serde_json::to_value(&l).unwrap()["refused"],
            serde_json::Value::Null
        );
    }

    #[test]
    fn a_limit_stays_within_its_bounds() {
        let mut l = Limits::default();
        l.set(DOCUMENT, 0);
        assert_eq!(l.document_mb, 1);
        l.set(DOCUMENT, 1_000_000);
        assert_eq!(l.document_mb, 2048);
        let edited: Limits =
            serde_json::from_str(r#"{"document_mb": 99999, "gedcom_mb": 0}"#).unwrap();
        let l = edited.bounded();
        assert_eq!((l.document_mb, l.gedcom_mb, l.note_image_mb), (2048, 1, 10));
    }

    #[test]
    fn a_kind_refused_is_the_only_one_refused() {
        let l = Limits {
            refused: vec!["video/x-msvideo".into()],
            ..Limits::default()
        };
        assert!(!l.allows("video/x-msvideo"));
        assert!(l.allows("video/mp4"));
        assert_eq!(kind_name("video/x-msvideo"), "AVI");
        // Every kind the archive recognises is listed, and only those.
        let listed: Vec<&str> = KINDS
            .iter()
            .flat_map(|(_, k)| k.iter().map(|(m, _)| *m))
            .collect();
        for m in &listed {
            assert!(listed.iter().filter(|x| *x == m).count() == 1, "{m} twice");
        }
        assert_eq!(listed.len(), 16);
    }
}
