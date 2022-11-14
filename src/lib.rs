pub mod forest;

type PairList = Vec<Option<usize>>;

#[allow(dead_code)]
const SEQ: &str = "CGCUUCAUAUAAUCCUAAUGAUAUGGUUUGGGAGUUUCUACCAAGAGCCUUAAACUCUUGAUUAUGAAGUG";
#[allow(dead_code)]
const SST: &str = "...(((((((..((((((.........))))))......).((((((.......))))))..))))))...";

pub fn run() {}

pub fn get_pair_list(secondary_structure: &str) -> PairList {
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
