use crate::grammar::BrimParser;
use crate::ast::Ast;

pub fn parse(source: &str) -> Option<Ast> {
	match BrimParser::new().parse(source) {
        Ok(ast) => Some(ast),
        Err(_) => panic!("parse failed")
    }
}

pub fn to_raw_string(string: &str) -> &str {
    let mut chars = string.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}
