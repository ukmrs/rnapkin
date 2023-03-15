use std::ffi::OsStr;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use rnapkin::draw::{self, colors::ColorTheme, Mirror};
use rnapkin::forest;
use rnapkin::rnamanip::{self, Nucleotide};
use rnapkin::utils::ParsedInput;

const BUBBLE_RADIUS: f64 = 0.5;

/// rnapkin: plotting utility for secondary RNA structure
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// file containing secondary_structure and sequence
    input: Option<String>,

    /// Output file; supported extensions: .svg and .png
    #[arg(short, long)]
    output: Option<String>,

    #[arg(short, long)]
    ghlight: Option<String>,

    /// Color theme; dark, bright, white/w, black/b
    #[arg(short, long, default_value = "dark")]
    theme: String,

    /// Angle in degrees
    #[arg(short, long, default_value_t = 0.)]
    angle: f64,

    /// background opacity between (1., 0.) (opaque, transparent)
    #[arg(short, long)]
    bgopacity: Option<f64>,

    /// Mirror along y axis
    #[arg(long, default_value_t = false)]
    my: bool,

    /// Mirror along x axis
    #[arg(long, default_value_t = false)]
    mx: bool,

    /// Height in pixels, width will be an appropriate ratio of height
    #[arg(long, default_value_t = 900)]
    height: u32,

    /// Print x,y,nucleotide,position (0 indexed) and exit
    #[arg(short, long, default_value_t = false)]
    points: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let pi = match args.input {
        Some(input) => ParsedInput::from_file(&input)?,
        None => ParsedInput::from_pipe()?, // carnivorous plant emerges
    };

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

    let mut theme = match args.theme.as_ref() {
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

    if let Some(bgopacity) = args.bgopacity {
        theme.bg.3 = bgopacity;
    }

    let (pairlist, sequence) = match (pi.secondary_structure, pi.sequence) {
        (Some(sst), Some(seq)) => {
            let pl = rnamanip::get_pair_list(&sst);
            let seq = rnamanip::read_sequence(&seq);
            assert_eq!(
                pl.len(),
                seq.len(),
                "sequence and structure have differents lenghts!"
            );
            (pl, seq)
        }
        (Some(sst), None) => {
            let pairlist = rnamanip::get_pair_list(&sst);
            let seq = vec![Nucleotide::X; pairlist.len()]; // TODO del XSequence if am not gonna use it
            (pairlist, seq)
        }
        (None, Some(_)) => unimplemented!(
            "Calling external soft e.g. RNAFold to get secondary_structure not yet implemented"
        ),
        (None, None) => panic!("Neither sequence nor secondary structure found in the input file!"),
    };

    let tree = forest::grow_tree(&pairlist);
    let mut bubbles =
        draw::gather_bubbles(&tree, &sequence, BUBBLE_RADIUS, args.angle.to_radians());
    let mirror = Mirror::new(args.mx, args.my);

    if args.points {
        bubbles.mirror(mirror);
        for bbl in &bubbles.bubbles {
            println!("{},{},{},{}", bbl.point.x, bbl.point.y, bbl.nt, bbl.pos);
        }
        return Ok(());
    }

    // TODO highlight is implementation is rushed
    // I need the functionality but haven't got the time to do it nicely :c
    let highlights = match pi.highlight {
        Some(hls) => draw::colors::user_input_to_highlight_indices(&hls),
        None => vec![None; sequence.len()],
    };

    draw::plot(
        &bubbles,
        BUBBLE_RADIUS,
        &filename,
        &theme,
        args.height,
        mirror,
        &highlights,
    )?;

    // rnapkin panics earlier if filename is not valid utf8
    println!("{}", &filename.to_str().unwrap());

    Ok(())
}
