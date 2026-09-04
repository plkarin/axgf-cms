//! Reading a position out of whatever the reader had in their clipboard.
//!
//! # Why this is the important half
//!
//! [`crate::geocode`] can look a name up, and on this bundle's names it mostly
//! cannot: of ten of the operator's places put through the live service, five
//! returned nothing and two returned the wrong kind of object. A village
//! recorded under Russian, Prussian or Austrian administration is generally not
//! findable under a modern search. So the position arrives by hand, and "by
//! hand" in practice means the reader found the spot in some other map and
//! copied something out of it.
//!
//! What they copied is not two decimal numbers in the order this form wants
//! them. It is a Google Maps URL, or an OpenStreetMap permalink, or a pair of
//! degrees-minutes-seconds readings off a scanned military map, or a `geo:`
//! URI from a phone. Making the reader retype any of those into two boxes is
//! asking them to be a parser, badly, at the exact moment they are most likely
//! to transpose a digit.
//!
//! Everything here is pure and server-side, so it works with scripting off,
//! and every format below is covered by a test.

/// A position parsed out of pasted text.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub lat: f64,
    pub lon: f64,
}

impl Position {
    /// Rendered the way the coordinate fields hold it.
    ///
    /// Seven decimal places is about 11 mm, which is past what any genealogical
    /// source supports and well short of what an `f64` will print if asked, so
    /// it is a round number of digits rather than a claim. Trailing zeros go,
    /// because `52.1000000` reads as a precision nobody asserted.
    pub fn format(self) -> (String, String) {
        (trim_zeros(self.lat), trim_zeros(self.lon))
    }
}

fn trim_zeros(v: f64) -> String {
    let s = format!("{v:.7}");
    let s = s.trim_end_matches('0').trim_end_matches('.');
    s.to_string()
}

/// Parse a pasted position, or return `None`.
///
/// Deliberately strict about the *result* and generous about the input: a
/// value outside the range of the earth is rejected rather than clamped,
/// because a clamped coordinate is a wrong answer that looks like a right one.
pub fn parse(input: &str) -> Option<Position> {
    let text = input.trim();
    if text.is_empty() {
        return None;
    }

    // A URL carries its position in a known place, and its other numbers —
    // zoom levels, place ids, altitudes — would otherwise be mistaken for one.
    if let Some(p) = from_url(text) {
        return check(p);
    }
    if let Some(rest) = text.strip_prefix("geo:") {
        let head = rest.split(&[';', '?'][..]).next().unwrap_or(rest);
        if let Some(p) = two_numbers(head) {
            return check(p);
        }
    }
    if let Some(p) = from_dms(text) {
        return check(p);
    }
    two_numbers(text).and_then(check)
}

fn check(p: Position) -> Option<Position> {
    ((-90.0..=90.0).contains(&p.lat) && (-180.0..=180.0).contains(&p.lon)).then_some(p)
}

/// Pull the position out of a map URL.
///
/// Three shapes cover almost everything a reader will paste: Google's
/// `@lat,lon,zoom`, a `q=`/`ll=`/`mlat=`&`mlon=` query parameter, and
/// OpenStreetMap's `#map=zoom/lat/lon` fragment. Anything else falls through
/// to the plain-number path, which is correct for a bare pair and returns
/// `None` for a URL full of unrelated digits.
fn from_url(text: &str) -> Option<Position> {
    if !(text.starts_with("http://") || text.starts_with("https://")) {
        return None;
    }

    // Google: /maps/@52.0782795,21.2508068,15z — the zoom follows the pair and
    // must not be read as a third coordinate.
    if let Some(at) = text.split("/@").nth(1) {
        let mut parts = at.split(',');
        if let (Some(a), Some(b)) = (parts.next(), parts.next()) {
            if let (Ok(lat), Ok(lon)) = (a.trim().parse(), trim_trailing(b).parse()) {
                return Some(Position { lat, lon });
            }
        }
    }

    // mlat=&mlon= is OSM's marker pair, and is two parameters rather than one.
    // It is read before the `#map=` fragment because the two disagree: the
    // fragment holds wherever the view happens to be centred and rounded to
    // the zoom, the marker holds the point the reader actually chose.
    let q = text.split(['?', '#']).nth(1).unwrap_or("");
    let param = |name: &str| {
        q.split('&')
            .find_map(|kv| kv.strip_prefix(name).and_then(|v| v.strip_prefix('=')))
    };
    if let (Some(a), Some(b)) = (param("mlat"), param("mlon")) {
        if let (Ok(lat), Ok(lon)) = (a.parse(), b.parse()) {
            return Some(Position { lat, lon });
        }
    }
    for name in ["q", "ll", "center", "sll"] {
        if let Some(v) = param(name) {
            let v = v.replace("%2C", ",").replace("%2c", ",");
            if let Some(p) = two_numbers(&v) {
                return Some(p);
            }
        }
    }

    // OpenStreetMap: #map=15/52.0782795/21.2508068 — zoom first, so the pair
    // has to be taken after it rather than by reading the first two numbers.
    if let Some(frag) = text.split("#map=").nth(1) {
        let mut parts = frag.split('/');
        let _zoom = parts.next();
        if let (Some(a), Some(b)) = (parts.next(), parts.next()) {
            if let (Ok(lat), Ok(lon)) = (a.trim().parse(), trim_trailing(b).parse()) {
                return Some(Position { lat, lon });
            }
        }
    }
    None
}

