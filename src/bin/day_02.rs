fn main() {
    let base_program = std::fs::read_to_string("src/inputs/day_02")
        .expect("Failed to read input file")
        .parse::<intcode::Program>()
        .expect("Failed to parse input");

    let mut program_1 = base_program.clone();
    program_1.set_noun(12);
    program_1.set_verb(2);

    program_1.execute();
    println!("Part 1: {} == 5482655", program_1.get_zero());

    // part 2, brute force algorithm
    let target: i32 = 19_690_720;

    let mut part_2: Option<i32> = None;
    'outer: for i in 0..99 {
        for j in 0..99 {
            let mut program_2 = base_program.clone();
            program_2.set_noun(i);
            program_2.set_verb(j);
            program_2.execute();

            if program_2.get_zero() == target {
                part_2 = Some(100 * i + j);
                break 'outer;
            }
        }
    }

    match part_2 {
        Some(x) => println!("Part 2: {} == 4967", x),
        None => println!(
            "Part 2: failed to find a noun, verb pair that results in {}",
            target
        ),
    }
}