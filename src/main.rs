use std::io;
use std::io::prelude::*;
use std::fs;
use std::env;
use std::path;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut args = env::args().skip(1).peekable();//skip executable name
    let name:Option<path::PathBuf> = args.peek().map(path::PathBuf::from);

    let mut f = name.as_ref().and_then(|path| if path.is_file() {
	fs::File::open(path).ok()
    }else{
	None
    });
    
    let input : Box<io::BufRead> = match f {
        Some(f) => {
	    args.next();//skip file name
            Box::new(io::BufReader::new(f))
        },
        None => Box::new(stdin.lock())
    };

    let line_numbers: Vec<usize> = args.map(|s|s.parse::<usize>().expect("encountered a non integer argument")-1).collect();
    let contents:Vec<(usize,io::Result<String>)> = input.lines().enumerate().filter(|&(i,_)|!line_numbers.contains(&i)).collect();

    f = name.as_ref().and_then(|path| if path.is_file() {
        fs::File::create(path).ok()
    }else{
	None
    });

    let mut output: Box<io::Write> = match f {
        Some(f) => Box::new(io::BufWriter::new(f)),
        None => Box::new(stdout.lock())
    };

    for (_,line) in contents {
        output.write_all((line.unwrap()+"\n").as_bytes()).expect("error writing file");
    }
}
