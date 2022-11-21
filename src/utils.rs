use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

use anyhow::{Context, Result};

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedInput {
    pub sequence: Option<String>,
    pub secondary_structure: Option<String>,
    pub rna_name: Option<String>,
}

fn empty_then_none(s: String) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

impl ParsedInput {
    pub fn from_file(input_file: &str) -> Result<Self> {
        let mut lines = read_lines(input_file)
            .with_context(|| format!("could not read file: {}", input_file))?
            .map(|x| x.expect("invalid utf8?"));
        Self::parse(&mut lines)
    }

    fn parse<L>(lines: &mut L) -> Result<Self>
    where
        L: Iterator<Item = String>,
    {
        let mut sequence = String::with_capacity(300);
        let mut secondary_structure = String::with_capacity(300);
        let mut rna_name: Option<String> = None;

        for line in lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            match &trimmed[0..1].as_bytes()[0] {
                0x41..=0x55 | 0x61..=0x75 => sequence.push_str(trimmed), // [A-Ua-u] can catch some non nt but then the input is doomed anyway
                0x2e | 0x28 | 0x29 => secondary_structure.push_str(trimmed), // .()
                0x3e => rna_name = Some(line[1..].replace(' ', "_")),    // >
                _ => continue,
            }
        }

        Ok(ParsedInput {
            sequence: empty_then_none(sequence),
            secondary_structure: empty_then_none(secondary_structure),
            rna_name,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TENA: &str = r#">TPP_riboswitch
        GCAGAACAATTCAATATGTATTCGTTTAACCACTAGGGGTGTCCTTCATAAGGGCTGAGA
        TAAAAGTGTGACTTTTAGACCCTCATAACTTGAACAGGTTCAGACCTGCGTAGGGAAGTG
        GAGCGGTATTTGTGTTATTTTACTATGCCAATTCCAAACCACTTTTCCTTGCGGGAAAGT
        GGTTTTTTTA

        .........(((..((((((...((((((((.....((((((((((...)))))).....
        (((((((...))))))).))))(((.....)))...)))).)))).))))))..)))..(
        (((.(((((..(((......))).)))))..))))(((((((((((((....))))))))
        )))))....."#;
    const TENA_SHUFFLED: &str = r#">TPP_riboswitch
        GCAGAACAATTCAATATGTATTCGTTTAACCACTAGGGGTGTCCTTCATAAGGGCTGAGA
        .........(((..((((((...((((((((.....((((((((((...)))))).....

        TAAAAGTGTGACTTTTAGACCCTCATAACTTGAACAGGTTCAGACCTGCGTAGGGAAGTG
        (((((((...))))))).))))(((.....)))...)))).)))).))))))..)))..(

        GAGCGGTATTTGTGTTATTTTACTATGCCAATTCCAAACCACTTTTCCTTGCGGGAAAGT
        (((.(((((..(((......))).)))))..))))(((((((((((((....))))))))

        GGTTTTTTTA
        )))))....."#;

    const TENASEQ: &str = "GCAGAACAATTCAATATGTATTCGTTTAACCACTAGGGGTG\
        TCCTTCATAAGGGCTGAGATAAAAGTGTGACTTTTAGACCCTCATAACTTGAACAGGTTC\
        AGACCTGCGTAGGGAAGTGGAGCGGTATTTGTGTTATTTTACTATGCCAATTCCAAACCA\
        CTTTTCCTTGCGGGAAAGTGGTTTTTTTA";
    const TENASST: &str = ".........(((..((((((...((((((((.....(((((\
        (((((...)))))).....(((((((...))))))).))))(((.....)))...)))).\
        )))).))))))..)))..((((.(((((..(((......))).)))))..))))((((((\
        (((((((....))))))))))))).....";
    const TENANAME: &str = "TPP_riboswitch";

    fn parse_helper(test_rna: &str) -> ParsedInput {
        let mut lineiter = test_rna.split("\n").map(|x| x.to_string());
        ParsedInput::parse(&mut lineiter).expect("failed parsing input")
    }

    #[test]
    fn parse_simple_input() {
        let seq =
            "UUAUAGGCGAUGGAGUUCGCCAUAAACGCUGCUUAGCUAAUGACUCCUACCAGUAUCACUACUGGUAGGAGUCUAUUUUUUU";
        let sst =
            ".....(((((......)))))......(((....)))....((((((((((((((....)))))))))))))).........";
        let name = "super molecule";
        let name_out = "super_molecule";
        let correct_pi = ParsedInput {
            sequence: Some(seq.to_string()),
            secondary_structure: Some(sst.to_string()),
            rna_name: None,
        };

        let test_rna = format!("{}\n{}\n", seq, sst);
        let pi = parse_helper(&test_rna);
        assert_eq!(correct_pi, pi);

        let switched = format!("\n\n \t {} \t \n{}\n", sst, seq);
        let pi = parse_helper(&switched);
        assert_eq!(correct_pi, pi);

        let only_sst = format!("\n{}\n", sst);
        let pi = parse_helper(&only_sst);
        let only_sst_correct_pi = ParsedInput {
            sequence: None,
            ..correct_pi.clone()
        };
        assert_eq!(only_sst_correct_pi, pi);

        let with_name = format!("\n>{}\n\n{}\n{}\n", name, sst, seq);
        let pi = parse_helper(&with_name);
        let named_correct_pi = ParsedInput {
            rna_name: Some(name_out.to_string()),
            ..correct_pi
        };
        assert_eq!(named_correct_pi, pi);
    }

    #[test]
    fn parse_multi_line() {
        let correct_pi = ParsedInput {
            sequence: TENASEQ.to_string().into(),
            secondary_structure: TENASST.to_string().into(),
            rna_name: TENANAME.to_string().into(),
        };

        let pi = parse_helper(TENA);
        assert_eq!(correct_pi, pi);

        let pi = parse_helper(TENA_SHUFFLED);
        assert_eq!(correct_pi, pi);
    }
}
