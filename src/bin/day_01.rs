fn main() {
    let modules = std::fs::read_to_string("src/inputs/day_01")
        .expect("Failed to read input file")
        .parse::<Modules>()
        .expect("Failed to parse input");

    let part_1 = modules.part_1();
    let part_2 = modules.part_2();

    println!("Day 01, part 1: {}", part_1);
    println!("Day 01, part 2: {}", part_2);
}

#[derive(Debug)]
struct Module {
    mass: u32,
}

impl Module {
    fn new(mass: u32) -> Module {
        Module { mass }
    }

    fn calculate_fuel(mass: u32) -> i64 {
        (mass as i64) / 3 - 2
    }

    fn part_1(&self) -> u32 {
        Self::calculate_fuel(self.mass) as u32
    }

    fn part_2(&self) -> u32 {
        let mut total: u32 = 0;
        let mut tmp: i64 = Self::calculate_fuel(self.mass);
        while tmp > 0 {
            total += tmp as u32;
            tmp = Self::calculate_fuel(tmp as u32);
        }
        total
    }
}

impl std::str::FromStr for Module {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Module::new(s.parse::<u32>()?))
    }
}

#[derive(Debug)]
struct Modules {
    modules: Vec<Module>,
}

impl Modules {
    fn from_vec(modules: Vec<Module>) -> Modules {
        Modules { modules }
    }

    fn part_1(&self) -> u32 {
        self.modules.iter().map(|module| module.part_1()).sum()
    }

    fn part_2(&self) -> u32 {
        self.modules.iter().map(|module| module.part_2()).sum()
    }
}

impl std::str::FromStr for Modules {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tmp: Result<Vec<Module>, Self::Err> =
            s.lines().map(|line| line.parse::<Module>()).collect();
        Ok(Modules::from_vec(tmp?))
    }
}

#[test]
fn test_part_1() {
    assert_eq!(Module::new(12).part_1(), 2);
    assert_eq!(Module::new(14).part_1(), 2);
    assert_eq!(Module::new(1969).part_1(), 654);
    assert_eq!(Module::new(100_756).part_1(), 33583);
}

#[test]
fn test_part_2() {
    assert_eq!(Module::new(14).part_2(), 2);
    assert_eq!(Module::new(1969).part_2(), 966);
    assert_eq!(Module::new(100_756).part_2(), 50346);
}

#[test]
fn test_module_parse() {
    assert_eq!("12".parse::<Module>().unwrap().mass, 12);
    assert_eq!("1234".parse::<Module>().unwrap().mass, 1234);
}
