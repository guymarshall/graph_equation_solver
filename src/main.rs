mod coordinate;
mod formula;
mod utilities;
use coordinate::Coordinate;

use crate::{
    formula::Formula,
    utilities::{get_coordinates_from_file, log_attempts, save_formula},
};

fn main() {
    let coordinates: Vec<Coordinate> = get_coordinates_from_file("input.csv");

    coordinates.iter().for_each(|coordinate: &Coordinate| {
        println!("Coordinate: {}", coordinate);
    });

    log_attempts("fit_attempts.log", "0, 0\n1, 1\n2, 2");

    let formula: Formula = Formula::new(5, 2);
    save_formula("output.txt", formula.to_string().as_str());
}
