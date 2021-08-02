#![allow(unused_imports)]

mod lexer;
use crate::lexer::*;

fn main() {
    // Text to be tokenized.
    let     text  = "if is_true { abracadabra = 42; } else { abracadabra = 0; }";
    let mut lexer = Lexer::new(text);
    
    // The lexer acts as an iterator that produces tokens.
    for token in &mut lexer {
        println!("{:?}", token);
    }

    // The lexer's status indicates whether the lexer finished in an error state
    // or the end of input was reached.
    println!("Lexer Status: {:?}", lexer.status());
}


