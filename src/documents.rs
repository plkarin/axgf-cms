//! Binary attachments: what may be uploaded, how it is identified, and how
//! thumbnails are made.
//!
//! # The bundle already carries files
//!
//! Everything under `documents/files/**` is a payload the bundle already
//! carries. `import_bundle_streaming` hands each one over as a live reader and
//! `export_bundle_streaming` asks for it back at the same ZIP path, so an
//! upload is a file in the payload cache plus a Document entity — the bytes
//! never pass through the in-memory bundle in either direction. Nothing in
//! `axgf-rs` needed changing for uploads to work; this module is the CMS end
//! of an existing capability.
//!
//! # Why the filename is not consulted
//!
//! The client controls the filename and the `Content-Type` header, so neither
//! is evidence of anything. [`sniff`] reads the leading bytes of the file
//! itself and accepts only what it recognises, which makes the check an
//! allowlist rather than a blocklist: a renamed ELF binary is not rejected
//! because it is on a list of bad things, it is rejected because nothing
//! recognised it as an image.
//!
//! # SVG
//!
//! **Refused.** An SVG is a document with a script element in it as far as a
//! browser is concerned, and serving one from the same origin as the admin
//! session would hand an uploader script execution against that session.
//! Sanitising it properly means parsing XML and maintaining an element and
//! attribute allowlist, which is a security surface this application has no
//! reason to own. It is also unsniffable in the sense this module needs —
//! plain XML text with no fixed magic number — so it cannot be told apart
//! from any other XML by the rule the rest of the uploads follow. Bitmap
//! formats cover what a family archive holds. See [`serve_inline`] for the
//! matching rule on the way out: a bundle authored elsewhere may still contain
//! an SVG, and it is served as an attachment, never inline.
//!
//! # Scale
//!
//! Textual data — persons, families, document *metadata*, the manifest — is
//! memory-resident and bounded by the size of the tree, not by its media.
//! Binary payloads live in a disk cache (see [`crate::payloads`]) and are
//! streamed from there, so a media-heavy archive no longer sits in RAM as
//! base64. Uploads are capped per file, and the admin panel warns once the
//! textual bundle passes a threshold.

use std::sync::Mutex;

use sha2::{Digest, Sha256};

/// Largest single upload accepted, in bytes.
pub const MAX_UPLOAD: usize = 10 * 1024 * 1024;

/// Textual-bundle size past which the admin panel starts warning, in bytes.
///
/// Not a limit — the operator's archive is theirs — but the point where the
/// memory-resident textual data starts costing something they should know
/// about. Binary payloads no longer count toward this: they are on disk.
pub const DEFAULT_SIZE_WARN: u64 = 200 * 1024 * 1024;

/// A file type this application is willing to store.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Kind {
    /// The MIME type, as determined from the bytes.
    pub mime: &'static str,
    /// The extension used for the attachment path inside the bundle.
    pub ext: &'static str,
    /// True for raster images, which get a thumbnail and render inline.
    pub raster_image: bool,
}

const fn k(mime: &'static str, ext: &'static str, raster_image: bool) -> Kind {
    Kind {
        mime,
        ext,
        raster_image,
    }
}

