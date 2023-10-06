use std::collections::HashMap;

mod lexer;
mod parser;

pub struct MathIntepreter {
    variables: HashMap<String, f64>,
}
impl MathIntepreter {
    pub fn new() -> MathIntepreter {
        MathIntepreter {
            variables: HashMap::new(),
        }
    }

    pub fn read(&mut self, code: &str) {
        let tokens = lexer::Lexer::new(code).tokenize();
        let ast = {
            let mut p = parser::Parser::new(&tokens);
            p.parse();
            p.get_ast()
        };

        for node in ast.nodes() {
            println!("{:?}", self.eval(&node));
        }
    }

    pub fn get_vars(&self) -> &HashMap<String, f64> {
        &self.variables
    }
    fn get_variable(&self, name: &str) -> f64 {
        match self.variables.get(name) {
            Some(v) => v.to_owned(),
            None => {
                println!("ReferenceError: variable \"{}\" not found! \n\tThe value \"0.0\" was given to not break.\n", name);
                0.0
            }
        }
    }

    fn eval(&mut self, node: &parser::Node) -> f64 {
        match node {
            parser::Node::Num(i) => i.to_owned(),
            parser::Node::Ident(v) => self.get_variable(v),
            parser::Node::UnaryExpr { op, value } => {
                if let parser::Operator::Minus = op {
                    -self.eval(&value)
                } else {
                    self.eval(value)
                }
            }
            parser::Node::BinaryExpr { op, left, right } => match op {
                parser::Operator::Plus => self.eval(left) + self.eval(right),
                parser::Operator::Minus => self.eval(left) - self.eval(right),
                parser::Operator::Multiply => self.eval(left) * self.eval(right),
                parser::Operator::Divide => self.eval(left) / self.eval(right),
                parser::Operator::Power => self.eval(left).powf(self.eval(right)),
            },
            parser::Node::AssignExpr { name, value } => {
                let value = self.eval(&value);
                let name = name.get_value();
                self.register_variable(name.clone(), value);
                self.get_variable(&name)
            }
        }
    }

    fn register_variable(&mut self, name: String, value: f64) {
        self.variables.insert(name, value);
    }
}
