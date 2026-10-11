//! Comparing text the way a reader means it.
//!
//! "Łodź" and "Łódź" are one city typed two ways, "Wierzbięta" and
//! "Wierzbieta" one family, and "anna" is what somebody types for "Anna".
//! [`fold`] is the one place that decides what counts as the same: the
//! masthead search matches through it, and the data-quality screen finds
//! near-duplicate place names with it.

use unicode_normalization::char::is_combining_mark;
use unicode_normalization::UnicodeNormalization;

/// Lowercase, compatibility-decomposed, accents removed, whitespace collapsed.
///
/// Decomposition takes the accents off most letters (ó → o + ́); the few
/// Latin letters that are distinct letters rather than accented ones — ł, đ,
/// ø, ß, æ, œ, ı — are mapped by hand, as a reader typing without them would.
pub fn fold(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut space = false;
    for c in s.nfkd() {
        if is_combining_mark(c) {
            continue;
        }
        if c.is_whitespace() {
            space = !out.is_empty();
            continue;
        }
        if space {
            out.push(' ');
            space = false;
        }
        for l in c.to_lowercase() {
            match l {
                'ł' => out.push('l'),
                'đ' => out.push('d'),
                'ø' => out.push('o'),
                'ß' => out.push_str("ss"),
                'æ' => out.push_str("ae"),
                'œ' => out.push_str("oe"),
                'ı' => out.push('i'),
                _ => out.push(l),
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::fold;

    #[test]
    fn one_city_typed_two_ways_folds_to_one() {
        assert_eq!(fold("Łodź"), fold("Łódź"));
        assert_eq!(fold("Łódź"), "lodz");
        assert_eq!(fold("  Wierzbięta  Jan "), "wierzbieta jan");
        assert_eq!(fold("Straße"), "strasse");
        assert_eq!(fold("山田花子"), "山田花子");
    }
}
