pub mod draw;
pub mod forest;
pub mod rnamanip;
use rnamanip::Nucleotide;

#[allow(dead_code)]
const SEQ: &str = "CGCUUCAUAUAAUCCUAAUGAUAUGGUUUGGGAGUUUCUACCAAGAGCCUUAAACUCUUGAUUAUGAAGUG";
#[allow(dead_code)]
const SST: &str = "...(((((((..((((((.........))))))......).((((((.......))))))..))))))...";
#[allow(dead_code)]
const FSEQ: &str =
    "UUAUAGGCGAUGGAGUUCGCCAUAAACGCUGCUUAGCUAAUGACUCCUACCAGUAUCACUACUGGUAGGAGUCUAUUUUUUU";
#[allow(dead_code)]
const FSST: &str =
    ".....(((((......)))))......(((....)))....((((((((((((((....)))))))))))))).........";
#[allow(dead_code)]
const TSEQ: &str = "GCAGAACAATTCAATATGTATTCGTTTAACCACTAGGGGTGTCCTTCATAAGGGCTGAGATAAAAGTGTGACTTTTAGACCCTCATAACTTGAACAGGTTCAGACCTGCGTAGGGAAGTGGAGCGGTATTTGTGTTATTTTACTATGCCAATTCCAAACCACTTTTCCTTGCGGGAAAGTGGTTTTTTTA";
#[allow(dead_code)]
const TOFFSST: &str = ".........(((..((((((...((((((((.....((((((((((...)))))).....(((((((...))))))).))))(((.....)))...)))).)))).))))))..)))..((((.(((((..(((......))).)))))..))))(((((((((((((....))))))))))))).....";
#[allow(dead_code)]
const TONSST: &str = "...(((((((.((...)).))).))))(((((((..((((((((((...)))))).....(((((((...))))))).))))(((.....))).(((((....)))))(((((((((((((((.(((((..(((......))).)))))..))))).......))))))))))....)))))))......";

#[allow(unused_variables)]
pub fn run() {
    let pl = rnamanip::get_pair_list(TOFFSST);
    let tree = forest::grow_tree(&pl);
    let seq: Vec<Nucleotide> = TSEQ
        .chars()
        .map(|c| Nucleotide::from_char(c).expect("invalid nt!"))
        .collect();

    let bubbles = draw::gather::gather_bubbles(&tree, &seq, 0.5);

    for bbl in bubbles {
        println!("{:?}", bbl);
    }
}