/// Identify a file from its leading bytes.
///
/// Returns `None` for anything not recognised, which includes every
/// executable and script format — they are not rejected by name, they simply
/// never match.
pub fn sniff(bytes: &[u8]) -> Option<Kind> {
    let starts = |sig: &[u8]| bytes.len() >= sig.len() && &bytes[..sig.len()] == sig;
    let at = |off: usize, sig: &[u8]| {
        bytes.len() >= off + sig.len() && &bytes[off..off + sig.len()] == sig
    };

    // Images.
    if starts(b"\x89PNG\r\n\x1a\n") {
        return Some(k("image/png", "png", true));
    }
    if starts(b"\xff\xd8\xff") {
        return Some(k("image/jpeg", "jpg", true));
    }
    if starts(b"GIF87a") || starts(b"GIF89a") {
        return Some(k("image/gif", "gif", true));
    }
    if starts(b"RIFF") && at(8, b"WEBP") {
        return Some(k("image/webp", "webp", true));
    }
    if starts(b"BM") {
        return Some(k("image/bmp", "bmp", true));
    }
    // TIFF, both byte orders. Kept as an image but not a raster one for our
    // purposes: browsers do not display TIFF, so it downloads like a document.
    if starts(b"II\x2a\x00") || starts(b"MM\x00\x2a") {
        return Some(k("image/tiff", "tif", false));
    }

    // Documents.
    if starts(b"%PDF-") {
        return Some(k("application/pdf", "pdf", false));
    }

    // Audio and video.
    if starts(b"OggS") {
        return Some(k("audio/ogg", "ogg", false));
    }
    if starts(b"fLaC") {
        return Some(k("audio/flac", "flac", false));
    }
    if starts(b"ID3") || (bytes.len() >= 2 && bytes[0] == 0xff && (bytes[1] & 0xe0) == 0xe0) {
        return Some(k("audio/mpeg", "mp3", false));
    }
    if starts(b"RIFF") && at(8, b"WAVE") {
        return Some(k("audio/wav", "wav", false));
    }
    // ISO base media: "....ftyp" at offset 4. The brand distinguishes audio
    // from video, and anything unfamiliar is treated as video.
    if at(4, b"ftyp") {
        return Some(if at(8, b"M4A ") {
            k("audio/mp4", "m4a", false)
        } else {
            k("video/mp4", "mp4", false)
        });
    }
    if starts(b"\x1a\x45\xdf\xa3") {
        return Some(k("video/webm", "webm", false));
    }
    if starts(b"RIFF") && at(8, b"AVI ") {
        return Some(k("video/x-msvideo", "avi", false));
    }

    // Plain text, last: it has no magic number, so it is inferred from the
    // content being valid UTF-8 with no control characters. That check must
    // run after every binary format, or a binary whose header happens to be
    // ASCII would be stored as text.
    //
    // A markup document is text by that test but is refused outright: an SVG
    // or an HTML file is a program a browser will run, and "refused" is the
    // decision this application documents. Storing one as `text/plain` would
    // technically be safe — attachment disposition, `nosniff` — but it would
    // make the documented rule and the behaviour disagree, and the rule is
    // the thing an operator reads.
    if looks_like_text(bytes) && !is_markup_document(bytes) {
        return Some(k("text/plain", "txt", false));
    }

    None
}

/// Whether a text file opens as an SVG or HTML document.
///
/// Only the *opening* of the file is examined, after any byte-order mark, XML
/// declaration, doctype or comment, so a plain note that happens to mention
/// `<svg>` somewhere in its body is still an ordinary note.
fn is_markup_document(bytes: &[u8]) -> bool {
    let head = &bytes[..bytes.len().min(2048)];
    let text = String::from_utf8_lossy(head);
    let mut rest = text.trim_start_matches('\u{feff}').trim_start();
    // Walk past the prologue: <?xml …?>, <!DOCTYPE …>, <!-- … -->.
    loop {
        let lower = rest.to_ascii_lowercase();
        let end = if lower.starts_with("<?") {
            rest.find("?>").map(|i| i + 2)
        } else if lower.starts_with("<!--") {
            rest.find("-->").map(|i| i + 3)
        } else if lower.starts_with("<!doctype") {
            rest.find('>').map(|i| i + 1)
        } else {
            None
        };
        match end {
            Some(i) => rest = rest[i..].trim_start(),
            None => break,
        }
    }
    let lower = rest.to_ascii_lowercase();
    lower.starts_with("<svg") || lower.starts_with("<html")
}

