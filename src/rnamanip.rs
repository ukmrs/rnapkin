use crate::draw::colors::ColorTheme;
use plotters::style::RGBColor;
use std::fmt;

const NTA: &str = "A";
const NTG: &str = "G";
const NTC: &str = "C";
const NTU: &str = "U";
const NTX: &str = "";

#[derive(Debug, Clone, Copy)]
pub enum Nucleotide {
    A,
    U,
    G,
    C,
    // if user provides just secondary structure without seq
    // or uninitiated default nt
    X,
}

impl Nucleotide {
    pub fn from_char(c: char) -> Option<Self> {
        let nt = match c.to_ascii_uppercase() {
            'A' => Nucleotide::A,
            'C' => Nucleotide::C,
            'G' => Nucleotide::G,
            'U' | 'T' => Nucleotide::U, // TODO think about how to handle T?
            'N' => Nucleotide::X,       // TODO maybe diffrentiate between X and N
            _ => return None,
        };

        Some(nt)
    }

    pub fn extract_text_and_color(self, theme: &ColorTheme) -> (&'static str, &RGBColor) {
        match self {
            Nucleotide::A => (NTA, &theme.a),
            Nucleotide::U => (NTU, &theme.u),
            Nucleotide::G => (NTG, &theme.g),
            Nucleotide::C => (NTC, &theme.c),
            Nucleotide::X => (NTX, &theme.x),
        }
    }
}

impl Default for Nucleotide {
    fn default() -> Self {
        Self::X
    }
}

impl fmt::Display for Nucleotide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Nucleotide::A => write!(f, "A"),
            Nucleotide::C => write!(f, "C"),
            Nucleotide::G => write!(f, "G"),
            Nucleotide::U => write!(f, "U"),
            Nucleotide::X => write!(f, "N"), // TODO again what I do about that
        }
    }
}

// TODO this one while fun and something I would
// definitely write in python scares me a little bit to use:
// I would need to introduce dynamic dispatch that kind of feels bad:
// box<dyn Index<usize, Output=Nucleotide>>
/// indexed with any usize always outputs Nucleotide::X
#[allow(dead_code)]
pub struct XSequence;

impl std::ops::Index<usize> for XSequence {
    type Output = Nucleotide;
    fn index(&self, _: usize) -> &Self::Output {
        &Nucleotide::X
    }
}

pub fn get_pair_list(secondary_structure: &str) -> Vec<Option<usize>> {
    let mut lovers = vec![None; secondary_structure.len()];
    let mut deck = vec![];

    for (position, constraint) in secondary_structure.chars().enumerate() {
        if constraint == '(' {
            deck.push(position)
        } else if constraint == ')' {
            let pair = deck.pop().expect("unpaired bracket");
            lovers[position] = pair.into();
            lovers[pair] = position.into();
        }
    }

    if !deck.is_empty() {
        panic!("invalid secondary structure: unclosed bracket")
    }

    lovers
}

pub fn read_sequence(sequence: &str) -> Vec<Nucleotide> {
    sequence
        .chars()
        .map(|c| Nucleotide::from_char(c).expect("invalid nt!"))
        .collect()
}
