use std::ffi::OsStr;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use rnapkin::utils::ParsedInput;

/// rnapkin: plotting utility for secondary RNA structure
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// file containing secondary_structure and sequence
    input: String,

    /// output file; supported extensions .svg and .png
    #[arg(short, long)]
    output: Option<String>,

    /// color theme; dark, bright, white/w, black/b
    #[arg(short, long, default_value = "dark")]
    theme: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let pi = ParsedInput::from_file(&args.input)?;

    println!("{:#?}", pi);

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

    // match (pi.secondary_structure, pi.sequence) {
    //     (None, Some(_)) => unimplemented!(
    //         "Calling external soft like RNAFold to get secondary_structure not yet implemented"
    //     ),
    //     (None, None) => panic!("Neither sequence nor secondary structure found in the input file!"),
    //     (Some(sst), Some(sequence)) => (0, Box::new(InfiniteXSource)),
    //     (Some(sst), None) => (0, Box::new(InfiniteXSource)),
    // };

    Ok(())
}
