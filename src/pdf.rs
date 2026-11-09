//! What can be read of a PDF without a PDF library: how many pages it has
//! (docs/adr/0040), so that the AFG is given no more pages than a message can
//! carry and the tab can say what a message costs.
//!
//! A PDF's page tree says its count: the dictionary of `/Type /Pages` at its
//! root holds `/Count n`. In a PDF of 1.5 or later that dictionary may sit in
//! a compressed object stream (`/Type /ObjStm`, `/FlateDecode`), which is
//! inflated here. Failing both, the pages themselves are counted. Failing
//! that too, the count is unknown, and said to be.

use std::io::Read;

/// The most an object stream is inflated to: a stream that would grow past
/// this is not a page tree, whatever it claims.
const MOST_INFLATED: u64 = 64 * 1024 * 1024;

/// The number of pages of a PDF, or `None` when it cannot be read.
pub fn pages(bytes: &[u8]) -> Option<usize> {
    // A PDF says so in its first kilobyte.
    find(&bytes[..bytes.len().min(1024)], b"%PDF-")?;
    count_in(bytes)
        .or_else(|| {
            object_streams(bytes)
                .iter()
                .filter_map(|s| count_in(s))
                .max()
        })
        .or_else(|| leaves(bytes))
}

/// Where `needle` first occurs in `hay` from `from` on.
fn find_from(hay: &[u8], needle: &[u8], from: usize) -> Option<usize> {
    if from >= hay.len() || needle.is_empty() {
        return None;
    }
    hay[from..]
        .windows(needle.len())
        .position(|w| w == needle)
        .map(|i| i + from)
}

fn find(hay: &[u8], needle: &[u8]) -> Option<usize> {
    find_from(hay, needle, 0)
}

fn is_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\r' | b'\n' | b'\x0c' | b'\0')
}

/// A PDF name ends at a space or a delimiter.
fn name_ends(hay: &[u8], at: usize) -> bool {
    hay.get(at)
        .is_none_or(|&b| is_space(b) || b"/<>[]()%".contains(&b))
}

/// Where each `/Type /<name>` occurs, the name whole (`/Pages`, not `/Page`
/// in `/Pages`).
fn types(hay: &[u8], name: &[u8]) -> Vec<usize> {
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(at) = find_from(hay, b"/Type", from) {
        from = at + 5;
        let mut i = from;
        while i < hay.len() && is_space(hay[i]) {
            i += 1;
        }
        if hay.get(i) == Some(&b'/')
            && hay.len() >= i + 1 + name.len()
            && &hay[i + 1..i + 1 + name.len()] == name
            && name_ends(hay, i + 1 + name.len())
        {
            out.push(at);
        }
    }
    out
}

/// The dictionary around `at`: from its `<<` to its `>>`.
fn dictionary(hay: &[u8], at: usize) -> Option<(usize, usize)> {
    // Back to the `<<` that opens the dictionary holding `at`.
    let mut depth = 0usize;
    let mut i = at;
    let start = loop {
        if i < 2 {
            return None;
        }
        i -= 1;
        if hay[i - 1] == b'>' && hay[i] == b'>' {
            depth += 1;
            i -= 1;
        } else if hay[i - 1] == b'<' && hay[i] == b'<' {
            if depth == 0 {
                break i - 1;
            }
            depth -= 1;
            i -= 1;
        }
    };
    // On to the `>>` that closes it.
    let mut depth = 0usize;
    let mut j = start + 2;
    while j + 1 < hay.len() {
        if hay[j] == b'<' && hay[j + 1] == b'<' {
            depth += 1;
            j += 2;
        } else if hay[j] == b'>' && hay[j + 1] == b'>' {
            if depth == 0 {
                return Some((start, j + 2));
            }
            depth -= 1;
            j += 2;
        } else {
            j += 1;
        }
    }
    None
}

/// The number after `/Count` in a dictionary, unless it is a reference
/// (`/Count 12 0 R`), whose number is an object's, not a count.
fn count_of(dict: &[u8]) -> Option<usize> {
    let at = find(dict, b"/Count")?;
    let rest = &dict[at + 6..];
    let digits_from = rest.iter().position(|b| !is_space(*b))?;
    let digits: Vec<u8> = rest[digits_from..]
        .iter()
        .copied()
        .take_while(u8::is_ascii_digit)
        .collect();
    let n: usize = std::str::from_utf8(&digits).ok()?.parse().ok()?;
    let after = &rest[digits_from + digits.len()..];
    let words: Vec<&[u8]> = after
        .split(|b| is_space(*b))
        .filter(|w| !w.is_empty())
        .take(2)
        .collect();
    let reference = words.len() == 2
        && words[0].iter().all(u8::is_ascii_digit)
        && words[1].first() == Some(&b'R');
    (!reference).then_some(n)
}

