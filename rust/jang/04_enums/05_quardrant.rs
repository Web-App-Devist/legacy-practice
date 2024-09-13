#[derive(Debug)]
enum Quardrant {
    FIRST,
    SECOND,
    THIRD,
    FOURTH,
    ORIGIN,
    AXIS,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn check_quadrant(&self) -> Quardrant {
        match (self.x.signum(), self.y.signum()) {
            (1, 1) => Quardrant::FIRST,
            (-1, 1) => Quardrant::SECOND,
            (-1, -1) => Quardrant::THIRD,
            (1, -1) => Quardrant::FOURTH,
            (0, 0) => Quardrant::ORIGIN,
            _ => Quardrant::AXIS,
        }
    }
}

fn main() {
    let points = [
        Point::new(3, 4),   // First Quadrant
        Point::new(-3, 4),  // Second Quadrant
        Point::new(-3, -4), // Third Quadrant
        Point::new(3, -4),  // Fourth Quadrant
        Point::new(0, 5),   // On Y-Axis
        Point::new(4, 0),   // On X-Axis
        Point::new(0, 0),   // Origin
    ];

    for point in points.iter() {
        println!("Point {:?} is in {:?}", point, point.check_quadrant());
    }
}
