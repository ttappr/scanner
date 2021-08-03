
mod lexer;
use crate::lexer::*;

fn main() {
    // Text to be tokenized.
    let text = r#"if is_true {
                    foo_var = "hello!";
                 } else {
                    bar_var = 0;
                    foo_var = "String \\ \"with\" escapes.";
                 }"#;
                 
    let mut lexer = Lexer::new(text);
    
    // The lexer acts as an iterator that produces tokens.
    for token in &mut lexer {
        println!("{:#?}", token);
    }

    // The lexer's status indicates whether the lexer encountered an error
    // or the end of input was reached.
    println!("Lexer Status: {:?}", lexer.status());
}


