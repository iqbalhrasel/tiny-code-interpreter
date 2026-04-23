use crate::{
    interpreter::{DataType, ExpressionType, StringExpression, Variable},
    tokenizer::{Token, Tokenizer},
};

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current: Option<Token>,
}
impl<'a> Parser<'a> {
    pub fn new(text: &'a str) -> Self {
        let mut tokenizer = Tokenizer::new(text);
        let current = tokenizer.next();
        return Self { tokenizer, current };
    }

    pub fn parse_expression(&mut self) -> ExpressionType {
        match self.current {
            Some(Token::Let) => self.parse_variable(),
            Some(Token::Printer) => self.parse_print(),
            _ => ExpressionType::Unrecognized,
        }
    }

    fn parse_variable(&mut self) -> ExpressionType {
        self.fetch_next();

        let data_type = match self.get_current() {
            Some(Token::TypeString) => {
                self.fetch_next();
                DataType::String
            }
            _ => panic!("Expected a type"),
        };

        let identifier = match self.get_current() {
            Some(Token::Identifier(s)) => {
                let name = s.clone();
                self.fetch_next();
                name
            }
            _ => panic!("Expected an identifier"),
        };

        match self.get_current() {
            Some(Token::Equals) => {
                self.fetch_next();
            }
            _ => panic!("Expected '='"),
        };

        let data = match self.get_current() {
            Some(Token::StringLiteral(s)) => {
                let val = s.clone();
                self.fetch_next();
                val
            }
            _ => panic!("Expected a data"),
        };

        match self.get_current() {
            Some(Token::Semicolon) => {
                self.fetch_next();
            }
            _ => panic!("Expected ';'"),
        };

        ExpressionType::Var(Variable {
            modifier: crate::interpreter::Modifier::Let,
            data_type,
            identifier,
            data,
        })
    }

    fn parse_print(&mut self) -> ExpressionType {
        self.fetch_next();

        match self.get_current() {
            Some(Token::LeftParen) => {
                self.fetch_next();
            }
            _ => panic!("Expected '('"),
        };

        let literal = match self.get_current() {
            Some(Token::StringLiteral(s)) => {
                let val = s.clone();
                self.fetch_next();
                val
            }
            _ => panic!("Expected string literal"),
        };

        match self.get_current() {
            Some(Token::RightParen) => {
                self.fetch_next();
            }
            _ => panic!("Expected ')'"),
        };

        match self.get_current() {
            Some(Token::Semicolon) => {
                self.fetch_next();
            }
            _ => panic!("Expected ';'"),
        };

        let mut head = String::new();
        let mut var = String::new();
        let mut tail = String::new();

        if let Some(start) = literal.find("{{") {
            if let Some(end) = literal.find("}}") {
                if end > start {
                    head = literal[..start].to_string();
                    var = literal[start + 1 + 1..end].trim().to_string();
                    tail = literal[end + 1 + 1..].to_string();
                }
            } else {
                panic!("missing 2 right braces")
            }
        } else {
            if let Some(_) = literal.find("}}") {
                panic!("missing 2 left braces")
            }
            head = literal;
        }

        return ExpressionType::Print(StringExpression {
            head: Some(head),
            var: Some(var),
            tail: Some(tail),
        });
    }

    fn get_current(&self) -> Option<&Token> {
        self.current.as_ref()
    }

    fn fetch_next(&mut self) {
        self.current = self.tokenizer.next();
    }
}
