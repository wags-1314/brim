use std::fmt;

#[derive(Debug, Clone)]
pub enum Operation {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub enum Ast {
    Integer(i64),
    Symbol(String),
    Apply {
        func: Box<Ast>,
        args: Vec<Ast>,
    },
    Nil,
    Func {
        name: String,
        params: Vec<String>,
        body: Box<Ast>,
    },

    // should never be reached in evaluation
    PrimFunction(fn(Vec<Ast>) -> Ast),
    DefFunction {
        params: Vec<String>,
        body: Box<Ast>,
    },
    String(String),
    Boolean(bool),
    Begin(Vec<Ast>),
    If(Box<Ast>, Box<Ast>, Option<Box<Ast>>),
    BinOp { operation: Operation, lhs: Box<Ast>, rhs: Box<Ast> }
}

impl Ast {
    pub fn make_symbol(string: String) -> Box<Ast> {
        Box::new(Ast::Symbol(string))
    }

    pub fn is_true(&self) -> bool {
        matches!(self, Ast::Boolean(true))
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Operation::*;
        match self {
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            Multiply => write!(f, "*"),
            Divide => write!(f, "/"),
        }
    }
}

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ast::Integer(i) => write!(f, "{}", i.to_string()),
            Ast::Symbol(s) => write!(f, "{}", s.to_string()),
            Ast::Apply { func, args } => {
                write!(f, "({}", func)?;
                for arg in args {
                    write!(f, " {}", arg)?;
                }
                write!(f, ")")
            }
            Ast::Nil => write!(f, "()"),
            Ast::Func { name, params, body } => {
                write!(f, "(func {} ", name)?;
                match params.len() {
                    0 => write!(f, "()")?,
                    1 => write!(f, "({})", params[0])?,
                    _ => {
                        write!(f, "(")?;
                        for param in &params[0..params.len() - 1] {
                            write!(f, "{} ", param)?;
                        }
                        write!(f, "{})", params.last().unwrap())?;
                    }
                };
                write!(f, " {})", body)
            },
            
            Ast::Begin(declarations) => {
                write!(f, "(begin")?;
                for declaration in declarations {
                    write!(f, " {}", declaration)?;
                }
                write!(f, ")")
            },
            Ast::PrimFunction(_) | Ast::DefFunction { params: _, body: _ } => write!(f, "<func>"),
            Ast::String(string) => write!(f, "\"{}\"", string),
            Ast::Boolean(boolean) => write!(f, "{}", if *boolean { true } else { false }),
            Ast::If(pred, conseq, alt) => {
                write!(f, "(if {} {}", pred, conseq)?;
                match alt {
                    Some(alt) => write!(f, " {})", alt),
                    None => write!(f, ")")
                }
            }
            Ast::BinOp { operation, lhs, rhs } => write!(f, "({} {} {})", operation, lhs, rhs)
        }
    }
}
