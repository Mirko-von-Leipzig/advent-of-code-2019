use itertools::Itertools;

fn main() {
    let code = std::fs::read_to_string("src/inputs/day_07")
        .expect("Failed to read input file")
        .parse::<intcode::Program>()
        .expect("Failed to parse input");

    let amp_control = AmplifierController::new(code);

    let part_1 = (0..=4)
        .permutations(5)
        .map(|i| amp_control.run_sequence((i[0], i[1], i[2], i[3], i[4])))
        .max()
        .unwrap()
        .unwrap();

    let part_2 = (5..=9)
        .permutations(5)
        .map(|i| amp_control.run_feedback_loop((i[0], i[1], i[2], i[3], i[4])))
        .max()
        .unwrap()
        .unwrap();

    println!("Part 1: {} == 255840", part_1);
    println!("Part 2: {} == 84088865", part_2);
}

struct AmplifierController {
    code: intcode::Program,
}

impl AmplifierController {
    fn new(code: intcode::Program) -> AmplifierController {
        AmplifierController { code }
    }

    fn run_sequence(&self, phases: (i32, i32, i32, i32, i32)) -> Result<i32, String> {
        let a_out = self.run_amplifier(phases.0, 0)?;
        let b_out = self.run_amplifier(phases.1, a_out)?;
        let c_out = self.run_amplifier(phases.2, b_out)?;
        let d_out = self.run_amplifier(phases.3, c_out)?;

        self.run_amplifier(phases.4, d_out)
    }

    fn run_feedback_loop(&self, phases: (i32, i32, i32, i32, i32)) -> Result<i32, String> {
        let mut a_amp = self.code.clone();
        let mut b_amp = self.code.clone();
        let mut c_amp = self.code.clone();
        let mut d_amp = self.code.clone();
        let mut e_amp = self.code.clone();

        a_amp.push_back_input(phases.0);
        b_amp.push_back_input(phases.1);
        c_amp.push_back_input(phases.2);
        d_amp.push_back_input(phases.3);
        e_amp.push_back_input(phases.4);

        let mut a_in = 0;

        loop {
            a_amp.push_back_input(a_in);
            a_amp.execute();

            b_amp.push_back_input(a_amp.pop_output().unwrap());
            b_amp.execute();
            c_amp.push_back_input(b_amp.pop_output().unwrap());
            c_amp.execute();
            d_amp.push_back_input(c_amp.pop_output().unwrap());
            d_amp.execute();
            e_amp.push_back_input(d_amp.pop_output().unwrap());
            let status = e_amp.execute();

            a_in = e_amp.pop_output().unwrap();

            if status == intcode::ProgramState::Complete {
                break;
            }
        }

        Ok(a_in)
    }

    fn run_amplifier(&self, phase: i32, input: i32) -> Result<i32, String> {
        let mut code = self.code.clone();

        code.push_back_input(phase);
        code.push_back_input(input);

        let _ = code.execute();

        match code.pop_latest_output() {
            Some(output) => Ok(output),
            None => Err("Amplifier has no output!".to_string()),
        }
    }
}

#[cfg(test)]
mod day_07 {
    use super::*;

    fn test_sequence(code_str: &str, input_sequence: (i32, i32, i32, i32, i32), output: i32) {
        let code = code_str.parse::<intcode::Program>().unwrap();
        let uut = AmplifierController::new(code);

        assert_eq!(uut.run_sequence(input_sequence).unwrap(), output);
    }

    fn test_feedback(code_str: &str, input_sequence: (i32, i32, i32, i32, i32), output: i32) {
        let code = code_str.parse::<intcode::Program>().unwrap();
        let uut = AmplifierController::new(code);

        assert_eq!(uut.run_feedback_loop(input_sequence).unwrap(), output);
    }

    #[test]
    fn part_1_1() {
        test_sequence(
            "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0",
            (4, 3, 2, 1, 0),
            43210,
        );
    }

    #[test]
    fn part_1_2() {
        test_sequence(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,\
             101,5,23,23,1,24,23,23,4,23,99,0,0",
            (0, 1, 2, 3, 4),
            54321,
        );
    }

    #[test]
    fn part_1_3() {
        test_sequence(
            "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,\
             1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0",
            (1, 0, 4, 3, 2),
            65210,
        );
    }

    #[test]
    fn part_2_1() {
        test_feedback(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,\
             27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
            (9, 8, 7, 6, 5),
            139_629_729,
        );
    }

    #[test]
    fn part_2_2() {
        test_feedback(
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,\
             -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,\
             53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
            (9, 7, 8, 5, 6),
            18_216,
        );
    }
}
