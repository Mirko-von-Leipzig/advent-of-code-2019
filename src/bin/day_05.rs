fn main() {
    let base_diagnostic = std::fs::read_to_string("src/inputs/day_05")
        .expect("Failed to read input file")
        .parse::<intcode::Program>()
        .expect("Failed to parse input");

    let mut diagnostic_1 = base_diagnostic.clone();
    diagnostic_1.set_input(1);
    diagnostic_1.execute();


    let mut diagnostic_2 = base_diagnostic;
    diagnostic_2.set_input(5);
    diagnostic_2.execute();


    let part_1 = diagnostic_1.get_output();
    let part_2 = diagnostic_2.get_output();

    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);
}