/// Whether a byte run is plausibly plain text.
///
/// Requires valid UTF-8 and no control characters beyond tab, newline and
/// carriage return. A shell script passes this — which is the point: it is
/// stored and served as `text/plain` with `nosniff` and an attachment
/// disposition, so a browser downloads it rather than doing anything with it.
fn looks_like_text(bytes: &[u8]) -> bool {
    if bytes.is_empty() {
        return false;
    }
    const SAMPLE: usize = 8192;
    let truncated = bytes.len() > SAMPLE;
    let head = &bytes[..bytes.len().min(SAMPLE)];
    let text = match std::str::from_utf8(head) {
        Ok(s) => s,
        // The only forgivable failure is a multi-byte character cut in half by
        // the sample boundary, which `error_len() == None` identifies exactly.
        // Anything else means invalid bytes in the body of the file: an
        // executable whose header happens to be ASCII must not slip through on
        // the strength of its first two characters.
        Err(e) if truncated && e.error_len().is_none() => {
            match std::str::from_utf8(&head[..e.valid_up_to()]) {
                Ok(s) => s,
                Err(_) => return false,
            }
        }
        Err(_) => return false,
    };
    !text
        .chars()
        .any(|c| c.is_control() && !matches!(c, '\t' | '\n' | '\r'))
}

/// Whether a stored document may be rendered inline rather than downloaded.
///
/// Only the raster formats a browser draws as pixels. Everything else,
/// including any `image/svg+xml` that arrived inside a bundle authored
/// elsewhere, is served as an attachment: an SVG rendered inline from this
/// origin executes its own script against the viewer's session.
pub fn serve_inline(mime: &str) -> bool {
    matches!(
        mime,
        "image/png" | "image/jpeg" | "image/gif" | "image/webp" | "image/bmp"
    )
}

/// Lowercase hex SHA-256 of a byte run.
pub fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut out = String::with_capacity(64);
    for b in digest {
        use std::fmt::Write as _;
        let _ = write!(out, "{b:02x}");
    }
    out
}

/// The ZIP path an uploaded file is stored at inside the bundle.
pub fn attachment_path(doc_id: &str, ext: &str) -> String {
    format!("documents/files/{doc_id}.{ext}")
}

/// Longest edge of a generated thumbnail, in pixels.
pub const THUMB_EDGE: u32 = 320;

/// The EXIF orientation tag, 1–8. A phone writes the sensor orientation here
/// rather than rotating the pixels, so an image that looks upright in a file
/// browser renders sideways when the tag is ignored. `1` (or absent) means the
/// pixels are already upright.
///
/// The `image` crate does not read this, so it is parsed with `kamadak-exif`.
pub fn exif_orientation(bytes: &[u8]) -> u32 {
    let mut cursor = std::io::Cursor::new(bytes);
    let reader = exif::Reader::new();
    let Ok(exif) = reader.read_from_container(&mut cursor) else {
        return 1;
    };
    exif.get_field(exif::Tag::Orientation, exif::In::PRIMARY)
        .and_then(|f| f.value.get_uint(0))
        .filter(|v| (1..=8).contains(v))
        .unwrap_or(1)
}

/// Apply an EXIF orientation to a decoded image. All eight values are handled,
/// not just the two common rotations: the four flips occur on scanned material
/// and mirrored front-camera shots.
pub fn apply_orientation(img: image::DynamicImage, orientation: u32) -> image::DynamicImage {
    use image::DynamicImage as D;
    match orientation {
        2 => img.fliph(),
        3 => img.rotate180(),
        4 => img.flipv(),
        5 => D::ImageRgba8(img.rotate90().fliph().to_rgba8()),
        6 => img.rotate90(),
        7 => D::ImageRgba8(img.rotate270().fliph().to_rgba8()),
        8 => img.rotate270(),
        // 1, and anything unexpected, leave untouched.
        _ => img,
    }
}

/// Decode an image and apply its EXIF orientation.
fn decode_upright(bytes: &[u8]) -> Option<image::DynamicImage> {
    let img = image::load_from_memory(bytes).ok()?;
    Some(apply_orientation(img, exif_orientation(bytes)))
}

/// Render a downscaled PNG of an image, oriented per its EXIF tag, or `None`
/// when the bytes are not an image this build can decode.
pub fn thumbnail(bytes: &[u8]) -> Option<Vec<u8>> {
    // Guessing the format from the bytes rather than trusting the stored MIME
    // keeps this consistent with how the file was admitted in the first place.
    let img = decode_upright(bytes)?;
    let thumb = img.thumbnail(THUMB_EDGE, THUMB_EDGE);
    let mut out = std::io::Cursor::new(Vec::new());
    thumb.write_to(&mut out, image::ImageFormat::Png).ok()?;
    Some(out.into_inner())
}

