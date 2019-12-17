extern crate intcode;

fn main() {
    let base_program = std::fs::read_to_string("src/inputs/day_02")
        .expect("Failed to read input file")
        .parse::<intcode::Program>()
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