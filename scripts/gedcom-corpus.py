#!/usr/bin/env python3
"""The GEDCOM import campaign's corpus (docs/GEDCOM-IMPORT.md).

    python3 scripts/gedcom-corpus.py OUT_DIR [--large]

Writes some seventy GEDCOM files into OUT_DIR, each aimed at one thing a
family's file can be — a version, an encoding, a line ending, a layout, a
shape of family, a date, a hostile value, a size — and OUT_DIR/index.json,
which says what each one is. `--large` adds the slow ones: two random
families of 20,000 and 36,000 people (cycles and conflicts included), a line
of 20,000 generations, and two families that read like real ones, of 5,000
and 40,000 people. Feed the directory to tests/gedcom_probe.rs.

Every name, place and note here is invented.
"""
import json
import os
import random
import sys
import unicodedata
import zipfile

OUT = sys.argv[1]
LARGE = "--large" in sys.argv[2:]
os.makedirs(OUT, exist_ok=True)
INDEX = {}


def write(name, data, what):
    path = os.path.join(OUT, name)
    with open(path, "wb") as f:
        f.write(data)
    INDEX[name] = {"what": what, "bytes": len(data)}


def head(vers="5.5.1", char="UTF-8", extra=()):
    out = ["0 HEAD", "1 SOUR AXGF-STRESS", "2 VERS 1.0", "1 GEDC", f"2 VERS {vers}"]
    if vers.startswith("5") or vers.startswith("4"):
        out.append("2 FORM LINEAGE-LINKED")
    if char:
        out.append(f"1 CHAR {char}")
    out.extend(extra)
    return out


def family(names=None):
    """A small family: two parents, two children, a marriage, a source, a note."""
    n = names or {
        "I1": "Basile /Fontaine/",
        "I2": "Adèle /Marchand/",
        "I3": "Céleste /Fontaine/",
        "I4": "Gaspard /Fontaine/",
    }
    return [
        f"0 @I1@ INDI", f"1 NAME {n['I1']}", "1 SEX M",
        "1 BIRT", "2 DATE 12 MAR 1850", "2 PLAC Lyon, Rhône, France",
        "1 DEAT", "2 DATE 1920", "2 PLAC Paris, France",
        "1 FAMS @F1@",
        f"0 @I2@ INDI", f"1 NAME {n['I2']}", "1 SEX F",
        "1 BIRT", "2 DATE ABT 1855", "1 FAMS @F1@",
        "1 NOTE @N1@",
        f"0 @I3@ INDI", f"1 NAME {n['I3']}", "1 SEX F",
        "1 BIRT", "2 DATE 3 JUL 1880", "2 PLAC Lyon, Rhône, France", "1 FAMC @F1@",
        f"0 @I4@ INDI", f"1 NAME {n['I4']}", "1 SEX M",
        "1 BIRT", "2 DATE 1883", "1 FAMC @F1@",
        "0 @F1@ FAM", "1 HUSB @I1@", "1 WIFE @I2@", "1 CHIL @I3@", "1 CHIL @I4@",
        "1 MARR", "2 DATE 5 MAY 1878", "2 PLAC Lyon, Rhône, France",
        "0 @S1@ SOUR", "1 TITL Registre fictif de Lyon",
        "0 @N1@ NOTE Une note partagée,", "1 CONT sur deux lignes.",
    ]


def text(lines, eol="\n", trlr=True):
    body = list(lines) + (["0 TRLR"] if trlr else [])
    return eol.join(body) + eol


# ---------------------------------------------------------------- encodings
ANSEL_LETTERS = {
    "Ł": 0xA1, "Ø": 0xA2, "Đ": 0xA3, "Þ": 0xA4, "Æ": 0xA5, "Œ": 0xA6, "ʹ": 0xA7,
    "·": 0xA8, "♭": 0xA9, "®": 0xAA, "±": 0xAB, "Ơ": 0xAC, "Ư": 0xAD, "ʼ": 0xAE,
    "ʻ": 0xB0, "ł": 0xB1, "ø": 0xB2, "đ": 0xB3, "þ": 0xB4, "æ": 0xB5, "œ": 0xB6,
    "ʺ": 0xB7, "ı": 0xB8, "£": 0xB9, "ð": 0xBA, "ơ": 0xBC, "ư": 0xBD, "°": 0xC0,
    "ℓ": 0xC1, "℗": 0xC2, "©": 0xC3, "♯": 0xC4, "¿": 0xC5, "¡": 0xC6, "ß": 0xC7,
    "€": 0xC8,
}
ANSEL_MARKS = {
    "\u0309": 0xE0, "\u0300": 0xE1, "\u0301": 0xE2, "\u0302": 0xE3, "\u0303": 0xE4,
    "\u0304": 0xE5, "\u0306": 0xE6, "\u0307": 0xE7, "\u0308": 0xE8, "\u030C": 0xE9,
    "\u030A": 0xEA, "\u0315": 0xED, "\u030B": 0xEE, "\u0310": 0xEF, "\u0327": 0xF0,
    "\u0328": 0xF1, "\u0323": 0xF2, "\u0324": 0xF3, "\u0325": 0xF4, "\u0333": 0xF5,
    "\u0332": 0xF6, "\u0326": 0xF7, "\u031C": 0xF8, "\u032E": 0xF9, "\u0313": 0xFE,
}


