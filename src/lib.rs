//// Library of utilities for the advent of code, 2021.

pub mod utils {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    /// Portably remove newline character(s) from end of string `s`
    fn trim_newline(s: &mut String) {
        if s.ends_with('\n') {
            s.pop();
            if s.ends_with('\r') {
                s.pop();
            }
        }
    }
    /// Read the next text line from `reader` and fill in `line` with its value.
    /// Return `true` if okay, `false` on EOF. On error, which is unlikely, print
    /// a message on stderr and return `false`.
    ///
    /// Very similar to the C `fgets()` routine, hence the name.
    pub fn fgets(reader: &mut BufReader<File>, line: &mut String) -> bool {
        line.clear();
        match reader.read_line(line) {
            Ok(0) => false,
            Ok(_) => {
                trim_newline(line);
                true
            }
            Err(err) => {
                eprintln!("Error {}", err);
                false
            }
        }
    }
}
