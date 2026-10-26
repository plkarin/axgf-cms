//! Any GEDCOM a family holds, read into the one dialect the converter reads
//! (v0.5.0, docs/adr/0005 §3).
//!
//! `axgf_rs::convert_gedcom` reads GEDCOM 5.5.1 as most software writes it
//! today: UTF-8, or UTF-16 behind a byte-order mark; lines ending in LF or
//! CRLF; a note as `NOTE`. A family's file is often older or newer than that,
//! and the converter's fallback for an encoding it does not recognise is
//! Latin-1, which turns ANSEL's "Józef Gęsiorek" into "Jâozef Gñesiorek"
//! without a word. Measured on one small family written seven ways: a 5.5
//! file in ANSEL lost every accent, a 5.5.5 file in UTF-16 without a mark was
//! refused, a file with old Macintosh line endings came through with nobody
//! in it, a GEDCOM 7 file counted its `@VOID@` placeholder as a child and
//! lost its shared note, and a GEDCOM 7 `.gdz` was refused. So the CMS reads
//! the file first:
//!
//! - **the container**: a `.ged`, or GEDCOM 7's `.gdz`, a zip whose dataset
//!   is `gedcom.ged`;
//! - **the encoding**: a byte-order mark when there is one; UTF-16 without
//!   one, from where the zero bytes fall; otherwise bytes that are valid
//!   UTF-8 are UTF-8 whatever the header says, and the rest are read as the
//!   header's `CHAR` declares — ANSEL (decoded here: its accents come before
//!   their letter, and are composed back onto it), Macintosh, IBM PC (code
//!   page 850, DOS software's), or ANSI — Windows-1252, a superset of
//!   Latin-1 — for anything else. A UTF-8 file with a few bytes that are not
//!   UTF-8 in it (one note pasted from another program) stays UTF-8, those
//!   bytes alone read as Windows-1252, rather than every accent in the file
//!   turning to "Ã©";
//! - **the line endings**: a CR alone (old Macintosh software), CRLF and LF
//!   all become LF;
//! - **the lines**: indentation, tabs or runs of spaces between the level,
//!   the cross-reference and the tag, a tag in lower case, a space left at
//!   the end of a line that a `CONC` continues, and a `@#DGREGORIAN@` mark on
//!   a date are mended — the converter reads one space between fields, an
//!   upper-case tag, and no calendar mark, and drops or mangles the rest; a
//!   line nested deeper than any GEDCOM structure goes is set aside with
//!   what it holds, because the converter's cost grows with the square of a
//!   file's depth;
//! - **GEDCOM 7**: a pointer to `@VOID@` — "no record" — is dropped with
//!   its substructure rather than read as a person, and a shared note
//!   (`SNOTE`) is read as the `NOTE` 5.5.1 calls it.
//!
//! What it did is reported, so the page that shows a conversion can say how
//! the file was read before anybody relies on the result.

use std::borrow::Cow;
use std::io::Read as _;

use serde::Serialize;
use unicode_normalization::UnicodeNormalization;

/// The encoding a file was read in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Encoding {
    Utf8,
    Utf16Le,
    Utf16Be,
    Ansel,
    Windows1252,
    MacRoman,
    /// IBM PC, as DOS software wrote it: code page 850, whose letters are
    /// code page 437's and more.
    Ibm850,
}

impl Encoding {
    /// The name a reader knows it by.
    pub fn label(self) -> &'static str {
        match self {
            Encoding::Utf8 => "UTF-8",
            Encoding::Utf16Le => "UTF-16 LE",
            Encoding::Utf16Be => "UTF-16 BE",
            Encoding::Ansel => "ANSEL",
            Encoding::Windows1252 => "Windows-1252",
            Encoding::MacRoman => "Mac Roman",
            Encoding::Ibm850 => "IBM PC (code page 850)",
        }
    }
}

/// How a file was read: what it was, and what was mended on the way.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Reading {
    /// A GEDCOM 7 `.gdz` rather than a bare file.
    pub gedzip: bool,
    /// Files in the `.gdz` beside its dataset (photographs, scans). Counted,
    /// not carried: a `.gdz`'s media are not documents yet.
    pub media: usize,
    /// The encoding the text was decoded from.
    pub encoding: Option<Encoding>,
    /// The header's own `CHAR`, as written, when it has one.
    pub declared: Option<String>,
    /// The GEDCOM version the header gives (`HEAD.GEDC.VERS`).
    pub version: Option<String>,
    /// Lines ended by a CR alone, mended to LF.
    pub lone_crs: usize,
    /// `@VOID@` pointers dropped, each with its substructure.
    pub voids: usize,
    /// Shared notes (`SNOTE`) read as `NOTE`.
    pub shared_notes: usize,
    /// Bytes that were not UTF-8 in a file read as UTF-8, read as
    /// Windows-1252 one by one.
    pub stray_bytes: usize,
    /// Lines whose layout was mended (see the module's comment).
    pub tidied: usize,
    /// Lines nested deeper than [`MAX_LEVEL`], set aside.
    pub too_deep: usize,
}

