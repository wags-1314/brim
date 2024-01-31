mod repl;
mod lexer;

use crate::{repl::*, lexer::*};

fn main() {
    let v = vec![1,2,3,4];
    let s = String::from("asdasd");
    let mut iter = s.chars().peekable();
    
    repl();
}