def ansel(s):
    out = bytearray()
    for ch in s:
        if ord(ch) < 0x80:
            out.append(ord(ch))
        elif ch in ANSEL_LETTERS:
            out.append(ANSEL_LETTERS[ch])
        else:
            d = unicodedata.normalize("NFD", ch)
            base, marks = d[0], d[1:]
            if not marks or any(m not in ANSEL_MARKS for m in marks):
                raise ValueError(f"no ANSEL for {ch!r}")
            for m in marks:
                out.append(ANSEL_MARKS[m])
            out += ansel(base)
    return bytes(out)


def utf16(s, little=True, bom=True):
    body = s.encode("utf-16-le" if little else "utf-16-be")
    return ((b"\xff\xfe" if little else b"\xfe\xff") if bom else b"") + body


# ----------------------------------------------------------------- versions
POLISH = {"I1": "Józef /Gęsiorek/", "I2": "Łucja /Żak/", "I3": "Zofia /Gęsiorek/", "I4": "Kazimierz /Gęsiorek/"}
write("v40-ascii.ged", text(head("4.0", "ASCII") + family({"I1": "Paul /Martin/", "I2": "Anne /Roux/", "I3": "Luc /Martin/", "I4": "Marc /Martin/"}), "\r\n").encode(), "GEDCOM 4.0, ASCII")
write("v55-ansel.ged", ansel(text(head("5.5", "ANSEL") + family(POLISH), "\r\n")), "GEDCOM 5.5, ANSEL with Polish letters")
write("v551-utf8.ged", text(head() + family()).encode(), "GEDCOM 5.5.1, UTF-8, the reference")
write("v551-utf8-bom.ged", b"\xef\xbb\xbf" + text(head() + family()).encode(), "UTF-8 with a byte-order mark")
write("v555-utf16le-bom.ged", utf16(text(head("5.5.5", "UTF-16") + family(POLISH), "\r\n")), "GEDCOM 5.5.5, UTF-16 LE with a mark")
write("v555-utf16be-bom.ged", utf16(text(head("5.5.5", "UTF-16") + family(POLISH), "\r\n"), little=False), "UTF-16 BE with a mark")
write("v555-utf16be-nobom.ged", utf16(text(head("5.5.5", "UNICODE") + family(POLISH), "\r\n"), little=False, bom=False), "UTF-16 BE without a mark")
seven = head("7.0", None, ["1 SCHMA", "2 TAG _SKYPE http://xmlns.example/skype"]) + [
    "0 @I1@ INDI", "1 NAME Józef /Gęsiorek/", "2 GIVN Józef", "2 SURN Gęsiorek", "1 SEX M",
    "1 BIRT", "2 DATE 12 MAR 1850", "3 PHRASE as the register says", "2 PLAC Kraków, Polska",
    "3 MAP", "4 LATI N50.0614", "4 LONG E19.9366",
    "1 FAMS @F1@", "1 SNOTE @N1@", "1 UID 0f8d7c2a-0000-4000-8000-000000000001",
    "1 EXID 12345", "2 TYPE http://example.org/ids",
    "0 @I2@ INDI", "1 NAME Éléonore /Gęsiorek/", "1 SEX X", "1 FAMC @F1@",
    "0 @I3@ INDI", "1 NAME Helena /Nowak/", "1 SEX F", "1 FAMS @F1@",
    "1 NO MARR", "1 DEAT", "2 DATE BET 1920 AND 1925",
    "0 @F1@ FAM", "1 HUSB @I1@", "1 WIFE @I3@", "1 CHIL @I2@", "1 CHIL @VOID@",
    "2 PHRASE a child the register names but does not identify",
    "1 MARR", "2 DATE 1878", "1 SNOTE @N1@",
    "0 @N1@ SNOTE Une note longue,", "1 CONT sur deux lignes.", "1 MIME text/plain",
    "0 @S1@ SOUR", "1 TITL Registre fictif",
]
write("v70-utf8.ged", text(seven).encode(), "GEDCOM 7.0: @VOID@, SNOTE, SEX X, PHRASE, MAP, UID, EXID, SCHMA")
buf = __import__("io").BytesIO()
with zipfile.ZipFile(buf, "w") as z:
    z.writestr("gedcom.ged", text(seven))
    z.writestr("photos/portrait.jpg", b"\xff\xd8\xff\xe0fictional")
write("v70.gdz", buf.getvalue(), "GEDCOM 7.0 GEDZIP with a photograph")
buf = __import__("io").BytesIO()
with zipfile.ZipFile(buf, "w") as z:
    z.writestr("export/arbre.ged", text(head() + family()))
write("v551-in-zip-subdir.gdz", buf.getvalue(), "a zip whose only .ged sits in a folder")
buf = __import__("io").BytesIO()
with zipfile.ZipFile(buf, "w") as z:
    z.writestr("a.ged", text(head() + family()))
    z.writestr("b.ged", text(head() + family()))
write("zip-two-geds.gdz", buf.getvalue(), "a zip with two .ged files and no gedcom.ged")
# A zip bomb: a dataset that would unpack far past the upload limit.
buf = __import__("io").BytesIO()
with zipfile.ZipFile(buf, "w", zipfile.ZIP_DEFLATED) as z:
    filler = "1 NOTE " + "x" * 200 + "\n"
    z.writestr("gedcom.ged", text(head() + family(), trlr=False) + filler * 600_000 + "0 TRLR\n")
