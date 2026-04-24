use std::{iter::Peekable, str::Chars};

pub struct Tokenizer<'a> {
    text: Peekable<Chars<'a>>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
    Let,
    TypeString,
    Identifier(String),
    Equals,
    Printer,
    LeftParen,
    StringLiteral(String),
    RightParen,
    Semicolon,
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a str) -> Self {
        return Self {
            text: text.trim().chars().peekable(),
        };
    }
}
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(&c) = self.text.peek() {
            if c.is_whitespace() {
                self.text.next();
            } else {
                break;
            }
        }

        let char_curr = self.text.next();

        let token = match char_curr {
            None => return None,
            Some('a'..='z' | 'A'..='Z' | '0'..='9') => {
                let mut word = char_curr?.to_string();

                while let Some(c) = self.text.peek() {
                    if c.is_alphabetic() || c.is_ascii_digit() {
                        word.push(self.text.next().unwrap());
                    } else {
                        break;
                    }
                }

                if !word.is_empty() && word.chars().next().unwrap().is_ascii_digit() {
                    panic!("can't start with number.")
                }

                match word.as_str() {
                    "let" => Some(Token::Let),
                    "String" => Some(Token::TypeString),
                    "printer" => Some(Token::Printer),
                    _ => Some(Token::Identifier(word)),
                }
            }
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            Some('=') => Some(Token::Equals),
            Some(';') => Some(Token::Semicolon),
            Some('"') => {
                let mut s = String::new();

                while let Some(c) = self.text.next() {
                    if c == '"' {
                        return Some(Token::StringLiteral(s));
                    } else {
                        s.push(c);
                    }
                }

                panic!("Expected closing double quote, reached EOF");
            }
            Some(_) => panic!("Unexpected character"),
        };

        return token;
    }
}
