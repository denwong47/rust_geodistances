mod args;
mod data;


fn main() -> Result<(), ()> {
    let stdin_input = args::stdin::get_stdin();

    println!("{}", stdin_input);
    return Ok(());
}
