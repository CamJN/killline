use std::io;
use std::io::prelude::*;
use std::fs;
use std::env;
use std::path;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut args = env::args().skip(1).peekable();
    let is_file = args.peek().map_or(false,|string|path::Path::new(&string).is_file());
    let (input, mut output) : (Box<io::BufRead>, Box<io::Write>) = if is_file {
        let f = fs::File::open(args.next().unwrap()).unwrap();
        (Box::new(io::BufReader::new(f.try_clone().unwrap())),
         Box::new(io::BufWriter::new(f.try_clone().unwrap())))
    } else {
        (Box::new(stdin.lock()), Box::new(stdout.lock()))
    };

    let line_numbers: Vec<usize> = args.map(|s|s.parse::<usize>().expect("encountered a non integer argument")-1).collect();

    for (_,line) in input.lines().enumerate().filter(|&(i,_)|!line_numbers.contains(&i)) {
        output.write_all((line.unwrap()+"\n").as_bytes()).expect("error writing file");
    }
}