impl Reading {
    /// Whether the encoding read differs from the one the header declares —
    /// worth saying, because the header is what the family's software claims.
    pub fn declared_otherwise(&self) -> bool {
        let Some(d) = &self.declared else {
            return false;
        };
        let d = d.to_ascii_uppercase();
        let said = match d.as_str() {
            "ANSEL" => Some(Encoding::Ansel),
            "MACINTOSH" | "MACROMAN" => Some(Encoding::MacRoman),
            d if is_ibm_pc(d) => Some(Encoding::Ibm850),
            "ANSI" | "WINDOWS-1252" | "CP1252" | "LATIN1" | "ISO-8859-1" | "ISO8859-1" => {
                Some(Encoding::Windows1252)
            }
            "UTF-8" | "UTF8" => Some(Encoding::Utf8),
            _ => None,
        };
        match (said, self.encoding) {
            // ASCII is a subset of everything: an ANSEL file with no accent in
            // it was read as UTF-8 and said exactly the same.
            (Some(s), Some(e)) => s != e && !(e == Encoding::Utf8),
            _ => false,
        }
    }
}

/// A file read and ready for the converter.
#[derive(Debug, Clone)]
pub struct Read {
    /// GEDCOM text in the 5.5.1 dialect the converter reads.
    pub text: String,
    pub reading: Reading,
}

/// Why a file could not be read as GEDCOM at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// Nothing in it begins `0 HEAD`.
    NotGedcom,
    /// A zip with no `gedcom.ged` and no single `.ged` in it.
    NoDataset,
    /// The dataset, once out of its zip, is larger than the upload limit.
    TooLarge,
}

/// Read `bytes` as a GEDCOM of any version, in any encoding, bare or in a
/// `.gdz`. `limit` bounds what a zip may unpack to.
pub fn read(bytes: &[u8], limit: usize) -> Result<Read, Refusal> {
    let mut reading = Reading::default();
    let raw: Cow<[u8]> = if bytes.starts_with(b"PK\x03\x04") {
        reading.gedzip = true;
        let (dataset, media) = unzip_dataset(bytes, limit)?;
        reading.media = media;
        Cow::Owned(dataset)
    } else {
        Cow::Borrowed(bytes)
    };

    let (text, encoding, strays) = decode(&raw);
    reading.encoding = Some(encoding);
    reading.stray_bytes = strays;
    reading.declared = declared_char(&raw);
    let text = text.trim_start_matches('\u{feff}');

    // Line endings: count the CRs that stand alone before mending them.
    let b = text.as_bytes();
    reading.lone_crs = b
        .iter()
        .enumerate()
        .filter(|&(i, &c)| c == b'\r' && b.get(i + 1) != Some(&b'\n'))
        .count();
    let text = text.replace("\r\n", "\n").replace('\r', "\n");

    let seven = gedcom_version(&text).is_some_and(|v| v.starts_with('7'));
    let (text, tidied, too_deep) = tidy(&text, seven);
    reading.tidied = tidied;
    reading.too_deep = too_deep;

    let (text, voids, shared) = from_seven(&text);
    reading.voids = voids;
    reading.shared_notes = shared;
    reading.version = gedcom_version(&text);

    // The first GEDCOM line is the header's. A line of prose before it (some
    // programs write who exported the file) is not GEDCOM and is passed
    // over, as the converter passes it over; a binary file has no such line.
    let opens = text
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .take(8)
        .find_map(parts)
        .is_some_and(|p| p.level == 0 && p.tag.eq_ignore_ascii_case("HEAD"));
    if !opens {
        return Err(Refusal::NotGedcom);
    }
    Ok(Read { text, reading })
}

/// GEDCOM 7's zip: the dataset is `gedcom.ged` at its root; a zip holding a
/// single `.ged` under any name is read too. Everything else in it is media.
fn unzip_dataset(bytes: &[u8], limit: usize) -> Result<(Vec<u8>, usize), Refusal> {
    let mut zip =
        zip::ZipArchive::new(std::io::Cursor::new(bytes)).map_err(|_| Refusal::NotGedcom)?;
    let names: Vec<String> = zip
        .file_names()
        .map(|n| n.map(std::borrow::Cow::into_owned))
        .collect::<Result<_, _>>()
        .map_err(|_| Refusal::NotGedcom)?;
    let geds: Vec<&String> = names
        .iter()
        .filter(|n| n.to_ascii_lowercase().ends_with(".ged"))
        .collect();
    let pick = names
        .iter()
        .find(|n| n.eq_ignore_ascii_case("gedcom.ged"))
        .or(if geds.len() == 1 { Some(geds[0]) } else { None })
        .cloned()
        .ok_or(Refusal::NoDataset)?;
    let media = names
        .iter()
        .filter(|n| !n.ends_with('/') && **n != pick)
        .count();
    let file = zip.by_name(&pick).map_err(|_| Refusal::NoDataset)?;
    if file.size() > limit as u64 {
        return Err(Refusal::TooLarge);
    }
    // The size a zip declares is a claim; what is read is bounded anyway.
    let mut out = Vec::new();
    file.take(limit as u64 + 1)
        .read_to_end(&mut out)
        .map_err(|_| Refusal::NotGedcom)?;
    if out.len() > limit {
        return Err(Refusal::TooLarge);
    }
    Ok((out, media))
}

/// The header's `1 CHAR`, read from its first lines as plain ASCII, which is
/// what every encoding GEDCOM allows writes its header in.
fn declared_char(raw: &[u8]) -> Option<String> {
    // UTF-16 interleaves zeros; dropping them leaves the ASCII header.
    let head: Vec<u8> = raw.iter().take(8192).copied().filter(|&c| c != 0).collect();
    let head = String::from_utf8_lossy(&head);
    head.split(['\n', '\r'])
        .map(str::trim)
        .take_while(|l| !l.starts_with("0 @") && !l.starts_with("0 TRLR"))
        .find_map(|l| {
            let p = parts(l)?;
            (p.level == 1 && p.tag.eq_ignore_ascii_case("CHAR")).then(|| p.value.trim().to_string())
        })
        .filter(|v| !v.is_empty())
}

