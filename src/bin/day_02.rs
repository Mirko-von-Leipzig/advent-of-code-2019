fn main() {
    let base_program = std::fs::read_to_string("src/inputs/day_02")
        .expect("Failed to read input file")
        .parse::<Program>()
        .expect("Failed to parse input");

    let mut program_1 = base_program.clone();
    program_1.init(12, 2);

    program_1.execute();
    println!("Part 1: {}", program_1.int_code[0]);

    // part 2, brute force algorithm
    let target: usize = 19_690_720;
    let mut part_2: Option<usize> = None;
    'outer: for i in 0..99 {
        for j in 0..99 {
            let mut program_2 = base_program.clone();
            program_2.init(i, j);
            program_2.execute();

            if program_2.int_code[0] == target {
                part_2 = Some(100 * i + j);
                break 'outer;
            }
        }
    }

    match part_2 {
        Some(x) => println!("Part 2: {}", x),
        None => println!(
            "Part 2: failed to find a noun, verb pair that results in {}",
            target
        ),
    }
}

enum OpCode {
    ADD = 1,
    MULTIPLY = 2,
    FINISH = 99,
}

impl OpCode {
    fn from_usize(i: usize) -> Option<OpCode> {
        match i {
            1 => Some(Self::ADD),
            2 => Some(Self::MULTIPLY),
            99 => Some(Self::FINISH),
            _ => None,
        }
    }
}

#[derive(Clone)]
struct Program {
    int_code: Vec<usize>,
}

impl Program {
    fn new(int_code: Vec<usize>) -> Program {
        Program { int_code }
    }

    fn init(&mut self, noun: usize, verb: usize) {
        self.int_code[1] = noun;
        self.int_code[2] = verb;
    }

    fn execute(&mut self) {
        let mut i: usize = 0;
        while i < self.int_code.len() {
            match OpCode::from_usize(self.int_code[i]).expect("Unknown OpCode found!") {
                OpCode::ADD => {
                    let a = self.int_code[i + 3];
                    self.int_code[a] =
                        self.int_code[self.int_code[i + 1]] + self.int_code[self.int_code[i + 2]];
                    i += 4;
                }
                OpCode::MULTIPLY => {
                    let a = self.int_code[i + 3];
                    self.int_code[a] =
                        self.int_code[self.int_code[i + 1]] * self.int_code[self.int_code[i + 2]];
                    i += 4;
                }
                OpCode::FINISH => break,
            }
        }
    }
}

impl std::str::FromStr for Program {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let int_codes: Result<Vec<usize>, Self::Err> = s
            .trim()
            .split(',')
            .map(|num_str| num_str.parse::<usize>())
            .collect();
        Ok(Program::new(int_codes?))
    }
}

#[test]
fn test_execute() {
    let test_program = |input: &str, output: Vec<usize>| {
        let mut program = input.parse::<Program>().unwrap();
        program.execute();
        assert_eq!(program.int_code, output);
    };

    test_program("1,0,0,0,99", vec![2, 0, 0, 0, 99]);
    test_program("2,3,0,3,99", vec![2, 3, 0, 6, 99]);
    test_program("2,4,4,5,99,0", vec![2, 4, 4, 5, 99, 9801]);
    test_program("1,1,1,4,99,5,6,0,99", vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    test_program(
        "1,9,10,3,2,3,11,0,99,30,40,50",
        vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
    );
}
