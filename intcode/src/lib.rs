use std::convert::TryFrom;

enum OpCode {
    ADD(ParamterMode, ParamterMode),
    MULTIPLY(ParamterMode, ParamterMode),
    INPUT,
    OUTPUT(ParamterMode),
    EXIT,
    TJUMP(ParamterMode, ParamterMode),
    FJUMP(ParamterMode, ParamterMode),
    LESSTHAN(ParamterMode, ParamterMode),
    EQUALS(ParamterMode, ParamterMode),
}

#[derive(Debug)]
enum ParamterMode {
    POSITION,
    IMMEDIATE,
}

impl TryFrom<i32> for ParamterMode {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ParamterMode::POSITION),
            1 => Ok(ParamterMode::IMMEDIATE),
            _ => Err(format!("Unknown paramter mode value {}", value)),
        }
    }
}

impl TryFrom<i32> for OpCode {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 {
            return Err(format!("Opcode cannot be negative, but received {}", value));
        }

        let div_mod =
            |dividend, divisor| -> (i32, i32) { (dividend / divisor, dividend % divisor) };

        let (modes, opcode_num) = div_mod(value, 100);

        let (opcode, modes_remainder) = match opcode_num {
            1 => {
                let (modes, mode_1) = div_mod(modes, 10);
                let (modes, mode_2) = div_mod(modes, 10);

                (
                    OpCode::ADD(
                        ParamterMode::try_from(mode_1)?,
                        ParamterMode::try_from(mode_2)?,
                    ),
                    modes,
                )
            }
            2 => {
                let (modes, mode_1) = div_mod(modes, 10);
                let (modes, mode_2) = div_mod(modes, 10);

                (
                    OpCode::MULTIPLY(
                        ParamterMode::try_from(mode_1)?,
                        ParamterMode::try_from(mode_2)?,
                    ),
                    modes,
                )
            }
            3 => (OpCode::INPUT, modes),
            4 => {
                let (modes, mode_1) = div_mod(modes, 10);

                (OpCode::OUTPUT(ParamterMode::try_from(mode_1)?), modes)
            }
            5 => {
                let (modes, mode_1) = div_mod(modes, 10);
                let (modes, mode_2) = div_mod(modes, 10);

                (
                    OpCode::TJUMP(
                        ParamterMode::try_from(mode_1)?,
                        ParamterMode::try_from(mode_2)?,
                    ),
                    modes,
                )
            }
            6 => {
                let (modes, mode_1) = div_mod(modes, 10);
                let (modes, mode_2) = div_mod(modes, 10);

                (
                    OpCode::FJUMP(
                        ParamterMode::try_from(mode_1)?,
                        ParamterMode::try_from(mode_2)?,
                    ),
                    modes,
                )
            }
            7 => {
                let (modes, mode_1) = div_mod(modes, 10);
                let (modes, mode_2) = div_mod(modes, 10);

                (
                    OpCode::LESSTHAN(
                        ParamterMode::try_from(mode_1)?,
                        ParamterMode::try_from(mode_2)?,
                    ),
                    modes,
                )
            }
            8 => {
                let (modes, mode_1) = div_mod(modes, 10);
                let (modes, mode_2) = div_mod(modes, 10);

                (
                    OpCode::EQUALS(
                        ParamterMode::try_from(mode_1)?,
                        ParamterMode::try_from(mode_2)?,
                    ),
                    modes,
                )
            }
            99 => (OpCode::EXIT, modes),
            _ => return Err(format!("Unknown opcode {} received", value)),
        };

        match modes_remainder {
            0 => Ok(opcode),
            _ => Err(format!(
                "Opcode value {} had remaining modes: {}",
                value, modes
            )),
        }
    }
}

#[derive(Clone)]
pub struct Program {
    int_codes: Vec<i32>,
    input: Option<i32>,
    output: Option<i32>,
}

impl Program {
    pub fn set_input(&mut self, input: i32) {
        self.input = Some(input);
    }

    pub fn set_noun(&mut self, noun: i32) {
        self.int_codes[1] = noun;
    }

    pub fn set_verb(&mut self, verb: i32) {
        self.int_codes[2] = verb;
    }