/// Bytes to text, the encoding they were read in, and how many bytes of a
/// UTF-8 file were not UTF-8.
fn decode(raw: &[u8]) -> (String, Encoding, usize) {
    if let Some(rest) = raw.strip_prefix(b"\xEF\xBB\xBF") {
        let (s, strays) = utf8_mending(rest);
        return (s, Encoding::Utf8, strays);
    }
    if raw.starts_with(b"\xFF\xFE") {
        return (utf16(&raw[2..], true), Encoding::Utf16Le, 0);
    }
    if raw.starts_with(b"\xFE\xFF") {
        return (utf16(&raw[2..], false), Encoding::Utf16Be, 0);
    }
    if let Some(little) = utf16_without_mark(raw) {
        let enc = if little {
            Encoding::Utf16Le
        } else {
            Encoding::Utf16Be
        };
        return (utf16(raw, little), enc, 0);
    }
    if let Ok(s) = std::str::from_utf8(raw) {
        return (s.to_string(), Encoding::Utf8, 0);
    }
    let declared = declared_char(raw).map(|c| c.to_ascii_uppercase());
    match declared.as_deref() {
        Some("ANSEL") => (ansel(raw), Encoding::Ansel, 0),
        Some("MACINTOSH" | "MACROMAN") => (
            encoding_rs::MACINTOSH
                .decode_without_bom_handling(raw)
                .0
                .into_owned(),
            Encoding::MacRoman,
            0,
        ),
        Some(d) if is_ibm_pc(d) => (raw.iter().map(|&b| cp850(b)).collect(), Encoding::Ibm850, 0),
        // A file written in UTF-8 with a few bytes from elsewhere in it: read
        // as UTF-8, those bytes alone as Windows-1252. Decided by what the
        // bytes are, not only by what the header says: valid UTF-8
        // sequences of two bytes or more almost never occur by chance in a
        // Windows-1252 file, and a file that has them and few strays was
        // written in UTF-8, whatever its header claims.
        _ if mostly_utf8(raw) => {
            let (s, strays) = utf8_mending(raw);
            (s, Encoding::Utf8, strays)
        }
        // ANSI, Windows-1252, Latin-1, a file that says UTF-8 and is not:
        // Windows-1252 is what such files nearly always are, and it reads
        // Latin-1 the same.
        _ => (
            encoding_rs::WINDOWS_1252
                .decode_without_bom_handling(raw)
                .0
                .into_owned(),
            Encoding::Windows1252,
            0,
        ),
    }
}

/// The names GEDCOM and the software that wrote it give to the IBM PC's
/// character set.
fn is_ibm_pc(declared: &str) -> bool {
    matches!(
        declared,
        "IBMPC"
            | "IBM PC"
            | "IBM-PC"
            | "IBM WINDOWS"
            | "CP850"
            | "CP437"
            | "IBM850"
            | "IBM437"
            | "DOS"
    )
}

/// UTF-16 without a byte-order mark, told by where its zero bytes fall: the
/// header is ASCII, so in its first few hundred bytes one byte of nearly
/// every pair is zero. `Some(true)` for little-endian. A leading blank line
/// does not hide it, as it hid the "0" of "0 HEAD" from the first test.
fn utf16_without_mark(raw: &[u8]) -> Option<bool> {
    let n = raw.len().min(512) & !1;
    if n < 8 {
        return None;
    }
    let (mut even, mut odd) = (0usize, 0usize);
    for (i, &b) in raw[..n].iter().enumerate() {
        if b == 0 {
            if i % 2 == 0 {
                even += 1;
            } else {
                odd += 1;
            }
        }
    }
    let pairs = n / 2;
    if odd * 10 >= pairs * 8 && even * 10 <= pairs {
        Some(true)
    } else if even * 10 >= pairs * 8 && odd * 10 <= pairs {
        Some(false)
    } else {
        None
    }
}

/// Whether bytes that are not valid UTF-8 were nevertheless written in it:
/// they hold multi-byte UTF-8 sequences, and no more stray bytes than such
/// sequences.
fn mostly_utf8(raw: &[u8]) -> bool {
    let (mut multi, mut strays) = (0usize, 0usize);
    for chunk in raw.utf8_chunks() {
        multi += chunk.valid().chars().filter(|c| c.len_utf8() > 1).count();
        strays += chunk.invalid().len();
    }
    multi > 0 && strays <= multi
}

/// UTF-8, with each byte that is not part of a valid sequence read as
/// Windows-1252. Returns the text and how many bytes were read that way.
fn utf8_mending(raw: &[u8]) -> (String, usize) {
    let mut out = String::with_capacity(raw.len());
    let mut strays = 0;
    for chunk in raw.utf8_chunks() {
        out.push_str(chunk.valid());
        for &b in chunk.invalid() {
            strays += 1;
            out.push_str(
                &encoding_rs::WINDOWS_1252
                    .decode_without_bom_handling(&[b])
                    .0,
            );
        }
    }
    (out, strays)
}

