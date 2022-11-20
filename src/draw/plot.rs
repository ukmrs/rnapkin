use super::gather::BubbleVec;
use super::Point;
use crate::rnamanip::Nucleotide;

use plotters::coord::types::RangedCoordf64;
use plotters::coord::Shift;
use plotters::prelude::*;
use plotters::style::text_anchor::{HPos, Pos, VPos};

const MULTI: f64 = 10.;

type Canvas<'a> = DrawingArea<BitMapBackend<'a>, Cartesian2d<RangedCoordf64, RangedCoordf64>>;

const NTA: &str = "A";
const NTG: &str = "G";
const NTC: &str = "C";
const NTU: &str = "U";

pub fn nucleotide_bubble<C: Color>(
    coords: Point,
    radius: f64,
    letter: &'static str,
    bbl_clr: C,
    drawing_area: &Canvas,
) -> Result<(), Box<dyn std::error::Error>> {
    let pos = Pos::new(HPos::Center, VPos::Center);

    let c = Circle::new(
        (0, 0),
        radius,
        Into::<ShapeStyle>::into(bbl_clr).filled(),
    );

    let style = TextStyle::from(("mono", 2. * MULTI).into_font())
        .pos(pos)
        .color(&BLACK);
    let text = Text::new(letter, (0, 4), style);

    let ee = EmptyElement::at((coords.x, coords.y)) + c + text;
    drawing_area.draw(&ee)?;
    Ok(())
}

fn get_ratio(p0: Point, p1: Point) -> f64 {
    let xr = ( p0.x - p1.x ).abs();
    let xy = (p0.y - p1.y).abs();
    xr / xy
}

fn get_somethin(p0: Point, p1: Point) -> (f64, f64) {
    let xr = ( p0.x - p1.x ).abs();
    let xy = (p0.y - p1.y).abs();
    (xr, xy)
}

pub fn plot(bblv: BubbleVec, bblr: f64) -> Result<(), Box<dyn std::error::Error>> {
    let rat = get_ratio(bblv.upper_bounds, bblv.lower_bounds);
    let (x, y) = get_somethin(bblv.upper_bounds, bblv.lower_bounds);
    println!("{}, {}", x, y);

    let rat = (rat * 1000.).round() as u32;
    let (ex, why) = (rat, 1000);
    println!("rat {}", rat);

    let root = BitMapBackend::new("img.gi.png", (ex, why)).into_drawing_area();
    // root.fill(&TRANSPARENT)?;
    root.fill(&RGBColor(57, 62, 70))?;

    // let root = root.apply_coord_spec(Cartesian2d::<RangedCoordf64, RangedCoordf64>::new(
    //     -8f64..20f64,
    //     -9f64..20f64,
    //     (0..640, 0..480),
    // ));

    let (ex, why) = (ex as i32, why as i32);
    let m = bblr;
    let root = root.apply_coord_spec(Cartesian2d::<RangedCoordf64, RangedCoordf64>::new(
        (bblv.lower_bounds.x - m)..(bblv.upper_bounds.x + m),
        (bblv.lower_bounds.y - m)..(bblv.upper_bounds.y + m),
        (0..ex, 0..why),
    ));

    // bblr - xrange (og points)
    // r - rat
    // bbld is current margin btw
    let bbld = bblr * 2.;
    let r = rat as f64 * bblr / (x + bbld); 
    for bbl in &bblv.bubbles {
        match bbl.nt {
            Nucleotide::A => nucleotide_bubble(bbl.point, r, NTA, &YELLOW, &root)?,
            Nucleotide::U => nucleotide_bubble(bbl.point, r, NTU, &BLUE, &root)?,
            Nucleotide::G => nucleotide_bubble(bbl.point, r, NTG, &RED, &root)?,
            Nucleotide::C => nucleotide_bubble(bbl.point, r, NTC, &GREEN, &root)?,
            _ => unreachable!(),
        }
    }

    root.present()?;

    Ok(())
}
