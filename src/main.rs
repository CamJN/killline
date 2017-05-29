use std::io;
use std::io::prelude::*;
use std::fs;
use std::env;
use std::path;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut args = env::args().skip(1).peekable();//skip executable name

    let mut f = args.peek().map_or(None,|string|{
    	if path::Path::new(&string).is_file(){
	    fs::OpenOptions::new().read(true).write(true).create(true).open(string).ok()
	}else{
	    None
	}
    });
    
    let input : Box<io::BufRead> = match f.as_mut() {
        Some(&mut ref f) => {
	    args.next();//skip file name
            Box::new(io::BufReader::new(f.try_clone().unwrap()))
        },
        None => Box::new(stdin.lock())
    };

    let line_numbers: Vec<usize> = args.map(|s|s.parse::<usize>().expect("encountered a non integer argument")-1).collect();
    let contents:Vec<(usize,io::Result<String>)> = input.lines().enumerate().filter(|&(i,_)|!line_numbers.contains(&i)).collect();

    let mut output: Box<io::Write> = match f.as_mut() {
        Some(f) => {
            f.set_len(0).expect("failed to truncate file, aborting");
            Box::new(io::BufWriter::new(f.try_clone().unwrap()))
        },
        None => Box::new(stdout.lock())
    };

    for (_,line) in contents {
        output.write_all((line.unwrap()+"\n").as_bytes()).expect("error writing file");
    }
}