/// Code page 850, the IBM PC's Western European set, above ASCII.
fn cp850(b: u8) -> char {
    if b < 0x80 {
        return b as char;
    }
    const HIGH: [char; 128] = [
        '\u{00C7}', '\u{00FC}', '\u{00E9}', '\u{00E2}', '\u{00E4}', '\u{00E0}', '\u{00E5}',
        '\u{00E7}', '\u{00EA}', '\u{00EB}', '\u{00E8}', '\u{00EF}', '\u{00EE}', '\u{00EC}',
        '\u{00C4}', '\u{00C5}', '\u{00C9}', '\u{00E6}', '\u{00C6}', '\u{00F4}', '\u{00F6}',
        '\u{00F2}', '\u{00FB}', '\u{00F9}', '\u{00FF}', '\u{00D6}', '\u{00DC}', '\u{00F8}',
        '\u{00A3}', '\u{00D8}', '\u{00D7}', '\u{0192}', '\u{00E1}', '\u{00ED}', '\u{00F3}',
        '\u{00FA}', '\u{00F1}', '\u{00D1}', '\u{00AA}', '\u{00BA}', '\u{00BF}', '\u{00AE}',
        '\u{00AC}', '\u{00BD}', '\u{00BC}', '\u{00A1}', '\u{00AB}', '\u{00BB}', '\u{2591}',
        '\u{2592}', '\u{2593}', '\u{2502}', '\u{2524}', '\u{00C1}', '\u{00C2}', '\u{00C0}',
        '\u{00A9}', '\u{2563}', '\u{2551}', '\u{2557}', '\u{255D}', '\u{00A2}', '\u{00A5}',
        '\u{2510}', '\u{2514}', '\u{2534}', '\u{252C}', '\u{251C}', '\u{2500}', '\u{253C}',
        '\u{00E3}', '\u{00C3}', '\u{255A}', '\u{2554}', '\u{2569}', '\u{2566}', '\u{2560}',
        '\u{2550}', '\u{256C}', '\u{00A4}', '\u{00F0}', '\u{00D0}', '\u{00CA}', '\u{00CB}',
        '\u{00C8}', '\u{0131}', '\u{00CD}', '\u{00CE}', '\u{00CF}', '\u{2518}', '\u{250C}',
        '\u{2588}', '\u{2584}', '\u{00A6}', '\u{00CC}', '\u{2580}', '\u{00D3}', '\u{00DF}',
        '\u{00D4}', '\u{00D2}', '\u{00F5}', '\u{00D5}', '\u{00B5}', '\u{00FE}', '\u{00DE}',
        '\u{00DA}', '\u{00DB}', '\u{00D9}', '\u{00FD}', '\u{00DD}', '\u{00AF}', '\u{00B4}',
        '\u{00AD}', '\u{00B1}', '\u{2017}', '\u{00BE}', '\u{00B6}', '\u{00A7}', '\u{00F7}',
        '\u{00B8}', '\u{00B0}', '\u{00A8}', '\u{00B7}', '\u{00B9}', '\u{00B3}', '\u{00B2}',
        '\u{25A0}', '\u{00A0}',
    ];
    HIGH[(b - 0x80) as usize]
}

fn utf16(raw: &[u8], little: bool) -> String {
    let enc = if little {
        encoding_rs::UTF_16LE
    } else {
        encoding_rs::UTF_16BE
    };
    enc.decode_without_bom_handling(raw).0.into_owned()
}

/// ANSEL (ANSI Z39.47, as GEDCOM 5.5 uses it): ASCII below 0x80; spacing
/// letters and signs from 0xA1; combining accents from 0xE0, written *before*
/// the letter they sit on — the reverse of Unicode — so each is held until
/// its letter comes and then composed onto it.
fn ansel(raw: &[u8]) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut marks: Vec<char> = Vec::new();
    for &c in raw {
        if c == b'\n' || c == b'\r' {
            // An accent with no letter before the end of the line has nothing
            // to sit on, and must not open the next line.
            marks.clear();
            out.push(c as char);
        } else if c < 0x80 {
            out.push(c as char);
            out.extend(marks.drain(..));
        } else if let Some(m) = ansel_mark(c) {
            marks.push(m);
        } else {
            out.push(ansel_letter(c).unwrap_or('\u{fffd}'));
            out.extend(marks.drain(..));
        }
    }
    out.nfc().collect()
}

fn ansel_letter(c: u8) -> Option<char> {
    Some(match c {
        0xA1 => 'Ł',
        0xA2 => 'Ø',
        0xA3 => 'Đ',
        0xA4 => 'Þ',
        0xA5 => 'Æ',
        0xA6 => 'Œ',
        0xA7 => 'ʹ',
        0xA8 => '·',
        0xA9 => '♭',
        0xAA => '®',
        0xAB => '±',
        0xAC => 'Ơ',
        0xAD => 'Ư',
        0xAE => 'ʼ',
        0xB0 => 'ʻ',
        0xB1 => 'ł',
        0xB2 => 'ø',
        0xB3 => 'đ',
        0xB4 => 'þ',
        0xB5 => 'æ',
        0xB6 => 'œ',
        0xB7 => 'ʺ',
        0xB8 => 'ı',
        0xB9 => '£',
        0xBA => 'ð',
        0xBC => 'ơ',
        0xBD => 'ư',
        // GEDCOM's own additions to ANSEL.
        0xBE => '□',
        0xBF => '■',
        0xC0 => '°',
        0xC1 => 'ℓ',
        0xC2 => '℗',
        0xC3 => '©',
        0xC4 => '♯',
        0xC5 => '¿',
        0xC6 => '¡',
        0xC7 => 'ß',
        0xC8 => '€',
        _ => return None,
    })
}

