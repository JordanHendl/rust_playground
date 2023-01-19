mod common;
#[derive(Debug)]
struct Structure(i32);

struct Point {
  x: f32,
  y: f32,
}

fn main() {
    // Reasonable
    let p = Point{x: 2.2, y: 5.1};

    // So ..<struct> is like the base struct? Like, all fields after this are inherited from the parent p.
    // I think?
    let _p2 = Point{x: 2.5, ..p};
    let _p2 = Point{..p};

    println!("Point: {}, {}", p.x, p.y);
    common::print_one();
    common::print_two();
    common::test::print_three();
}
 