pub mod draw;
pub mod forest;

use forest::{DotBracket, Tree}; // admittedly "from forest import tree" would be funnier

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

fn stem_walk(
    mut tree: Tree<DotBracket>,
    pair_list: &PairList,
    pos: usize,
    tail: usize,
) -> (Tree<DotBracket>, usize) {
    if pair_list[pos] == Some(tail) {
        let node_ix = tree.sprout(DotBracket::newsome(pos, tail));
        let (mut tree, ix) = stem_walk(tree, pair_list, pos + 1, tail - 1);
        tree[node_ix].push(ix);
        (tree, node_ix)
    } else {
        let node_ix = tree.sprout(DotBracket::new_loop());
        rna_walk(tree, pair_list, node_ix, pos, tail)
    }
}

fn rna_walk(
    mut tree: Tree<DotBracket>,
    pair_list: &PairList,
    root_ix: usize,
    pos: usize,
    tail: usize,
) -> (Tree<DotBracket>, usize) {
    let mut pos = pos;
    while pos <= tail {
        if let Some(x) = pair_list[pos] {
            let node_ix: usize;
            (tree, node_ix) = stem_walk(tree, pair_list, pos, x);
            tree[root_ix].push(node_ix);
            pos = x + 1
        } else {
            let node_ix = tree.sprout(DotBracket::new(Some(pos), None));
            tree[root_ix].push(node_ix);
            pos += 1;
        }
    }
    (tree, root_ix)
}

pub fn construct_tree(pair_list: &PairList) -> Tree<DotBracket> {
    let mut tree = Tree::default();
    let root_ix = tree.sprout(DotBracket::new_loop());
    (tree, _) = rna_walk(tree, pair_list, root_ix, 0, pair_list.len() - 1);
    tree
}