fn ansel_mark(c: u8) -> Option<char> {
    Some(match c {
        0xE0 => '\u{0309}', // hook above
        0xE1 => '\u{0300}', // grave
        0xE2 => '\u{0301}', // acute
        0xE3 => '\u{0302}', // circumflex
        0xE4 => '\u{0303}', // tilde
        0xE5 => '\u{0304}', // macron
        0xE6 => '\u{0306}', // breve
        0xE7 => '\u{0307}', // dot above
        0xE8 => '\u{0308}', // diaeresis
        0xE9 => '\u{030C}', // caron
        0xEA => '\u{030A}', // ring above
        0xEB => '\u{FE20}', // ligature, left half
        0xEC => '\u{FE21}', // ligature, right half
        0xED => '\u{0315}', // comma above right
        0xEE => '\u{030B}', // double acute
        0xEF => '\u{0310}', // candrabindu
        0xF0 => '\u{0327}', // cedilla
        0xF1 => '\u{0328}', // ogonek
        0xF2 => '\u{0323}', // dot below
        0xF3 => '\u{0324}', // diaeresis below
        0xF4 => '\u{0325}', // ring below
        0xF5 => '\u{0333}', // double low line
        0xF6 => '\u{0332}', // low line
        0xF7 => '\u{0326}', // comma below
        0xF8 => '\u{031C}', // left half ring below
        0xF9 => '\u{032E}', // breve below
        0xFA => '\u{FE22}', // double tilde, left half
        0xFB => '\u{FE23}', // double tilde, right half
        0xFE => '\u{0313}', // comma above
        _ => return None,
    })
}

/// One GEDCOM line's head: its level, cross-reference, tag and value. The
/// value is the line's own text after the tag, spaces kept.
struct Parts<'a> {
    level: u32,
    tag: &'a str,
    value: &'a str,
}

fn parts(line: &str) -> Option<Parts<'_>> {
    let s = line.trim_start_matches([' ', '\t']);
    let digits = s.bytes().take_while(u8::is_ascii_digit).count();
    if digits == 0 {
        return None;
    }
    let level = s[..digits].parse().ok()?;
    let mut rest = s[digits..].trim_start_matches(' ');
    if rest.starts_with('@') {
        let end = rest[1..].find('@')? + 2;
        rest = rest[end..].trim_start_matches(' ');
    }
    let tag_len = rest.find(' ').unwrap_or(rest.len());
    let tag = &rest[..tag_len];
    if tag.is_empty() {
        return None;
    }
    let value = rest[tag_len..]
        .strip_prefix(' ')
        .unwrap_or(&rest[tag_len..]);
    Some(Parts { level, tag, value })
}

/// Deeper than any GEDCOM structure goes: 5.5.1's deepest is six levels
/// (a citation's text under an event), 7.0's seven, and a program's own
/// extensions rarely add more than two. A line below this is set aside with
/// everything under it. Without a floor, a file of 300 KB whose lines each go
/// one level deeper than the last cost the converter time in the square of
/// its depth and then its stack: the process was aborted, the server with it.
pub const MAX_LEVEL: u32 = 32;

/// The tags of GEDCOM 5.5.1 and 7.0, by which a tag written in lower case is
/// recognised and set upright. A tag the standards do not have is left as it
/// is: an extension (`_UID`) is the writer's to spell.
const TAGS: &[&str] = &[
    "ABBR", "ADDR", "ADOP", "ADR1", "ADR2", "ADR3", "AFN", "AGE", "AGNC", "ALIA", "ANCE", "ANCI",
    "ANUL", "ASSO", "AUTH", "BAPL", "BAPM", "BARM", "BASM", "BIRT", "BLES", "BLOB", "BURI", "CALN",
    "CAST", "CAUS", "CENS", "CHAN", "CHAR", "CHIL", "CHR", "CHRA", "CITY", "CONC", "CONF", "CONL",
    "CONT", "COPR", "CORP", "CREA", "CREM", "CROP", "CTRY", "DATA", "DATE", "DEAT", "DESC", "DESI",
    "DEST", "DIV", "DIVF", "DSCR", "EDUC", "EMAIL", "EMIG", "ENDL", "ENGA", "EVEN", "EXID", "FACT",
    "FAM", "FAMC", "FAMS", "FAX", "FCOM", "FILE", "FONE", "FORM", "GEDC", "GIVN", "GRAD", "HEAD",
    "HEIGHT", "HUSB", "IDNO", "IMMI", "INDI", "INIL", "LANG", "LATI", "LEFT", "LEGA", "LONG",
    "MAP", "MARB", "MARC", "MARL", "MARR", "MARS", "MEDI", "MIME", "NAME", "NATI", "NATU", "NCHI",
    "NICK", "NMR", "NO", "NOTE", "NPFX", "NSFX", "OBJE", "OCCU", "ORDI", "ORDN", "PAGE", "PEDI",
    "PHON", "PHRASE", "PLAC", "POST", "PROB", "PROP", "PUBL", "QUAY", "REFN", "RELA", "RELI",
    "REPO", "RESI", "RESN", "RETI", "RFN", "RIN", "ROLE", "ROMN", "SCHMA", "SDATE", "SEX", "SLGC",
    "SLGS", "SNOTE", "SOUR", "SPFX", "SSN", "STAE", "STAT", "SUBM", "SUBN", "SURN", "TAG", "TEMP",
    "TEXT", "TIME", "TITL", "TOP", "TRAN", "TRLR", "TYPE", "UID", "VERS", "WIDTH", "WIFE", "WILL",
    "WWW",
];

