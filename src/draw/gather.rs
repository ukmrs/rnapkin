//! Gathers x, y coordinates of the nucleotide bubbles
//! so they can be plotted later
//! some dumb nomenclature used in this mod:
//! bubble / bbl = circle symbolizing a nucleotide
//! skelly = skeleton circle never drawn but upon its edges bubbles rest in a loop structure
//! bblr = bubble radius
//! bbla = amount of bubbles
use super::point::Point;
use crate::forest::{DotBracket, Tree};
use crate::rnamanip::Nucleotide;
use std::f64::consts::{FRAC_PI_2, PI, TAU};

#[derive(Default, Debug, Clone, Copy)]
struct Bubble {
    point: Point,
    nt: Nucleotide,
}

impl Bubble {
    fn new(point: Point, nt: Nucleotide) -> Self {
        Bubble { point, nt }
    }
}

#[cfg(debug_assertions)]
fn print_points(points: Vec<Point>) {
    for point in &points {
        println!("{:?}", point);
    }
}

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

    let delta = points[1] - points[0];

    let skelly_radius = get_skelly_radius(bblr, delta);
    let skelly_center = midpoint + Point::new(0., skelly_radius * sign).rotate(angle);

    for point in &mut points {
        point.x = point.x * skelly_radius + skelly_center.x;
        point.y = point.y * skelly_radius + skelly_center.y;
    }

    points
}

// propably gonna return a bubble: a point and a nt
pub fn gather_points<T>(tree: &Tree<DotBracket>, seq: &T, bubble_radius: f64) -> Vec<Point>
where
    T: std::ops::Index<usize, Output = Nucleotide>,
{
    let mut stack = vec![0_usize];

    while let Some(idx) = stack.pop() {
        let node = &tree[idx];
        let childrena = node.children.len();
        let midpoint = Point::new(0., 0.5);

        if childrena > 1 {
            let mut local_bubbles: Vec<Bubble> = vec![];
            let pair_pos: Vec<usize> = vec![];

            for idx in &node.children {
                let node = &tree[*idx];
                println!("{:?}", node);
            }

            let points = place_bubbles_upon_skelly(childrena, bubble_radius, midpoint, 0., false);
            print_points(points);
        }
    }

    vec![]
}
