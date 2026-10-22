//! A note as a PDF to download (v0.5.0, docs/adr/0014).
//!
//! Drawn here rather than printed by the reader's browser: the window that
//! shows a note offers a file, and a file is what a family keeps, sends and
//! prints. A4, in DejaVu Sans (embedded; static/fonts, its licence beside
//! it), which writes every Latin, Greek, Cyrillic and Hebrew letter a family
//! of this archive uses; italic is the same face leaned, as there is no
//! italic file. Each line is measured with the shaper the PDF library draws
//! with, so the line measured is the line drawn.
//!
//! What it does not do: right-to-left lines are drawn in one run without the
//! bidirectional algorithm, and Chinese or Japanese have no glyphs in this
//! face — the page shows such a note whole in the reader's own browser.

use krilla::color::rgb;
use krilla::geom::{PathBuilder, Point, Size, Transform};
use krilla::num::NormalizedF32;
use krilla::page::PageSettings;
use krilla::paint::{Fill, Stroke};
use krilla::text::{Font, TextDirection};
use krilla::Document;

static REGULAR: &[u8] = include_bytes!("../static/fonts/DejaVuSans.ttf");
static BOLD: &[u8] = include_bytes!("../static/fonts/DejaVuSans-Bold.ttf");

const PAGE_W: f32 = 595.28;
const PAGE_H: f32 = 841.89;
const MARGIN_X: f32 = 60.0;
const MARGIN_TOP: f32 = 64.0;
const MARGIN_BOTTOM: f32 = 70.0;
const BODY: f32 = 10.5;
const LEADING: f32 = 1.45;

/// What a note's PDF shows: its title, the lines under it (category, date,
/// author; who it is about), its text, and the line at the foot of each page.
pub struct PdfNote<'a> {
    pub title: &'a str,
    pub meta: Vec<String>,
    pub markdown: &'a str,
    pub footer: String,
    pub language: String,
}

#[derive(Clone, Copy, PartialEq)]
struct Style {
    bold: bool,
    italic: bool,
    mono: bool,
}

const PLAIN: Style = Style {
    bold: false,
    italic: false,
    mono: false,
};

#[derive(Clone)]
struct Run {
    text: String,
    style: Style,
}

enum Block {
    Heading(u8, Vec<Run>),
    Para(Vec<Run>),
    Item {
        depth: usize,
        marker: String,
        runs: Vec<Run>,
    },
    Quote(Vec<Run>),
    Code(String),
    Image {
        dest: String,
        alt: String,
    },
    Rule,
}