/// One line, taken apart when it is a GEDCOM line.
enum Tidied {
    /// Not a GEDCOM line (blank, prose, no level): as it was.
    Raw(String),
    Line {
        /// The level and, when there is one, the cross-reference.
        head: String,
        tag: String,
        value: Option<String>,
        /// What it was before any mending, when nothing was mended.
        raw: Option<String>,
    },
    /// Nested deeper than [`MAX_LEVEL`].
    TooDeep,
}

impl Tidied {
    fn render(self, out: &mut String) {
        match self {
            Tidied::Raw(r) | Tidied::Line { raw: Some(r), .. } => out.push_str(&r),
            Tidied::Line {
                head, tag, value, ..
            } => {
                out.push_str(&head);
                out.push(' ');
                out.push_str(&tag);
                if let Some(v) = value {
                    out.push(' ');
                    out.push_str(&v);
                }
            }
            Tidied::TooDeep => {}
        }
    }
}

/// The lines laid out as the converter reads them (see the module's
/// comment). Returns the text, how many lines were mended and how many were
/// set aside for their depth.
fn tidy(text: &str, seven: bool) -> (String, usize, usize) {
    let (mut mended, mut too_deep) = (0, 0);
    let mut lines: Vec<Tidied> = Vec::new();
    for raw in text.split('\n') {
        let line = tidy_line(raw, seven);
        match &line {
            Tidied::TooDeep => {
                too_deep += 1;
                continue;
            }
            Tidied::Line { raw: None, .. } => mended += 1,
            _ => {}
        }
        lines.push(line);
    }
    // A space at the end of a line that a CONC continues belongs to the
    // text; the converter trims every line's end, which glued "coupée" and
    // "au milieu" into one word. Moved to the head of the CONC's value, where
    // it is kept.
    for i in 1..lines.len() {
        let (before, after) = lines.split_at_mut(i);
        let (
            Tidied::Line {
                value: Some(v),
                raw: raw_a,
                ..
            },
            Tidied::Line {
                tag,
                value: next,
                raw: raw_b,
                ..
            },
        ) = (&mut before[i - 1], &mut after[0])
        else {
            continue;
        };
        if tag.as_str() != "CONC" {
            continue;
        }
        let kept = v.trim_end_matches([' ', '\t']).len();
        if kept == v.len() || kept == 0 {
            continue;
        }
        let space: String = v[kept..].chars().map(|_| ' ').collect();
        v.truncate(kept);
        *next = Some(space + next.as_deref().unwrap_or(""));
        if raw_a.take().is_some() {
            mended += 1;
        }
        if raw_b.take().is_some() {
            mended += 1;
        }
    }
    let mut out = String::with_capacity(text.len());
    let n = lines.len();
    for (i, line) in lines.into_iter().enumerate() {
        line.render(&mut out);
        if i + 1 < n {
            out.push('\n');
        }
    }
    (out, mended, too_deep)
}

/// One line, mended: no indentation; one space between the level, the
/// cross-reference, the tag and the value; a standard tag in upper case; no
/// control character in the value; `@@` read as the `@` it stands for; a sex
/// written in full or in lower case set as the letter; a Gregorian date's
/// calendar mark, which says nothing the date does not, dropped.
fn tidy_line(raw: &str, seven: bool) -> Tidied {
    let s = raw.trim_start_matches([' ', '\t']);
    let digits = s.bytes().take_while(u8::is_ascii_digit).count();
    if digits == 0 {
        return Tidied::Raw(raw.to_string());
    }
    let Ok(level) = s[..digits].parse::<u32>() else {
        return Tidied::Raw(raw.to_string());
    };
    if level > MAX_LEVEL {
        return Tidied::TooDeep;
    }
    let delimited = |r: &str| r.len() - r.trim_start_matches([' ', '\t']).len();
    let mut rest = &s[digits..];
    let d = delimited(rest);
    if d == 0 {
        return Tidied::Raw(raw.to_string());
    }
    rest = &rest[d..];
    let mut head = level.to_string();
    if rest.starts_with('@') {
        if let Some(end) = rest[1..].find('@') {
            head.push(' ');
            head.push_str(&rest[..end + 2]);
            rest = &rest[end + 2..];
            rest = &rest[delimited(rest)..];
        }
    }
    let tag_len = rest.find([' ', '\t']).unwrap_or(rest.len());
    let written = &rest[..tag_len];
    if written.is_empty() {
        return Tidied::Raw(raw.to_string());
    }
    let upper = written.to_ascii_uppercase();
    let tag = if TAGS.contains(&upper.as_str()) {
        upper
    } else {
        written.to_string()
    };
    // The value: everything after the one delimiter that follows the tag.
    let mut value = rest[tag_len..]
        .strip_prefix([' ', '\t'])
        .map(str::to_string);
    if let Some(v) = value.as_mut() {
        if v.chars().any(|c| c.is_control() && c != '\t') {
            v.retain(|c| !c.is_control() || c == '\t');
        }
        // `@@` is an `@` in a value; GEDCOM 7 escapes only a leading one.
        if v.contains("@@") {
            let read = if seven {
                v.strip_prefix('@')
                    .filter(|r| r.starts_with('@'))
                    .map(str::to_string)
            } else {
                Some(v.replace("@@", "@"))
            };
            // Read back, it must not become a pointer and lose the text.
            if let Some(r) =
                read.filter(|r| !(r.starts_with('@') && r.ends_with('@') && r.len() > 1))
            {
                *v = r;
            }
        }
        if tag == "SEX" {
            let up = v.trim().to_ascii_uppercase();
            let letter = match up.as_str() {
                "MALE" | "M" => Some("M"),
                "FEMALE" | "F" => Some("F"),
                "U" | "UNKNOWN" => Some("U"),
                "X" => Some("X"),
                _ => None,
            };
            // Already the letter, give or take a space: left as written.
            if let Some(l) = letter.filter(|l| v.trim() != *l) {
                *v = l.to_string();
            }
        }
        if tag == "DATE" {
            if let Some(rest) = v.trim_start().strip_prefix("@#DGREGORIAN@") {
                *v = rest.trim_start().to_string();
            }
        }
    }
    let rebuilt = {
        let mut r = head.clone();
        r.push(' ');
        r.push_str(&tag);
        if let Some(v) = &value {
            r.push(' ');
            r.push_str(v);
        }
        r
    };
    let raw_kept = (rebuilt == raw).then(|| raw.to_string());
    Tidied::Line {
        head,
        tag,
        value,
        raw: raw_kept,
    }
}

