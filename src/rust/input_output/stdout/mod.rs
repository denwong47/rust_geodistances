/// =============
///  Data Output
/// =============
///
/// This package relies on stdout to output its bytes data.
/// This module helps printing &[u8] arrays in the correct manner - println! will not
/// print bytes in a useful way.
use std::io;
use std::io::Write;

#[allow(dead_code)]
pub fn display_bytes(data:&[u8]) {
    let mut stdout = io::stdout();
    if let Ok(_) = stdout.write_all(data){};

    return ();
}