    pub fn get_zero(&self) -> i32 {
        self.int_codes[0]
    }

    pub fn get_output(&self) -> Option<i32> {
        self.output
    }

    pub fn execute(&mut self) {
        let mut i: usize = 0;
        while i < self.int_codes.len() {
            match OpCode::try_from(self.int_codes[i]).unwrap() {
                OpCode::ADD(mode_1, mode_2) => {
                    let x = match mode_1 {
                        ParamterMode::POSITION => self.int_codes[self.int_codes[i + 1] as usize],
                        ParamterMode::IMMEDIATE => self.int_codes[i + 1],
                    };
                    let y = match mode_2 {
                        ParamterMode::POSITION => self.int_codes[self.int_codes[i + 2] as usize],
                        ParamterMode::IMMEDIATE => self.int_codes[i + 2],
                    };
                    let target = self.int_codes[i + 3] as usize;

                    self.int_codes[target as usize] = x + y;
                    i += 4;
                }
                OpCode::MULTIPLY(mode_1, mode_2) => {
                    let x = match mode_1 {
                        ParamterMode::POSITION => self.int_codes[self.int_codes[i + 1] as usize],
                        ParamterMode::IMMEDIATE => self.int_codes[i + 1],
                    };
                    let y = match mode_2 {
                        ParamterMode::POSITION => self.int_codes[self.int_codes[i + 2] as usize],
                        ParamterMode::IMMEDIATE => self.int_codes[i + 2],
                    };
                    let target = self.int_codes[i + 3] as usize;

                    self.int_codes[target as usize] = x * y;
                    i += 4;
                }
                OpCode::INPUT => {
                    let target = self.int_codes[i + 1] as usize;
                    match self.input {
                        Some(v) => self.int_codes[target] = v,
                        None => panic!("No input set for INPUT opcode"),
                    }
                    i += 2;
                }
                OpCode::OUTPUT(mode_1) => {
                    let value = match mode_1 {
                        ParamterMode::POSITION => self.int_codes[self.int_codes[i + 1] as usize],
                        ParamterMode::IMMEDIATE => self.int_codes[i + 1],
                    };

                    self.output = Some(value);
                    println!("{}", value);
                    i += 2;
                }
                OpCode::TJUMP(mode_1, mode_2) => {
                    let x = match mode_1 {
                        ParamterMode::POSITION => self.int_codes[self.int_codes[i + 1] as usize],
                        ParamterMode::IMMEDIATE => self.int_codes[i + 1],
                    };
                    let y = match mode_2 {
                        ParamterMode::POSITION => self.int_codes[self.int_codes[i + 2] as usize],
                        ParamterMode::IMMEDIATE => self.int_codes[i + 2],
                    };

                    if x != 0 {
                        i = usize::try_from(y).unwrap();
                    } else {
                        i += 3;
                    }
                }

                OpCode::FJUMP(mode_1, mode_2) => {
                    let x = match mode_1 {
                        ParamterMode::POSITION => self.int_codes[self.int_codes[i + 1] as usize],
                        ParamterMode::IMMEDIATE => self.int_codes[i + 1],
                    };
                    let y = match mode_2 {
                        ParamterMode::POSITION => self.int_codes[self.int_codes[i + 2] as usize],
                        ParamterMode::IMMEDIATE => self.int_codes[i + 2],
                    };

                    if x == 0 {
                        i = usize::try_from(y).unwrap();
                    } else {
                        i += 3;
                    }
                }

                OpCode::LESSTHAN(mode_1, mode_2) => {
                    let x = match mode_1 {
                        ParamterMode::POSITION => self.int_codes[self.int_codes[i + 1] as usize],
                        ParamterMode::IMMEDIATE => self.int_codes[i + 1],
                    };
                    let y = match mode_2 {
                        ParamterMode::POSITION => self.int_codes[self.int_codes[i + 2] as usize],
                        ParamterMode::IMMEDIATE => self.int_codes[i + 2],
                    };
                    let target = self.int_codes[i + 3] as usize;

                    if x < y {
                        self.int_codes[target] = 1;
                    } else {
                        self.int_codes[target] = 0;
                    }

                    i += 4;
                }

                OpCode::EQUALS(mode_1, mode_2) => {
                    let x = match mode_1 {
                        ParamterMode::POSITION => self.int_codes[self.int_codes[i + 1] as usize],
                        ParamterMode::IMMEDIATE => self.int_codes[i + 1],
                    };
                    let y = match mode_2 {
                        ParamterMode::POSITION => self.int_codes[self.int_codes[i + 2] as usize],
                        ParamterMode::IMMEDIATE => self.int_codes[i + 2],
                    };
                    let target = self.int_codes[i + 3] as usize;

                    if x == y {
                        self.int_codes[target] = 1;
                    } else {
                        self.int_codes[target] = 0;
                    }

                    i += 4;
                }

                OpCode::EXIT => break,
            }
        }
    }
}

