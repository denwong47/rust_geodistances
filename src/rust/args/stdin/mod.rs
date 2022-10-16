use std::io;
use std::io::BufRead;

/// Get the stdin.
/// Without converting this function to async, this will not be supporting any sort of
/// timeout at all, so if there is nothing piped into stdin, this function will block
/// forever until EOF (ctrl+D) is reached.
///
/// However since this is meant to be a backend for Python, which is the only supposed
/// caller, then this should be fine.
pub fn get_stdin() -> String {

    let mut stdin_input = String::new();
    let mut stdin_line = String::new();
    let stdin = io::stdin();

    while let Ok(stdin_len) = stdin.lock().read_line(&mut stdin_line) {
        if stdin_len == 0 {
            break;
        }

        stdin_input.push_str(&stdin_line);

        stdin_line.clear();
    }

    return stdin_input;
}
