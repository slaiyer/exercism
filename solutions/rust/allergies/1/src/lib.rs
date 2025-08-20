use std::collections::BTreeSet;
use strum::{EnumIter, IntoEnumIterator};

pub struct Allergies {
    known: BTreeSet<Allergen>,
}

#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Allergen {
    Eggs = 1 << 0,
    Peanuts = 1 << 1,
    Shellfish = 1 << 2,
    Strawberries = 1 << 3,
    Tomatoes = 1 << 4,
    Chocolate = 1 << 5,
    Pollen = 1 << 6,
    Cats = 1 << 7,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self {
            known: Allergen::iter()
                .filter(|&allergen| score & allergen as u32 == allergen as u32)
                .collect(),
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.known.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.known.iter().copied().collect()
    }
}