/// The note's text as blocks the page knows how to set.
fn blocks(markdown: &str) -> Vec<Block> {
    use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
    let mut out = Vec::new();
    let mut runs: Vec<Run> = Vec::new();
    let mut style = PLAIN;
    let mut bold = 0usize;
    let mut italic = 0usize;
    // (ordered next number, or None for bullets), per open list.
    let mut lists: Vec<Option<u64>> = Vec::new();
    let mut item_marker: Option<(usize, String)> = None;
    let mut quote = 0usize;
    let mut heading: Option<u8> = None;
    let mut code: Option<String> = None;
    let mut image: Option<(String, String)> = None;
    let flush = |runs: &mut Vec<Run>,
                 out: &mut Vec<Block>,
                 heading: Option<u8>,
                 item: &mut Option<(usize, String)>,
                 quote: usize| {
        if runs.iter().all(|r| r.text.trim().is_empty()) {
            runs.clear();
            return;
        }
        let taken = std::mem::take(runs);
        if let Some(level) = heading {
            out.push(Block::Heading(level, taken));
        } else if let Some((depth, marker)) = item.take() {
            out.push(Block::Item {
                depth,
                marker,
                runs: taken,
            });
        } else if quote > 0 {
            out.push(Block::Quote(taken));
        } else {
            out.push(Block::Para(taken));
        }
    };
    for e in Parser::new_ext(
        markdown,
        Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH,
    ) {
        match e {
            Event::Start(Tag::Heading { level, .. }) => {
                flush(&mut runs, &mut out, heading, &mut item_marker, quote);
                heading = Some(level as u8);
            }
            Event::End(TagEnd::Heading(_)) => {
                flush(&mut runs, &mut out, heading, &mut item_marker, quote);
                heading = None;
            }
            Event::Start(Tag::Paragraph) => {}
            Event::End(TagEnd::Paragraph) | Event::End(TagEnd::TableRow) => {
                flush(&mut runs, &mut out, heading, &mut item_marker, quote)
            }
            Event::Start(Tag::BlockQuote(_)) => quote += 1,
            Event::End(TagEnd::BlockQuote(_)) => {
                flush(&mut runs, &mut out, heading, &mut item_marker, quote);
                quote = quote.saturating_sub(1);
            }
            Event::Start(Tag::List(start)) => lists.push(start),
            Event::End(TagEnd::List(_)) => {
                flush(&mut runs, &mut out, heading, &mut item_marker, quote);
                lists.pop();
            }
            Event::Start(Tag::Item) => {
                flush(&mut runs, &mut out, heading, &mut item_marker, quote);
                let depth = lists.len().saturating_sub(1);
                let marker = match lists.last_mut() {
                    Some(Some(n)) => {
                        let m = format!("{n}.");
                        *n += 1;
                        m
                    }
                    _ => "•".to_string(),
                };
                item_marker = Some((depth, marker));
            }
            Event::End(TagEnd::Item) => {
                flush(&mut runs, &mut out, heading, &mut item_marker, quote)
            }
            Event::Start(Tag::CodeBlock(_)) => code = Some(String::new()),
            Event::End(TagEnd::CodeBlock) => {
                if let Some(c) = code.take() {
                    out.push(Block::Code(c.trim_end().to_string()));
                }
            }
            Event::Start(Tag::Strong) => bold += 1,
            Event::End(TagEnd::Strong) => bold = bold.saturating_sub(1),
            Event::Start(Tag::Emphasis) => italic += 1,
            Event::End(TagEnd::Emphasis) => italic = italic.saturating_sub(1),
            Event::Start(Tag::Image { dest_url, .. }) => {
                flush(&mut runs, &mut out, heading, &mut item_marker, quote);
                image = Some((dest_url.to_string(), String::new()));
            }
            Event::End(TagEnd::Image) => {
                if let Some((dest, alt)) = image.take() {
                    out.push(Block::Image { dest, alt });
                }
            }
            Event::Rule => {
                flush(&mut runs, &mut out, heading, &mut item_marker, quote);
                out.push(Block::Rule);
            }
            Event::Text(t) | Event::Html(t) | Event::InlineHtml(t) => {
                if let Some(c) = code.as_mut() {
                    c.push_str(&t);
                } else if let Some((_, alt)) = image.as_mut() {
                    alt.push_str(&t);
                } else {
                    style.bold = bold > 0 || heading.is_some();
                    style.italic = italic > 0 || quote > 0;
                    style.mono = false;
                    runs.push(Run {
                        text: t.to_string(),
                        style,
                    });
                }
            }
            Event::Code(t) => runs.push(Run {
                text: t.to_string(),
                style: Style {
                    mono: true,
                    ..PLAIN
                },
            }),
            Event::SoftBreak | Event::HardBreak => runs.push(Run {
                text: " ".into(),
                style: PLAIN,
            }),
            Event::End(TagEnd::TableCell) => runs.push(Run {
                text: "  ·  ".into(),
                style: PLAIN,
            }),
            _ => {}
        }
    }
    flush(&mut runs, &mut out, heading, &mut item_marker, quote);
    out
}

/// A face, measured the way the PDF library shapes it.
struct Face {
    font: Font,
    shaper: rustybuzz::Face<'static>,
}

impl Face {
    fn new(bytes: &'static [u8]) -> Result<Self, String> {
        Ok(Face {
            font: Font::new(bytes.into(), 0).ok_or("the embedded font does not load")?,
            shaper: rustybuzz::Face::from_slice(bytes, 0)
                .ok_or("the embedded font does not shape")?,
        })
    }

