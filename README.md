# scanner
A simple example project for a hand written lexer that can be used to scan and 
tokenize text. This is simply an example project to pull ideas from; the code
in its current state has not been rigorously tested.

This approach can be useful for writing simple text tokenizers or scanners. 
Alternatives to this approach include regular expressions as provided by the 
[regex crate](https://docs.rs/regex/1.5.4/regex/), and parser generators like 
[ANTLR](https://docs.rs/antlr-rust/0.2.0/antlr_rust/) for more extensive
text processing requirements.

## Example

```rust
use lexer::*;

fn main() {
    // Text to be tokenized.
    let text = r#"if is_true {
                    foo_var = "hello";
                 } else {
                    bar_var = 0;
                    foo_var = "String \"with\" escape";
                 }"#;
                 
    let mut lexer = Lexer::new(text);
    
    // The lexer acts as an iterator that produces tokens.
    for token in &mut lexer {
        println!("{:?}", token);
    }

    // The lexer's status indicates whether the lexer finished in an error state
    // or the end of input was reached.
    println!("Lexer Status: {:?}", lexer.status());
}
```
