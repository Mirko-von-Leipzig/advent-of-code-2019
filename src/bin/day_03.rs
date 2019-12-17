fn main() {
    let (part_1, part_2) = std::fs::read_to_string("src/inputs/day_03")
        .expect("Failed to read input file")
        .parse::<Circuit>()
        .unwrap()
        .run()
        .unwrap();

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

struct Circuit {
    wires: (Wire, Wire),
}

struct Wire {
    segments: Vec<Segment>,
}

struct Segment {
    point: Point,
    translation: Translation,
    steps: u64,
}

#[derive(Clone)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone)]
struct Translation {
    direction: Direction,
    distance: u64,
}

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

struct Intersection {
    point: Point,
    steps: u64,
}

enum Orientation {
    HORIZONTAL,
    VERTICAL,
}

impl Translation {
    fn new(direction: Direction, distance: u64) -> Translation {
        Translation {
            direction,
            distance,
        }
    }
}

impl Intersection {
    fn new(point: Point, steps: u64) -> Intersection {
        Intersection { point, steps }
    }
}

impl Segment {
    fn new(point: Point, translation: Translation, steps: u64) -> Segment {
        Segment {
            point,
            translation,
            steps,
        }
    }

    fn endpoints(&self) -> (i32, i32) {
        match self.translation.direction {
            Direction::UP => (
                self.point.y,
                self.point.y + self.translation.distance as i32,
            ),
            Direction::DOWN => (
                self.point.y - self.translation.distance as i32,
                self.point.y,
            ),
            Direction::LEFT => (
                self.point.x - self.translation.distance as i32,
                self.point.x,
            ),
            Direction::RIGHT => (
                self.point.x,
                self.point.x + self.translation.distance as i32,
            ),
        }
    }

    fn orientation(&self) -> Orientation {
        match self.translation.direction {
            Direction::UP | Direction::DOWN => Orientation::VERTICAL,
            Direction::LEFT | Direction::RIGHT => Orientation::HORIZONTAL,
        }
    }

    fn intersect(&self, other: &Segment) -> Option<Intersection> {
        match (self.orientation(), other.orientation()) {
            (Orientation::HORIZONTAL, Orientation::VERTICAL) => {
                let (x_min, x_max) = self.endpoints();
                let (y_min, y_max) = other.endpoints();
                let x = other.point.x;
                let y = self.point.y;

                if (x_min..=x_max).contains(&x) && (y_min..=y_max).contains(&y) {
                    let x_steps = self.steps + (self.point.x - x).abs() as u64;
                    let y_steps = other.steps + (other.point.y - y).abs() as u64;
                    let steps = x_steps + y_steps;
                    Some(Intersection::new(Point::new(x, y), steps))
                } else {
                    None
                }
            }
            (Orientation::VERTICAL, Orientation::HORIZONTAL) => {
                let (x_min, x_max) = other.endpoints();
                let (y_min, y_max) = self.endpoints();
                let x = self.point.x;
                let y = other.point.y;

                if (x_min..=x_max).contains(&x) && (y_min..=y_max).contains(&y) {
                    let x_steps = other.steps + (other.point.x - x).abs() as u64;
                    let y_steps = self.steps + (self.point.y - y).abs() as u64;
                    let steps = x_steps + y_steps;
                    Some(Intersection::new(Point::new(x, y), steps))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Point {
    fn origin() -> Point {
        Point::new(0, 0)
    }

    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn manhattan_distance(&self) -> u64 {
        self.x.abs() as u64 + self.y.abs() as u64
    }

    fn is_origin(&self) -> bool {
        self.x == 0 && self.y == 0
    }

    fn translate(&mut self, translation: &Translation) {
        match translation.direction {
            Direction::UP => self.y += translation.distance as i32,
            Direction::DOWN => self.y -= translation.distance as i32,
            Direction::LEFT => self.x -= translation.distance as i32,
            Direction::RIGHT => self.x += translation.distance as i32,
        }
    }
}

impl Circuit {
    fn run(&self) -> Result<(u64, u64), String> {
        let mut part_1: Option<u64> = None;
        let mut part_2: Option<u64> = None;

        for s0 in self.wires.0.segments.iter() {
            for s1 in self.wires.1.segments.iter() {
                if let Some(intersect) = s0.intersect(s1) {
                    if !intersect.point.is_origin() {
                        set_min(&mut part_1, intersect.point.manhattan_distance());
                        set_min(&mut part_2, intersect.steps);
                    }
                }
            }
        }

        match (part_1, part_2) {
            (Some(a), Some(b)) => Ok((a, b)),
            (None, None) => Err("Found neither part 1 nor part 2".to_string()),
            (None, Some(b)) => Err(format!("Could not find part 1: (?, {})", b)),
            (Some(a), None) => Err(format!("Could not find part 2: (?, {})", a)),
        }
    }
}

fn set_min(maybe_val: &mut Option<u64>, new_val: u64) {
    match maybe_val {
        Some(current) => *maybe_val = Some(std::cmp::min(*current, new_val)),
        None => *maybe_val = Some(new_val),
    }
}

impl std::str::FromStr for Circuit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let line_1 = lines.next().ok_or("First line of input missing")?;
        let line_2 = lines.next().ok_or("Second line of input missing")?;

        match lines.next() {
            Some(_) => Err("More than two lines of input found".to_string()),
            None => Ok(()),
        }?;

        let wire_1 = line_1.parse::<Wire>()?;
        let wire_2 = line_2.parse::<Wire>()?;

        Ok(Circuit {
            wires: (wire_1, wire_2),
        })
    }
}

impl std::str::FromStr for Wire {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let translations = s
            .split(',')
            .map(|word| word.parse::<Translation>())
            .collect::<Result<Vec<Translation>, Self::Err>>()?;

        let mut current_origin = Point::origin();
        let mut current_steps: u64 = 0;

        let segments = translations
            .iter()
            .map(|translation| {
                let segment = Segment::new(
                    current_origin.clone(),
                    (*translation).clone(),
                    current_steps,
                );

                current_origin.translate(translation);
                current_steps += translation.distance;
                segment
            })
            .collect::<Vec<Segment>>();

        Ok(Wire { segments })
    }
}

impl std::str::FromStr for Translation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut c = s.chars();
        let direction = match c.next() {
            Some('U') => Ok(Direction::UP),
            Some('D') => Ok(Direction::DOWN),
            Some('L') => Ok(Direction::LEFT),
            Some('R') => Ok(Direction::RIGHT),
            Some(unknown) => Err(format!("Bad Translation direction {}", unknown)),
            None => Err("No direction char found".to_string()),
        }?;

        let distance = c.as_str().parse::<u64>().map_err(|err| err.to_string())?;

        Ok(Translation::new(direction, distance))
    }
}

#[cfg(test)]
mod day_03 {
    use super::*;
    #[test]
    fn test_1() {
        let (p1, p2) = "R8,U5,L5,D3\nU7,R6,D4,L4"
            .parse::<Circuit>()
            .unwrap()
            .run()
            .unwrap();

        assert_eq!(p1, 6);
        assert_eq!(p2, 30);
    }

    #[test]
    fn test_2() {
        let (p1, p2) = "R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
                        U62,R66,U55,R34,D71,R55,D58,R83"
            .parse::<Circuit>()
            .unwrap()
            .run()
            .unwrap();

        assert_eq!(p1, 159);
        assert_eq!(p2, 610);
    }

    #[test]
    fn test_3() {
        let (p1, p2) = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
                        U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            .parse::<Circuit>()
            .unwrap()
            .run()
            .unwrap();

        assert_eq!(p1, 135);
        assert_eq!(p2, 410);
    }
}
