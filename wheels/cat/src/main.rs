use std::env;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::fs::File;

fn main() {

    let arguments: Vec<String> = env::args().collect();

    if arguments.len() < 2 {
        let stdin = io::stdin();
        let mut reader = stdin.lock();
        output(&mut reader);
    } else {
        for file_path in &arguments[1..] {
            let f = match File::open(file_path) {
                Err(_) => panic!("none"),
                Ok(file) => file,
            };
            let mut reader = BufReader::new(f);
            output(&mut reader);
        }
    }

}

fn output(reader: &mut BufRead) {
    let mut input = String::new();
    while reader.read_line(&mut input).unwrap() > 0 {
        io::stdout().write(input.as_bytes()).unwrap();
        input.clear();
    }
}