    fn width(&self, text: &str, size: f32) -> f32 {
        let mut buf = rustybuzz::UnicodeBuffer::new();
        buf.push_str(text);
        let shaped = rustybuzz::shape(&self.shaper, &[], buf);
        let units: i32 = shaped.glyph_positions().iter().map(|p| p.x_advance).sum();
        units as f32 / self.shaper.units_per_em() as f32 * size
    }
}

/// One thing drawn on a page.
enum Cmd {
    Text {
        x: f32,
        y: f32,
        size: f32,
        bold: bool,
        italic: bool,
        grey: u8,
        text: String,
    },
    Line {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        grey: u8,
    },
    Image {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        image: krilla::image::Image,
    },
}

struct Layout<'f> {
    regular: &'f Face,
    bold: &'f Face,
    pages: Vec<Vec<Cmd>>,
    y: f32,
}

impl<'f> Layout<'f> {
    fn face(&self, bold: bool) -> &'f Face {
        if bold {
            self.bold
        } else {
            self.regular
        }
    }

    fn page(&mut self) -> &mut Vec<Cmd> {
        if self.pages.is_empty() {
            self.pages.push(Vec::new());
        }
        self.pages.last_mut().expect("a page")
    }

    fn room_for(&mut self, height: f32) {
        if self.y + height > PAGE_H - MARGIN_BOTTOM && self.y > MARGIN_TOP + 1.0 {
            self.pages.push(Vec::new());
            self.y = MARGIN_TOP;
        }
    }

    /// Set runs as lines within `width`, from `x`, each word in its run's
    /// style; returns nothing, moves `y` down.
    #[allow(clippy::too_many_arguments)]
    fn lines(
        &mut self,
        runs: &[Run],
        x: f32,
        width: f32,
        size: f32,
        grey: u8,
        first_indent: Option<(&str, f32)>,
    ) {
        // Words, each with its style, and whether a space came before it.
        let mut words: Vec<(String, Style, bool)> = Vec::new();
        let mut space_before = false;
        for r in runs {
            let mut current = String::new();
            for c in r.text.chars() {
                if c.is_whitespace() {
                    if !current.is_empty() {
                        words.push((std::mem::take(&mut current), r.style, space_before));
                    }
                    space_before = true;
                } else {
                    current.push(c);
                }
            }
            if !current.is_empty() {
                words.push((current, r.style, space_before));
                space_before = false;
            }
        }
        let line_h = size * LEADING;
        let space = self.regular.width(" ", size);
        let mut line: Vec<(String, Style, f32)> = Vec::new();
        let mut used = 0.0f32;
        let mut first = true;
        let flush = |this: &mut Self, line: &mut Vec<(String, Style, f32)>, first: &mut bool| {
            this.room_for(line_h);
            let baseline = this.y + size;
            if *first {
                if let Some((marker, at)) = first_indent {
                    let page = this.page();
                    page.push(Cmd::Text {
                        x: at,
                        y: baseline,
                        size,
                        bold: false,
                        italic: false,
                        grey,
                        text: marker.to_string(),
                    });
                }
            }
            let page = this.page();
            for (w, st, wx) in line.drain(..) {
                page.push(Cmd::Text {
                    x: x + wx,
                    y: baseline,
                    size,
                    bold: st.bold,
                    italic: st.italic,
                    grey: if st.mono {
                        grey.saturating_add(40)
                    } else {
                        grey
                    },
                    text: w,
                });
            }
            this.y += line_h;
            *first = false;
        };
        for (w, st, sp) in words {
            let face = self.face(st.bold);
            let ww = face.width(&w, size);
            let gap = if line.is_empty() || !sp { 0.0 } else { space };
            if !line.is_empty() && used + gap + ww > width {
                flush(self, &mut line, &mut first);
                used = 0.0;
            }
            // A word longer than the line: cut where it runs out.
            if line.is_empty() && ww > width {
                let mut piece = String::new();
                for c in w.chars() {
                    piece.push(c);
                    if face.width(&piece, size) > width && piece.chars().count() > 1 {
                        let last = piece.pop().expect("a char");
                        line.push((std::mem::take(&mut piece), st, 0.0));
                        flush(self, &mut line, &mut first);
                        piece.push(last);
                    }
                }
                used = face.width(&piece, size);
                line.push((piece, st, 0.0));
                continue;
            }
            let gap = if line.is_empty() || !sp { 0.0 } else { space };
            line.push((w, st, used + gap));
            used += gap + ww;
        }
        if !line.is_empty() {
            flush(self, &mut line, &mut first);
        }
    }
}

