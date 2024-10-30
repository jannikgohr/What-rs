use std::cmp::Ordering;
use crate::identifier::Match;

#[derive(Debug)]
pub enum SortKey {
    Name,
    Rarity,
    Matched,
    None
}

pub struct Sorter {
    key: SortKey,
    reverse: bool,
}

impl Sorter {
    #[inline]
    pub fn reverse(mut self, reverse: bool) -> Self {
        self.reverse = reverse;
        self
    }
    #[inline]
    pub fn key(mut self, key: &String) -> Self {
        self.key = match key.as_str() {
            "name" => SortKey::Name,
            "rarity" => SortKey::Rarity,
            "matched" => SortKey::Matched,
            "none" => SortKey::None,
            _ => {
                panic!("Unsupported sorting key: {}", key);
            }
        };
        self
    }

    #[inline]
    pub fn sort(mut self, mut matches: &mut Vec<Match>) -> Self {

        match self.key {
            SortKey::Name => {
                matches.sort_by(|a, b| {
                    reverse_conditionally(a.name.cmp(&b.name), self.reverse)
                });
            }
            SortKey::Rarity => {
                matches.sort_by(|a, b| {
                    reverse_conditionally(a.rarity.partial_cmp(&b.rarity).unwrap(), self.reverse)
                });
            }
            SortKey::Matched => {
                matches.sort_by(|a, b| {
                    reverse_conditionally(a.matched_on.cmp(&b.matched_on), self.reverse)
                });
            }
            _ => { }
        }
        self
    }
}

impl Default for Sorter {
    fn default() -> Self {
        Sorter {
            key: SortKey::None,
            reverse: false,
        }
    }
}

fn reverse_conditionally(mut order: Ordering, reversed: bool) -> Ordering {
    if reversed {
        order = order.reverse();
    }
    order
}