/// The largest `/Count` of a page tree's dictionary in `hay`: the root's,
/// which counts every page.
fn count_in(hay: &[u8]) -> Option<usize> {
    types(hay, b"Pages")
        .into_iter()
        .filter_map(|at| dictionary(hay, at))
        .filter_map(|(a, b)| count_of(&hay[a..b]))
        .max()
}

/// The pages themselves, when no page tree says their count.
fn leaves(hay: &[u8]) -> Option<usize> {
    let n = types(hay, b"Page").len();
    (n > 0).then_some(n)
}

/// The compressed object streams of a PDF, inflated.
fn object_streams(hay: &[u8]) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    for at in types(hay, b"ObjStm") {
        let Some((a, b)) = dictionary(hay, at) else {
            continue;
        };
        if find(&hay[a..b], b"/FlateDecode").is_none() {
            continue;
        }
        let Some(kw) = find_from(hay, b"stream", b) else {
            continue;
        };
        if hay[b..kw].iter().any(|c| !is_space(*c)) {
            continue;
        }
        let mut data = kw + 6;
        if hay.get(data) == Some(&b'\r') {
            data += 1;
        }
        if hay.get(data) == Some(&b'\n') {
            data += 1;
        }
        let Some(end) = find_from(hay, b"endstream", data) else {
            continue;
        };
        let mut inflated = Vec::new();
        let read = flate2::read::ZlibDecoder::new(&hay[data..end])
            .take(MOST_INFLATED)
            .read_to_end(&mut inflated);
        // The decoder stops at the end of the compressed data, so the line
        // end before `endstream` does no harm; a stream cut short may still
        // hold the page tree.
        if read.is_ok() || !inflated.is_empty() {
            out.push(inflated);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    /// A PDF of `n` pages, its page tree in plain sight.
    fn plain(n: usize) -> Vec<u8> {
        let kids: Vec<String> = (0..n).map(|i| format!("{} 0 R", i + 3)).collect();
        let mut s = format!(
            "%PDF-1.4\n1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n\
             2 0 obj\n<< /Type /Pages /Kids [{}] /Count {n} >>\nendobj\n",
            kids.join(" ")
        );
        for i in 0..n {
            s.push_str(&format!(
                "{} 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 595 842] \
                 /Resources << /Font << >> >> >>\nendobj\n",
                i + 3
            ));
        }
        s.push_str("trailer\n<< /Root 1 0 R >>\n%%EOF\n");
        s.into_bytes()
    }

    #[test]
    fn a_page_tree_says_its_count() {
        assert_eq!(pages(&plain(1)), Some(1));
        assert_eq!(pages(&plain(37)), Some(37));
    }

    #[test]
    fn a_page_tree_in_an_object_stream_is_inflated_and_read() {
        // The objects as a PDF 1.5 writer packs them: the catalogue, the page
        // tree and its pages in one compressed stream.
        let objects = b"1 0 2 52 3 120 << /Type /Catalog /Pages 2 0 R >> \
            << /Type /Pages /Kids [3 0 R] /Count 214 >> \
            << /Type /Page /Parent 2 0 R >>";
        let mut z = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        z.write_all(objects).unwrap();
        let packed = z.finish().unwrap();
        let mut pdf = format!(
            "%PDF-1.7\n%\u{e2}\u{e3}\n4 0 obj\n<< /Type /ObjStm /N 3 /First 14 /Filter /FlateDecode /Length {} >>\nstream\r\n",
            packed.len()
        )
        .into_bytes();
        pdf.extend_from_slice(&packed);
        pdf.extend_from_slice(b"\r\nendstream\nendobj\nstartxref\n0\n%%EOF\n");
        assert_eq!(pages(&pdf), Some(214));
    }

    #[test]
    fn without_a_count_the_pages_are_counted() {
        let pdf = b"%PDF-1.3\n3 0 obj\n<< /Type /Page >>\nendobj\n4 0 obj\n<</Type/Page/Parent 2 0 R>>\nendobj\n";
        assert_eq!(pages(pdf), Some(2));
    }

    #[test]
    fn a_count_given_by_reference_is_not_taken_for_one() {
        let pdf = b"%PDF-1.4\n2 0 obj\n<< /Type /Pages /Count 9 0 R /Kids [] >>\nendobj\n\
            3 0 obj\n<< /Type /Page >>\nendobj\n";
        assert_eq!(pages(pdf), Some(1), "the page counted, not object 9");
    }

    #[test]
    fn what_is_not_a_pdf_has_no_pages() {
        assert_eq!(pages(b"GIF89a"), None);
        assert_eq!(pages(b""), None);
        assert_eq!(pages(b"%PDF-1.4\nnothing to read\n%%EOF"), None);
    }
}
