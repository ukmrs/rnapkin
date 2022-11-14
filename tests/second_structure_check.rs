use rnapkin;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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

        let rnapking_pair_list = rnapkin::get_pair_list(&structure);
        assert_eq!(rnapking_pair_list, pair_list)
    }
}
