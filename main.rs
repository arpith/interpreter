pub mod token;
pub mod tokenizer;
pub mod parser;

use parser::Parser;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("need a filename :)");
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);

//    let mut tokenizer = Tokenizer::new(&mut contents);

  //  loop {
    //    let tok = tokenizer.next();
      //  println!("{:?}", tok);
        //if tok == Token::EndOfFile {
          //  break;
        //}
    //}

    let mut parser = Parser::new(&mut contents);
    parser.parse();
}
