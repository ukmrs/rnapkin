use std::borrow::Borrow;
use std::ffi::OsStr;
use std::path::Path;

use super::colors::ColorTheme;
use super::gather::BubbleVec;
use super::Point;

use anyhow::Result;
use plotters::coord::types::RangedCoordf64;
use plotters::prelude::*;
use plotters::style::text_anchor::{HPos, Pos, VPos};

/// Indicates if and along which axes
/// to mirror points
pub struct Mirror {
    pub x: bool,
    pub y: bool,
}

impl Mirror {
    pub fn new(x: bool, y: bool) -> Self {
        Self { x, y }
    }
}

fn nucleotide_bubble<C, D, S>(
    coords: Point,
    radius: f64,
    letter: S,
    bbl_clr: &C,
    drawing_area: &DrawingArea<D, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
) -> Result<()>
where
    C: Color,
    D: DrawingBackend,
    S: Borrow<str> + 'static,
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

fn highlighted_bubble<C, D, S>(
    coords: Point,
    radius: f64,
    letter: S,
    bbl_clr: &C,
    hhl_clr: &C,
    bg_clr: &RGBAColor,
    drawing_area: &DrawingArea<D, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
) -> Result<()>
where
    C: Color,
    D: DrawingBackend,
    S: Borrow<str> + 'static,
{
    let pos = Pos::new(HPos::Center, VPos::Center);
    let nc = Circle::new((0, 0), radius, Into::<ShapeStyle>::into(hhl_clr).filled());
    let bc = Circle::new(
        (0, 0),
        radius * 0.8,
        Into::<ShapeStyle>::into(bg_clr).filled(),
    );
    let c = Circle::new(
        (0, 0),
        radius * 0.72,
        Into::<ShapeStyle>::into(bbl_clr).filled(),
    );
    let style = TextStyle::from(("mono", 0.8 * radius).into_font())
        .pos(pos)
        .color(&BLACK);

    let text = Text::new(letter, (0, 0), style);
    let ee = EmptyElement::at((coords.x, coords.y)) + nc + bc + c + text;
    drawing_area.draw(&ee).unwrap(); // Cant "?", because there is extremely cursed lifetime on the error
    Ok(())
}

fn get_distance(p0: Point, p1: Point) -> (f64, f64) {
    let xr = (p0.x - p1.x).abs();
    let xy = (p0.y - p1.y).abs();
    (xr, xy)
}

fn draw_ends<D: DrawingBackend>(
    root: &DrawingArea<D, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    bblv: &BubbleVec,
    radius: f64,
    theme: &ColorTheme,
) -> Result<()> {
    let pos = Pos::new(HPos::Center, VPos::Center);
    let style = TextStyle::from(("mono", 1.1 * radius).into_font())
        .pos(pos)
        .color(&theme.fg);

    let end5 = Text::new("5'", (bblv.sp0.x, bblv.sp0.y), style.clone());

    let pos = Pos::new(HPos::Center, VPos::Center);
    let style = TextStyle::from(("mono", 1.1 * radius).into_font())
        .pos(pos)
        .color(&theme.fg);
    let end3 = Text::new("3'", (bblv.sp1.x, bblv.sp1.y), style);

    // Cant "?", because there is extremely cursed lifetime on the error
    // that I cant figure out
    // its very akin to this:
    // https://github.com/plotters-rs/plotters/issues/62
    // it also happened when I got rid of hard-coded filename
    root.draw(&end3).unwrap();
    root.draw(&end5).unwrap();
    Ok(())
}

fn draw<D: DrawingBackend>(
    root: &DrawingArea<D, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    bblv: &BubbleVec,
    radius: f64,
    theme: &ColorTheme,
    highlights: &[Option<usize>],
) -> Result<()> {
    for bbl in &bblv.bubbles {
        let (letter, bubble_color) = bbl.nt.extract_text_and_color(theme);
        if let Some(highlight_index) = highlights[bbl.pos] {
            let highlight_color = theme.highlights[highlight_index];
            highlighted_bubble(
                bbl.point,
                radius,
                letter,
                bubble_color,
                &highlight_color,
                &theme.bg,
                root,
            )?;
        } else {
            nucleotide_bubble(bbl.point, radius, letter, bubble_color, root)?;
        }
    }

    draw_ends(root, bblv, radius, theme)?;
    root.present().unwrap();
    Ok(())
}

#[allow(dead_code)]
fn simple_draw<D: DrawingBackend>(
    root: &DrawingArea<D, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    bblv: &BubbleVec,
    radius: f64,
    theme: &ColorTheme,
) -> Result<()> {
    for bbl in &bblv.bubbles {
        let (letter, bubble_color) = bbl.nt.extract_text_and_color(theme);
        nucleotide_bubble(bbl.point, radius, letter, bubble_color, root)?;
    }
    draw_ends(root, bblv, radius, theme)?;
    root.present().unwrap();
    Ok(())
}

fn calculate_coords(
    upper_bounds: Point,
    lower_bounds: Point,
    x: i32,
    y: i32,
    margin: f64,
    mirror: Mirror,
) -> Cartesian2d<RangedCoordf64, RangedCoordf64> {
    let xrange = if mirror.y {
        (upper_bounds.x + margin)..(lower_bounds.x - margin)
    } else {
        (lower_bounds.x - margin)..(upper_bounds.x + margin)
    };

    let yrange = if mirror.x {
        (lower_bounds.y - margin)..(upper_bounds.y + margin)
    } else {
        (upper_bounds.y + margin)..(lower_bounds.y - margin)
    };

    Cartesian2d::<RangedCoordf64, RangedCoordf64>::new(xrange, yrange, (0..x, 0..y))
}

pub fn plot<P: AsRef<Path>>(
    bblv: &BubbleVec,
    bblr: f64,
    filename: &P,
    theme: &ColorTheme,
    height: u32,
    mirror: Mirror,
    highlights: &[Option<usize>],
) -> Result<Option<String>> {
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
                mirror,
            ));
            root.fill(&theme.bg)?;
            draw(&root, bblv, radius, theme, highlights)?;
        }
        Some("png") => {
            let root = BitMapBackend::new(filename, (ex, why)).into_drawing_area();
            let root = root.apply_coord_spec(calculate_coords(
                bblv.upper_bounds,
                bblv.lower_bounds,
                ex as i32,
                why as i32,
                margin,
                mirror,
            ));
            root.fill(&theme.bg)?;
            draw(&root, bblv, radius, theme, highlights)?;
        }
        Some("x") => {
            let mut svgstring = String::with_capacity(0x10000);
            {
                let root = SVGBackend::with_string(&mut svgstring, (ex, why)).into_drawing_area();
                let root = root.apply_coord_spec(calculate_coords(
                    bblv.upper_bounds,
                    bblv.lower_bounds,
                    ex as i32,
                    why as i32,
                    margin,
                    mirror,
                ));
                root.fill(&theme.bg)?;
                draw(&root, bblv, radius, theme, highlights)?
            }
            return Ok(Some(svgstring));
        }
        _ => panic!("correct extension should be determined beforehand"),
    };

    Ok(None)
}