write("zip-bomb.gdz", buf.getvalue(), "a .gdz whose dataset unpacks to ~120 MB")

# --------------------------------------------------------------- encodings
FR = {"I1": "Hélène /Müller/", "I2": "François /Leçon/", "I3": "Zoë /Müller/", "I4": "Noël /Müller/"}
write("e-ansi-cp1252.ged", text(head(char="ANSI") + family(FR) + ["0 @I9@ INDI", "1 NAME Œdipe /D’Arc/", "1 NOTE prix : 5 € « guillemets »"], "\r\n").encode("cp1252"), "Windows-1252 with €, ’ and «»")
write("e-ibmpc-cp850.ged", text(head(char="IBMPC") + family(FR), "\r\n").encode("cp850"), "IBMPC (DOS code page 850)")
write("e-macintosh.ged", text(head(char="MACINTOSH") + family(FR), "\r").encode("mac_roman"), "Macintosh, MacRoman with CR line endings")
write("e-ascii-declared-highbytes.ged", text(head(char="ASCII") + family(FR)).encode("latin-1"), "declares ASCII, holds Latin-1 bytes")
write("e-utf8-declared-ansel.ged", text(head("5.5", "ANSEL") + family(FR)).encode(), "declares ANSEL, written in UTF-8")
bad = text(head() + family(FR) + ["0 @I9@ INDI", "1 NAME Prix /Euro/", "1 NOTE 5 EURO"]).encode()
bad = bad.replace(b"5 EURO", b"5 \x80")  # one stray Windows-1252 byte in a UTF-8 file
write("e-utf8-one-stray-byte.ged", bad, "UTF-8 with one stray Windows-1252 byte")
write("e-ansel-vietnamese.ged", ansel(text(head("5.5", "ANSEL") + family({"I1": "Nguyễn /Văn Bảo/", "I2": "Trần /Thị Ánh/", "I3": "Øster /Æbeltoft/", "I4": "Ñuño /Muñoz/"}), "\r\n")), "ANSEL with stacked accents (Vietnamese) and Nordic letters")
write("e-ansel-undefined-bytes.ged", ansel(text(head("5.5", "ANSEL") + family(), "\r\n").replace("Basile", "Bas\x00ile")).replace(b"\x00", b"\xaf\xbb"), "ANSEL with bytes outside the table")
write("e-utf16le-nobom-leading-blank.ged", utf16("\r\n" + text(head("5.5.5", "UNICODE") + family(POLISH), "\r\n"), bom=False), "UTF-16 LE, no mark, a blank line first")

# ----------------------------------------------------------- line endings
write("l-lf.ged", text(head() + family(), "\n").encode(), "LF")
write("l-crlf.ged", text(head() + family(), "\r\n").encode(), "CRLF")
write("l-cr.ged", text(head() + family(), "\r").encode(), "CR alone")
write("l-lfcr.ged", text(head() + family(), "\n\r").encode(), "LF CR, reversed")
mixed = "".join(l + random.Random(7).choice(["\n", "\r\n", "\r"]) for l in head() + family() + ["0 TRLR"])
write("l-mixed.ged", mixed.encode(), "mixed line endings")
write("l-no-final-eol.ged", text(head() + family()).rstrip("\n").encode(), "no line ending after TRLR")

