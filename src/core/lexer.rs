use std::{fmt::Display, str::Chars};

pub struct Lexer<'a> {
    code: Chars<'a>,
}
impl Lexer<'_> {
    pub fn new(code: &str) -> Lexer {
        Lexer { code: code.chars() }
    }

    fn eat(&mut self) -> Option<char> {
        self.code.next()
    }

    fn next(&self) -> char {
        match self.code.clone().next() {
            Some(c) => c,
            None => '\0',
        }
    }

    pub fn tokenize(&mut self) -> Tokens {
        let mut tokens: Vec<Token> = Vec::new();
        loop {
            let char_ = match self.eat() {
                Some(c) => c,
                None => {
                    tokens.push(Token::EOF);
                    break;
                }
            };
            let kind = self.get_kind(char_);

            if kind.is(Token::Space) || kind.is(Token::EndLine) {
                continue;
            }
            tokens.push(kind);
        }
        return Tokens::from(&tokens);
    }

    fn get_kind(&mut self, character: char) -> Token {
        match character {
            c if c.is_numeric() || c == '.' => {
                Token::Number(self.make_long_token(c, |next_character| {
                    !next_character.is_numeric() && next_character != '.'
                }))
            }
            c if c.is_alphabetic() => Token::Ident(
                self.make_long_token(c, |next_character| !next_character.is_alphanumeric()),
            ),
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            ';' | '\n' => Token::EndLine,
            ' ' | '\r' => Token::Space,
            '(' => Token::LParam,
            ')' => Token::RParam,
            '=' => Token::Assign,
            any => Token::Unknown(any),
        }
    }

    fn make_long_token(&mut self, base: char, break_cond: fn(c: char) -> bool) -> String {
        let mut num = String::from(base);
        loop {
            if break_cond(self.next()) {
                break;
            }
            num.push(self.eat().unwrap());
        }
        num
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    Number(String),
    Ident(String),
    Plus,
    Minus,
    Divide,
    Multiply,
    EndLine,
    LParam,
    RParam,
    Assign,
    Space,
    EOF,
    Unknown(char),
}
impl Token {
    pub fn is(&self, token: Token) -> bool {
        self.kind() == token.kind()
    }
    pub fn is_in(&self, tokens: &Vec<Token>) -> bool {
        for i in tokens {
            if self.is(i.to_owned()) {
                return true;
            }
        }
        false
    }

    pub fn is_not(&self, token: Token) -> bool {
        !self.is(token)
    }

    fn kind(&self) -> &str {
        match self {
            Token::Number(_) => "number",
            Token::Ident(_) => "identifier",
            Token::Unknown(_) => "unknown",
            Token::Plus => "plus",
            Token::Minus => "minus",
            Token::Divide => "divide",
            Token::Multiply => "multiply",
            Token::EndLine => "endline",
            Token::LParam => "lparam",
            Token::RParam => "rparam",
            Token::Assign => "equals",
            Token::Space => "space",
            Token::EOF => "eof",
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Token::Number(v) => format!("<Num:{}>", v),
            Token::Ident(v) => format!("<Ident:{}>", v),
            Token::Unknown(v) => format!("<Unknown:{}>", v),
            Token::Plus => format!("<Plus>"),
            Token::Minus => format!("<Minus>"),
            Token::Divide => format!("<Divide>"),
            Token::Multiply => format!("<Multiply>"),
            Token::EndLine => format!("<EndLine>"),
            Token::LParam => format!("<LParam>"),
            Token::RParam => format!("<RParam>"),
            Token::Assign => format!("<Assign>"),
            Token::Space => format!("<Space>"),
            Token::EOF => format!("<EOF>"),
        }
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct Tokens(Vec<Token>);
impl Tokens {
    pub fn from(tokens: &Vec<Token>) -> Tokens {
        Tokens(tokens.clone())
    }

    pub fn get_at(&self, index: usize) -> Token {
        match self.0.get(index) {
            Some(t) => t.clone(),
            None => Token::EOF,
        }
    }
}
impl Iterator for Tokens {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.reverse();
        let v = self.0.pop();
        self.0.reverse();
        v
    }
}
impl Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tokens = self
            .0
            .clone()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        write!(f, "Tokens [{}]", tokens.join(", "))
    }
}