/// The note, as the bytes of a PDF. `image` gives the bytes of a picture the
/// text names (`axgf:document/<id>`) that this reader may see, or nothing.
pub fn render(
    note: &PdfNote<'_>,
    image: &dyn Fn(&str) -> Option<Vec<u8>>,
) -> Result<Vec<u8>, String> {
    let regular = Face::new(REGULAR)?;
    let bold = Face::new(BOLD)?;
    let pages = layout(note, image, &regular, &bold);
    draw(note, pages, &regular, &bold)
}

/// The note set on pages: what is drawn where, page by page.
fn layout(
    note: &PdfNote<'_>,
    image: &dyn Fn(&str) -> Option<Vec<u8>>,
    regular: &Face,
    bold: &Face,
) -> Vec<Vec<Cmd>> {
    let width = PAGE_W - 2.0 * MARGIN_X;
    let mut l = Layout {
        regular,
        bold,
        pages: vec![Vec::new()],
        y: MARGIN_TOP,
    };

    // The head: the title, the lines under it, a rule.
    let title = [Run {
        text: note.title.to_string(),
        style: Style {
            bold: true,
            ..PLAIN
        },
    }];
    l.lines(&title, MARGIN_X, width, 20.0, 20, None);
    l.y += 4.0;
    for m in &note.meta {
        let run = [Run {
            text: m.clone(),
            style: PLAIN,
        }];
        l.lines(&run, MARGIN_X, width, 9.5, 95, None);
    }
    l.y += 8.0;
    let y = l.y;
    l.page().push(Cmd::Line {
        x1: MARGIN_X,
        y1: y,
        x2: PAGE_W - MARGIN_X,
        y2: y,
        grey: 190,
    });
    l.y += 16.0;

    for b in blocks(note.markdown) {
        match b {
            Block::Heading(level, runs) => {
                let size = match level {
                    1 => 15.0,
                    2 => 13.0,
                    _ => 11.5,
                };
                l.y += size * 0.6;
                l.room_for(size * 3.0);
                l.lines(&runs, MARGIN_X, width, size, 25, None);
                l.y += 3.0;
            }
            Block::Para(runs) => {
                l.lines(&runs, MARGIN_X, width, BODY, 35, None);
                l.y += BODY * 0.7;
            }
            Block::Item {
                depth,
                marker,
                runs,
            } => {
                let indent = 18.0 * (depth as f32 + 1.0);
                l.lines(
                    &runs,
                    MARGIN_X + indent,
                    width - indent,
                    BODY,
                    35,
                    Some((&marker, MARGIN_X + indent - 13.0)),
                );
                l.y += BODY * 0.25;
            }
            Block::Quote(runs) => {
                let top = l.y;
                l.lines(&runs, MARGIN_X + 16.0, width - 16.0, BODY, 70, None);
                let bottom = l.y;
                if bottom > top {
                    l.page().push(Cmd::Line {
                        x1: MARGIN_X + 4.0,
                        y1: top + 2.0,
                        x2: MARGIN_X + 4.0,
                        y2: bottom - 2.0,
                        grey: 180,
                    });
                }
                l.y += BODY * 0.7;
            }
            Block::Code(text) => {
                for line in text.lines() {
                    let run = [Run {
                        text: line.to_string(),
                        style: Style {
                            mono: true,
                            ..PLAIN
                        },
                    }];
                    l.lines(&run, MARGIN_X + 12.0, width - 12.0, 9.5, 35, None);
                }
                l.y += BODY * 0.7;
            }
            Block::Rule => {
                l.room_for(14.0);
                l.y += 6.0;
                let y = l.y;
                l.page().push(Cmd::Line {
                    x1: MARGIN_X + width * 0.3,
                    y1: y,
                    x2: MARGIN_X + width * 0.7,
                    y2: y,
                    grey: 190,
                });
                l.y += 10.0;
            }
            Block::Image { dest, alt } => {
                let picture = image(&dest).and_then(|bytes| {
                    let kind = crate::notes::image_kind(&bytes)?.0;
                    let data: krilla::Data = bytes.into();
                    match kind {
                        "image/png" => krilla::image::Image::from_png(data, true).ok(),
                        "image/jpeg" => krilla::image::Image::from_jpeg(data, true).ok(),
                        "image/gif" => krilla::image::Image::from_gif(data, true).ok(),
                        "image/webp" => krilla::image::Image::from_webp(data, true).ok(),
                        _ => None,
                    }
                });
                match picture {
                    Some(img) => {
                        let (pw, ph) = img.size();
                        let (pw, ph) = (pw.max(1) as f32, ph.max(1) as f32);
                        // Its own size at 96 pixels to the inch, made smaller
                        // to fit the line and three fifths of a page.
                        let max_h = (PAGE_H - MARGIN_TOP - MARGIN_BOTTOM) * 0.6;
                        let scale = 0.75_f32.min(width / pw).min(max_h / ph);
                        let (w, h) = (pw * scale, ph * scale);
                        l.room_for(h + 8.0);
                        let (x, y) = (MARGIN_X + (width - w) / 2.0, l.y + 4.0);
                        l.page().push(Cmd::Image {
                            x,
                            y,
                            w,
                            h,
                            image: img,
                        });
                        l.y += h + 10.0;
                        if !alt.trim().is_empty() {
                            let run = [Run {
                                text: alt,
                                style: Style {
                                    italic: true,
                                    ..PLAIN
                                },
                            }];
                            l.lines(&run, MARGIN_X, width, 9.0, 90, None);
                            l.y += 6.0;
                        }
                    }
                    None if !alt.trim().is_empty() => {
                        let run = [Run {
                            text: format!("[{alt}]"),
                            style: Style {
                                italic: true,
                                ..PLAIN
                            },
                        }];
                        l.lines(&run, MARGIN_X, width, BODY, 90, None);
                        l.y += BODY * 0.7;
                    }
                    None => {}
                }
            }
        }
    }

    l.pages
}