# ----------------------------------------------------------------- syntax
fam = family()
write("s-indented-spaces.ged", text(head() + ["  " * int(l.split(" ")[0]) + l for l in fam]).encode(), "lines indented by level, with spaces")
write("s-indented-tabs.ged", text(head() + ["\t" * int(l.split(" ")[0]) + l for l in fam]).encode(), "lines indented by level, with tabs")
write("s-double-spaces.ged", text(head() + [l.replace(" ", "  ", 1) for l in fam]).encode(), "two spaces after the level")
write("s-tab-delimiters.ged", text(head() + [l.replace(" ", "\t", 2) for l in fam]).encode(), "tabs between level, tag and value")
write("s-trailing-spaces.ged", text(head() + [l + "   " for l in fam]).encode(), "trailing spaces on every line")
write("s-lowercase-tags.ged", text(head() + [" ".join(p.lower() if i == 1 or (i == 2 and l.split(" ")[1].startswith("@")) else p for i, p in enumerate(l.split(" "))) for l in fam]).encode(), "tags in lower case")
write("s-no-trlr.ged", text(head() + fam, trlr=False).encode(), "no TRLR")
write("s-no-head.ged", text(fam).encode(), "no HEAD")
write("s-blank-lines.ged", ("\n\n" + text(head() + fam).replace("\n", "\n\n")).encode(), "blank lines everywhere")
write("s-garbage-before-head.ged", ("Exported by FictiveTree 3.2\n" + text(head() + fam)).encode(), "a line of text before HEAD")
write("s-level-jump.ged", text(head() + ["0 @I1@ INDI", "1 NAME Saut /Deniveau/", "3 NOTE orphan level 3", "1 BIRT", "2 DATE 1900", "0 @I2@ INDI", "2 NAME Mal /Placé/"]).encode(), "levels that jump")
write("s-bad-levels.ged", text(head() + ["0 @I1@ INDI", "1 NAME Bon /Niveau/", "-1 NOTE negative", "X NOTE letter", "99999999999 NOTE huge", "1 SEX M"]).encode(), "levels that are not levels")
write("s-indi-without-xref.ged", text(head() + ["0 INDI", "1 NAME Sans /Référence/", "0 @I2@ INDI", "1 NAME Avec /Référence/"]).encode(), "an INDI without a cross-reference")
write("s-duplicate-xref.ged", text(head() + ["0 @I1@ INDI", "1 NAME Premier /Doublon/", "0 @I1@ INDI", "1 NAME Second /Doublon/"]).encode(), "two records share one xref")
write("s-dangling-pointers.ged", text(head() + ["0 @I1@ INDI", "1 NAME Seul /Pointeur/", "1 FAMS @F1@", "1 NOTE @N404@", "1 OBJE @M404@", "1 BIRT", "2 SOUR @S404@", "0 @F1@ FAM", "1 HUSB @I1@", "1 WIFE @I404@", "1 CHIL @I405@"]).encode(), "pointers to records that do not exist")
write("s-wrong-type-pointers.ged", text(head() + ["0 @I1@ INDI", "1 NAME Type /Erroné/", "0 @F1@ FAM", "1 HUSB @F1@", "1 CHIL @S1@", "1 WIFE @I1@", "0 @S1@ SOUR", "1 TITL Une source"]).encode(), "pointers to records of the wrong kind")
write("s-at-escapes.ged", text(head() + ["0 @I1@ INDI", "1 NAME Arobase /Échappé/", "1 NOTE écrire à jean@@exemple.org", "1 EMAIL jean@@exemple.org"]).encode(), "@@ escapes in values")
write("s-note-that-looks-like-pointer.ged", text(head() + ["0 @I1@ INDI", "1 NAME Note /Trompeuse/", "1 NOTE @home@"]).encode(), "a note whose text is @home@")
write("s-long-line.ged", text(head() + ["0 @I1@ INDI", "1 NAME Ligne /Longue/", "1 NOTE " + "abcdefghij" * 100_000]).encode(), "a 1 MB line")
conc = ["0 @I1@ INDI", "1 NAME Note /Enchaînée/", "1 NOTE début"] + [f"2 CONC  mot{i}" for i in range(20_000)] + ["2 CONT", "2 CONT fin"]
write("s-long-conc-chain.ged", text(head() + conc).encode(), "20,000 CONC lines")
write("s-conc-trailing-space.ged", text(head() + ["0 @I1@ INDI", "1 NAME Espace /Final/", "1 NOTE Une phrase coupée ", "2 CONC au milieu ", "2 CONC des mots."]).encode(), "CONC split after a space (the space ends the line)")
write("s-conc-leading-space.ged", text(head() + ["0 @I1@ INDI", "1 NAME Espace /Initial/", "1 NOTE Une phrase coupée", "2 CONC  au milieu", "2 CONC  des mots."]).encode(), "CONC split before a space (the space starts the line)")
write("s-cont-on-name.ged", text(head() + ["0 @I1@ INDI", "1 NAME Jean-Baptiste Marie", "2 CONC  /Delacroix-Fontaine/", "1 PLAC x"]).encode(), "a NAME continued by CONC")
write("s-empty-file.ged", b"", "an empty file")
write("s-only-head.ged", text(head(), trlr=False).encode(), "only a header")
write("s-binary.ged", bytes(random.Random(1).randrange(256) for _ in range(4096)), "random bytes")
write("s-nul-bytes.ged", text(head() + ["0 @I1@ INDI", "1 NAME Nul\x00 /Octet/", "1 NOTE a\x00b"]).encode(), "NUL bytes in values")
write("s-control-chars.ged", text(head() + ["0 @I1@ INDI", "1 NAME Bell\x07 /Ctrl\x1b[31m/", "1 NOTE vertical\x0btab\x0cfeed"]).encode(), "control characters in values")
write("s-xref-special.ged", text(head() + ["0 @I-1_é#x@ INDI", "1 NAME Xref /Spéciale/", "1 FAMS @F 1@", "0 @F 1@ FAM", "1 HUSB @I-1_é#x@"]).encode(), "xrefs with spaces, accents and punctuation")
write("s-xref-long.ged", text(head() + ["0 @" + "I" * 5000 + "@ INDI", "1 NAME Xref /Longue/"]).encode(), "a 5,000-character xref")

# ------------------------------------------------------------------ names
names = [
    "Jean /Dupont/ Jr.", "/Dupont/ Jean", "Jean Dupont", "Jean /Dupont", "Jean Dupont/",
    "Jean /Dupont/ /Martin/", "/ /", "//", "", "   ", "Marie-Ève /de la Tour d'Auvergne/",
    "Ann /O'Brien/", "Ana María /García Márquez/", "王 /小明/", "فاطمة /الزهراء/", "שרה /כהן/",
    "Zoë 😀 /Émoji/", "Jo\u200dsé /Zwj/", "e\u0301mile /Combinant/", "<script>alert('n')</script> /Xss/",
    "A" * 5000 + " /Long/", "Ingrid /Ål/", "Иван /Петров/", "Γιώργος /Παπαδόπουλος/",
]
lines = []
for i, n in enumerate(names, 1):
    lines += [f"0 @I{i}@ INDI", f"1 NAME {n}".rstrip() if n.strip() else "1 NAME", "1 SEX U"]
