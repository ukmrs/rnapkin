use std::ffi::OsStr;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use rnapkin::draw::{self, colors::ColorTheme};
use rnapkin::forest;
use rnapkin::rnamanip;
use rnapkin::utils::ParsedInput;

const BUBBLE_RADIUS: f64 = 0.5;

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

    /// angle in degrees
    #[arg(short, long, default_value_t = 0.)]
    angle: f64,

    /// mirror along y axis
    #[arg(long, default_value_t = false)]
    my: bool,

    /// mirror along x axis
    #[arg(long, default_value_t = false)]
    mx: bool,

    /// height in pixels, width will be an appropriate ratio of height
    #[arg(long, default_value_t = 900)]
    height: u32,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let pi = ParsedInput::from_file(&args.input)?;

    let mut filename: PathBuf = args
        .output
        .unwrap_or_else(|| pi.rna_name.unwrap_or_else(|| "rnaimg.svg".to_owned()))
        .into();

    match filename.extension().and_then(OsStr::to_str) {
        Some("png") | Some("svg") => (),
        _ => {
            // slapping .svg on top of filename; filename.set_extension() does work
            // but may overwrite something not meant to be an extension
            filename = PathBuf::from(format!(
                "{}.svg",
                filename.to_str().expect("filename is not valid utf8?")
            ));
        }
    };

    let theme = match args.theme.as_ref() {
        "dark" => ColorTheme::dark(),
        "white" | "w" => ColorTheme::white(),
        "black" | "b" => ColorTheme::black(),
        "bright" => ColorTheme::bright(),
        _ => {
            eprintln!(
                "theme: \"{}\" not recognized!\nfalling back to default",
                args.theme
            );
            ColorTheme::default()
        }
    };

    let mirror = draw::Mirror {
        x: args.mx,
        y: args.my,
    };

    match (pi.secondary_structure, pi.sequence) {
        (Some(sst), Some(sequence)) => {
            let pairlist = rnamanip::get_pair_list(&sst);
            let seq = rnamanip::read_sequence(&sequence);
            if pairlist.len() != seq.len() {
                panic!("sequence and secondary structure are different lengths!")
            }
            let tree = forest::grow_tree(&pairlist);
            let bubbles = draw::gather_bubbles(&tree, &seq, BUBBLE_RADIUS, args.angle.to_radians());

            draw::plot(
                &bubbles,
                BUBBLE_RADIUS,
                &filename,
                &theme,
                args.height,
                mirror,
            )?;

            println!("drawn: {:?}", &filename);
        }
        (Some(sst), None) => {
            let pairlist = rnamanip::get_pair_list(&sst);
            let seq = rnamanip::XSequence;
            let tree = forest::grow_tree(&pairlist);
            let bubbles = draw::gather_bubbles(&tree, &seq, BUBBLE_RADIUS, args.angle.to_radians());
            draw::plot(
                &bubbles,
                BUBBLE_RADIUS,
                &filename,
                &theme,
                args.height,
                mirror,
            )?;
            println!("drawn: {:?}", &filename);
        }
        (None, Some(_)) => unimplemented!(
            "Calling external soft e.g. RNAFold to get secondary_structure not yet implemented"
        ),
        (None, None) => panic!("Neither sequence nor secondary structure found in the input file!"),
    };

    Ok(())
}