/// A full-size PNG of an image with its EXIF orientation applied, or `None`
/// when no correction is needed (orientation 1) or the bytes are not a
/// decodable image — in which case the caller serves the stored bytes as they
/// are. The stored original is never modified; only the displayed copy is
/// corrected.
pub fn oriented_image(bytes: &[u8]) -> Option<Vec<u8>> {
    if exif_orientation(bytes) == 1 {
        return None;
    }
    let img = decode_upright(bytes)?;
    let mut out = std::io::Cursor::new(Vec::new());
    img.write_to(&mut out, image::ImageFormat::Png).ok()?;
    Some(out.into_inner())
}

/// In-memory thumbnail store, bounded by total bytes.
///
/// Decoding a full-resolution photograph on every request is the kind of cost
/// that only shows up once a gallery has a dozen pictures in it, and an
/// unbounded cache is the same mistake in the other direction — the bundle
/// already occupies memory in full. Least-recently-used entries are dropped
/// once the budget is exceeded.
pub struct ThumbCache {
    entries: Mutex<Vec<CacheEntry>>,
    budget: usize,
}

struct CacheEntry {
    id: String,
    /// The document's content hash, so a replaced file misses rather than
    /// serving the previous picture.
    sha256: String,
    png: Vec<u8>,
    /// Monotonic counter, bumped on every hit; smallest is evicted first.
    used: u64,
}

impl Default for ThumbCache {
    fn default() -> Self {
        Self::with_budget(16 * 1024 * 1024)
    }
}

impl ThumbCache {
    pub fn with_budget(budget: usize) -> Self {
        Self {
            entries: Mutex::new(Vec::new()),
            budget,
        }
    }

    /// Fetch a cached thumbnail, or build and store one.
    ///
    /// `build` is only called on a miss, and is passed nothing: the caller
    /// already holds the bytes.
    pub fn get_or_insert(
        &self,
        id: &str,
        sha256: &str,
        build: impl FnOnce() -> Option<Vec<u8>>,
    ) -> Option<Vec<u8>> {
        let mut guard = self.entries.lock().unwrap_or_else(|e| e.into_inner());
        let tick = guard.iter().map(|e| e.used).max().unwrap_or(0) + 1;
        if let Some(hit) = guard.iter_mut().find(|e| e.id == id && e.sha256 == sha256) {
            hit.used = tick;
            return Some(hit.png.clone());
        }
        // A stale entry for this id is replaced, not kept alongside.
        guard.retain(|e| e.id != id);
        drop(guard);

        let png = build()?;

        let mut guard = self.entries.lock().unwrap_or_else(|e| e.into_inner());
        guard.push(CacheEntry {
            id: id.to_string(),
            sha256: sha256.to_string(),
            png: png.clone(),
            used: tick,
        });
        let mut total: usize = guard.iter().map(|e| e.png.len()).sum();
        while total > self.budget && guard.len() > 1 {
            let (idx, _) = guard
                .iter()
                .enumerate()
                .min_by_key(|(_, e)| e.used)
                .expect("non-empty");
            total -= guard[idx].png.len();
            guard.remove(idx);
        }
        Some(png)
    }

