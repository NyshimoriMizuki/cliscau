use super::lexer::{Token, Tokens};

#[derive(Debug, Clone)]
pub enum Node {
    Num(f64),
    Ident(String),
    UnaryExpr {
        op: Operator,
        value: Box<Node>,
    },
    BinaryExpr {
        op: Operator,
        left: Box<Node>,
        right: Box<Node>,
    },
    AssignExpr {
        name: Box<Node>,
        value: Box<Node>,
    },
    // Some(Token),
}
impl Node {
    fn make_binary_expr(op: Operator, left: Node, right: Node) -> Node {
        Node::BinaryExpr {
            op,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
    pub fn get_value(&self) -> String {
        match self {
            Node::Num(v) => v.to_string(),
            Node::Ident(v) => v.to_owned(),
            n => panic!("No value in the node {:?}", n),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
}
impl Operator {
    fn from(token: Token) -> Operator {
        match token {
            Token::Plus => Operator::Plus,
            Token::Minus => Operator::Minus,
            Token::Divide => Operator::Divide,
            Token::Multiply => Operator::Multiply,
            Token::Power => Operator::Power,
            any => panic!("ParseError: {} is not a operator!", any),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ast(Vec<Node>);
impl Ast {
    pub fn new() -> Ast {
        Ast(Vec::new())
    }

    pub fn push(&mut self, node: Node) {
        self.0.push(node);
    }

    pub fn nodes(&self) -> Vec<Node> {
        self.0.clone()
    }
}

pub struct Parser {
    tokens: Tokens,
    nodes: Ast,
}
impl Parser {
    pub fn new(tokens: &Tokens) -> Parser {
        Parser {
            tokens: tokens.clone(),
            nodes: Ast::new(),
        }
    }

    pub fn get_ast(&self) -> Ast {
        self.nodes.clone()
    }

    pub fn parse(&mut self) {
        while self.tokens.get_at(0).is_not(Token::EOF) {
            let new_node = self.get_node();
            self.nodes.push(new_node);

            if self.tokens.get_at(0).is_not(Token::EndLine)
                && self.tokens.get_at(0).is_not(Token::EOF)
            {
                panic!(
                    "SyntaxError: expected \";\" or \"end of input\", not {}",
                    self.tokens.get_at(0)
                );
            } else {
                self.tokens.next();
            }
        }
    }
    fn get_node(&mut self) -> Node {
        self.get_add_expr()
    }

    fn get_add_expr(&mut self) -> Node {
        let mut left = self.get_mult_expr();

        while self
            .tokens
            .get_at(0)
            .is_in(&vec![Token::Plus, Token::Minus])
        {
            left = Node::make_binary_expr(
                match self.tokens.next() {
                    Some(t) => Operator::from(t),
                    None => panic!("SyntaxError: unexpected end of input."),
                },
                left.clone(),
                self.get_mult_expr(),
            );
        }
        left
    }

    fn get_mult_expr(&mut self) -> Node {
        let mut left = self.get_exponential_expr();
        while self
            .tokens
            .get_at(0)
            .is_in(&vec![Token::Multiply, Token::Divide])
        {
            left = Node::make_binary_expr(
                match self.tokens.next() {
                    Some(t) => Operator::from(t),
                    None => panic!("SyntaxError: unexpected end of input."),
                },
                left.clone(),
                self.get_exponential_expr(),
            );
        }
        left
    }

    fn get_exponential_expr(&mut self) -> Node {
        let mut left = self.get_term();
        while self.tokens.get_at(0) == Token::Power {
            left = Node::make_binary_expr(
                match self.tokens.next() {
                    Some(t) => Operator::from(t),
                    None => panic!("SyntaxError: unexpected end of input."),
                },
                left.clone(),
                self.get_term(),
            );
        }
        left
    }

    fn get_term(&mut self) -> Node {
        let token = match self.tokens.next() {
            Some(t) => t,
            None => Token::EOF,
        };

        match token {
            Token::Number(n) => {
                let value: f64 = match n.parse() {
                    Ok(v) => v,
                    Err(err) => panic!(
                        "ParseError: cannot convert {} into a number be cause of \"{}\"",
                        n, err
                    ),
                };
                Node::Num(value)
            }
            Token::Ident(i) => {
                let name = Node::Ident(i);
                if let Token::Assign = self.tokens.get_at(0) {
                    self.tokens.next();
                    Node::AssignExpr {
                        name: Box::new(name),
                        value: Box::new(self.get_node()),
                    }
                } else {
                    name
                }
            }
            Token::Minus | Token::Plus => Node::UnaryExpr {
                op: Operator::from(token),
                value: Box::new(self.get_term()),
            },
            Token::LParam => {
                let content = self.get_node();
                let _ = match self.tokens.next() {
                    Some(Token::RParam) => "good working way",
                    Some(t) => panic!("SyntaxError: expected ')', not the token {}", t),
                    None => panic!("SyntaxError: paranthesis is never closed"),
                };
                content
            }
            Token::RParam => panic!("SyntaxError: empty parenthesis."),
            t => panic!("SyntaxError: unexpected token {}.", t),
        }
    }
}