lines += ["0 @I90@ INDI", "1 NAME", "2 GIVN Pièces", "2 SURN Séparées", "2 NPFX Dr", "2 NSFX III",
          "0 @I91@ INDI", "1 NAME Marie /Curie/", "2 TYPE married", "1 NAME Maria /Skłodowska/", "2 TYPE birth",
          "0 @I92@ INDI", "1 NAME Jean /Valjean/", "2 NICK Le Forçat", "2 SPFX de",
          "0 @I93@ INDI", "1 SEX M", "1 BIRT", "2 DATE 1900",
          "0 @I94@ INDI", "1 NAME Anne /Martin/", "2 _MARNM Durand", "1 _AKA Annette"]
write("n-names.ged", text(head() + lines).encode(), "names in every shape")

sexes = ["M", "F", "U", "X", "m", "f", "Male", "FEMALE", "N", "", "NB"]
lines = []
for i, s in enumerate(sexes, 1):
    lines += [f"0 @I{i}@ INDI", f"1 NAME Sexe /{s or 'vide'}/", f"1 SEX {s}".rstrip()]
write("n-sexes.ged", text(head() + lines).encode(), "SEX values")

# ------------------------------------------------------------------ dates
DATES = [
    "12 MAR 1850", "MAR 1850", "1850", "12 mar 1850", "12 Mar 1850", "850", "15 MAR 850", "05 MAR 1850",
    "ABT 1850", "ABT MAR 1850", "ABT 12 MAR 1850", "CAL 1850", "EST 1850", "EST 12 MAR 1850",
    "BEF 1850", "AFT 1850", "BET 1850 AND 1860", "BET MAR 1850 AND 12 APR 1851", "BET 1860 AND 1850",
    "FROM 1850 TO 1860", "FROM 1850", "TO 1860", "INT 1850 (selon la famille)", "(vers la guerre)",
    "1750/51", "12 FEB 1750/51", "@#DJULIAN@ 1 JAN 1700", "@#DGREGORIAN@ 1 JAN 1900",
    "@#DFRENCH R@ 1 VEND 0002", "@#DFRENCH R@ 18 BRUM 0008", "@#DHEBREW@ 1 TSH 5600", "44 B.C.",
    "31 FEB 1900", "0 JAN 1900", "32 JAN 1900", "12 XYZ 1900", "1900-03-12", "12/03/1900", "3/12/1900",
    "ca. 1850", "c 1850", "~1850", "1850?", "1850s", "19th century", "vers 1850", "avant 1850",
    "après 1850", "entre 1850 et 1860", "env 1850", "VERS MARS 1850", "12 MARS 1850", "12 AOÛT 1850",
    "1 JANVIER 1850", "um 1850", "vor 1850", "nach 1850", "zwischen 1850 und 1860", "12 MÄRZ 1850",
    "około 1850", "przed 1850", "po 1850", "między 1850 i 1860", "12 KWIETNIA 1850 R.",
    "   12   MAR   1850  ", "ABT", "BET 1850", "AND 1860", "12 MAR 99999", "12 MAR -5", "MAR", "unknown",
    "@#DUNKNOWN@ 1850", "INT 12 MAR 1850", "BET ABT 1850 AND EST 1860", "FROM BEF 1850 TO AFT 1860",
    "1 JAN 0001", "12 MAR 1850 12:30", "Y",
]
lines = []
for i, d in enumerate(DATES, 1):
    lines += [f"0 @I{i}@ INDI", f"1 NAME Date /Cas{i:03d}/", "1 BIRT", f"2 DATE {d}".rstrip()]
write("d-dates.ged", text(head() + lines).encode(), "dates in every form")
INDEX["d-dates.ged"]["dates"] = DATES

# ----------------------------------------------------------------- places
places = ["Paris, France", "Paris,France", "paris, france", " Paris, France ", "Paris, Île-de-France, France",
          "", "  ", ", , ,", "Lyon", "Lyon, Rhône, France", "Saint-Étienne", "<b>Ville</b>", "X" * 2000]
lines = []
for i, p in enumerate(places, 1):
    lines += [f"0 @I{i}@ INDI", f"1 NAME Lieu /Cas{i:02d}/", "1 BIRT", "2 DATE 1900", f"2 PLAC {p}".rstrip()]
lines += ["0 @I50@ INDI", "1 NAME Lieu /Coordonnées/", "1 BIRT", "2 PLAC Quimper, Finistère, France",
          "3 MAP", "4 LATI N47.9960", "4 LONG W4.0970",
          "0 @I51@ INDI", "1 NAME Lieu /Formes/", "1 RESI", "2 PLAC Brest", "3 FORM Ville, Département"]
write("p-places.ged", text(head(extra=["1 PLAC", "2 FORM Ville, Département, Pays"]) + lines).encode(), "places: spacing, case, empty, coordinates, hierarchy")

