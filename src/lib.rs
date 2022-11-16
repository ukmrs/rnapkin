pub mod draw;
pub mod forest;
pub mod rnamanip;

#[allow(dead_code)]
const SEQ: &str = "CGCUUCAUAUAAUCCUAAUGAUAUGGUUUGGGAGUUUCUACCAAGAGCCUUAAACUCUUGAUUAUGAAGUG";
#[allow(dead_code)]
const SST: &str = "...(((((((..((((((.........))))))......).((((((.......))))))..))))))...";

pub fn run() {
    let pl = rnamanip::get_pair_list(SST);
    let tree = forest::grow_tree(&pl);
    for idx in tree.iter() {
        if !tree[idx].children.is_empty() {
            let node = &tree[idx];
            println!("{:?}", node);
            println!("{:?}", node.children);
        }
    }
}
