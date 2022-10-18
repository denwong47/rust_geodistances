use std::f64::consts::PI;

mod input_output;
use input_output::pickle::traits::{PickleImport, PickleExport};
mod data;
mod geodistances;
use geodistances::traits::{CalculateDistance, CheckDistance};

use rand::Rng;


#[allow(unused_variables)]
fn main() -> Result<(), ()> {
    let stdin_input = input_output::stdin::get_stdin();

    let array_inputs = data::structs::IOCoordinateLists::from_pickle(&stdin_input);

    // let mut array_outputs = data::structs::IOResultArray::like_input_full(&array_inputs, data::structs::CalculationResult::Geodistance(Some(0.)));

    let array_outputs = geodistances::distance_map_unthreaded::<geodistances::Vincenty>(&array_inputs);

    // let mut slice_outputs = data::structs::IOResultArray::full((5, 10), data::structs::CalculationResult::Geodistance(Some(PI)));

    // array_outputs.splice((16, 5), slice_outputs);

    // println!("{:?}", array_outputs);

    input_output::stdout::display_bytes(&array_outputs.to_pickle());

    return Ok(());
}