/// Trim anything after a number that is not part of it — Google's `,15z`
/// zoom suffix arrives glued to the longitude.
fn trim_trailing(s: &str) -> &str {
    let s = s.trim();
    let end = s
        .find(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-' || c == '+'))
        .unwrap_or(s.len());
    &s[..end]
}

/// Degrees, minutes and seconds, in the shapes a scanned map and a phone both
/// produce: `52°04'41.8"N 21°15'02.9"E`, `N 52 4 41.8, E 21 15 2.9`.
///
/// The hemisphere letter is what makes this unambiguous, so a string without
/// one is not treated as DMS at all — `52 21` is two decimal degrees, not
/// 52 degrees 21 minutes of a longitude that was never given.
fn from_dms(text: &str) -> Option<Position> {
    let upper = text.to_uppercase();
    if !upper.contains(['N', 'S']) || !upper.contains(['E', 'W']) {
        return None;
    }
    // Anything that is not a digit, a sign, a separator or a hemisphere letter
    // becomes a space, which folds °, ', ", ´, ’ and friends into one rule.
    let cleaned: String = upper
        .chars()
        .map(|c| {
            // Digits, signs and the four hemisphere letters survive; degree
            // marks, primes, commas and everything else become separators.
            if c.is_ascii_digit() || matches!(c, '.' | '-' | '+' | 'N' | 'S' | 'E' | 'W') {
                c
            } else {
                ' '
            }
        })
        .collect();

    // Split into the two halves at the hemisphere letters. The letter may lead
    // its numbers (`N 52 4 41.8`, `N52 4 41.8`) or trail them (`52 4 41.8N`),
    // and both shapes turn up — the first off a phone, the second off a map
    // legend. A leading letter cannot close a group, because its numbers have
    // not arrived yet; a trailing one closes immediately.
    let mut halves: Vec<(char, Vec<f64>)> = Vec::new();
    let mut pending: Vec<f64> = Vec::new();
    let mut held: Option<char> = None;
    for tok in cleaned.split_whitespace() {
        let (letter, digits, leading) = split_letter(tok);
        if leading {
            if let Some(l) = letter {
                match (pending.is_empty(), held) {
                    // Numbers are waiting and nothing claimed them: this
                    // letter is trailing them after all, across a space.
                    (false, None) => halves.push((l, std::mem::take(&mut pending))),
                    // Numbers are waiting for the letter we are holding, and
                    // this one opens the next half.
                    (false, Some(h)) => {
                        halves.push((h, std::mem::take(&mut pending)));
                        held = Some(l);
                    }
                    (true, _) => held = Some(l),
                }
            }
        }
        if !digits.is_empty() {
            if let Ok(v) = digits.parse::<f64>() {
                pending.push(v);
            }
        }
        if !leading {
            if let Some(l) = letter {
                halves.push((l, std::mem::take(&mut pending)));
                held = None;
            }
        }
    }
    if let (Some(h), false) = (held, pending.is_empty()) {
        halves.push((h, pending));
    }
    if halves.len() != 2 {
        return None;
    }

    let value = |nums: &[f64]| -> Option<f64> {
        let d = *nums.first()?;
        let m = nums.get(1).copied().unwrap_or(0.0);
        let s = nums.get(2).copied().unwrap_or(0.0);
        // Minutes and seconds are unsigned magnitudes; only the degrees or the
        // hemisphere letter carries direction.
        Some(d.abs() + m / 60.0 + s / 3600.0)
    };

    let mut lat = None;
    let mut lon = None;
    for (letter, nums) in &halves {
        let v = value(nums)?;
        match letter {
            'N' => lat = Some(v),
            'S' => lat = Some(-v),
            'E' => lon = Some(v),
            'W' => lon = Some(-v),
            _ => return None,
        }
    }
    Some(Position {
        lat: lat?,
        lon: lon?,
    })
}

/// Peel a hemisphere letter off one token, saying which end it came from.
fn split_letter(tok: &str) -> (Option<char>, &str, bool) {
    let first = tok.chars().next();
    if matches!(first, Some('N' | 'S' | 'E' | 'W')) {
        return (first, &tok[1..], true);
    }
    let last = tok.chars().last();
    if matches!(last, Some('N' | 'S' | 'E' | 'W')) {
        return (last, &tok[..tok.len() - 1], false);
    }
    (None, tok, false)
}

/// The plain case: exactly two numbers, separated by a comma, whitespace or a
/// semicolon.
///
/// Exactly two, not "the first two". A string with three numbers in it is
/// something this function has misunderstood, and guessing which two were
/// meant is how a zoom level ends up stored as a longitude.
fn two_numbers(text: &str) -> Option<Position> {
    let nums: Vec<f64> = text
        .split(|c: char| c == ',' || c == ';' || c.is_whitespace())
        .map(str::trim)
        .filter(|t| !t.is_empty())
        .map(|t| t.parse::<f64>())
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
    match nums.as_slice() {
        [lat, lon] => Some(Position {
            lat: *lat,
            lon: *lon,
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(s: &str) -> Position {
        parse(s).unwrap_or_else(|| panic!("{s:?} should parse"))
    }

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn a_plain_pair_in_the_obvious_shapes() {
        for s in [
            "52.0782795, 21.2508068",
            "52.0782795,21.2508068",
            "52.0782795 21.2508068",
            "  52.0782795 ; 21.2508068  ",
        ] {
            let q = p(s);
            assert!(close(q.lat, 52.0782795) && close(q.lon, 21.2508068), "{s}");
        }
    }

    #[test]
    fn a_third_number_means_this_was_misread_rather_than_guessed_at() {
        // A zoom level, an altitude, a place id. Taking the first two would
        // store something plausible and wrong.
        assert!(parse("52.07, 21.25, 15").is_none());
        assert!(parse("1 2 3 4").is_none());
    }

    #[test]
    fn a_google_maps_url() {
        let q = p("https://www.google.com/maps/@52.0782795,21.2508068,15z");
        assert!(close(q.lat, 52.0782795) && close(q.lon, 21.2508068));
        // The zoom suffix is glued to the longitude and must not survive.
        assert!(!q.lon.to_string().contains('z'));
        let q = p("https://maps.google.com/?q=52.0782795,21.2508068");
        assert!(close(q.lat, 52.0782795));
        let q = p("https://maps.google.com/?q=52.0782795%2C21.2508068");
        assert!(
            close(q.lon, 21.2508068),
            "an encoded comma is still a comma"
        );
    }

    #[test]
    fn an_openstreetmap_permalink() {
        // The fragment puts the zoom first, so reading the first two numbers
        // would give a latitude of 15.
        let q = p("https://www.openstreetmap.org/#map=15/52.0782795/21.2508068");
        assert!(close(q.lat, 52.0782795) && close(q.lon, 21.2508068));
        let q =
            p("https://www.openstreetmap.org/?mlat=52.0782795&mlon=21.2508068#map=15/52.07/21.25");
        assert!(close(q.lat, 52.0782795) && close(q.lon, 21.2508068));
    }

    #[test]
    fn a_geo_uri_from_a_phone() {
        let q = p("geo:52.0782795,21.2508068");
        assert!(close(q.lat, 52.0782795));
        let q = p("geo:52.0782795,21.2508068;u=35");
        assert!(
            close(q.lon, 21.2508068),
            "the accuracy parameter is not a third coordinate"
        );
    }

    #[test]
    fn degrees_minutes_and_seconds_off_a_scanned_map() {
        let q = p("52°04'41.8\"N 21°15'02.9\"E");
        assert!(close(q.lat, 52.078278), "{}", q.lat);
        assert!(close(q.lon, 21.250806), "{}", q.lon);
        // Letter first, spaces instead of symbols.
        let q = p("N 52 4 41.8, E 21 15 2.9");
        assert!(close(q.lat, 52.078278) && close(q.lon, 21.250806));
        // Degrees and minutes with no seconds.
        let q = p("52 4.7N 21 15.05E");
        assert!(close(q.lat, 52.0783333), "{}", q.lat);
    }

    #[test]
    fn the_southern_and_western_hemispheres_are_negative() {
        let q = p("33°51'54\"S 151°12'36\"E");
        assert!(q.lat < 0.0 && q.lon > 0.0, "{q:?}");
        let q = p("40°42'46\"N 74°00'22\"W");
        assert!(q.lat > 0.0 && q.lon < 0.0, "{q:?}");
    }

    #[test]
    fn a_bare_pair_is_not_read_as_degrees_and_minutes() {
        // Without a hemisphere letter "52 21" is two decimal degrees. Reading
        // it as 52°21' would silently move the point 21 minutes north.
        let q = p("52 21");
        assert!(close(q.lat, 52.0) && close(q.lon, 21.0));
    }

    #[test]
    fn a_position_off_the_earth_is_refused_rather_than_clamped() {
        // A clamped coordinate is a wrong answer wearing the costume of a
        // right one. Far better to say the paste was not understood.
        assert!(parse("91, 0").is_none());
        assert!(parse("0, 181").is_none());
        assert!(parse("-90.0001, 0").is_none());
        assert!(parse("-90, 180").is_some(), "the corners are on the earth");
    }

    #[test]
    fn nonsense_is_none_rather_than_a_guess() {
        for s in [
            "",
            "   ",
            "Słomniki",
            "własciciel majatku Niezgórze",
            "https://example.org/a/page",
            "52.0782795",
        ] {
            assert!(parse(s).is_none(), "{s:?} should not parse");
        }
    }

    #[test]
    fn formatting_does_not_invent_precision() {
        let (lat, lon) = Position {
            lat: 52.1,
            lon: -21.0,
        }
        .format();
        assert_eq!(lat, "52.1");
        assert_eq!(lon, "-21");
    }
}
