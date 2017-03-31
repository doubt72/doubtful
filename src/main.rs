extern crate doubtful;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use doubtful::tokenizer;
use doubtful::parser;
use doubtful::evaluator;

fn main() {
  let args: Vec<String> = env::args().collect();

  // TODO: better command line
  if args.len() != 2 {
    panic!("incorrect number of arguments: expecting source file as argument");
  }
  let filename = &args[1];

  match File::open(&filename) {
    Ok(mut file) => {
      let mut source = String::new();
      match &file.read_to_string(&mut source) {
        &Ok(_) => {
          let tokens = tokenizer::tokenize(&source);
          let block = parser::parse(&tokens);
          evaluator::evaluate(&block);
        },
        _ => {
          panic!("failed to read source file");
        }
      }
    },
    _ => {
      panic!("failed to open source file");
    },
  }
}
