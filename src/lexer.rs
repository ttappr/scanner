
#![allow(dead_code)]

use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt;
use std::str::Chars;


use lazy_static::lazy_static;


lazy_static! {
    /// A set of the keywords. Used to distinguish keywords from identifiers.
    ///
    static ref KEYWORDS: HashSet<&'static str> = {
        vec!["if", "else", "for", "while"].into_iter().collect()
    };
}

/// Various token types. This populates the `Token.type_` field.
///
#[derive(Debug, Clone, Copy)]
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

/// Everything returned by the lexer is a Token.
///
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
    /// Creates a new `Token`. Only the lexer creates these.
    ///
    fn new(type_: TokenType, text: &'input str, line: usize, col: usize) -> Self
    {
        Token { type_, text, line, col }
    }
    
    /// Returns the token type.
    ///
    pub fn token_type(&self) -> TokenType
    {
        self.type_
    }
    
    /// Returns the text for the token.
    ///
    pub fn text(&self) -> &str
    {
        self.text
    }
    
    /// Returns the line and column offsets for the start of the token text.
    ///
    pub fn pos(&self) -> (usize, usize)
    {
        (self.line, self.col)
    }
}

/// An enum that implements Error that represents the various types of error 
/// the lexer can generate.
///
#[derive(Debug)]
pub enum LexerError
{
    //GeneralError      { message: String },
    UnrecognizedStart { message: String, line: usize, col: usize },
    InvalidEscape     { message: String, line: usize, col: usize }
}
impl Error for LexerError { }

impl fmt::Display for LexerError 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        use LexerError::*;
        match self {
            //GeneralError      { message     } => write!(f, "{}", message),
            UnrecognizedStart { message, .. } => write!(f, "{}", message),
            InvalidEscape     { message, .. } => write!(f, "{}", message),
        }
    }
}

/// When the lexer stops producing tokens, its status will be a variant of
/// this enum.
///
#[derive(Debug)]
pub enum LexerStatus
{
    Okay,
    EndOfStream,
    Error(LexerError),
}

/// Represents the lexer and its state. Keeps track of position information in
/// the text being scanned and produces `Token`'s.
///
pub struct Lexer<'input>
{
    status  : LexerStatus,
    text    : &'input str,
    chars   : Chars<'input>,
    buf     : VecDeque<char>,
    offset  : usize,
    line    : usize,
    col     : usize,
}

impl<'input> Lexer<'input>
{
    /// Creates a new lexer to tokenize the given `text`.
    ///
    pub fn new(text: &'input str) -> Self
    {
        Lexer { 
            status  : LexerStatus::Okay, 
            text, 
            chars   : text.chars(),
            buf     : VecDeque::new(),
            offset  : 0,
            line    : 0,
            col     : 0,
        }
    }
    
    /// Returns the status of the lexer. This can be called after the lexer
    /// stops producing tokens to find out if it parsed the full text or
    /// encounted an error along the way.
    ///
    pub fn status(&self) -> &LexerStatus
    {
        &self.status
    }
    
    /// Produces the next character to process as `Some(<ch>)`, or `None` if 
    /// finished.
    ///
    fn next_char(&mut self) -> Option<char>
    {
        use LexerStatus::*;
        match self.status {
            Okay => {
                if !self.buf.is_empty() {
                    self.buf.pop_front()
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
    
    /// Look ahead `ahead` number of characters.
    ///
    fn look_ahead(&mut self, ahead: usize) -> Option<char>
    {
        let mut end = false;
        let     len = self.buf.len();
        
        for _ in len..ahead {
            match self.chars.next() {
                Some(ch) => { self.buf.push_back(ch); },
                None     => { end = true; break; },
            }       
        }
        if !end && ahead > 0 {
            Some(self.buf[ahead - 1])
        } else {
            None
        }
    }
    
    /// Put a single character back in the text stream at the front. The 
    /// `next_char()` method will return this character on its next invocation.
    ///
    fn put_back(&mut self, ch: char) 
    {
        self.buf.push_front(ch);
    }
    
    /// Produces the next token for the lexer as `Some(<token>)`. `None` is 
    /// returned if the lexer reached the end of the input text or an error
    /// occurred, which can be checked using `.status()`.
    ///
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
                ' ' | '\t'| '\r' => {
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
                            '\\' => { 
                                if let Some(la) = self.look_ahead(1) {
                                if la != '"' {
                                    self.status = Error(
                                        InvalidEscape { 
                                            message: format!(
                                                "Invalid escape in string, \
                                                \"\\{}\".", la),
                                            line: self.line,
                                            col : self.col + end,
                                     });
                                     break 'outer;
                                }}
                                escaped = true;
                            },
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
                                self.put_back(ch);
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
                                self.put_back(ch);
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
             
/// Enables the lexer to be used as an iterator in loops.
/// 
impl<'input> Iterator for Lexer<'input>
{
    type Item = Token<'input>;
    
    fn next(&mut self) -> Option<Self::Item>
    {
        self.next_token()
    }
}

