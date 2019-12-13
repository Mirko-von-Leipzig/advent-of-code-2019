fn main() {}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance(&self) -> u32 {
        (self.x.abs() as u32) + (self.y.abs() as u32)
    }

    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn origin() -> Point {
        Self::new(0, 0)
    }
}

enum Translation {
    UP(i32),
    DOWN(i32),
    LEFT(i32),
    RIGHT(i32),
}

struct Wire {
    translations: Vec<Translation>,
}

struct Segment {
    a: Point,
    b: Point,
}

impl Segment {
    fn new(a: Point, b: Point) -> Segment {
        Segment { a, b }
    }

    fn from_translation(origin: Point, translation: &Translation) -> Segment {
        match translation {
            Translation::UP(d) => Segment::new(origin, Point::new(origin.x + d, origin.y)),
            Translation::DOWN(d) => Segment::new(origin, Point::new(origin.x - d, origin.y)),
            Translation::LEFT(d) => Segment::new(origin, Point::new(origin.x, origin.y - d)),
            Translation::RIGHT(d) => Segment::new(origin, Point::new(origin.x, origin.y + d)),
        }
    }
}

impl Wire {
    fn segments(&self) -> Vec<Segment> {
        let mut a = Point::origin();

        self.translations
            .iter()
            .map(|t| Segment::from_translation(*a.borrow_mut(), t))
            .collect()
    }
}