# ----------------------------------------------------------------- events
ev = ["0 @I1@ INDI", "1 NAME Toute /Saisons/", "1 SEX F",
      "1 BIRT", "2 DATE 1 JAN 1850", "2 PLAC Nantes", "2 SOUR @S1@", "3 PAGE acte 12", "3 QUAY 3", "2 NOTE né à la maison",
      "1 BIRT", "2 DATE 2 JAN 1850", "2 NOTE une autre version",
      "1 CHR", "2 DATE 3 JAN 1850", "2 PLAC Nantes",
      "1 BAPM", "2 DATE 4 JAN 1850",
      "1 RESI", "2 DATE 1870", "2 PLAC Rennes",
      "1 CENS", "2 DATE 1876", "2 PLAC Rennes",
      "1 EDUC Lycée fictif", "1 EMIG", "2 DATE 1880", "1 IMMI", "2 DATE 1881", "2 PLAC Montréal",
      "1 NATU", "2 DATE 1890", "1 RELI Catholique", "1 NATI Française", "1 DSCR Cheveux bruns",
      "1 EVEN", "2 TYPE Médaille", "2 DATE 1900",
      "1 FACT Pianiste", "2 TYPE Talent",
      "1 OCCU Couturière", "2 DATE FROM 1868 TO 1880", "2 PLAC Rennes",
      "1 OCCU", "1 TITL Comtesse",
      "1 DEAT", "2 DATE 1930", "2 CAUS Grippe", "2 AGE 80y",
      "1 BURI", "2 DATE 1930", "2 PLAC Cimetière fictif",
      "1 FAMS @F1@",
      "0 @I2@ INDI", "1 NAME Époux /Saisons/", "1 SEX M", "1 FAMS @F1@",
      "0 @I3@ INDI", "1 NAME Enterré /Sansdécès/", "1 BURI", "2 DATE 1801",
      "0 @I4@ INDI", "1 NAME Baptisé /Sansnaissance/", "1 CHR", "2 DATE 1702",
      "0 @I5@ INDI", "1 NAME Ancien /Sansdates/", "1 BIRT", "2 DATE 1700",
      "0 @I6@ INDI", "1 NAME Mort /Sansdate/", "1 DEAT Y",
      "0 @F1@ FAM", "1 HUSB @I2@", "1 WIFE @I1@",
      "1 ENGA", "2 DATE 1874", "1 MARR", "2 DATE 1875", "2 TYPE civil", "1 MARR", "2 DATE 1876", "2 TYPE religieux",
      "1 DIV", "2 DATE 1890", "1 EVEN", "2 TYPE Séparation", "1 NOTE note de famille",
      "0 @S1@ SOUR", "1 TITL Registre fictif", "1 AUTH Mairie fictive", "1 PUBL 1850", "1 TEXT texte de l'acte", "1 REPO @R1@",
      "0 @R1@ REPO", "1 NAME Archives fictives",
      ]
write("ev-events.ged", text(head() + ev).encode(), "every kind of event and attribute")

# ---------------------------------------------------- families and lineage
ln = ["0 @I1@ INDI", "1 NAME Père /Lignée/", "1 SEX M", "1 FAMS @F1@",
      "0 @I2@ INDI", "1 NAME Mère /Lignée/", "1 SEX F", "1 FAMS @F1@",
      "0 @I3@ INDI", "1 NAME Adopté /Lignée/", "1 FAMC @F1@", "2 PEDI adopted",
      "0 @I4@ INDI", "1 NAME Accueilli /Lignée/", "1 FAMC @F1@", "2 PEDI foster",
      "0 @I5@ INDI", "1 NAME Biologique /Lignée/", "1 FAMC @F1@", "2 PEDI birth",
      "0 @I6@ INDI", "1 NAME FTM /Lignée/", "1 FAMC @F1@",
      "0 @I7@ INDI", "1 NAME Seulement /FAMC/", "1 FAMC @F1@",
      "0 @F1@ FAM", "1 HUSB @I1@", "1 WIFE @I2@", "1 CHIL @I3@", "1 CHIL @I4@", "1 CHIL @I5@",
      "1 CHIL @I6@", "2 _FREL Adopted", "2 _MREL Natural", "1 CHIL @I3@",
      ]
write("f-lineage.ged", text(head() + ln).encode(), "PEDI on FAMC, FTM _FREL/_MREL, a child listed twice, a child only in its FAMC")
same = ["0 @I1@ INDI", "1 NAME Un /Couple/", "1 SEX M", "0 @I2@ INDI", "1 NAME Deux /Couple/", "1 SEX M",
        "0 @I3@ INDI", "1 NAME Seul /Parent/", "1 SEX F", "0 @I4@ INDI", "1 NAME Enfant /Seul/",
        "0 @F1@ FAM", "1 HUSB @I1@", "1 HUSB @I2@", "1 MARR",
        "0 @F2@ FAM", "1 WIFE @I3@", "1 CHIL @I4@",
        "0 @F3@ FAM", "1 CHIL @I4@",
        "0 @F4@ FAM",
        "0 @F5@ FAM", "1 HUSB @I4@", "1 CHIL @I4@"]
write("f-shapes.ged", text(head() + same).encode(), "same-sex couple, single parent, sibling group, empty family, own child")
cyc = ["0 @I1@ INDI", "1 NAME Boucle /Un/", "1 FAMC @F2@", "1 FAMS @F1@",
       "0 @I2@ INDI", "1 NAME Boucle /Deux/", "1 FAMC @F1@", "1 FAMS @F2@",
       "0 @F1@ FAM", "1 HUSB @I1@", "1 CHIL @I2@",
       "0 @F2@ FAM", "1 HUSB @I2@", "1 CHIL @I1@"]
write("g-cycle.ged", text(head() + cyc).encode(), "two people each other's parent")


