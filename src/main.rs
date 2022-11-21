use anyhow::{Context, Result};
use clap::Parser;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Index;
use std::path::{Path, PathBuf};

use rnapkin::rnamanip::{InfiniteXSource, Nucleotide};

/// rnapkin: plotting utility for secondary RNA structure
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// output file; supported extensions .svg and .png
    #[arg(short, long)]
    output: Option<String>,

    /// input file; needs to have secondary structure somewhere in .() notation and optionally a sequence
    #[arg(short, long)]
    input: String,

    /// color theme; dark, bright, white/w, black/b
    #[arg(short, long, default_value = "dark")]
    theme: String,
}

fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Clone)]
struct ParsedInput {
    sequence: Option<String>,
    secondary_structure: Option<String>,
    rna_name: Option<String>,
}

impl ParsedInput {
    fn from_file(input_file: &str) -> Result<Self> {
        let mut sequence = String::with_capacity(300);
        let mut secondary_structure = String::with_capacity(300);
        let mut rna_name: Option<String> = None;
        let mut lines = read_lines(input_file)
            .with_context(|| format!("could not read file: {}", input_file))?
            .map(|x| x.expect("invalid utf8?"));

        while let Some(line) = lines.next() {
            let trimmed = line.trim();
            if trimmed.len() < 1 {
                continue;
            }
            match &trimmed[0..1].as_bytes()[0] {
                0x41..=0x55 | 0x61..=0x75 => sequence.push_str(&line), // [A-Ua-u] can catch some non nt but then the input is doomed anyway
                0x2e | 0x28 | 0x29 => secondary_structure.push_str(&line), // .()
                0x3e => rna_name = Some(line[1..].replace(" ", "_")),  // >
                _ => continue,
            }
        }

        Ok(ParsedInput {
            sequence: sequence.into(),
            secondary_structure: secondary_structure.into(),
            rna_name,
        })
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let pi = ParsedInput::from_file(&args.input)?;
    println!("{:#?}", pi);

    let (sst, seq): (usize, Box<dyn Index<usize, Output = Nucleotide>>) =
        match (pi.secondary_structure, pi.sequence) {
            (None, Some(_)) => unimplemented!(
                "Calling external soft like RNAFold to get secondary_structure not yet implemented"
            ),
            (None, None) => panic!("Nor sequence nor secondary structure found in the input file!"),
            (Some(sst), Some(sequence)) => (0, Box::new(InfiniteXSource)),
            (Some(sst), None) => (0, Box::new(InfiniteXSource)),
        };

    let mut filename: PathBuf = args
        .output
        .unwrap_or_else(|| pi.rna_name.unwrap_or_else(|| "rnaimg.svg".to_owned()))
        .into();

    match filename.extension().and_then(OsStr::to_str) {
        Some("png") => println!("png"),
        Some("svg") => println!("svg"),
        _ => {
            filename.set_extension("svg");
        }
    }

    println!("{:#?}", filename);

    Ok(())
}