/// The pages drawn, each with its foot.
fn draw(
    note: &PdfNote<'_>,
    pages: Vec<Vec<Cmd>>,
    regular: &Face,
    bold: &Face,
) -> Result<Vec<u8>, String> {
    let mut doc = Document::new();
    doc.set_metadata(
        krilla::metadata::Metadata::new()
            .title(note.title.to_string())
            .language(note.language.clone())
            .creator("ax-genealogy".to_string()),
    );
    let total = pages.len();
    for (i, cmds) in pages.into_iter().enumerate() {
        let mut page =
            doc.start_page_with(PageSettings::from_wh(PAGE_W, PAGE_H).ok_or("page size")?);
        let mut s = page.surface();
        let foot = format!("{}  ·  {} / {}", note.footer, i + 1, total);
        let foot_cmd = Cmd::Text {
            x: MARGIN_X,
            y: PAGE_H - MARGIN_BOTTOM / 2.0,
            size: 8.0,
            bold: false,
            italic: false,
            grey: 130,
            text: foot,
        };
        for c in cmds.into_iter().chain(std::iter::once(foot_cmd)) {
            match c {
                Cmd::Text {
                    x,
                    y,
                    size,
                    bold: b,
                    italic,
                    grey,
                    text,
                } => {
                    s.set_fill(Some(Fill {
                        paint: rgb::Color::new(grey, grey, grey).into(),
                        opacity: NormalizedF32::ONE,
                        rule: Default::default(),
                    }));
                    let font = if b {
                        bold.font.clone()
                    } else {
                        regular.font.clone()
                    };
                    if italic {
                        // Leaned from the baseline: the face has no italic.
                        s.push_transform(&Transform::from_row(1.0, 0.0, -0.2, 1.0, x, y));
                        s.draw_text(
                            Point::from_xy(0.0, 0.0),
                            font,
                            size,
                            &text,
                            false,
                            TextDirection::Auto,
                        );
                        s.pop();
                    } else {
                        s.draw_text(
                            Point::from_xy(x, y),
                            font,
                            size,
                            &text,
                            false,
                            TextDirection::Auto,
                        );
                    }
                }
                Cmd::Line {
                    x1,
                    y1,
                    x2,
                    y2,
                    grey,
                } => {
                    let mut pb = PathBuilder::new();
                    pb.move_to(x1, y1);
                    pb.line_to(x2, y2);
                    if let Some(path) = pb.finish() {
                        s.set_fill(None);
                        s.set_stroke(Some(Stroke {
                            paint: rgb::Color::new(grey, grey, grey).into(),
                            width: 0.8,
                            ..Default::default()
                        }));
                        s.draw_path(&path);
                        s.set_stroke(None);
                    }
                }
                Cmd::Image { x, y, w, h, image } => {
                    if let Some(size) = Size::from_wh(w, h) {
                        s.push_transform(&Transform::from_translate(x, y));
                        s.draw_image(image, size);
                        s.pop();
                    }
                }
            }
        }
        s.finish();
        page.finish();
    }
    doc.finish().map_err(|e| format!("{e:?}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn note(md: &str) -> Vec<u8> {
        render(
            &PdfNote {
                title: "Souvenirs d'Éléonore Gęsiorek",
                meta: vec![
                    "Souvenir · 14 juin 1950 · marie".into(),
                    "Au sujet de : Józef Gęsiorek".into(),
                ],
                markdown: md,
                footer: "Famille fictive — ax-genealogy".into(),
                language: "fr".into(),
            },
            &|_| None,
        )
        .expect("a pdf")
    }

    #[test]
    fn a_note_is_a_pdf() {
        let pdf = note("Un *souvenir* **précis**.\n\n- un\n- deux\n\n1. premier\n\n> cité\n\n`code`\n\n---\n\n## Suite\n\nŁódź, Αθήνα, Москва, שלום.");
        assert!(pdf.starts_with(b"%PDF-"), "{:?}", &pdf[..8]);
        assert!(pdf.len() > 1000);
    }

    #[test]
    fn a_long_note_runs_onto_more_pages() {
        let (r, b) = (Face::new(REGULAR).unwrap(), Face::new(BOLD).unwrap());
        let pages = |md: &str| {
            let n = PdfNote {
                title: "T",
                meta: vec![],
                markdown: md,
                footer: String::new(),
                language: "fr".into(),
            };
            layout(&n, &|_| None, &r, &b).len()
        };
        assert_eq!(pages("Un paragraphe."), 1);
        let long =
            "Un paragraphe assez long pour remplir des lignes et des lignes, encore et encore.\n\n"
                .repeat(200);
        assert!(pages(&long) > 5, "{}", pages(&long));
        assert!(note(&long).starts_with(b"%PDF-"));
    }

    #[test]
    fn a_word_longer_than_the_line_is_cut_not_lost() {
        let pdf = note(&"a".repeat(600));
        assert!(pdf.starts_with(b"%PDF-"));
    }

    #[test]
    fn a_picture_it_cannot_read_is_its_description() {
        let pdf = render(
            &PdfNote {
                title: "Photo",
                meta: vec![],
                markdown: "![Le portrait](axgf:document/x)",
                footer: String::new(),
                language: "fr".into(),
            },
            &|_| Some(b"not an image".to_vec()),
        )
        .unwrap();
        assert!(pdf.starts_with(b"%PDF-"));
    }
}