/// GEDCOM 7, in 5.5.1's words: `@VOID@` pointers dropped with everything
/// under them, `SNOTE` read as `NOTE`. Returns the text and the two counts.
fn from_seven(text: &str) -> (String, usize, usize) {
    let mut out = String::with_capacity(text.len());
    let (mut voids, mut shared) = (0, 0);
    let mut below: Option<u32> = None;
    for line in text.split('\n') {
        let p = parts(line);
        if let (Some(l), Some(p)) = (below, p.as_ref()) {
            if p.level > l {
                continue;
            }
        }
        below = None;
        match p {
            Some(p) if p.value.trim() == "@VOID@" => {
                voids += 1;
                below = Some(p.level);
                continue;
            }
            Some(p) if p.tag == "SNOTE" => {
                shared += 1;
                // The tag replaced where it stands, the rest of the line as is.
                let at = p.tag.as_ptr() as usize - line.as_ptr() as usize;
                out.push_str(&line[..at]);
                out.push_str("NOTE");
                out.push_str(&line[at + "SNOTE".len()..]);
            }
            _ => out.push_str(line),
        }
        out.push('\n');
    }
    // `split` yields an empty piece after a final newline; do not add one.
    if !text.ends_with('\n') {
        out.pop();
    }
    (out, voids, shared)
}

