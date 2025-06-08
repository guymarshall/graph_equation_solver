mod coordinate;
use coordinate::Coordinate;

fn main() {
    // TODO: extract file functions to separate module
    let input: String = std::fs::read_to_string("input.csv").expect("input.csv should exist");
    let coordinates: Vec<Coordinate> = input
        .lines()
        .map(|line: &str| {
            let parts: Vec<&str> = line.split(',').collect();

            Coordinate::new(
                parts[0]
                    .parse::<i32>()
                    .expect("parts[0] should be a valid integer"),
                parts[1]
                    .parse::<i32>()
                    .expect("parts[1] should be a valid integer"),
            )
        })
        .collect();

    coordinates.iter().for_each(|coordinate: &Coordinate| {
        println!("Coordinate: {}", coordinate);
    });

    // TODO: write guesses to log file
    std::fs::write("fit_attempts.log", "0, 0\n1, 1\n2, 2")
        .expect("Fit attempt should be written to log.txt");

    let formula: &str = "y = 2x + 3";
    std::fs::write("output.txt", formula).expect("formula should be written to output.txt");
}
