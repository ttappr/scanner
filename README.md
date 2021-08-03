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

    // The lexer's status indicates whether the lexer finished in an error state
    // or the end of input was reached.
    println!("Lexer Status: {:?}", lexer.status());
}
```

Output:

```console
Token {
    type_: Keyword,
    text: "if",
    line: 0,
    col: 0,
}
Token {
    type_: Identifier,
    text: "is_true",
    line: 0,
    col: 3,
}
Token {
    type_: LParen,
    text: "{",
    line: 0,
    col: 11,
}
Token {
    type_: Identifier,
    text: "foo_var",
    line: 1,
    col: 20,
}
Token {
    type_: Operator,
    text: "=",
    line: 1,
    col: 28,
}
Token {
    type_: StringLiteral,
    text: "\"hello!\"",
    line: 1,
    col: 30,
}
Token {
    type_: Semicolon,
    text: ";",
    line: 1,
    col: 38,
}
Token {
    type_: RParen,
    text: "}",
    line: 2,
    col: 17,
}
Token {
    type_: Keyword,
    text: "else",
    line: 2,
    col: 19,
}
Token {
    type_: LParen,
    text: "{",
    line: 2,
    col: 24,
}
Token {
    type_: Identifier,
    text: "bar_var",
    line: 3,
    col: 20,
}
Token {
    type_: Operator,
    text: "=",
    line: 3,
    col: 28,
}
Token {
    type_: NumericLiteral,
    text: "0",
    line: 3,
    col: 30,
}
Token {
    type_: Semicolon,
    text: ";",
    line: 3,
    col: 31,
}
Token {
    type_: Identifier,
    text: "foo_var",
    line: 4,
    col: 20,
}
Token {
    type_: Operator,
    text: "=",
    line: 4,
    col: 28,
}
Token {
    type_: StringLiteral,
    text: "\"String \\\\ \\\"with\\\" escapes.\"",
    line: 4,
    col: 30,
}
Token {
    type_: Semicolon,
    text: ";",
    line: 4,
    col: 59,
}
Token {
    type_: RParen,
    text: "}",
    line: 5,
    col: 17,
}
Lexer Status: EndOfStream
```