/// `HEAD.GEDC.VERS`: the `VERS` under `GEDC`, not the one under `SOUR`, which
/// is the version of the software that wrote the file.
fn gedcom_version(text: &str) -> Option<String> {
    let mut in_gedc = false;
    for line in text.lines() {
        let Some(p) = parts(line) else { continue };
        if p.level == 0 && !p.tag.eq_ignore_ascii_case("HEAD") {
            return None;
        }
        if p.level == 1 {
            in_gedc = p.tag.eq_ignore_ascii_case("GEDC");
        } else if p.level == 2 && in_gedc && p.tag.eq_ignore_ascii_case("VERS") {
            return Some(p.value.trim().to_string()).filter(|v| !v.is_empty());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIMIT: usize = 1 << 20;

    fn names(text: &str) -> Vec<String> {
        text.lines()
            .filter_map(parts)
            .filter(|p| p.level == 1 && p.tag == "NAME")
            .map(|p| p.value.replace('/', "").trim().to_string())
            .collect()
    }

    #[test]
    fn utf8_with_and_without_a_mark_is_read_as_is() {
        let body = "0 HEAD\n1 GEDC\n2 VERS 5.5.1\n1 CHAR UTF-8\n0 @I1@ INDI\n1 NAME Józef /Gęsiorek/\n0 TRLR\n";
        for bytes in [
            body.as_bytes().to_vec(),
            [b"\xEF\xBB\xBF".as_slice(), body.as_bytes()].concat(),
        ] {
            let r = read(&bytes, LIMIT).unwrap();
            assert_eq!(names(&r.text), ["Józef Gęsiorek"]);
            assert_eq!(r.reading.encoding, Some(Encoding::Utf8));
            assert_eq!(r.reading.version.as_deref(), Some("5.5.1"));
        }
    }

    #[test]
    fn ansel_accents_come_before_their_letter_and_are_composed_onto_it() {
        // "Józef Gęsiorek", "Łódź", "Éléonore" as GEDCOM 5.5 wrote them.
        let mut bytes = b"0 HEAD\r\n1 GEDC\r\n2 VERS 5.5\r\n1 CHAR ANSEL\r\n0 @I1@ INDI\r\n1 NAME J\xE2ozef /G\xF1esiorek/\r\n".to_vec();
        bytes.extend_from_slice(
            b"1 NAME \xA1\xE2od\xE2z /x/\r\n1 NAME \xE2El\xE2eonore /x/\r\n0 TRLR\r\n",
        );
        let r = read(&bytes, LIMIT).unwrap();
        assert_eq!(names(&r.text), ["Józef Gęsiorek", "Łódź x", "Éléonore x"]);
        assert_eq!(r.reading.encoding, Some(Encoding::Ansel));
        // Composed, not a letter followed by a loose accent.
        assert!(r.text.contains('\u{00f3}') && !r.text.contains('\u{0301}'));
    }

    #[test]
    fn an_accent_at_the_end_of_a_line_does_not_open_the_next_one() {
        let r = read(b"0 HEAD\n1 CHAR ANSEL\n1 NOTE x\xE2\n0 TRLR\n", LIMIT).unwrap();
        assert!(r.text.lines().any(|l| l == "0 TRLR"));
    }

    #[test]
    fn utf16_is_read_with_or_without_a_byte_order_mark() {
        let body = "0 HEAD\r\n1 GEDC\r\n2 VERS 5.5.5\r\n1 CHAR UNICODE\r\n0 @I1@ INDI\r\n1 NAME Łucja /Żak/\r\n0 TRLR\r\n";
        let le: Vec<u8> = body.encode_utf16().flat_map(u16::to_le_bytes).collect();
        let be: Vec<u8> = body.encode_utf16().flat_map(u16::to_be_bytes).collect();
        for (bytes, enc) in [
            ([b"\xFF\xFE".as_slice(), &le].concat(), Encoding::Utf16Le),
            ([b"\xFE\xFF".as_slice(), &be].concat(), Encoding::Utf16Be),
            (le.clone(), Encoding::Utf16Le),
            (be.clone(), Encoding::Utf16Be),
        ] {
            let r = read(&bytes, LIMIT).unwrap();
            assert_eq!(names(&r.text), ["Łucja Żak"], "{enc:?}");
            assert_eq!(r.reading.encoding, Some(enc));
            assert_eq!(r.reading.version.as_deref(), Some("5.5.5"));
        }
    }

    #[test]
    fn windows_1252_keeps_its_quotation_marks_and_mac_roman_its_letters() {
        let r = read(
            b"0 HEAD\r\n1 CHAR ANSI\r\n0 @I1@ INDI\r\n1 NAME Ren\xE9 /d\x92Arc/\r\n0 TRLR\r\n",
            LIMIT,
        )
        .unwrap();
        assert_eq!(names(&r.text), ["René d’Arc"]);
        assert_eq!(r.reading.encoding, Some(Encoding::Windows1252));
        let r = read(
            b"0 HEAD\r1 CHAR MACINTOSH\r0 @I1@ INDI\r1 NAME Ren\x8E /x/\r0 TRLR\r",
            LIMIT,
        )
        .unwrap();
        assert_eq!(names(&r.text), ["René x"]);
        assert_eq!(r.reading.encoding, Some(Encoding::MacRoman));
    }

    #[test]
    fn old_macintosh_line_endings_are_mended() {
        let r = read(
            b"0 HEAD\r1 CHAR UTF-8\r0 @I1@ INDI\r1 NAME A /B/\r0 TRLR\r",
            LIMIT,
        )
        .unwrap();
        assert_eq!(names(&r.text), ["A B"]);
        assert_eq!(r.reading.lone_crs, 5);
    }

    #[test]
    fn gedcom_7_voids_are_dropped_and_shared_notes_read_as_notes() {
        let src = "0 HEAD\n1 GEDC\n2 VERS 7.0\n0 @I1@ INDI\n1 NAME A /B/\n1 SNOTE @N1@\n0 @F1@ FAM\n1 CHIL @I1@\n1 CHIL @VOID@\n2 PHRASE an unknown child\n1 HUSB @VOID@\n0 @N1@ SNOTE First line\n1 CONT second\n0 TRLR\n";
        let r = read(src.as_bytes(), LIMIT).unwrap();
        assert_eq!(r.reading.voids, 2);
        assert_eq!(r.reading.shared_notes, 2);
        assert!(!r.text.contains("@VOID@") && !r.text.contains("PHRASE"));
        assert!(
            r.text.contains("1 NOTE @N1@")
                && r.text.contains("0 @N1@ NOTE First line\n1 CONT second")
        );
        assert!(
            r.text.contains("1 CHIL @I1@\n0 @N1@"),
            "the real child stays"
        );
        assert_eq!(r.reading.version.as_deref(), Some("7.0"));
    }

    #[test]
    fn the_software_version_is_not_taken_for_the_gedcom_version() {
        let r = read(
            b"0 HEAD\n1 SOUR PAF\n2 VERS 5.2\n1 GEDC\n2 VERS 5.5\n0 TRLR\n",
            LIMIT,
        )
        .unwrap();
        assert_eq!(r.reading.version.as_deref(), Some("5.5"));
    }

    fn zip_of(files: &[(&str, &[u8])]) -> Vec<u8> {
        use std::io::Write;
        let mut buf = std::io::Cursor::new(Vec::new());
        {
            let mut z = zip::ZipWriter::new(&mut buf);
            for (name, bytes) in files {
                z.start_file(*name, zip::write::SimpleFileOptions::default())
                    .unwrap();
                z.write_all(bytes).unwrap();
            }
            z.finish().unwrap();
        }
        buf.into_inner()
    }

    #[test]
    fn a_gedzip_is_read_from_its_gedcom_ged_and_its_media_counted() {
        let gdz = zip_of(&[
            (
                "gedcom.ged",
                b"0 HEAD\n1 GEDC\n2 VERS 7.0\n0 @I1@ INDI\n1 NAME A /B/\n0 TRLR\n",
            ),
            ("photos/a.jpg", b"\xFF\xD8"),
        ]);
        let r = read(&gdz, LIMIT).unwrap();
        assert!(r.reading.gedzip);
        assert_eq!(r.reading.media, 1);
        assert_eq!(names(&r.text), ["A B"]);
        assert_eq!(
            read(&zip_of(&[("a.jpg", b"x")]), LIMIT).unwrap_err(),
            Refusal::NoDataset
        );
        assert_eq!(read(&gdz, 10).unwrap_err(), Refusal::TooLarge);
    }

    #[test]
    fn what_is_not_gedcom_is_refused() {
        assert_eq!(read(b"hello", LIMIT).unwrap_err(), Refusal::NotGedcom);
        assert_eq!(read(b"", LIMIT).unwrap_err(), Refusal::NotGedcom);
    }
}
