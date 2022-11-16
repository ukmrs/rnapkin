use rnapkin::forest::{grow_tree, DotBracket, Tree};
use rnapkin::rnamanip::get_pair_list;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

const PAIR_SET: &str = "tests/pair_seq_set";

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[test]
fn pair_list_creation() {
    let mut lines = read_lines(PAIR_SET).expect("couldn't open pair_set for tests");
    loop {
        let structure = match lines.next() {
            Some(line) => line.unwrap(),
            None => break,
        };

        let pair_list: Vec<_> = lines
            .next()
            .expect("no matching pair_table")
            .unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().ok())
            .collect();

        let rnapking_pair_list = get_pair_list(&structure);
        assert_eq!(rnapking_pair_list, pair_list)
    }
}

/// manually makes small tree structure
/// of the following small hairpin
/// ACGCUGCUUAGCUA
/// ..(((....)))..
fn make_simple_tree() -> Tree<DotBracket> {
    let mut tree: Tree<DotBracket> = Tree::default();
    let root = tree.sprout(DotBracket::new_loop());

    let external_loop: Vec<DotBracket> =
        [(0, None), (1, None), (2, Some(11)), (12, None), (13, None)]
            .into_iter()
            .map(|(a, b)| DotBracket::new(Some(a), b))
            .collect();

    let internal_loop: Vec<DotBracket> = [(5, None), (6, None), (7, None), (8, None)]
        .into_iter()
        .map(|(a, b)| DotBracket::new(Some(a), b))
        .collect();

    for node in external_loop {
        let ix = tree.sprout(node);
        tree[root].push(ix);
    }

    let three = tree.sprout(DotBracket::new(Some(3), Some(10)));
    tree[3].push(three);
    let four = tree.sprout(DotBracket::new(Some(4), Some(9)));
    tree[three].push(four);
    let new_loop = tree.sprout(DotBracket::new_loop());
    tree[four].push(new_loop);

    for node in internal_loop {
        let ix = tree.sprout(node);
        tree[new_loop].push(ix);
    }

    tree
}

fn compare_trees<P: AsRef<Path>>(tree_file: P, tree: Tree<DotBracket>) {
    let mut lines = read_lines(tree_file).expect("failed to open tree_file :c");
    let mut itertree = tree.iter();

    while let Some(Ok(value_line)) = lines.next() {
        let treenode = &tree[itertree.next().expect("tree is too short")];

        let mut vals = value_line.split(",").map(|x| x.parse::<usize>().ok());
        let db = DotBracket::new(vals.next().unwrap(), vals.next().unwrap());
        assert_eq!(db, treenode.val);

        let offspring_line = lines.next().expect("offspring line not there").unwrap();
        let mut offspring = offspring_line.split(",").map(|x| x.parse::<usize>().ok());

        for kid in &treenode.children {
            let db = DotBracket::new(offspring.next().unwrap(), offspring.next().unwrap());
            assert_eq!(tree[*kid].val, db);
        }

        // check if the offspring in the test tree was properly exhausted
        // if there were no kids first next() won't be None but ""
        assert!(offspring.next().is_none() || offspring.next().is_none());
    }
}

#[test]
fn tree_parsing_test() {
    let tree = make_simple_tree();
    compare_trees("tests/extras/fribo_tree", tree);
}

fn get_set_of_testfilses() -> Vec<Vec<PathBuf>> {
    let mut test_set: Vec<Vec<PathBuf>> = vec![];
    let trees_paths = Path::new("tests/sample_rnas/")
        .read_dir()
        .expect("oof")
        .map(|x| x.unwrap().path().read_dir().unwrap());

    for direcory in trees_paths {
        let mut file_set: Vec<PathBuf> = vec![PathBuf::default(), PathBuf::default()];
        for file in direcory.map(|x| x.unwrap().path()) {
            if file.ends_with("tree") {
                file_set[1] = file.to_owned();
            } else if file.ends_with("seq_struct_pair") {
                file_set[0] = file.to_owned();
            }
        }
        test_set.push(file_set);
    }
    test_set
}

#[test]
fn tree_creation_test() {
    let testfiles = get_set_of_testfilses();
    for rna_case in &testfiles {
        let lines = read_lines(&rna_case[0]).unwrap();
        let pair_list = lines.skip(2).next().unwrap().unwrap();
        let pair_list: Vec<Option<usize>> = pair_list
            .split(",")
            .map(|x| x.parse::<usize>().ok())
            .collect();
        let tree = grow_tree(&pair_list);
        compare_trees(&rna_case[1], tree);
    }
}
