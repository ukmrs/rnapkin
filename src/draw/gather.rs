//! Gathers x, y coordinates of the nucleotide bubbles
//! so they can be plotted later
//! some dumb nomenclature used in this mod:
//! bubble / bbl = circle symbolizing a nucleotide
//! skelly = skeleton circle never drawn but upon its edges bubbles rest in a loop structure
//! bblr = bubble radius
//! bbla = amount of bubbles
//! bbld = bubble diamater; since pair **bubbles** are touching also distance between their center
//! **not balls**
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

// TODO think of a name for this
struct Plate {
    pub idx: usize,
    pub angle: f64,
    pub p0: Point,
    pub p1: Point,
    // precalculated step; otherwise could be calculated by:
    // Point::new(0., bblr * 2).rotate(ang)
    pub step: Point,
    pub swap: bool,
}

// propably gonna return a bubble: a point and a nt
pub fn gather_points<T>(tree: &Tree<DotBracket>, seq: &T, bblr: f64) -> Vec<Point>
where
    T: std::ops::Index<usize, Output = Nucleotide>,
{
    let mut stack = vec![];
    let bbld = bblr * 2.;

    let starter = Plate {
        idx: 0,
        angle: 0.,
        p0: Point::new(0., bbld),
        p1: Point::new(bbld, bbld),
        step: Point::new(0., bbld),
        swap: false,
    };

    stack.push(starter);
    let mut bubbles: Vec<Bubble> = vec![];
    let mut dbgtc: usize = 0;

    while let Some(plate) = stack.pop() {

        let node = &tree[plate.idx];
        let childrena = node.children.len();
        let midpoint = plate.p1.get_middle(plate.p0);
        let bubbbles_offset = bubbles.len();
        let mut local_bubbles_counter: usize = 0; 

        if childrena > 1 {
            let mut pair_pos: Vec<usize> = vec![];

            for (n, idx) in node.children.iter().enumerate() {
                local_bubbles_counter += 1;
                let db = &tree[*idx].val;
                bubbles
                    .push(seq[db.pos.expect("kids should always have a position!?")].into());

                if let Some(pair) = db.pair {
                    // pair_pos.push(n);
                    pair_pos.push(local_bubbles_counter - 1);
                    bubbles.push(seq[pair].into());
                    local_bubbles_counter += 1;
                }
            }

            let skelly =
                place_bubbles_upon_skelly(local_bubbles_counter, bblr, midpoint, plate.angle, plate.swap);

            let mut points = skelly.points.into_iter().enumerate();

            let mut pair_sync: usize = 0;
            while let Some((n, p)) = points.next() {
                // pair_pos.len() will be very small up to 3 maybe 4 but usually less
                // Seems like vec is prolly better than hashset in the situation
                if pair_pos.contains(&(n)) {
                    eprintln!("{}, {:?}", n, p);

                    // swap depended?
                    let angle_around = skelly.angle_slice * (local_bubbles_counter - n) as f64;

                    let newp0 = plate.p0.rotate_around_origin(skelly.center, angle_around);
                    bubbles[n + bubbbles_offset + 1].point = newp0;

                    let newp1 = plate.p1.rotate_around_origin(skelly.center, angle_around);
                    bubbles[n + bubbbles_offset].point = newp1;

                    let next_idx = tree[node.children[n - pair_sync]].children[0];
                    assert_eq!(tree[node.children[n - pair_sync]].children.len(), 1);
                    let new_angle = angle_around + plate.angle;

                    let step = match plate.swap {
                        false => Point::new(0., -bbld).rotate(new_angle),
                        true => Point::new(0., bbld).rotate(new_angle),
                    };

                    let next_plate = Plate {
                        idx: next_idx,
                        angle: new_angle, // TODO prolly not correct; just guessin
                        p0: newp0,
                        p1: newp1,
                        swap: !plate.swap,
                        step,
                    };
                    stack.push(next_plate);

                    points.next(); // Discard next point
                    pair_sync += 1;
                } else {
                    bubbles[n + bubbbles_offset].point = p;
                }
            }

            dbgtc += 1;
            if dbgtc == 7 {
                break;
            }


        } else {


            let new_p0 = plate.p0 + plate.step;
            let new_p1 = plate.p1 + plate.step;

            let (pos_nt, pair_nt);
            if plate.swap {
                pos_nt = seq[node.val.pair.unwrap()];
                pair_nt = seq[node.val.pos.unwrap()];
            } else {
                pos_nt = seq[node.val.pos.unwrap()];
                pair_nt = seq[node.val.pair.unwrap()];
            }


            bubbles.push(Bubble::new(new_p0, pos_nt).into());
            bubbles.push(Bubble::new(new_p1, pair_nt).into());

            let next_plate = Plate {
                idx: node.children[0],
                p0: new_p0,
                p1: new_p1,
                ..plate
            };
            stack.push(next_plate);
        }
    }

    print_bubbles(&bubbles);
    vec![]
}
