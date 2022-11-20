use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// rnapkin: plotting utility for secondary RNA structure
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// output file; supported extensions .svg and .png
    // #[arg(short, long)]
    // output: String,

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

struct ParsedInput {
    sequence: Option<String>,
    secondary_structure: Option<String>,
    rna_name: Option<String>,
}

impl ParsedInput {
    fn from_file(input_file: &str) -> Result<()> {
        let mut lines = read_lines(input_file)
            .with_context(|| format!("could not read file: {}", input_file))?
            .map(|x| x.expect("invalid utf8?"));

        let line = lines.next();
        println!("{:?}", line);
        Ok(())
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    // println!("{:?}", args.output);
    ParsedInput::from_file(&args.input)?;

    Ok(())
}
