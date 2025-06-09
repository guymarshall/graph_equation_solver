use crate::Coordinate;

pub(crate) fn get_coordinates_from_file(file_path: &str) -> Vec<Coordinate> {
    let input: String = std::fs::read_to_string(file_path).expect("input.csv should exist");

    input
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
        .collect()
}

pub(crate) fn log_attempts(file_path: &str, attempts: &str) {
    std::fs::write(file_path, attempts).expect("Fit attempt should be written to log.txt");
}

pub(crate) fn save_formula(file_path: &str, formula: &str) {
    std::fs::write(file_path, formula).expect("Formula should be written to output.txt");
}
