use std::io;
use std::io::BufRead;

/// Get the stdin.
/// Without converting this function to async, this will not be supporting any sort of
/// timeout at all, so if there is nothing piped into stdin, this function will block
/// forever until EOF (ctrl+D) is reached.
///
/// However since this is meant to be a backend for Python, which is the only supposed
/// caller, then this should be fine.
pub fn get_stdin() -> Vec<u8> {

    let mut output_bytes:Vec<u8> = Vec::new();
    let mut buffer_bytes:Vec<u8> = Vec::new();
    let stdin = io::stdin();

    // The delimiting character doesn't really matter - we are just looping until done.
    while let Ok(read_len) = stdin.lock().read_until(b'\x00', &mut buffer_bytes) {
        if read_len == 0 {
            // Reading has ended.
            break
        }
        output_bytes.append(&mut buffer_bytes);
    }

    return output_bytes;
}
