use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::path::Path;
use std::process;

use lozenge::scanner::Scanner;
use lozenge::parser::Parser;
use lozenge::interp::Interp;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: lozenge <file>");
        process::exit(64);
    } else {
        run_file(&args[1])
    }
}

fn run_file(file: &String) {
    let path = Path::new(file);
    let mut file = File::open(&path)
        .expect("Failed to open file");

    let mut source = String::new();
    file.read_to_string(&mut source)
        .expect("Failed to read file");

    let source: Vec<char> = source.chars().collect();
    run(source);
}

fn run(source: Vec<char>) {
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens();
    //println!("{:#?}", scanner.tokens);

    let mut parser = Parser::new(scanner.tokens);
    let program = parser.parse();
    if program.is_ok() {
        let mut interp = Interp::new();
        interp.eval(program.unwrap());
        println!("{:?}", interp.env);
    } else {
        println!("error: {}", program.unwrap_err());
    }
    //println!("{:?}", parser.current);
    //println!("{:?}", parser.tokens.len());
    //// println!("{:?}", parser.tokens);
    //println!("stopped on token: {:?}", parser.tokens[parser.current]);
}
