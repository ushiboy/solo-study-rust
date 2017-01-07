use std::io;
use std::io::Write;

fn main() {
    let mut input = String::new();
    while io::stdin().read_line(&mut input).unwrap() > 0 {
        io::stdout().write(input.as_bytes()).unwrap();
        input.clear();
    }
}