def chain(n):
    out = []
    for i in range(1, n + 1):
        out += [f"0 @I{i}@ INDI", f"1 NAME Génération{i} /Chaîne/", "1 SEX M",
                "1 BIRT", f"2 DATE {1000 + i if i < 9000 else 1000}"]
        if i > 1:
            out.append(f"1 FAMC @F{i - 1}@")
        if i < n:
            out.append(f"1 FAMS @F{i}@")
    for i in range(1, n):
        out += [f"0 @F{i}@ FAM", f"1 HUSB @I{i}@", f"1 CHIL @I{i + 1}@"]
    return out


write("g-chain-1000.ged", text(head() + chain(1000)).encode(), "a line of 1,000 generations")
if LARGE:
    write("g-chain-20000.ged", text(head() + chain(20000)).encode(), "a line of 20,000 generations")
wide = ["0 @I0@ INDI", "1 NAME Patriarche /Large/", "1 SEX M", "0 @F1@ FAM", "1 HUSB @I0@"]
wide += [f"1 CHIL @I{i}@" for i in range(1, 3001)]
for i in range(1, 3001):
    wide += [f"0 @I{i}@ INDI", f"1 NAME Enfant{i} /Large/", "1 FAMC @F1@"]
write("g-wide-3000-children.ged", text(head() + wide).encode(), "one family with 3,000 children")
spouses = ["0 @I0@ INDI", "1 NAME Volage /Large/", "1 SEX M"]
for i in range(1, 501):
    spouses += [f"0 @I{i}@ INDI", f"1 NAME Épouse{i} /Large/", "1 SEX F", f"0 @F{i}@ FAM", "1 HUSB @I0@", f"1 WIFE @I{i}@"]
write("g-500-spouses.ged", text(head() + spouses).encode(), "one man in 500 couples")

# ----------------------------------------------------------- deep nesting
for n in (2_000, 50_000):
    deep = ["0 @I1@ INDI", "1 NAME Niveaux /Profonds/"] + [f"{lv} _DEEP x" for lv in range(1, n + 1)]
    write(f"x-deep-levels-{n}.ged", text(head() + deep).encode(), f"levels nested {n} deep")

# --------------------------------------------------------------- security
xss = "<script>alert(1)</script>"
sec = ["0 @I1@ INDI", f"1 NAME {xss} /\"><img src=x onerror=alert(2)>/", f"1 NOTE {xss}",
       "1 BIRT", f"2 PLAC <svg onload=alert(3)>", "2 DATE 1900",
       "1 OBJE", "2 FILE ../../../../etc/passwd", "3 FORM txt", "3 TITL <b>titre</b>",
       "1 OBJE", "2 FILE javascript:alert(4)", "2 FORM jpg",
       "1 OBJE", "2 FILE C:\\Users\\Fictif\\Images\\portrait.JPG", "2 FORM JPEG",
       "1 OBJE", "2 FILE https://example.org/scan.pdf", "2 FORM application/pdf",
       "1 OBJE", "2 FILE /home/someone/.ssh/id_rsa",
       "0 @S1@ SOUR", f"1 TITL {xss}"]
write("x-injection.ged", text(head() + sec).encode(), "HTML in names, notes, places, titles; hostile file paths")
media = ["0 @I1@ INDI", "1 NAME Média /Partagé/", "1 OBJE @M1@", "1 OBJE @M1@", "1 OBJE @M2@",
         "0 @M1@ OBJE", "1 FILE photos/portrait.jpg", "2 FORM jpg", "2 TITL Portrait",
         "0 @M2@ OBJE", "1 FORM bmp", "1 TITL Ancien style", "1 BLOB", "2 CONT .HM.......k.1..F.jJ..",
         "0 @M3@ OBJE", "1 FILE orpheline.png"]
write("m-media.ged", text(head() + media).encode(), "shared media, a 5.5 BLOB, an orphan media record")

