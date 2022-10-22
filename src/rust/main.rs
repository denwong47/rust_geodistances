mod input_output;
use input_output::pickle::traits::{PickleImport, PickleExport};
mod data;
mod geodistances;
mod config;

use data::traits::Slicable;

#[allow(unused_variables)]
fn main() -> Result<(), ()> {
    let stdin_input = input_output::stdin::get_stdin();

    let array_inputs = data::structs::IOCoordinateLists::from_pickle(&stdin_input);

    // let mut array_outputs = data::structs::IOResultArray::like_input_full(&array_inputs, data::structs::CalculationResult::Geodistance(Some(0.)));

    let mut array_outputs = data::structs::IOResultArray::like_input(&array_inputs);

    let origin = (0,0);
    let size = array_outputs.shape();

    println!("{:?}", array_inputs.slice((1,1), (3,2)));

    array_outputs.splice(
        origin,
        geodistances::distance_map_unthreaded::<geodistances::Vincenty>(&array_inputs, origin, size)
    );

    // let mut slice_outputs = data::structs::IOResultArray::full((5, 10), data::structs::CalculationResult::Geodistance(Some(PI)));

    // array_outputs.splice((16, 5), slice_outputs);

    // println!("{:?}", array_outputs);

    input_output::stdout::display_bytes(&array_outputs.to_pickle());

    return Ok(());
}
