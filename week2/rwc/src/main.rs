use std::env;
use std::process;
use std::io::{self,BufRead};
use std::fs::File;

fn read_file_lines(filename: &String) -> Result<Vec<String>,io::Error>{
    let file = File::open(filename)?;
    let mut ret = Vec::new();
    for line in io::BufReader::new(file).lines(){
        let line_str=line?;        
        ret.push(line_str);
    }
    Ok(ret)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    let ret = read_file_lines(&filename).unwrap();
    println!("{}",ret.len());
    // Your code here :)
}
