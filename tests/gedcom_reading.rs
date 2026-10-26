//! A family's GEDCOM, however it was written, read into the same family
//! (v0.5.0, the stress campaign of 22 October 2026).
//!
//! The same four people — two parents, two children, a marriage, a source
//! and a shared note — written every way a family's software writes them:
//! GEDCOM 4.0 to 7.0, bare and zipped; ASCII, ANSEL, Windows-1252, Mac Roman,
//! IBM PC, UTF-8 and UTF-16 with and without a mark; every line ending;
//! indented, tab-separated, lower-case. Each must come out as the same family
//! as the reference, and say how it was read. Then what is not a family's
//! file: refused clearly, quickly, and without taking the server with it.
//!
//! Every name, place and date here is invented.

mod common;

use std::time::{Duration, Instant};

use axgf_cms::gedcom_in::{self, Encoding, Refusal};
use serde_json::Value;

const LIMIT: usize = axgf_cms::convert::MAX_UPLOAD;

/// What a file became: how it was read, and the family that came out of it.
struct Converted {
    reading: gedcom_in::Reading,
    bundle: Value,
}

fn convert(bytes: &[u8]) -> Converted {
    let read = gedcom_in::read(bytes, LIMIT).unwrap_or_else(|e| panic!("refused: {e:?}"));
    let env = axgf_rs::convert_gedcom(read.text.as_bytes(), 0.8, "fr");
    let data = axgf_cms::state::envelope_into_data(env).expect("converted");
    Converted {
        reading: read.reading,
        bundle: data["bundle"].clone(),
    }
}

impl Converted {
    fn names(&self) -> Vec<String> {
        let mut v: Vec<String> = self.bundle["persons"]
            .as_object()
            .map(|m| {
                m.values()
                    .map(|p| {
                        p["identity"]["name"]["display"]
                            .as_str()
                            .unwrap_or("")
                            .to_string()
                    })
                    .collect()
            })
            .unwrap_or_default();
        v.sort();
        v
    }

    fn name_of(&self, id: &str) -> String {
        self.bundle["persons"][id]["identity"]["name"]["display"]
            .as_str()
            .unwrap_or("?")
            .to_string()
    }

