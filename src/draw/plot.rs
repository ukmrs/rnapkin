use std::ffi::OsStr;
use std::path::Path;

use super::colors::ColorTheme;
use super::gather::{get_starter_points, BubbleVec};
use super::Point;
use crate::rnamanip::Nucleotide;

use anyhow::Result;
use plotters::coord::types::RangedCoordf64;
use plotters::prelude::*;
use plotters::style::text_anchor::{HPos, Pos, VPos};

const NTA: &str = "A";
const NTG: &str = "G";
const NTC: &str = "C";
const NTU: &str = "U";

fn nucleotide_bubble<C, D>(
    coords: Point,
    radius: f64,
    letter: &'static str,
    bbl_clr: &C,
    drawing_area: &DrawingArea<D, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
) -> Result<()>
where
    C: Color,
    D: DrawingBackend,
{
    let pos = Pos::new(HPos::Center, VPos::Center);

    let c = Circle::new((0, 0), radius, Into::<ShapeStyle>::into(bbl_clr).filled());

    let style = TextStyle::from(("mono", 0.8 * radius).into_font())
        .pos(pos)
        .color(&BLACK);
    let text = Text::new(letter, (0, 0), style);

    let ee = EmptyElement::at((coords.x, coords.y)) + c + text;
    drawing_area.draw(&ee).unwrap(); // Cant "?", because there is extremely cursed lifetime on the error
    Ok(())
}

fn get_distance(p0: Point, p1: Point) -> (f64, f64) {
    let xr = (p0.x - p1.x).abs();
    let xy = (p0.y - p1.y).abs();
    (xr, xy)
}

fn draw<D: DrawingBackend>(
    root: &DrawingArea<D, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    bblv: &BubbleVec,
    bblr: f64,
    radius: f64,
    theme: &ColorTheme,
) -> Result<()> {
    for bbl in &bblv.bubbles {
        match bbl.nt {
            Nucleotide::A => nucleotide_bubble(bbl.point, radius, NTA, &theme.a, root)?,
            Nucleotide::U => nucleotide_bubble(bbl.point, radius, NTU, &theme.u, root)?,
            Nucleotide::G => nucleotide_bubble(bbl.point, radius, NTG, &theme.g, root)?,
            Nucleotide::C => nucleotide_bubble(bbl.point, radius, NTC, &theme.c, root)?,
            Nucleotide::X => nucleotide_bubble(bbl.point, radius, "", &theme.x, root)?,
        }
    }

    let (sp0, sp1) = get_starter_points(bblr * 2.);

    let pos = Pos::new(HPos::Right, VPos::Center);
    let style = TextStyle::from(("mono", 1.1 * radius).into_font())
        .pos(pos)
        .color(&theme.fg);

    let end5 = Text::new("5'", (sp0.x, sp0.y), style.clone());

    let pos = Pos::new(HPos::Left, VPos::Center);
    let style = TextStyle::from(("mono", 1.1 * radius).into_font())
        .pos(pos)
        .color(&theme.fg);
    let end3 = Text::new("3'", (sp1.x, sp1.y), style);

    // Cant "?", because there is extremely cursed lifetime on the error
    // that I cant figure out
    // its very akin to this:
    // https://github.com/plotters-rs/plotters/issues/62
    // it also happened when I got rid of hard-coded filename
    root.draw(&end3).unwrap();
    root.draw(&end5).unwrap();
    root.present().unwrap();

    Ok(())
}

fn calculate_coords(
    upper_bounds: Point,
    lower_bounds: Point,
    x: i32,
    y: i32,
    margin: f64,
) -> Cartesian2d<RangedCoordf64, RangedCoordf64> {
    Cartesian2d::<RangedCoordf64, RangedCoordf64>::new(
        (lower_bounds.x - margin)..(upper_bounds.x + margin),
        (lower_bounds.y - margin)..(upper_bounds.y + margin),
        (0..x, 0..y),
    )
}

pub fn plot<P: AsRef<Path>>(
    bblv: &BubbleVec,
    bblr: f64,
    filename: &P,
    theme: &ColorTheme,
    height: u32,
) -> Result<()> {
    let (dx, dy) = get_distance(bblv.upper_bounds, bblv.lower_bounds);
    let xyratio = dx / dy;

    let xsize = (xyratio * height as f64).round() as u32;
    let (ex, why) = (xsize, height);

    let margin = bblr * 2.2;

    // bblr    - (dx + margin)
    // radius? - xsize
    // looks slightly better with 0.99 than 1, pairs overlap less
    let radius = xsize as f64 * bblr * 0.99 / (dx + margin);

    match filename.as_ref().extension().and_then(OsStr::to_str) {
        Some("svg") => {
            let root = SVGBackend::new(filename, (ex, why)).into_drawing_area();
            let root = root.apply_coord_spec(calculate_coords(
                bblv.upper_bounds,
                bblv.lower_bounds,
                ex as i32,
                why as i32,
                margin,
            ));
            root.fill(&theme.bg)?;
            draw(&root, bblv, bblr, radius, theme)?;
        }
        Some("png") => {
            let root = BitMapBackend::new(filename, (ex, why)).into_drawing_area();
            let root = root.apply_coord_spec(calculate_coords(
                bblv.upper_bounds,
                bblv.lower_bounds,
                ex as i32,
                why as i32,
                margin,
            ));
            root.fill(&theme.bg)?;
            draw(&root, bblv, bblr, radius, theme)?;
        }
        _ => panic!("correct extension should be determined beforehand"),
    };

    Ok(())
}
