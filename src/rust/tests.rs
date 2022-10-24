mod input_output;
use input_output::pickle::traits::{PickleImport, PickleExport};
mod data;
mod geodistances;
mod config;

use data::traits::Slicable;

fn main() {
    let shape:(usize, usize) = (10, 10);
    let mut array_outputs = data::structs::IOResultArray::new((10,10));

    for row in 0..shape.0 {
        for col in 0..shape.1 {
            array_outputs.array[row][col] = data::structs::CalculationResult::Geodistance(
                Some((row*10 + col) as f64)
            )
        }
    }

    println!("{:?}", array_outputs.slice((2,4), (3,20)));
}