impl std::str::FromStr for Program {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let int_codes = s
            .trim()
            .split(',')
            .map(|num_str| num_str.parse::<i32>())
            .collect::<Result<Vec<i32>, Self::Err>>()?;
        Ok(Program {
            int_codes,
            input: None,
            output: None,
        })
    }
}

#[cfg(test)]
mod int_code {
    use super::*;

    fn test_program_state(code: &str, state: Vec<i32>) {
        let mut program = code.parse::<Program>().unwrap();
        program.execute();
        assert_eq!(program.int_codes, state);
    }

    fn test_program_inout(code: &str, input: i32, output: i32) {
        let mut program = code.parse::<Program>().unwrap();
        program.set_input(input);
        program.execute();
        assert_eq!(program.output.unwrap(), output);
    }

    #[test]
    fn day_02_basic_sum() {
        test_program_state("1,0,0,0,99", vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn day_02_basic_multiplication() {
        test_program_state("2,3,0,3,99", vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn day_02_complex_multiplication() {
        test_program_state("2,4,4,5,99,0", vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn day_02_complex_sum() {
        test_program_state("1,1,1,4,99,5,6,0,99", vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn day_02_complex() {
        test_program_state(
            "1,9,10,3,2,3,11,0,99,30,40,50",
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        );
    }

    #[test]
    fn day_03_inout() {
        let input = 20;
        test_program_inout("3,0,4,0,99", input, input);
    }

    #[test]
    fn day_03_paramter_modes() {
        test_program_state("1002,4,3,4,33", vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn day_03_negatives() {
        test_program_state("1101,100,-1,4,0", vec![1101, 100, -1, 4, 99]);
    }

    #[test]
    fn day_03_position_equals() {
        // input == 8 -> output
        test_program_inout("3,9,8,9,10,9,4,9,99,-1,8", 8, 1);
        test_program_inout("3,9,8,9,10,9,4,9,99,-1,8", 1, 0);
    }

    #[test]
    fn day_03_immediate_equals() {
        // input == 8 -> output
        test_program_inout("3,3,1108,-1,8,3,4,3,99", 8, 1);
        test_program_inout("3,3,1108,-1,8,3,4,3,99", 1, 0);
    }

    #[test]
    fn day_03_position_less_than() {
        // input < 8 -> output
        test_program_inout("3,9,7,9,10,9,4,9,99,-1,8", 8, 0);
        test_program_inout("3,9,7,9,10,9,4,9,99,-1,8", 7, 1);
    }

    #[test]
    fn day_03_immediate_less_than() {
        // input < 8 -> output
        test_program_inout("3,3,1107,-1,8,3,4,3,99", 8, 0);
        test_program_inout("3,3,1107,-1,8,3,4,3,99", 7, 1);
    }

    #[test]
    fn day_03_position_jump() {
        // input != 0 -> output
        test_program_inout("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 0, 0);
        test_program_inout("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 4, 1);
    }

    #[test]
    fn day_03_immediate_jump() {
        // input != 0 -> output
        test_program_inout("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 0, 0);
        test_program_inout("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 4, 1);
    }

    #[test]
    fn day_03_complex() {
        // input <  8 -> 999
        // input == 8 -> 1000
        // input >  8 -> 1001
        let code = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,\
                    1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,\
                    999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";

        test_program_inout(code, 7, 999);
        test_program_inout(code, 8, 1000);
        test_program_inout(code, 9, 1001);
    }
}
