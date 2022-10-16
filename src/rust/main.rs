mod input_output;
use input_output::pickle::traits::{PickleImport, PickleExport};
mod data;


fn main() -> Result<(), ()> {
    let stdin_input = input_output::stdin::get_stdin();

    let io_arrays = data::structs::IOCoordinateLists::from_pickle(&stdin_input);

    let random_output = data::structs::IOResultArray::new((15, 5));
    input_output::stdout::display_bytes(&random_output.to_pickle());
    return Ok(());
}
