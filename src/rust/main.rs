mod input_output;
mod data;


fn main() -> Result<(), ()> {
    let stdin_input = input_output::stdin::get_stdin();

    let io_arrays = data::structs::IOCoordinateLists::from_pickle(&stdin_input);

    input_output::stdout::display_bytes(&io_arrays.to_pickle());
    return Ok(());
}
