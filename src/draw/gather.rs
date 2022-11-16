/// bubble / bbl = circle symbolizing a nucleotide
/// skelly = skeleton circle never drawn but upon its edges bubbles rest in a loop structure
/// bblr = bubble radius
/// bbla = amount of bubbles
use super::point::Point;
use crate::forest::{DotBracket, Tree};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

fn get_skelly_radius(bblr: f64, delta: Point) -> f64 {
    2. * bblr / (delta.x.powi(2) + delta.y.powi(2)).sqrt()
}

pub fn place_bubbles_upon_skelly(
    bbla: usize,
    bblr: f64,
    midpoint: Point,
    angle: f64,
    swap: bool,
) -> Vec<Point> {
    let angle_slice = TAU / (bbla + 2) as f64;

    // allows to place skelly center ahead of the anchored pair
    // also rotates skelly so the gap is in the correct place
    let (offset, sign) = match swap {
        true => (0., -1.),
        false => (PI, 1.),
    };

    let multi = 1. - (angle + offset) / TAU;

    // 2 accounts for the anchored pair
    // 1.5 I pulled out of thin air: it just aligns nicely then
    let nudge = (bbla as f64 + 2.) * multi + 1.5;

    // not skipping pairs because I need two contigous points
    // which might not always happen if I filter here
    // the function will also get more bloated
    // Its seems way more elegant and its not a huge deal anyway
    // I could though always take 2 first points and then filter and add center?
    let mut points = Vec::with_capacity(bbla);
    for i in 0..bbla {
        let inudge = nudge + i as f64;
        let x = (angle_slice * inudge).sin();
        let y = (angle_slice * inudge).cos();
        points.push(Point::new(x, y));
    }

    // !points are correct

    let delta = points[1] - points[0];

    let skelly_radius = get_skelly_radius(bblr, delta);
    let skelly_center = midpoint + Point::new(0., skelly_radius * sign).rotate(angle);

    for point in &mut points {
        point.x = point.x * skelly_radius + skelly_center.x;
        point.y = point.y * skelly_radius + skelly_center.y;
    }

    points
}

pub fn gather_points(tree: Tree<DotBracket>, bubble_radius: f64) -> Vec<Point> {
    vec![]
}
