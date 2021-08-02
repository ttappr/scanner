#![allow(unused_imports, dead_code)]

use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::str::Chars;


use lazy_static::lazy_static;


lazy_static! {
    static ref KEYWORDS: HashSet<&'static str> = {
        vec!["if", "else", "for", "while"].into_iter().collect()
    };
}

#[derive(Debug)]
pub enum TokenType 
{
    Keyword,
    Identifier,
    StringLiteral,
    NumericLiteral,
    Operator,
    LParen,
    RParen,
    Semicolon,
}

#[derive(Debug)]
pub struct Token<'input> 
{
    type_ : TokenType,
    text  : &'input str,
    line  : usize,
    col   : usize,
}

impl<'input> Token<'input>
{
    fn new(type_: TokenType, text: &'input str, line: usize, col: usize) -> Self
    {
        Token { type_, text, line, col }
    }
}

#[derive(Debug)]
pub enum LexerError
{
    GeneralError { message : String },
    UnrecognizedStart { message: String, line: usize, col: usize },
}
impl Error for LexerError { }

impl fmt::Display for LexerError 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        use LexerError::*;
        match self {
            GeneralError      { message     } => write!(f, "{}", message),
            UnrecognizedStart { message, .. } => write!(f, "{}", message),
        }
    }
}

#[derive(Debug)]
pub enum LexerStatus
{
    Okay,
    EndOfStream,
    Error(LexerError),
}

pub struct Lexer<'input>
{
    status  : LexerStatus,
    text    : &'input str,
    chars   : Chars<'input>,
    buf     : Option<char>,
    offset  : usize,
    line    : usize,
    col     : usize,
}

impl<'input> Lexer<'input>
{
    pub fn new(text: &'input str) -> Self
    {
        Lexer { 
            status  : LexerStatus::Okay, 
            text, 
            chars   : text.chars(),
            buf     : None,
            offset  : 0,
            line    : 0,
            col     : 0,
        }
    }
    pub fn status(&self) -> &LexerStatus
    {
        &self.status
    }
    fn next_char(&mut self) -> Option<char>
    {
        use LexerStatus::*;
        match self.status {
            Okay => {
                if self.buf.is_some() {
                    self.buf.take()
                } else {
                    let next = self.chars.next();
                    if next.is_none() {
                        self.status = EndOfStream;
                    }
                    next
                }
            },
            _ => { None }
        }
    }
    fn push_back(&mut self, ch: char) 
    {
        self.buf = Some(ch);
    }
    fn next_token(&mut self) -> Option<Token<'input>>
    {
        use TokenType::*;
        use LexerError::*;
        use LexerStatus::*;
        let mut ret = None;
        
        'outer: while let Some(ch) = self.next_char() {
            match ch {
                '\n' => {
                    self.offset += 1;
                    self.line   += 1;
                    self.col     = 0;
                },
                ' ' | '\t' => {
                    self.offset += 1;
                    self.col    += 1;
                },
                '{'  => {
                    ret = Some(Token::new(LParen, "{", self.line, self.col));
                    self.offset += 1;
                    self.col    += 1;
                    break 'outer;
                },
                '}'  => {
                    ret = Some(Token::new(RParen, "}", self.line, self.col));
                    self.offset += 1;
                    self.col    += 1;
                    break 'outer;
                },
                '+' | '-' | '*' | '/' | '=' => {
                    // Operator.
                    
                    let off = self.offset;
                    ret = Some(Token::new(Operator, 
                                          &self.text[off..off + 1], 
                                          self.line, 
                                          self.col));
                    self.offset += 1;
                    self.col    += 1;
                    break 'outer;
                },
                ';' => {
                    ret = Some(Token::new(Semicolon, ";", self.line, self.col));
                    self.offset += 1;
                    self.col    += 1;
                    break 'outer;
                },
                '"' => {
                    // StringLiteral.
                    
                    let mut escaped = false;
                    let mut end     = 1;
                    
                    while let Some(ch) = self.next_char() {
                        end += 1;
                        match ch {
                            '\\' => { escaped = true; },
                            '"'  => {
                                if !escaped {
                                    let off  = self.offset;
                                    let text = &self.text[off..off + end];
                                    ret = Some(Token::new(StringLiteral,
                                                          text,
                                                          self.line,
                                                          self.col));
                                    self.offset += end;
                                    self.col    += end;
                                    break 'outer;
                                }
                            },
                            _    => { escaped = false; },
                        }
                    }
                },
                'a'..='z' | 'A'..='Z' | '_' => {
                    // Identifier or Keyword.
                    
                    let mut end = 1;
                    while let Some(ch) = self.next_char() {
                        end += 1;
                        match ch {
                            'a'..='z' | 'A'..='Z' | '_' => {},
                            _ => {
                                self.push_back(ch);
                                end -= 1;

                                let off   = self.offset;
                                let text  = &self.text[off..off + end];
                                let is_kw = KEYWORDS.contains(text);
                                let token = if is_kw { Keyword    } 
                                            else     { Identifier };
                                ret = Some(Token::new(token,
                                                      text,
                                                      self.line,
                                                      self.col));
                                self.offset += end;
                                self.col    += end;
                                break 'outer; 
                            }
                        }
                    }
                },
                '0'..='9' => {
                    // NumericLiteral.
                    
                    let mut end = 1;
                    while let Some(ch) = self.next_char() {
                        end += 1;
                        match ch {
                            '0'..='9' => {},
                            _ => {
                                self.push_back(ch);
                                end -= 1;

                                let off   = self.offset;
                                let text  = &self.text[off..off + end];
                                ret = Some(Token::new(NumericLiteral,
                                                      text,
                                                      self.line,
                                                      self.col));
                                self.offset += end;
                                self.col    += end;
                                break 'outer;
                            }
                        }
                    }
                }
                _ => {
                    // Uh oh!
                    
                    self.status = Error(
                        UnrecognizedStart { 
                            message: format!("Unrecognized start \
                                             character, '{}'.", 
                                             ch),
                            line: self.line,
                            col : self.col,
                     });
                     break 'outer;
                },
            }
        }
        ret
    }
}
              
impl<'input> Iterator for Lexer<'input>
{
    type Item = Token<'input>;
    
    fn next(&mut self) -> Option<Self::Item>
    {
        self.next_token()
    }
}

