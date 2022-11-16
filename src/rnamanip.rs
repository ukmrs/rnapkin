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
            'U' => Nucleotide::U, // TODO I could allow 'U'|'T' => Nucleotide::U but should I?
            _ => return None,
        };

        Some(nt)
    }
}

impl Default for Nucleotide {
    fn default() -> Self {
        Self::X
    }
}

// TODO future dumb hack to draw without seq
// idk if its a good idea yet but it exists
#[allow(dead_code)]
pub struct InfiniteXSource;

impl std::ops::Index<usize> for InfiniteXSource {
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

    assert!(deck.is_empty());
    lovers
}