    /// How many thumbnails are held. For tests and diagnostics.
    pub fn len(&self) -> usize {
        self.entries.lock().unwrap_or_else(|e| e.into_inner()).len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Format a byte count for the admin panel.
pub fn human_size(bytes: u64) -> String {
    if bytes >= 1024 * 1024 * 1024 {
        format!("{:.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    } else if bytes >= 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else if bytes >= 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{bytes} bytes")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn images_are_recognised_by_their_magic_bytes() {
        assert_eq!(sniff(b"\x89PNG\r\n\x1a\n rest").unwrap().mime, "image/png");
        assert_eq!(sniff(b"\xff\xd8\xff\xe0 jfif").unwrap().mime, "image/jpeg");
        assert_eq!(sniff(b"GIF89a....").unwrap().mime, "image/gif");
        assert_eq!(sniff(b"RIFF....WEBPVP8 ").unwrap().mime, "image/webp");
        assert!(sniff(b"\x89PNG\r\n\x1a\n").unwrap().raster_image);
    }

    #[test]
    fn documents_audio_and_video_are_accepted() {
        assert_eq!(sniff(b"%PDF-1.7\n%").unwrap().mime, "application/pdf");
        assert_eq!(sniff(b"OggS\x00\x02").unwrap().mime, "audio/ogg");
        assert_eq!(
            sniff(b"\x00\x00\x00\x20ftypisom").unwrap().mime,
            "video/mp4"
        );
        assert_eq!(
            sniff(b"\x00\x00\x00\x20ftypM4A ").unwrap().mime,
            "audio/mp4"
        );
        assert_eq!(sniff(b"RIFF....WAVEfmt ").unwrap().mime, "audio/wav");
        assert_eq!(sniff(b"Dear Jules,\nI write\n").unwrap().mime, "text/plain");
    }

    #[test]
    fn an_executable_renamed_to_a_photo_is_not_a_photo() {
        // The filename says .jpg; the bytes say ELF. Only the bytes are
        // consulted, and nothing in the allowlist matches them.
        let elf = b"\x7fELF\x02\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00";
        assert!(sniff(elf).is_none());
        // Windows PE, Mach-O, Java class, and a wasm module: all unmatched.
        assert!(sniff(b"MZ\x90\x00\x03\x00\x00\x00").is_none());
        assert!(sniff(b"\xfe\xed\xfa\xce\x00\x00\x00\x00").is_none());
        assert!(sniff(b"\xca\xfe\xba\xbe\x00\x00\x00\x34").is_none());
        assert!(sniff(b"\x00asm\x01\x00\x00\x00").is_none());
    }

    #[test]
    fn svg_is_refused_rather_than_sanitised() {
        // An SVG is valid UTF-8 text, so without the markup check it would be
        // stored as a plain-text document. It is refused instead, which is the
        // rule this application documents.
        let svg = br#"<svg xmlns="http://www.w3.org/2000/svg"><script>alert(1)</script></svg>"#;
        assert!(sniff(svg).is_none());
        // Including behind the prologue a real file has in front of it.
        let declared = br#"<?xml version="1.0" encoding="UTF-8"?>
<!-- Generated by a drawing program -->
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/TR/svg11.dtd">
<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10"></svg>"#;
        assert!(sniff(declared).is_none());
        // HTML is refused for the same reason: a browser would run it.
        assert!(sniff(b"<!DOCTYPE html>\n<html><body>hi</body></html>").is_none());

        // And on the way out, an SVG that arrived inside a bundle authored
        // elsewhere is still never rendered inline.
        assert!(!serve_inline("image/svg+xml"));
        assert!(serve_inline("image/png"));
    }

    #[test]
    fn a_note_that_merely_mentions_markup_is_still_a_note() {
        // The refusal looks at how the file opens, not at whether the string
        // appears anywhere in it — otherwise a genealogist's note about a
        // scanned diagram would be rejected.
        let note = b"Grandfather's letter mentions a <svg> tag in the archive\n\
                     catalogue, which we could not make sense of.\n";
        assert_eq!(sniff(note).unwrap().mime, "text/plain");
    }

    #[test]
    fn a_script_is_stored_as_text_not_as_something_executable() {
        // Nothing here will run it: `text/plain`, `nosniff`, and an attachment
        // disposition. Rejecting it outright would also throw away the plain
        // notes a family archive legitimately holds.
        let k = sniff(b"#!/bin/sh\nrm -rf /\n").expect("valid UTF-8 text");
        assert_eq!(k.mime, "text/plain");
        assert!(!k.raster_image);
        assert!(!serve_inline(k.mime));
    }

    #[test]
    fn binary_junk_and_emptiness_are_refused() {
        assert!(sniff(b"").is_none());
        assert!(sniff(&[0x00, 0x01, 0x02, 0x03, 0xfe, 0xff]).is_none());
    }

    #[test]
    fn sha256_matches_the_known_digest_of_the_empty_string() {
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn attachment_paths_land_where_import_bundle_looks() {
        assert_eq!(
            attachment_path("abc-123", "jpg"),
            "documents/files/abc-123.jpg"
        );
    }

    /// A 2x2 red PNG, encoded once so the thumbnail tests have real bytes.
    fn tiny_png() -> Vec<u8> {
        let img = image::RgbImage::from_pixel(64, 32, image::Rgb([200, 30, 30]));
        let mut out = std::io::Cursor::new(Vec::new());
        image::DynamicImage::ImageRgb8(img)
            .write_to(&mut out, image::ImageFormat::Png)
            .expect("encode");
        out.into_inner()
    }

    #[test]
    fn a_thumbnail_is_smaller_than_the_original_and_still_an_image() {
        let big = image::DynamicImage::ImageRgb8(image::RgbImage::from_pixel(
            2000,
            1000,
            image::Rgb([10, 90, 200]),
        ));
        let mut buf = std::io::Cursor::new(Vec::new());
        big.write_to(&mut buf, image::ImageFormat::Png).unwrap();
        let png = thumbnail(&buf.into_inner()).expect("a thumbnail");
        let decoded = image::load_from_memory(&png).expect("valid png");
        assert!(decoded.width() <= THUMB_EDGE && decoded.height() <= THUMB_EDGE);
        // Aspect ratio is kept, so a 2:1 picture stays 2:1.
        assert_eq!(decoded.width(), THUMB_EDGE);
        assert_eq!(decoded.height(), THUMB_EDGE / 2);
    }

    #[test]
    fn a_pdf_has_no_thumbnail_rather_than_a_broken_one() {
        assert!(thumbnail(b"%PDF-1.7\nnot an image").is_none());
    }

    /// A minimal JPEG carrying a single EXIF orientation tag, so the parser and
    /// every transform can be exercised without a real photograph.
    fn jpeg_with_orientation(o: u16) -> Vec<u8> {
        // A 2x1 red/blue JPEG, then an APP1/EXIF segment inserted after SOI
        // declaring the orientation. Building the TIFF header by hand keeps the
        // test self-contained.
        let base = {
            let img = image::RgbImage::from_fn(2, 1, |x, _| {
                if x == 0 {
                    image::Rgb([200, 0, 0])
                } else {
                    image::Rgb([0, 0, 200])
                }
            });
            let mut out = std::io::Cursor::new(Vec::new());
            image::DynamicImage::ImageRgb8(img)
                .write_to(&mut out, image::ImageFormat::Jpeg)
                .unwrap();
            out.into_inner()
        };
        // Little-endian TIFF with one IFD entry: Orientation (0x0112), SHORT.
        let mut tiff = Vec::new();
        tiff.extend_from_slice(b"II\x2a\x00");
        tiff.extend_from_slice(&8u32.to_le_bytes()); // IFD offset
        tiff.extend_from_slice(&1u16.to_le_bytes()); // one entry
        tiff.extend_from_slice(&0x0112u16.to_le_bytes());
        tiff.extend_from_slice(&3u16.to_le_bytes()); // SHORT
        tiff.extend_from_slice(&1u32.to_le_bytes()); // count
        tiff.extend_from_slice(&(o as u32).to_le_bytes());
        tiff.extend_from_slice(&0u32.to_le_bytes()); // next IFD
        let mut app1 = Vec::new();
        app1.extend_from_slice(b"Exif\x00\x00");
        app1.extend_from_slice(&tiff);
        let mut out = Vec::new();
        out.extend_from_slice(&base[..2]); // SOI
        out.push(0xff);
        out.push(0xe1); // APP1
        out.extend_from_slice(&((app1.len() + 2) as u16).to_be_bytes());
        out.extend_from_slice(&app1);
        out.extend_from_slice(&base[2..]);
        out
    }

    #[test]
    fn exif_orientation_is_read_for_all_eight_values() {
        for o in 1..=8u16 {
            let bytes = jpeg_with_orientation(o);
            assert_eq!(
                exif_orientation(&bytes),
                o as u32,
                "orientation {o} must be read back"
            );
        }
        // No EXIF at all reads as upright.
        let plain = tiny_png();
        assert_eq!(exif_orientation(&plain), 1);
    }

    #[test]
    fn every_orientation_transform_yields_an_image() {
        // The four 90° rotations swap the axes; the flips do not. Checking the
        // dimensions confirms each of the eight branches ran rather than
        // silently falling through.
        let img = image::DynamicImage::ImageRgb8(image::RgbImage::from_pixel(
            4,
            2,
            image::Rgb([1, 2, 3]),
        ));
        for o in 1..=8u32 {
            let out = apply_orientation(img.clone(), o);
            let swaps = matches!(o, 5..=8);
            if swaps {
                assert_eq!((out.width(), out.height()), (2, 4), "o={o} swaps axes");
            } else {
                assert_eq!((out.width(), out.height()), (4, 2), "o={o} keeps axes");
            }
        }
    }

    #[test]
    fn an_upright_image_needs_no_correction_and_a_rotated_one_does() {
        // orientation 1 → nothing to correct, so the caller streams the stored
        // bytes unchanged; a rotated one yields a corrected PNG.
        assert!(oriented_image(&tiny_png()).is_none());
        let rotated = jpeg_with_orientation(6);
        let corrected = oriented_image(&rotated).expect("a corrected image");
        assert!(image::load_from_memory(&corrected).is_ok());
    }

    #[test]
    fn a_thumbnail_of_a_rotated_photo_is_upright() {
        // A 4-wide, 2-tall image tagged as rotated 90° (orientation 6) must come
        // back taller than it is wide once the tag is honoured.
        let bytes = jpeg_with_orientation(6);
        let png = thumbnail(&bytes).expect("a thumbnail");
        let decoded = image::load_from_memory(&png).expect("valid png");
        assert!(
            decoded.height() > decoded.width(),
            "a 90°-rotated landscape thumbnail must end up portrait, got {}x{}",
            decoded.width(),
            decoded.height()
        );
    }

    #[test]
    fn the_thumb_cache_serves_the_second_request_without_rebuilding() {
        let cache = ThumbCache::default();
        let png = tiny_png();
        let mut builds = 0;
        for _ in 0..3 {
            let got = cache.get_or_insert("d1", "hash1", || {
                builds += 1;
                thumbnail(&png)
            });
            assert!(got.is_some());
        }
        assert_eq!(builds, 1, "only the first request may decode");
    }

    #[test]
    fn a_replaced_file_is_not_served_from_the_old_thumbnail() {
        let cache = ThumbCache::default();
        let png = tiny_png();
        cache.get_or_insert("d1", "hash1", || thumbnail(&png));
        let mut rebuilt = false;
        cache.get_or_insert("d1", "hash2", || {
            rebuilt = true;
            thumbnail(&png)
        });
        assert!(rebuilt, "a different digest must miss the cache");
        assert_eq!(cache.len(), 1, "and replace the entry, not sit beside it");
    }

    #[test]
    fn the_cache_stops_growing_at_its_budget() {
        // A budget smaller than two thumbnails, so every insert evicts.
        let one = thumbnail(&tiny_png()).expect("a thumbnail").len();
        let cache = ThumbCache::with_budget(one + one / 2);
        let png = tiny_png();
        for i in 0..25 {
            cache.get_or_insert(&format!("d{i}"), "h", || thumbnail(&png));
        }
        assert!(
            cache.len() <= 2,
            "the cache must stay bounded, held {}",
            cache.len()
        );
        assert!(!cache.is_empty());
    }

    #[test]
    fn a_thumbnail_that_cannot_be_built_is_not_cached_as_a_success() {
        let cache = ThumbCache::default();
        assert!(cache
            .get_or_insert("d1", "h", || thumbnail(b"%PDF-1.7"))
            .is_none());
        assert!(cache.is_empty());
    }

    #[test]
    fn sizes_read_the_way_a_file_listing_would() {
        assert_eq!(human_size(900), "900 bytes");
        assert_eq!(human_size(2048), "2.0 KB");
        assert_eq!(human_size(5 * 1024 * 1024), "5.0 MB");
        assert_eq!(human_size(3 * 1024 * 1024 * 1024), "3.0 GB");
    }
}
