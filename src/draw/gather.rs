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
use crate::draw::Mirror;
use crate::forest::{DotBracket, Tree};
use crate::rnamanip::Nucleotide;
use std::convert::From;
use std::f64::consts::{PI, TAU};
use std::ops::Index;

#[derive(Default, Debug, Clone, Copy)]
pub struct Bubble {
    pub point: Point,
    pub nt: Nucleotide,
    pub pos: usize,
}

impl Bubble {
    fn new(point: Point, nt: Nucleotide, pos: usize) -> Self {
        Bubble { point, nt, pos }
    }

    /// constructs a bubble suspended in endless void
    /// i.e. in x: 0., y: 0. coordinates.
    /// Function has a scary name because you that is
    /// prolly me has to give them real coordinates later
    fn new_in_void(nt: Nucleotide, pos: usize) -> Self {
        Self {
            point: Point::default(),
            nt,
            pos,
        }
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

#[derive(Debug)]
struct Skelly {
    pub points: Vec<Point>,
    pub angle_slice: f64,
    pub center: Point,
}

fn get_skelly_radius(bblr: f64, delta: Point) -> f64 {
    2. * bblr / (delta.x.powi(2) + delta.y.powi(2)).sqrt()
}

fn place_bubbles_upon_skelly(
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

    // TODO this whole operation is very akward; consider options
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
#[derive(Debug)]
struct Plate {
    pub idx: usize,
    pub angle: f64,
    pub p0: Point,
    pub p1: Point,
    pub step: Point, // precalculated; could be calculated on a fly instead
    pub swap: bool,
}

/// Serves as gather\_bubbles output, it may grow if there will be
/// more valuable information to harvest during calculating coordinates.
/// For now it only keeps track of the bounds which saves one iteration XD
/// bounds are needed to request appropriate canvas size during drawing
pub struct BubbleVec {
    pub bubbles: Vec<Bubble>,
    pub upper_bounds: Point,
    pub lower_bounds: Point,
    pub sp0: Point,
    pub sp1: Point,
}

impl Index<usize> for BubbleVec {
    type Output = Bubble;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bubbles[index]
    }
}

impl BubbleVec {
    fn new(sp0: Point, sp1: Point) -> Self {
        Self {
            bubbles: vec![],
            upper_bounds: Point::new(f64::NEG_INFINITY, f64::NEG_INFINITY),
            lower_bounds: Point::new(f64::INFINITY, f64::INFINITY),
            sp0,
            sp1,
        }
    }

    fn len(&self) -> usize {
        self.bubbles.len()
    }

    fn push(&mut self, bbl: Bubble) {
        self.upper_bounds = self.upper_bounds.max(bbl.point);
        self.lower_bounds = self.lower_bounds.min(bbl.point);
        self.bubbles.push(bbl);
    }

    /// Assign concrete point value later
    fn allocate(&mut self, bbl: Bubble) {
        self.bubbles.push(bbl)
    }

    fn set_point(&mut self, idx: usize, p: Point) {
        self.upper_bounds = self.upper_bounds.max(p);
        self.lower_bounds = self.lower_bounds.min(p);
        self.bubbles[idx].point = p;
    }

    pub fn mirror(&mut self, mirror: Mirror) {
        let (x, y) = match (mirror.x, mirror.y) {
            (false, false) => return,
            (true, true) => (-1., -1.),
            (false, true) => (-1., 1.),
            (true, false) => (1., -1.),
        };

        for bbl in self.bubbles.iter_mut() {
            bbl.point.x *= x;
            bbl.point.y *= y;
        }
    }
}

pub fn get_starter_points(bbld: f64, angle: f64) -> (Point, Point) {
    (
        Point::new(0., bbld).rotate(angle),
        Point::new(bbld, bbld).rotate(angle),
    )
}

/// gathers x, y coordinates of the nucleotide bubbles
/// there's little point to setting bblr to something other than bblr=0.5
/// because points and bubble radius can be easily upscaled later
pub fn gather_bubbles<T>(
    tree: &Tree<DotBracket>,
    seq: &T,
    bblr: f64,
    starting_angle: f64,
) -> BubbleVec
where
    T: std::ops::Index<usize, Output = Nucleotide>,
{
    let mut stack = vec![];

    let bbld = bblr * 2.;
    let (p0, p1) = get_starter_points(bbld, starting_angle);
    let mut bubbles = BubbleVec::new(p0, p1);

    // accounts for rna starting with a stem right off the bat
    let starter_idx = usize::from(tree[0].children.len() == 1);

    let starter = Plate {
        p0,
        p1,
        idx: starter_idx,
        angle: starting_angle,
        step: Point::new(0., bbld).rotate(starting_angle),
        swap: false,
    };

    stack.push(starter);

    while let Some(plate) = stack.pop() {
        let node = &tree[plate.idx];
        let childrena = node.children.len();
        let midpoint = plate.p1.get_middle(plate.p0);
        let bubbbles_offset = bubbles.len();
        let mut local_bubbles_counter: usize = 0;

        if childrena > 1 {
            let mut pair_pos: Vec<usize> = vec![];

            for idx in node.children.iter() {
                local_bubbles_counter += 1;
                let db = &tree[*idx].val;
                let pos = db.pos.expect("kids should always have a position");
                bubbles.allocate(Bubble::new_in_void(seq[pos], pos));

                if let Some(pair) = db.pair {
                    pair_pos.push(local_bubbles_counter - 1);
                    bubbles.allocate(Bubble::new_in_void(seq[pair], pair));
                    local_bubbles_counter += 1;
                }
            }

            let skelly = place_bubbles_upon_skelly(
                local_bubbles_counter,
                bblr,
                midpoint,
                plate.angle,
                plate.swap,
            );

            let mut points = skelly.points.into_iter().enumerate();

            let mut pair_sync: usize = 0;
            while let Some((n, p)) = points.next() {
                // pair_pos.len() will be very small up to 3 maybe 4 but usually less
                // Seems like vec is prolly better than hashset in the situation
                if pair_pos.contains(&(n)) {
                    // swap depended?
                    let angle_around = skelly.angle_slice * (local_bubbles_counter - n) as f64;
                    let new_angle = angle_around + plate.angle;

                    let (step, kickp0, kickp1) = match plate.swap {
                        false => (Point::new(0., -bbld).rotate(new_angle), 1, 0),
                        true => (Point::new(0., bbld).rotate(new_angle), 0, 1),
                    };

                    let newp0 = plate.p0.rotate_around_origin(skelly.center, angle_around);
                    bubbles.set_point(n + bubbbles_offset + kickp0, newp0);
                    let newp1 = plate.p1.rotate_around_origin(skelly.center, angle_around);
                    bubbles.set_point(n + bubbbles_offset + kickp1, newp1);

                    let next_idx = tree[node.children[n - pair_sync]].children[0];
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
                    bubbles.set_point(n + bubbbles_offset, p)
                }
            }
        } else {
            // this branch walks down the stem
            let new_p0 = plate.p0 + plate.step;
            let new_p1 = plate.p1 + plate.step;

            let mut pair_position = node.val.pair.unwrap();
            let mut pos_position = node.val.pos.unwrap();

            if plate.swap {
                (pair_position, pos_position) = (pos_position, pair_position)
            }

            let pair_nt = seq[pair_position];
            let pos_nt = seq[pos_position];

            bubbles.push(Bubble::new(new_p0, pos_nt, pos_position));
            bubbles.push(Bubble::new(new_p1, pair_nt, pair_position));

            let next_plate = Plate {
                idx: node.children[0],
                p0: new_p0,
                p1: new_p1,
                ..plate
            };
            stack.push(next_plate);
        }
    }

    bubbles
}

#[cfg(test)]
mod tests {}
