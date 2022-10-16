mod input_output;
use input_output::pickle::traits::{PickleImport, PickleExport};
mod data;

#[allow(unused_variables)]
fn main() -> Result<(), ()> {
    let stdin_input = input_output::stdin::get_stdin();

    let array_inputs = data::structs::IOCoordinateLists::from_pickle(&stdin_input);

    let array_outputs = data::structs::IOResultArray::new(array_inputs.shape());

    input_output::stdout::display_bytes(&array_inputs.to_pickle());

    return Ok(());
}
