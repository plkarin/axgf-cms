//! The pictures a personal assistant's project or conversation may take as
//! its avatar (the operator's request of 3 November 2026, docs/adr/0026):
//! made for this site with Canva, four for each of the domains of a family's
//! life below, 192 pixels square. Written by the pack's build script from
//! `static/avatars/`; a picture of the account's own is kept in its space
//! instead ([`crate::assistant`]).

/// One avatar of the pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct Avatar {
    /// Its name, which a project or a conversation keeps.
    pub name: &'static str,
    /// The domain it stands for, which names it on the page.
    pub domain: &'static str,
    #[serde(skip)]
    pub bytes: &'static [u8],
}

/// The domains, in the order the picker shows them.
pub const DOMAINS: &[&str] = &[
    "genealogy",
    "medicine",
    "work",
    "money",
    "law",
    "learning",
    "home",
    "cooking",
    "travel",
    "nature",
    "wellbeing",
    "family",
    "arts",
    "sports",
    "tech",
    "science",
    "history",
    "admin",
];

/// Every avatar, its domain's four together.
pub const PACK: &[Avatar] = &[
    Avatar {
        name: "genealogy-tree",
        domain: "genealogy",
        bytes: include_bytes!("../static/avatars/genealogy-tree.webp"),
    },
    Avatar {
        name: "genealogy-scroll",
        domain: "genealogy",
        bytes: include_bytes!("../static/avatars/genealogy-scroll.webp"),
    },
    Avatar {
        name: "genealogy-portrait",
        domain: "genealogy",
        bytes: include_bytes!("../static/avatars/genealogy-portrait.webp"),
    },
    Avatar {
        name: "genealogy-shield",
        domain: "genealogy",
        bytes: include_bytes!("../static/avatars/genealogy-shield.webp"),
    },
    Avatar {
        name: "medicine-stethoscope",
        domain: "medicine",
        bytes: include_bytes!("../static/avatars/medicine-stethoscope.webp"),
    },
    Avatar {
        name: "medicine-heart",
        domain: "medicine",
        bytes: include_bytes!("../static/avatars/medicine-heart.webp"),
    },
    Avatar {
        name: "medicine-capsule",
        domain: "medicine",
        bytes: include_bytes!("../static/avatars/medicine-capsule.webp"),
    },
    Avatar {
        name: "medicine-kit",
        domain: "medicine",
        bytes: include_bytes!("../static/avatars/medicine-kit.webp"),
    },
    Avatar {
        name: "work-briefcase",
        domain: "work",
        bytes: include_bytes!("../static/avatars/work-briefcase.webp"),
    },
    Avatar {
        name: "work-handshake",
        domain: "work",
        bytes: include_bytes!("../static/avatars/work-handshake.webp"),
    },
    Avatar {
        name: "work-resume",
        domain: "work",
        bytes: include_bytes!("../static/avatars/work-resume.webp"),
    },
    Avatar {
        name: "work-building",
        domain: "work",
        bytes: include_bytes!("../static/avatars/work-building.webp"),
    },
    Avatar {
        name: "money-coins",
        domain: "money",
        bytes: include_bytes!("../static/avatars/money-coins.webp"),
    },
    Avatar {
        name: "money-piggy",
        domain: "money",
        bytes: include_bytes!("../static/avatars/money-piggy.webp"),
    },
    Avatar {
        name: "money-chart",
        domain: "money",
        bytes: include_bytes!("../static/avatars/money-chart.webp"),
    },
    Avatar {
        name: "money-wallet",
        domain: "money",
        bytes: include_bytes!("../static/avatars/money-wallet.webp"),
    },
    Avatar {
        name: "law-scales",
        domain: "law",
        bytes: include_bytes!("../static/avatars/law-scales.webp"),
    },
    Avatar {
        name: "law-gavel",
        domain: "law",
        bytes: include_bytes!("../static/avatars/law-gavel.webp"),
    },
    Avatar {
        name: "law-seal",
        domain: "law",
        bytes: include_bytes!("../static/avatars/law-seal.webp"),
    },
    Avatar {
        name: "law-pen",
        domain: "law",
        bytes: include_bytes!("../static/avatars/law-pen.webp"),
    },
    Avatar {
        name: "learning-cap",
        domain: "learning",
        bytes: include_bytes!("../static/avatars/learning-cap.webp"),
    },
    Avatar {
        name: "learning-book",
        domain: "learning",
        bytes: include_bytes!("../static/avatars/learning-book.webp"),
    },
    Avatar {
        name: "learning-pencil",
        domain: "learning",
        bytes: include_bytes!("../static/avatars/learning-pencil.webp"),
    },
    Avatar {
        name: "learning-bag",
        domain: "learning",
        bytes: include_bytes!("../static/avatars/learning-bag.webp"),
    },
    Avatar {
        name: "home-house",
        domain: "home",
        bytes: include_bytes!("../static/avatars/home-house.webp"),
    },
    Avatar {
        name: "home-key",
        domain: "home",
        bytes: include_bytes!("../static/avatars/home-key.webp"),
    },
    Avatar {
        name: "home-armchair",
        domain: "home",
        bytes: include_bytes!("../static/avatars/home-armchair.webp"),
    },
    Avatar {
        name: "home-tools",
        domain: "home",
        bytes: include_bytes!("../static/avatars/home-tools.webp"),
    },
    Avatar {
        name: "cooking-hat",
        domain: "cooking",
        bytes: include_bytes!("../static/avatars/cooking-hat.webp"),
    },
    Avatar {
        name: "cooking-pot",
        domain: "cooking",
        bytes: include_bytes!("../static/avatars/cooking-pot.webp"),
    },
    Avatar {
        name: "cooking-bread",
        domain: "cooking",
        bytes: include_bytes!("../static/avatars/cooking-bread.webp"),
    },
    Avatar {
        name: "cooking-whisk",
        domain: "cooking",
        bytes: include_bytes!("../static/avatars/cooking-whisk.webp"),
    },
    Avatar {
        name: "travel-plane",
        domain: "travel",
        bytes: include_bytes!("../static/avatars/travel-plane.webp"),
    },
    Avatar {
        name: "travel-suitcase",
        domain: "travel",
        bytes: include_bytes!("../static/avatars/travel-suitcase.webp"),
    },
    Avatar {
        name: "travel-pin",
        domain: "travel",
        bytes: include_bytes!("../static/avatars/travel-pin.webp"),
    },
    Avatar {
        name: "travel-compass",
        domain: "travel",
        bytes: include_bytes!("../static/avatars/travel-compass.webp"),
    },
    Avatar {
        name: "nature-leaf",
        domain: "nature",
        bytes: include_bytes!("../static/avatars/nature-leaf.webp"),
    },
    Avatar {
        name: "nature-plant",
        domain: "nature",
        bytes: include_bytes!("../static/avatars/nature-plant.webp"),
    },
    Avatar {
        name: "nature-can",
        domain: "nature",
        bytes: include_bytes!("../static/avatars/nature-can.webp"),
    },
    Avatar {
        name: "nature-mountain",
        domain: "nature",
        bytes: include_bytes!("../static/avatars/nature-mountain.webp"),
    },
    Avatar {
        name: "wellbeing-lotus",
        domain: "wellbeing",
        bytes: include_bytes!("../static/avatars/wellbeing-lotus.webp"),
    },
    Avatar {
        name: "wellbeing-shoe",
        domain: "wellbeing",
        bytes: include_bytes!("../static/avatars/wellbeing-shoe.webp"),
    },
    Avatar {
        name: "wellbeing-moon",
        domain: "wellbeing",
        bytes: include_bytes!("../static/avatars/wellbeing-moon.webp"),
    },
    Avatar {
        name: "wellbeing-apple",
        domain: "wellbeing",
        bytes: include_bytes!("../static/avatars/wellbeing-apple.webp"),
    },
    Avatar {
        name: "family-rings",
        domain: "family",
        bytes: include_bytes!("../static/avatars/family-rings.webp"),
    },
    Avatar {
        name: "family-stroller",
        domain: "family",
        bytes: include_bytes!("../static/avatars/family-stroller.webp"),
    },
    Avatar {
        name: "family-hands",
        domain: "family",
        bytes: include_bytes!("../static/avatars/family-hands.webp"),
    },
    Avatar {
        name: "family-group",
        domain: "family",
        bytes: include_bytes!("../static/avatars/family-group.webp"),
    },
    Avatar {
        name: "arts-palette",
        domain: "arts",
        bytes: include_bytes!("../static/avatars/arts-palette.webp"),
    },
    Avatar {
        name: "arts-masks",
        domain: "arts",
        bytes: include_bytes!("../static/avatars/arts-masks.webp"),
    },
    Avatar {
        name: "arts-music",
        domain: "arts",
        bytes: include_bytes!("../static/avatars/arts-music.webp"),
    },
    Avatar {
        name: "arts-camera",
        domain: "arts",
        bytes: include_bytes!("../static/avatars/arts-camera.webp"),
    },
    Avatar {
        name: "sports-ball",
        domain: "sports",
        bytes: include_bytes!("../static/avatars/sports-ball.webp"),
    },
    Avatar {
        name: "sports-bike",
        domain: "sports",
        bytes: include_bytes!("../static/avatars/sports-bike.webp"),
    },
    Avatar {
        name: "sports-trophy",
        domain: "sports",
        bytes: include_bytes!("../static/avatars/sports-trophy.webp"),
    },
    Avatar {
        name: "sports-racket",
        domain: "sports",
        bytes: include_bytes!("../static/avatars/sports-racket.webp"),
    },
    Avatar {
        name: "tech-laptop",
        domain: "tech",
        bytes: include_bytes!("../static/avatars/tech-laptop.webp"),
    },
    Avatar {
        name: "tech-phone",
        domain: "tech",
        bytes: include_bytes!("../static/avatars/tech-phone.webp"),
    },
    Avatar {
        name: "tech-chip",
        domain: "tech",
        bytes: include_bytes!("../static/avatars/tech-chip.webp"),
    },
    Avatar {
        name: "tech-robot",
        domain: "tech",
        bytes: include_bytes!("../static/avatars/tech-robot.webp"),
    },
    Avatar {
        name: "science-atom",
        domain: "science",
        bytes: include_bytes!("../static/avatars/science-atom.webp"),
    },
    Avatar {
        name: "science-microscope",
        domain: "science",
        bytes: include_bytes!("../static/avatars/science-microscope.webp"),
    },
    Avatar {
        name: "science-telescope",
        domain: "science",
        bytes: include_bytes!("../static/avatars/science-telescope.webp"),
    },
    Avatar {
        name: "science-flask",
        domain: "science",
        bytes: include_bytes!("../static/avatars/science-flask.webp"),
    },
    Avatar {
        name: "history-hourglass",
        domain: "history",
        bytes: include_bytes!("../static/avatars/history-hourglass.webp"),
    },
    Avatar {
        name: "history-castle",
        domain: "history",
        bytes: include_bytes!("../static/avatars/history-castle.webp"),
    },
    Avatar {
        name: "history-watch",
        domain: "history",
        bytes: include_bytes!("../static/avatars/history-watch.webp"),
    },
    Avatar {
        name: "history-quill",
        domain: "history",
        bytes: include_bytes!("../static/avatars/history-quill.webp"),
    },
    Avatar {
        name: "admin-envelope",
        domain: "admin",
        bytes: include_bytes!("../static/avatars/admin-envelope.webp"),
    },
    Avatar {
        name: "admin-checklist",
        domain: "admin",
        bytes: include_bytes!("../static/avatars/admin-checklist.webp"),
    },
    Avatar {
        name: "admin-calendar",
        domain: "admin",
        bytes: include_bytes!("../static/avatars/admin-calendar.webp"),
    },
    Avatar {
        name: "admin-id",
        domain: "admin",
        bytes: include_bytes!("../static/avatars/admin-id.webp"),
    },
];

/// The avatar of that name, if the pack has one.
pub fn find(name: &str) -> Option<&'static Avatar> {
    PACK.iter().find(|a| a.name == name)
}

/// The avatar an id stands for when none was chosen: always the same one
/// for the same id, and different ones across a list.
pub fn default_for(id: &str) -> &'static Avatar {
    // FNV-1a: stable across releases, unlike the standard library's hash.
    let mut h: u64 = 0xcbf29ce484222325;
    for b in id.bytes() {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x100000001b3);
    }
    &PACK[(h % PACK.len() as u64) as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_avatar_is_a_webp_of_its_domain_and_found_by_name() {
        assert!(!PACK.is_empty());
        for a in PACK {
            assert!(
                a.bytes.starts_with(b"RIFF") && &a.bytes[8..12] == b"WEBP",
                "{}",
                a.name
            );
            assert!(a.name.starts_with(&format!("{}-", a.domain)), "{}", a.name);
            assert!(DOMAINS.contains(&a.domain));
            assert_eq!(find(a.name), Some(a));
        }
        assert!(find("../app").is_none());
        assert_eq!(
            default_for("0123456789abcdef"),
            default_for("0123456789abcdef")
        );
    }
}
