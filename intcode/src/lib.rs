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
pub struct Program {
    pub int_code: Vec<usize>,
}

impl Program {
    fn new(int_code: Vec<usize>) -> Program {
        Program { int_code }
    }

    pub fn init(&mut self, noun: usize, verb: usize) {
        self.int_code[1] = noun;
        self.int_code[2] = verb;
    }

    pub fn execute(&mut self) {
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

#[cfg(test)]
mod int_code {
    use super::*;
    
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
}
