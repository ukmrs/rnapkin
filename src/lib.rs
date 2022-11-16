pub mod draw;
pub mod forest;
pub mod rnamanip;

use draw::point::Point;

#[allow(dead_code)]
const SEQ: &str = "CGCUUCAUAUAAUCCUAAUGAUAUGGUUUGGGAGUUUCUACCAAGAGCCUUAAACUCUUGAUUAUGAAGUG";
#[allow(dead_code)]
const SST: &str = "...(((((((..((((((.........))))))......).((((((.......))))))..))))))...";

pub fn run() {
    let pl = rnamanip::get_pair_list(SST);
    let tree = forest::grow_tree(&pl);

    let midpoint = Point::new(-1.0606, 1.7677);
    let points = draw::gather::place_bubbles_upon_skelly(
        9,
        0.5,
        midpoint,
        std::f64::consts::FRAC_PI_4,
        false,
    );

    for point in &points {
        println!("{:?}", point);
    }
}