    /// The family without its identifiers, which are fresh on every run:
    /// who is in it, who married whom and when, whose children are whose,
    /// where they were born, and the notes.
    fn shape(&self) -> String {
        let mut out = self.names();
        let mut fams: Vec<String> = self.bundle["families"]
            .as_object()
            .map(|m| m.values().cloned().collect::<Vec<_>>())
            .unwrap_or_default()
            .iter()
            .map(|f| {
                let mut spouses: Vec<String> = f["union"]["persons"]
                    .as_array()
                    .map(|a| {
                        a.iter()
                            .map(|p| self.name_of(p["person_id"].as_str().unwrap_or("")))
                            .collect()
                    })
                    .unwrap_or_default();
                spouses.sort();
                let mut kids: Vec<String> = f["children"]
                    .as_array()
                    .map(|a| {
                        a.iter()
                            .map(|c| self.name_of(c["person_id"].as_str().unwrap_or("")))
                            .collect()
                    })
                    .unwrap_or_default();
                kids.sort();
                format!(
                    "{} married {} have {}",
                    spouses.join(" & "),
                    f["union"]["start"]["date"]["value"],
                    kids.join(", ")
                )
            })
            .collect();
        fams.sort();
        out.extend(fams);
        let mut births: Vec<String> = self.bundle["persons"]
            .as_object()
            .map(|m| {
                m.values()
                    .map(|p| {
                        let place = p["birth"]["place_id"]
                            .as_str()
                            .map(|id| self.bundle["places"][id]["names"][0]["value"].to_string())
                            .unwrap_or_default();
                        format!(
                            "{} born {} {} {}",
                            p["identity"]["name"]["display"],
                            p["birth"]["date"]["value"],
                            p["birth"]["date"]["circa"],
                            place
                        )
                    })
                    .collect()
            })
            .unwrap_or_default();
        births.sort();
        out.extend(births);
        let mut notes: Vec<String> = self.bundle["persons"]
            .as_object()
            .map(|m| {
                m.values()
                    .filter_map(|p| p["notes"].as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default();
        notes.sort();
        out.extend(notes);
        out.extend(["sources", "places", "events"].iter().map(|c| {
            format!(
                "{c}: {}",
                self.bundle[*c].as_object().map_or(0, |m| m.len())
            )
        }));
        out.join("\n")
    }
}

/// The reference family, its four names given.
fn family(names: [&str; 4]) -> Vec<String> {
    let [a, b, c, d] = names;
    [
        "0 @I1@ INDI".to_string(),
        format!("1 NAME {a}"),
        "1 SEX M".into(),
        "1 BIRT".into(),
        "2 DATE 12 MAR 1850".into(),
        "2 PLAC Lyon, Rhône, France".into(),
        "1 FAMS @F1@".into(),
        "0 @I2@ INDI".into(),
        format!("1 NAME {b}"),
        "1 SEX F".into(),
        "1 BIRT".into(),
        "2 DATE ABT 1855".into(),
        "1 FAMS @F1@".into(),
        "1 NOTE @N1@".into(),
        "0 @I3@ INDI".into(),
        format!("1 NAME {c}"),
        "1 SEX F".into(),
        "1 BIRT".into(),
        "2 DATE 3 JUL 1880".into(),
        "2 PLAC Lyon, Rhône, France".into(),
        "1 FAMC @F1@".into(),
        "0 @I4@ INDI".into(),
        format!("1 NAME {d}"),
        "1 SEX M".into(),
        "1 FAMC @F1@".into(),
        "0 @F1@ FAM".into(),
        "1 HUSB @I1@".into(),
        "1 WIFE @I2@".into(),
        "1 CHIL @I3@".into(),
        "1 CHIL @I4@".into(),
        "1 MARR".into(),
        "2 DATE 5 MAY 1878".into(),
        "0 @S1@ SOUR".into(),
        "1 TITL Registre fictif".into(),
        "0 @N1@ NOTE Une note partagée,".into(),
        "1 CONT sur deux lignes.".into(),
    ]
    .to_vec()
}

const FONTAINE: [&str; 4] = [
    "Basile /Fontaine/",
    "Adèle /Marchand/",
    "Céleste /Fontaine/",
    "Gaspard /Fontaine/",
];
const POLISH: [&str; 4] = [
    "Józef /Gęsiorek/",
    "Łucja /Żak/",
    "Zofia /Gęsiorek/",
    "Kazimierz /Gęsiorek/",
];
const LATIN1: [&str; 4] = [
    "Hélène /Müller/",
    "François /Leçon/",
    "Zoë /Müller/",
    "Noël /Müller/",
];

fn head(version: &str, charset: Option<&str>) -> Vec<String> {
    let mut h = vec![
        "0 HEAD".to_string(),
        "1 SOUR FICTIVE".into(),
        "2 VERS 1.0".into(),
        "1 GEDC".into(),
        format!("2 VERS {version}"),
    ];
    if !version.starts_with('7') {
        h.push("2 FORM LINEAGE-LINKED".into());
    }
    if let Some(c) = charset {
        h.push(format!("1 CHAR {c}"));
    }
    h
}

fn text(lines: &[String], eol: &str) -> String {
    let mut s = lines.join(eol);
    s.push_str(eol);
    s.push_str("0 TRLR");
    s.push_str(eol);
    s
}

fn reference(names: [&str; 4]) -> String {
    text(
        &[head("5.5.1", Some("UTF-8")), family(names)].concat(),
        "\n",
    )
}

fn utf16(s: &str, little: bool, mark: bool) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    if mark {
        out.extend_from_slice(if little { b"\xFF\xFE" } else { b"\xFE\xFF" });
    }
    for u in s.encode_utf16() {
        out.extend_from_slice(&if little {
            u.to_le_bytes()
        } else {
            u.to_be_bytes()
        });
    }
    out
}

/// ANSEL as GEDCOM 5.5 wrote it: each accent before its letter.
fn ansel(s: &str) -> Vec<u8> {
    let mut out = Vec::new();
    for c in s.chars() {
        let bytes: &[u8] = match c {
            'é' => b"\xE2e",
            'è' => b"\xE1e",
            'ó' => b"\xE2o",
            'ę' => b"\xF1e",
            'Ł' => b"\xA1",
            'Ż' => b"\xE7Z",
            'ô' => b"\xE3o",
            'ễ' => b"\xE3\xE4e",
            'ă' => b"\xE6a",
            'ả' => b"\xE0a",
            'ầ' => b"\xE3\xE1a",
            'Ø' => b"\xA2",
            'Æ' => b"\xA5",
            'ñ' => b"\xE4n",
            'Ñ' => b"\xE4N",
            c if c.is_ascii() => {
                out.push(c as u8);
                continue;
            }
            c => panic!("no ANSEL for {c}"),
        };
        out.extend_from_slice(bytes);
    }
    out
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

// ------------------------------------------------------------------ versions

/// GEDCOM 4.0 in ASCII, 5.5 in ANSEL, 5.5.1 in UTF-8 with and without a
/// mark, 5.5.5 in UTF-16 both ways round with and without a mark: one family.
#[test]
fn every_version_from_4_0_to_5_5_5_reads_as_the_same_family() {
    let want = convert(reference(POLISH).as_bytes()).shape();
    let unicode = text(
        &[head("5.5.5", Some("UNICODE")), family(POLISH)].concat(),
        "\r\n",
    );
    let cases: Vec<(&str, Vec<u8>, Encoding, &str)> = vec![
        (
            "5.5 ANSEL",
            ansel(&text(
                &[head("5.5", Some("ANSEL")), family(POLISH)].concat(),
                "\r\n",
            )),
            Encoding::Ansel,
            "5.5",
        ),
        (
            "5.5.1 UTF-8 with a mark",
            [b"\xEF\xBB\xBF".as_slice(), reference(POLISH).as_bytes()].concat(),
            Encoding::Utf8,
            "5.5.1",
        ),
        (
            "5.5.5 UTF-16 LE",
            utf16(&unicode, true, true),
            Encoding::Utf16Le,
            "5.5.5",
        ),
        (
            "5.5.5 UTF-16 BE",
            utf16(&unicode, false, true),
            Encoding::Utf16Be,
            "5.5.5",
        ),
        (
            "5.5.5 UTF-16 LE, no mark",
            utf16(&unicode, true, false),
            Encoding::Utf16Le,
            "5.5.5",
        ),
        (
            "5.5.5 UTF-16 BE, no mark",
            utf16(&unicode, false, false),
            Encoding::Utf16Be,
            "5.5.5",
        ),
        (
            "UTF-16 LE, no mark, a blank line first",
            utf16(&format!("\r\n{unicode}"), true, false),
            Encoding::Utf16Le,
            "5.5.5",
        ),
    ];
    for (what, bytes, enc, version) in cases {
        let c = convert(&bytes);
        assert_eq!(c.shape(), want, "{what}");
        assert_eq!(c.reading.encoding, Some(enc), "{what}");
        assert_eq!(c.reading.version.as_deref(), Some(version), "{what}");
    }
    let ascii = [
        "Paul /Martin/",
        "Anne /Roux/",
        "Luc /Martin/",
        "Marc /Martin/",
    ];
    let old = convert(
        text(
            &[head("4.0", Some("ASCII")), family(ascii)].concat(),
            "\r\n",
        )
        .as_bytes(),
    );
    assert_eq!(
        old.shape(),
        convert(reference(ascii).as_bytes()).shape(),
        "GEDCOM 4.0"
    );
}

/// GEDCOM 7.0, bare and as a `.gdz`: `@VOID@` is no one, a shared note is a
/// note, and the photograph beside the dataset is counted.
#[test]
fn gedcom_7_bare_and_zipped_reads_as_the_same_family() {
    let mut lines = head("7.0", None);
    lines.extend(
        [
            "0 @I1@ INDI",
            "1 NAME Józef /Gęsiorek/",
            "1 SEX M",
            "1 FAMS @F1@",
            "1 SNOTE @N1@",
            "0 @I2@ INDI",
            "1 NAME Éléonore /Gęsiorek/",
            "1 SEX F",
            "1 FAMC @F1@",
            "0 @F1@ FAM",
            "1 HUSB @I1@",
            "1 CHIL @I2@",
            "1 CHIL @VOID@",
            "2 PHRASE an unnamed child",
            "1 WIFE @VOID@",
            "0 @N1@ SNOTE Une note longue,",
            "1 CONT sur deux lignes.",
        ]
        .map(String::from),
    );
    let ged = text(&lines, "\n");
    let gdz = zip_of(&[
        ("gedcom.ged", ged.as_bytes()),
        ("photos/portrait.jpg", b"\xFF\xD8"),
    ]);
    for (what, bytes) in [("bare", ged.as_bytes().to_vec()), ("gdz", gdz)] {
        let c = convert(&bytes);
        assert_eq!(c.names(), ["Józef Gęsiorek", "Éléonore Gęsiorek"], "{what}");
        assert_eq!(c.reading.version.as_deref(), Some("7.0"), "{what}");
        assert_eq!(c.reading.voids, 2, "{what}");
        assert_eq!(c.reading.shared_notes, 2, "{what}");
        let fam = c.bundle["families"]
            .as_object()
            .unwrap()
            .values()
            .next()
            .unwrap()
            .clone();
        assert_eq!(
            fam["children"].as_array().map(Vec::len),
            Some(1),
            "{what}: @VOID@ is no child"
        );
        assert_eq!(
            fam["union"]["persons"].as_array().map(Vec::len),
            Some(1),
            "{what}: nor a wife"
        );
        let jozef = c.bundle["persons"]
            .as_object()
            .unwrap()
            .values()
            .find(|p| p["identity"]["name"]["display"] == "Józef Gęsiorek")
            .unwrap();
        assert_eq!(
            jozef["notes"], "Une note longue,\nsur deux lignes.",
            "{what}"
        );
    }
    let c = convert(&zip_of(&[
        ("gedcom.ged", ged.as_bytes()),
        ("photos/portrait.jpg", b"\xFF\xD8"),
    ]));
    assert!(c.reading.gedzip);
    assert_eq!(c.reading.media, 1);
}

/// A zip that is not quite a `.gdz`: its only `.ged` in a folder is read.
#[test]
fn a_zip_whose_only_gedcom_sits_in_a_folder_is_read() {
    let c = convert(&zip_of(&[(
        "export/arbre.ged",
        reference(FONTAINE).as_bytes(),
    )]));
    assert_eq!(c.shape(), convert(reference(FONTAINE).as_bytes()).shape());
}

// ----------------------------------------------------------------- encodings

/// The single-byte encodings of the 1990s, each declared as its software
/// declared it: the same accented names come out of all of them.
#[test]
fn every_single_byte_encoding_reads_the_same_names() {
    let want = [
        "François Leçon",
        "Hélène Müller",
        "Noël Müller",
        "Zoë Müller",
    ];
    let utf8 = text(
        &[head("5.5.1", Some("ANSI")), family(LATIN1)].concat(),
        "\r\n",
    );
    let cp1252 = encoding_rs::WINDOWS_1252.encode(&utf8).0.into_owned();
    let mac = encoding_rs::MACINTOSH
        .encode(&text(
            &[head("5.5.1", Some("MACINTOSH")), family(LATIN1)].concat(),
            "\r",
        ))
        .0
        .into_owned();
    // Code page 850, as DOS software wrote it: é 82, è 8A, ü 81, ç 87, ë 89.
    let dos = text(
        &[
            head("5.5", Some("IBMPC")),
            family([
                "H\u{1}l\u{2}ne /M\u{3}ller/",
                "Fran\u{4}ois /Le\u{4}on/",
                "Zo\u{5} /M\u{3}ller/",
                "No\u{5}l /M\u{3}ller/",
            ]),
        ]
        .concat(),
        "\r\n",
    )
    .replace("Rhône", "Rh\u{6}ne")
    .replace("partagée", "partag\u{1}e");
    let dos: Vec<u8> = dos
        .bytes()
        .map(|b| match b {
            1 => 0x82,
            2 => 0x8A,
            3 => 0x81,
            4 => 0x87,
            5 => 0x89,
            6 => 0x93,
            b => b,
        })
        .collect();
    let latin1_as_ascii = encoding_rs::WINDOWS_1252
        .encode(&text(
            &[head("5.5.1", Some("ASCII")), family(LATIN1)].concat(),
            "\n",
        ))
        .0
        .into_owned();
    for (what, bytes, enc) in [
        ("Windows-1252 declared ANSI", cp1252, Encoding::Windows1252),
        ("Mac Roman", mac, Encoding::MacRoman),
        ("IBM PC, code page 850", dos, Encoding::Ibm850),
        (
            "Latin-1 declared ASCII",
            latin1_as_ascii,
            Encoding::Windows1252,
        ),
    ] {
        let c = convert(&bytes);
        assert_eq!(c.names(), want, "{what}");
        assert_eq!(c.reading.encoding, Some(enc), "{what}");
        assert!(
            c.shape().contains("Lyon, Rhône, France"),
            "{what}: the place too"
        );
        assert!(
            c.shape().contains("Une note partagée,"),
            "{what}: and the note"
        );
    }
}

/// Windows-1252's own letters and signs, the ones Latin-1 does not have.
#[test]
fn windows_1252_keeps_its_euro_its_quotes_and_its_oe() {
    let mut lines = head("5.5.1", Some("ANSI"));
    lines.extend(
        [
            "0 @I1@ INDI",
            "1 NAME Œdipe /D’Arc/",
            "1 NOTE prix : 5 € « guillemets » — fin…",
        ]
        .map(String::from),
    );
    let c = convert(&encoding_rs::WINDOWS_1252.encode(&text(&lines, "\r\n")).0);
    assert_eq!(c.names(), ["Œdipe D’Arc"]);
    let p = c.bundle["persons"]
        .as_object()
        .unwrap()
        .values()
        .next()
        .unwrap();
    assert_eq!(p["notes"], "prix : 5 € « guillemets » — fin…");
}

/// ANSEL's accents stacked two to a letter (Vietnamese) and its letters of
/// their own (Ø, Æ, Ł), composed the way Unicode writes them.
#[test]
fn ansel_stacks_two_accents_on_a_letter_and_keeps_its_own_letters() {
    let names = [
        "Nguyễn /Văn Bảo/",
        "Trầm /Ñuño/",
        "Øster /Æbeltoft/",
        "Łucja /Żak/",
    ];
    let c = convert(&ansel(&text(
        &[head("5.5", Some("ANSEL")), family(names)].concat(),
        "\r\n",
    )));
    let mut want = ["Nguyễn Văn Bảo", "Trầm Ñuño", "Øster Æbeltoft", "Łucja Żak"]
        .map(String::from)
        .to_vec();
    want.sort();
    assert_eq!(c.names(), want);
    assert!(
        c.names()
            .iter()
            .all(|n| n.chars().all(|ch| !('\u{300}'..='\u{36f}').contains(&ch))),
        "composed: {:?}",
        c.names()
    );
}

/// A file whose header claims one thing and whose bytes are another is read
/// as its bytes are, and the page says so.
#[test]
fn a_header_that_misstates_the_encoding_is_read_by_the_bytes() {
    let c = convert(text(&[head("5.5", Some("ANSEL")), family(LATIN1)].concat(), "\n").as_bytes());
    assert_eq!(c.reading.encoding, Some(Encoding::Utf8));
    assert!(c.names().contains(&"Hélène Müller".to_string()));
}

/// A UTF-8 file with one byte from elsewhere in it (a note pasted from a
/// Windows program): the file stays UTF-8, that byte alone is Windows-1252.
/// It used to turn every accent in the file into "Ã©".
#[test]
fn a_utf8_file_with_one_stray_byte_keeps_every_other_accent() {
    let s = reference(LATIN1).replace("sur deux lignes.", "sur deux lignes, prix 5 EURO.");
    let bytes = s
        .replace("EURO", "\u{0}")
        .into_bytes()
        .iter()
        .map(|&b| if b == 0 { 0x80 } else { b })
        .collect::<Vec<u8>>();
    let c = convert(&bytes);
    assert_eq!(c.reading.encoding, Some(Encoding::Utf8));
    assert_eq!(c.reading.stray_bytes, 1);
    assert_eq!(
        c.names(),
        [
            "François Leçon",
            "Hélène Müller",
            "Noël Müller",
            "Zoë Müller"
        ]
    );
    assert!(
        c.shape().contains("sur deux lignes, prix 5 €."),
        "{}",
        c.shape()
    );
}

/// Bytes that ANSEL has no letter for become the replacement character,
/// visibly, rather than vanishing or taking their neighbours with them.
#[test]
fn ansel_bytes_outside_its_table_are_marked_not_lost() {
    let mut s = ansel(&reference(FONTAINE).replace("1 CHAR UTF-8", "1 CHAR ANSEL"));
    let at = s.windows(6).position(|w| w == b"Basile").unwrap();
    s.splice(at + 3..at + 3, [0xAF, 0xBB]);
    let c = convert(&s);
    assert!(
        c.names()
            .contains(&"Bas\u{fffd}\u{fffd}ile Fontaine".to_string()),
        "{:?}",
        c.names()
    );
    assert_eq!(c.names().len(), 4);
}

// -------------------------------------------------------------- line endings

#[test]
fn every_line_ending_reads_the_same() {
    let lines = [head("5.5.1", Some("UTF-8")), family(FONTAINE)].concat();
    let want = convert(text(&lines, "\n").as_bytes()).shape();
    for (what, eol) in [("CRLF", "\r\n"), ("CR", "\r"), ("LF CR", "\n\r")] {
        assert_eq!(
            convert(text(&lines, eol).as_bytes()).shape(),
            want,
            "{what}"
        );
    }
    let mixed: String = lines
        .iter()
        .chain(["0 TRLR".to_string()].iter())
        .enumerate()
        .map(|(i, l)| format!("{l}{}", ["\n", "\r\n", "\r"][i % 3]))
        .collect();
    assert_eq!(convert(mixed.as_bytes()).shape(), want, "mixed");
    let no_final = text(&lines, "\n").trim_end().to_string();
    assert_eq!(
        convert(no_final.as_bytes()).shape(),
        want,
        "no line ending after TRLR"
    );
    let no_trlr = lines.join("\n");
    assert_eq!(convert(no_trlr.as_bytes()).shape(), want, "no TRLR at all");
    let blank = format!("\n\n{}", text(&lines, "\n").replace('\n', "\n\n"));
    assert_eq!(
        convert(blank.as_bytes()).shape(),
        want,
        "blank lines everywhere"
    );
}

// -------------------------------------------------------------------- layout

/// Lines laid out in ways the converter does not read: each used to lose
/// people (the double-spaced, tab-separated and lower-case files came out
/// with nobody in them) or their families. Now the same family, and the
/// page says how many lines were mended.
#[test]
fn indented_tabbed_double_spaced_or_lower_case_lines_read_as_the_same_family() {
    let fam = family(FONTAINE);
    let want = convert(reference(FONTAINE).as_bytes()).shape();
    let level = |l: &str| l.split(' ').next().unwrap().parse::<usize>().unwrap();
    let variants: Vec<(&str, Vec<String>)> = vec![
        (
            "indented with spaces",
            fam.iter()
                .map(|l| format!("{}{l}", "  ".repeat(level(l))))
                .collect(),
        ),
        (
            "indented with tabs",
            fam.iter()
                .map(|l| format!("{}{l}", "\t".repeat(level(l))))
                .collect(),
        ),
        (
            "two spaces after the level",
            fam.iter().map(|l| l.replacen(' ', "  ", 1)).collect(),
        ),
        (
            "tabs between the fields",
            fam.iter().map(|l| l.replacen(' ', "\t", 2)).collect(),
        ),
        (
            "trailing spaces",
            fam.iter().map(|l| format!("{l}   ")).collect(),
        ),
        (
            "tags in lower case",
            fam.iter()
                .map(|l| {
                    let mut p: Vec<String> = l.split(' ').map(String::from).collect();
                    let t = if p[1].starts_with('@') { 2 } else { 1 };
                    p[t] = p[t].to_lowercase();
                    p.join(" ")
                })
                .collect(),
        ),
        (
            "a level written 01",
            fam.iter()
                .map(|l| {
                    if l.starts_with('1') {
                        format!("0{l}")
                    } else {
                        l.clone()
                    }
                })
                .collect(),
        ),
    ];
    for (what, lines) in variants {
        let c = convert(text(&[head("5.5.1", Some("UTF-8")), lines].concat(), "\n").as_bytes());
        assert_eq!(c.shape(), want, "{what}");
        if what != "trailing spaces" {
            assert!(c.reading.tidied > 0, "{what}: the mending is said");
        }
    }
}

/// A line of prose before the header (who exported the file) is passed over.
#[test]
fn a_line_of_prose_before_the_header_is_passed_over() {
    let c = convert(format!("Exported by FictiveTree 3.2\n{}", reference(FONTAINE)).as_bytes());
    assert_eq!(c.shape(), convert(reference(FONTAINE).as_bytes()).shape());
}

/// Software splits a long note either after a space or before one; the
/// converter trimmed each line's end and glued words together after the
/// first kind.
#[test]
fn a_note_split_around_its_spaces_keeps_them_both_ways() {
    let want = "Une phrase coupée au milieu des mots.";
    for (what, lines) in [
        (
            "space ends the line",
            [
                "1 NOTE Une phrase coupée ",
                "2 CONC au milieu ",
                "2 CONC des mots.",
            ],
        ),
        (
            "space starts the next",
            [
                "1 NOTE Une phrase coupée",
                "2 CONC  au milieu",
                "2 CONC  des mots.",
            ],
        ),
    ] {
        let mut l = head("5.5.1", Some("UTF-8"));
        l.extend(["0 @I1@ INDI".to_string(), "1 NAME Note /Coupée/".into()]);
        l.extend(lines.map(String::from));
        let c = convert(text(&l, "\n").as_bytes());
        let p = c.bundle["persons"]
            .as_object()
            .unwrap()
            .values()
            .next()
            .unwrap();
        assert_eq!(p["notes"], want, "{what}");
    }
}

/// A note of twenty thousand pieces and a line of a million characters:
/// whole, and quickly.
#[test]
fn a_very_long_note_and_a_very_long_line_come_through_whole_and_quickly() {
    let mut l = head("5.5.1", Some("UTF-8"));
    l.extend([
        "0 @I1@ INDI".to_string(),
        "1 NAME Note /Longue/".into(),
        "1 NOTE début".into(),
    ]);
    l.extend((0..20_000).map(|i| format!("2 CONC  mot{i}")));
    l.extend([
        "0 @I2@ INDI".to_string(),
        "1 NAME Ligne /Longue/".into(),
        format!("1 NOTE {}", "abcdefghij".repeat(100_000)),
    ]);
    let t = Instant::now();
    let c = convert(text(&l, "\n").as_bytes());
    assert!(t.elapsed() < Duration::from_secs(20), "{:?}", t.elapsed());
    let notes: Vec<usize> = c.bundle["persons"]
        .as_object()
        .unwrap()
        .values()
        .map(|p| p["notes"].as_str().unwrap().len())
        .collect();
    assert!(notes.contains(&1_000_000), "{notes:?}");
    let chain = c.bundle["persons"]
        .as_object()
        .unwrap()
        .values()
        .find(|p| p["identity"]["name"]["display"] == "Note Longue")
        .unwrap();
    assert!(chain["notes"].as_str().unwrap().ends_with(" mot19999"));
    assert_eq!(
        chain["notes"].as_str().unwrap().matches(" mot").count(),
        20_000
    );
}

/// `@@` is how GEDCOM writes an `@` in a value.
#[test]
fn an_escaped_at_sign_reads_as_one() {
    let mut l = head("5.5.1", Some("UTF-8"));
    l.extend(
        [
            "0 @I1@ INDI",
            "1 NAME Arobase /Échappée/",
            "1 NOTE écrire à jean@@exemple.org",
        ]
        .map(String::from),
    );
    let c = convert(text(&l, "\n").as_bytes());
    let p = c.bundle["persons"]
        .as_object()
        .unwrap()
        .values()
        .next()
        .unwrap();
    assert_eq!(p["notes"], "écrire à jean@exemple.org");
}

/// Control characters have no business in a name or a note, and one in a
/// page (a bell, an escape sequence, a NUL) shows as garbage or worse.
#[test]
fn control_characters_are_taken_out_of_values() {
    let mut l = head("5.5.1", Some("UTF-8"));
    l.extend(
        [
            "0 @I1@ INDI",
            "1 NAME Bell\u{7} /Ctrl\u{1b}[31m/",
            "1 NOTE a\u{0}b\u{b}c\tthen a tab",
        ]
        .map(String::from),
    );
    let c = convert(text(&l, "\n").as_bytes());
    assert_eq!(c.names(), ["Bell Ctrl[31m"]);
    let p = c.bundle["persons"]
        .as_object()
        .unwrap()
        .values()
        .next()
        .unwrap();
    assert_eq!(p["notes"], "abc\tthen a tab");
}

/// The sex as other programs write it: in lower case, in full.
#[test]
fn a_sex_in_lower_case_or_in_full_is_read() {
    let mut l = head("5.5.1", Some("UTF-8"));
    for (i, s) in ["m", "f", "Male", "FEMALE", "M", "F", "U"]
        .iter()
        .enumerate()
    {
        l.extend([
            format!("0 @I{i}@ INDI"),
            format!("1 NAME Sexe{i} /Test/"),
            format!("1 SEX {s}"),
        ]);
    }
    let c = convert(text(&l, "\n").as_bytes());
    let mut by: Vec<(String, String)> = c.bundle["persons"]
        .as_object()
        .unwrap()
        .values()
        .map(|p| {
            (
                p["identity"]["name"]["display"]
                    .as_str()
                    .unwrap()
                    .to_string(),
                p["identity"]["gender"]["value"]
                    .as_str()
                    .unwrap()
                    .to_string(),
            )
        })
        .collect();
    by.sort();
    let got: Vec<&str> = by.iter().map(|(_, g)| g.as_str()).collect();
    assert_eq!(got, ["M", "F", "M", "F", "M", "F", "U"]);
}

/// A Gregorian date marked as Gregorian is the same date.
#[test]
fn a_gregorian_calendar_mark_is_the_same_date() {
    let mut l = head("5.5.1", Some("UTF-8"));
    l.extend(
        [
            "0 @I1@ INDI",
            "1 NAME Marque /Grégorienne/",
            "1 BIRT",
            "2 DATE @#DGREGORIAN@ 1 JAN 1900",
        ]
        .map(String::from),
    );
    let c = convert(text(&l, "\n").as_bytes());
    let p = c.bundle["persons"]
        .as_object()
        .unwrap()
        .values()
        .next()
        .unwrap();
    assert_eq!(p["birth"]["date"]["value"], "1900-01-01");
    assert_eq!(p["birth"]["date"]["precision"], "exact");
}

// -------------------------------------------------------------------- depth

/// A file of 300 KB whose lines each go one level deeper than the last.
/// Before the floor, converting it overflowed the stack and aborted the
/// process — through `/convert`, which anyone may use. Now it is read in a
/// moment, the depth set aside and said.
#[test]
fn lines_nested_fifty_thousand_deep_are_set_aside_not_a_crash() {
    let mut l = head("5.5.1", Some("UTF-8"));
    l.extend([
        "0 @I1@ INDI".to_string(),
        "1 NAME Niveaux /Profonds/".into(),
    ]);
    l.extend((1..=50_000).map(|lv| format!("{lv} _DEEP x")));
    l.extend(["0 @I2@ INDI".to_string(), "1 NAME Après /Profonds/".into()]);
    let t = Instant::now();
    let c = convert(text(&l, "\n").as_bytes());
    assert!(t.elapsed() < Duration::from_secs(10), "{:?}", t.elapsed());
    assert_eq!(c.reading.too_deep, 50_000 - gedcom_in::MAX_LEVEL as usize);
    assert_eq!(c.names(), ["Après Profonds", "Niveaux Profonds"]);
}

/// Levels that are not levels — negative, a letter, too large for a number
/// — and levels that jump: no line is taken for another, and nothing fails.
#[test]
fn malformed_levels_are_passed_over() {
    let mut l = head("5.5.1", Some("UTF-8"));
    l.extend(
        [
            "0 @I1@ INDI",
            "1 NAME Bon /Niveau/",
            "-1 NOTE négatif",
            "X NOTE lettre",
            "99999999999 NOTE énorme",
            "3 NOTE orphelin",
            "1 SEX M",
            "0 @I2@ INDI",
            "2 NAME Mal /Placé/",
        ]
        .map(String::from),
    );
    let c = convert(text(&l, "\n").as_bytes());
    let bon = c.bundle["persons"]
        .as_object()
        .unwrap()
        .values()
        .find(|p| p["identity"]["name"]["display"] == "Bon Niveau")
        .unwrap();
    assert_eq!(bon["identity"]["gender"]["value"], "M");
    assert!(bon.get("notes").is_none(), "{bon}");
}

// ------------------------------------------------------------------ refusals

/// What is not a family's file is refused, and said to be.
#[test]
fn what_is_not_gedcom_is_refused() {
    let mut random = vec![0u8; 4096];
    let mut x: u32 = 1;
    for b in random.iter_mut() {
        x = x.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        *b = (x >> 16) as u8;
    }
    for (what, bytes) in [
        ("empty", Vec::new()),
        ("random bytes", random),
        ("no header", family(FONTAINE).join("\n").into_bytes()),
        (
            "HTML",
            b"<!doctype html><html><body>0 HEAD</body></html>".to_vec(),
        ),
        ("JSON", br#"{"persons": {}}"#.to_vec()),
        ("a PDF", b"%PDF-1.7\n%\xE2\xE3\xCF\xD3\n1 0 obj\n".to_vec()),
    ] {
        assert_eq!(
            gedcom_in::read(&bytes, LIMIT).err(),
            Some(Refusal::NotGedcom),
            "{what}"
        );
    }
    assert_eq!(
        gedcom_in::read(
            &zip_of(&[("a.ged", b"0 HEAD\n"), ("b.ged", b"0 HEAD\n")]),
            LIMIT
        )
        .err(),
        Some(Refusal::NoDataset),
        "two datasets and no gedcom.ged"
    );
    assert_eq!(
        gedcom_in::read(&zip_of(&[("photo.jpg", b"\xFF\xD8")]), LIMIT).err(),
        Some(Refusal::NoDataset),
        "no dataset at all"
    );
}

/// A header and nothing else is an empty family, not an error.
#[test]
fn a_header_alone_is_an_empty_family() {
    let c = convert(text(&head("5.5.1", Some("UTF-8")), "\n").as_bytes());
    assert!(c.names().is_empty());
}

/// A `.gdz` whose dataset unpacks past the upload limit from a few dozen
/// kilobytes: refused once past the limit, without unpacking the rest.
#[test]
fn a_zip_bomb_is_refused_without_being_unpacked() {
    let bomb = {
        use std::io::Write;
        let mut buf = std::io::Cursor::new(Vec::new());
        {
            let mut z = zip::ZipWriter::new(&mut buf);
            z.start_file(
                "gedcom.ged",
                zip::write::SimpleFileOptions::default()
                    .compression_method(zip::CompressionMethod::Deflated),
            )
            .unwrap();
            z.write_all(reference(FONTAINE).replace("0 TRLR\n", "").as_bytes())
                .unwrap();
            let filler = format!("1 NOTE {}\n", "x".repeat(200));
            for _ in 0..60_000 {
                z.write_all(filler.as_bytes()).unwrap();
            }
            z.write_all(b"0 TRLR\n").unwrap();
            z.finish().unwrap();
        }
        buf.into_inner()
    };
    assert!(bomb.len() < 1 << 20, "the bomb is small: {}", bomb.len());
    let t = Instant::now();
    assert_eq!(gedcom_in::read(&bomb, LIMIT).err(), Some(Refusal::TooLarge));
    assert!(t.elapsed() < Duration::from_secs(5), "{:?}", t.elapsed());
}
