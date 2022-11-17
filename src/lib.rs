pub mod draw;
pub mod forest;
pub mod rnamanip;

use draw::point::Point;
use rnamanip::Nucleotide;

#[allow(dead_code)]
const SEQ: &str = "CGCUUCAUAUAAUCCUAAUGAUAUGGUUUGGGAGUUUCUACCAAGAGCCUUAAACUCUUGAUUAUGAAGUG";
#[allow(dead_code)]
const SST: &str = "...(((((((..((((((.........))))))......).((((((.......))))))..))))))...";

#[allow(unused_variables)]
pub fn run() {
    let pl = rnamanip::get_pair_list(SST);
    let tree = forest::grow_tree(&pl);
    let seq: Vec<Nucleotide> = SEQ
        .chars()
        .map(|c| Nucleotide::from_char(c).expect("invalid nt!"))
        .collect();

    draw::gather::gather_points(&tree, &seq, 0.5);
}