# ------------------------------------------------------------- big files
def big(n_people, seed):
    rnd = random.Random(seed)
    given = ["Basile", "Adèle", "Céleste", "Gaspard", "Zofia", "Lucien", "Hélène", "Jules", "Marthe", "Octave", "Rose", "Victor"]
    surname = ["Fontaine", "Marchand", "Delaunay", "Zając", "Lefèvre", "Morin", "Garnier", "Nowak"]
    towns = [f"Ville{i}, Région{i % 40}, Pays{i % 7}" for i in range(2000)]
    out = []
    fams = []
    for i in range(1, n_people + 1):
        y = 1600 + (i * 400) // n_people
        out += [f"0 @I{i}@ INDI", f"1 NAME {rnd.choice(given)} /{rnd.choice(surname)}/", f"1 SEX {rnd.choice('MF')}",
                "1 BIRT", f"2 DATE {rnd.randint(1, 28)} {rnd.choice(['JAN','FEB','MAR','APR','MAY','JUN','JUL','AUG','SEP','OCT','NOV','DEC'])} {y}",
                f"2 PLAC {rnd.choice(towns)}"]
        if y < 1940:
            out += ["1 DEAT", f"2 DATE ABT {y + rnd.randint(20, 90)}", f"2 PLAC {rnd.choice(towns)}"]
        if rnd.random() < 0.3:
            out += [f"1 OCCU {rnd.choice(['Laboureur', 'Tisserand', 'Meunier', 'Institutrice'])}"]
        if rnd.random() < 0.2:
            out += ["1 NOTE " + "Une note fictive. " * rnd.randint(1, 10)]
    for f in range(1, n_people // 3):
        h, w = rnd.randint(1, n_people), rnd.randint(1, n_people)
        kids = [rnd.randint(1, n_people) for _ in range(rnd.randint(0, 4))]
        fams += [f"0 @F{f}@ FAM", f"1 HUSB @I{h}@", f"1 WIFE @I{w}@"] + [f"1 CHIL @I{k}@" for k in kids]
        if rnd.random() < 0.5:
            fams += ["1 MARR", f"2 DATE {1600 + rnd.randint(0, 400)}", f"2 PLAC {rnd.choice(towns)}"]
    return out + fams + ["0 @S1@ SOUR", "1 TITL Registre fictif"]


if LARGE:
    b = text(head() + big(20_000, 3)).encode()
    write("big-20000.ged", b, f"20,000 people at random, {len(b) // 1024} KB")
    b = text(head() + big(36_000, 4)).encode()
    write("big-36000.ged", b, f"36,000 people at random, {len(b) // 1024} KB")


def real(n_target, seed):
    """A family that reads like a real one: founders in 1600, each couple's
    children born 20-40 years after their parents, children marrying within
    their generation; deep, wide, dated, and without a cycle."""
    rnd = random.Random(seed)
    given_m = ["Basile", "Gaspard", "Lucien", "Jules", "Octave", "Victor", "Kazimierz", "Józef", "Henri", "Émile"]
    given_f = ["Adèle", "Céleste", "Zofia", "Hélène", "Marthe", "Rose", "Łucja", "Éléonore", "Agathe", "Irène"]
    surnames = ["Fontaine", "Marchand", "Delaunay", "Zając", "Lefèvre", "Morin", "Garnier", "Nowak", "Gęsiorek", "Roux"]
    towns = [f"Bourg{i}, Pays{i % 30}, Contrée{i % 5}" for i in range(800)]
    months = "JAN FEB MAR APR MAY JUN JUL AUG SEP OCT NOV DEC".split()
    persons, lines, fams, famc, fams_of = [], [], [], {}, {}

    def person(sex, surname, year):
        persons.append((len(persons) + 1, sex, surname, year))
        return len(persons)

    gen = [(person("M", rnd.choice(surnames), 1600 + rnd.randint(0, 10)),
            person("F", rnd.choice(surnames), 1600 + rnd.randint(0, 10))) for _ in range(60)]
    fid = 0
    while len(persons) < n_target and gen:
        nxt = []
        for h, w in gen:
            fid += 1
            fams.append([fid, h, w, []])
            fams_of.setdefault(h, []).append(fid)
            fams_of.setdefault(w, []).append(fid)
            for _ in range(rnd.choice([0, 1, 2, 2, 3, 3, 4, 5, 6])):
                if len(persons) >= n_target:
                    break
                c = person(rnd.choice("MF"), persons[h - 1][2], persons[h - 1][3] + rnd.randint(20, 40))
                fams[-1][3].append(c)
                famc[c] = fid
                nxt.append(c)
        men = [p for p in nxt if persons[p - 1][1] == "M"]
        women = [p for p in nxt if persons[p - 1][1] == "F"]
        rnd.shuffle(men)
        rnd.shuffle(women)
        gen = list(zip(men, women))
    for (i, sex, surname, year) in persons:
        lines += [f"0 @I{i}@ INDI", f"1 NAME {rnd.choice(given_m if sex == 'M' else given_f)} /{surname}/", f"1 SEX {sex}",
                  "1 BIRT", f"2 DATE {rnd.randint(1, 28)} {rnd.choice(months)} {year}", f"2 PLAC {rnd.choice(towns)}"]
        if year < 1930:
            lines += ["1 DEAT", f"2 DATE {rnd.choice(['', 'ABT ', 'BEF '])}{year + rnd.randint(30, 85)}", f"2 PLAC {rnd.choice(towns)}"]
        if rnd.random() < 0.3:
            lines += [f"1 OCCU {rnd.choice(['Laboureur', 'Tisserand', 'Meunier', 'Institutrice', 'Couturière'])}"]
        if rnd.random() < 0.15:
            lines += ["1 NOTE " + "Une note fictive. " * rnd.randint(1, 8)]
        if i in famc:
            lines.append(f"1 FAMC @F{famc[i]}@")
        lines += [f"1 FAMS @F{f}@" for f in fams_of.get(i, [])]
    for f, h, w, kids in fams:
        lines += [f"0 @F{f}@ FAM", f"1 HUSB @I{h}@", f"1 WIFE @I{w}@"] + [f"1 CHIL @I{k}@" for k in kids]
        lines += ["1 MARR", f"2 DATE {persons[h - 1][3] + rnd.randint(18, 30)}", f"2 PLAC {rnd.choice(towns)}"]
    return lines


if LARGE:
    for n, seed in ((5_000, 11), (40_000, 12)):
        b = text(head() + real(n, seed)).encode()
        write(f"real-{n}.ged", b, f"{n:,} people, as a real family reads, {len(b) // 1024} KB")

with open(os.path.join(OUT, "index.json"), "w") as f:
    json.dump(INDEX, f, ensure_ascii=False, indent=1)
print(len(INDEX), "files")
for k, v in INDEX.items():
    if v["bytes"] > 1_000_000:
        print(" ", k, v["bytes"] // 1024, "KB")
