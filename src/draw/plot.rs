use super::colors::default_pallette::*;
use super::gather::{get_starter_points, BubbleVec};
use super::Point;
use crate::rnamanip::Nucleotide;

use plotters::coord::types::RangedCoordf64;
use plotters::prelude::*;
use plotters::style::text_anchor::{HPos, Pos, VPos};

type Canvas<'a> = DrawingArea<BitMapBackend<'a>, Cartesian2d<RangedCoordf64, RangedCoordf64>>;

const NTA: &str = "A";
const NTG: &str = "G";
const NTC: &str = "C";
const NTU: &str = "U";

fn nucleotide_bubble<C: Color>(
    coords: Point,
    radius: f64,
    letter: &'static str,
    bbl_clr: C,
    drawing_area: &Canvas,
) -> Result<(), Box<dyn std::error::Error>> {
    let pos = Pos::new(HPos::Center, VPos::Center);

    let c = Circle::new((0, 0), radius, Into::<ShapeStyle>::into(bbl_clr).filled());

    let style = TextStyle::from(("mono", 0.8 * radius).into_font())
        .pos(pos)
        .color(&BLACK);
    let text = Text::new(letter, (0, 0), style);

    let ee = EmptyElement::at((coords.x, coords.y)) + c + text;
    drawing_area.draw(&ee)?;
    Ok(())
}

fn get_distance(p0: Point, p1: Point) -> (f64, f64) {
    let xr = (p0.x - p1.x).abs();
    let xy = (p0.y - p1.y).abs();
    (xr, xy)
}

pub fn plot(bblv: BubbleVec, bblr: f64) -> Result<(), Box<dyn std::error::Error>> {
    let (dx, dy) = get_distance(bblv.upper_bounds, bblv.lower_bounds);
    let xyratio = dx / dy;

    let xsize = (xyratio * 900.).round() as u32;
    let (ex, why) = (xsize, 900);

    let root = BitMapBackend::new("img.gi.png", (ex, why)).into_drawing_area();
    root.fill(&DARK_BG)?;

    let (ex, why) = (ex as i32, why as i32);
    let margin = bblr * 1.5;

    let root = root.apply_coord_spec(Cartesian2d::<RangedCoordf64, RangedCoordf64>::new(
        (bblv.lower_bounds.x - margin)..(bblv.upper_bounds.x + margin),
        (bblv.lower_bounds.y - margin)..(bblv.upper_bounds.y + margin),
        (0..ex, 0..why),
    ));

    // bblr    - (dx + margin)
    // radius? - xsize
    // looks slightly better with 0.99 than 1, pairs overlap less
    let radius = xsize as f64 * bblr * 0.99 / (dx + margin);

    for bbl in &bblv.bubbles {
        match bbl.nt {
            Nucleotide::A => nucleotide_bubble(bbl.point, radius, NTA, DARK_A, &root)?,
            Nucleotide::U => nucleotide_bubble(bbl.point, radius, NTU, DARK_U, &root)?,
            Nucleotide::G => nucleotide_bubble(bbl.point, radius, NTG, DARK_G, &root)?,
            Nucleotide::C => nucleotide_bubble(bbl.point, radius, NTC, DARK_C, &root)?,
            Nucleotide::X => nucleotide_bubble(bbl.point, radius, "", DARK_X, &root)?,
        }
    }

    let (sp0, sp1) = get_starter_points(bblr * 2.);

    let pos = Pos::new(HPos::Right, VPos::Center);
    let style = TextStyle::from(("mono", 1.1 * radius).into_font())
        .pos(pos)
        .color(&DARK_FG);

    let end5 = Text::new("5'", (sp0.x, sp0.y), style.clone());

    let pos = Pos::new(HPos::Left, VPos::Center);
    let style = TextStyle::from(("mono", 1.1 * radius).into_font())
        .pos(pos)
        .color(&DARK_FG);
    let end3 = Text::new("3'", (sp1.x, sp1.y), style);
    root.draw(&end3)?;
    root.draw(&end5)?;

    root.present()?;

    Ok(())
}
