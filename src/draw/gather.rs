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
use std::convert::From;
use std::f64::consts::{PI, TAU};

#[derive(Default, Debug, Clone, Copy)]
struct Bubble {
    pub point: Point,
    pub nt: Nucleotide,
}

impl Bubble {
    fn new(point: Point, nt: Nucleotide) -> Self {
        Bubble { point, nt }
    }
}

impl From<Nucleotide> for Bubble {
    fn from(nt: Nucleotide) -> Self {
        Bubble {
            nt,
            ..Self::default()
        }
    }
}

pub struct Skelly {
    pub points: Vec<Point>,
    pub angle_slice: f64,
    pub center: Point,
}

#[cfg(debug_assertions)]
fn print_points(points: &Vec<Point>) {
    for point in points {
        println!("{:?}", point);
    }
}

#[cfg(debug_assertions)]
fn print_bubbles(bbls: &Vec<Bubble>) {
    for bbl in bbls {
        println!("{:?}", bbl);
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
) -> Skelly {
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

    Skelly {
        points,
        angle_slice,
        center: skelly_center,
    }
}

// propably gonna return a bubble: a point and a nt
pub fn gather_points<T>(tree: &Tree<DotBracket>, seq: &T, bblr: f64) -> Vec<Point>
where
    T: std::ops::Index<usize, Output = Nucleotide>,
{
    let mut stack = vec![0_usize];

    while let Some(idx) = stack.pop() {
        let node = &tree[idx];
        let childrena = node.children.len();
        let base_pair0 = Point::new(0., 0.);
        let base_pair1 = Point::new(bblr * 2., 0.);
        let midpoint = base_pair1.get_middle(base_pair0);
        let angle = 0.;

        if childrena > 1 {
            let mut local_bubbles: Vec<Bubble> = vec![];
            let mut pair_pos: Vec<usize> = vec![];

            for (n, idx) in node.children.iter().enumerate() {
                let db = &tree[*idx].val;
                local_bubbles
                    .push(seq[db.pos.expect("kids should always have a position!?")].into());

                if let Some(pair) = db.pair {
                    pair_pos.push(n);
                    local_bubbles.push(seq[pair].into());
                }
            }

            let mut skelly =
                place_bubbles_upon_skelly(local_bubbles.len(), bblr, midpoint, angle, false);

            let mut points = skelly.points.into_iter().enumerate();

            while let Some((n, p)) = points.next() {
                // pair_pos.len() will be very small up to 3 maybe 4 but usually less
                // Seems like vec is prolly better than hashset in the situation
                if pair_pos.contains(&(n)) {
                    // swap depended?
                    let angle_around = skelly.angle_slice * (local_bubbles.len() - n) as f64;
                    local_bubbles[n].point =
                        base_pair0.rotate_around_origin(skelly.center, angle_around);
                    local_bubbles[n + 1].point =
                        base_pair1.rotate_around_origin(skelly.center, angle_around);

                    // TODO push onto stack? or do a sick backflip and recursion
                    points.next();
                } else {
                    local_bubbles[n].point = p;
                }
            }
            print_bubbles(&local_bubbles);
        }
    }

    vec![]
}
