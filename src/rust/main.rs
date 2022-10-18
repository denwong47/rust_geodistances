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

    let mut array_outputs = data::structs::IOResultArray::like_input_full(&array_inputs, data::structs::CalculationResult::Geodistance(Some(1.2)));



    input_output::stdout::display_bytes(&array_outputs.to_pickle());

    return Ok(());
}
