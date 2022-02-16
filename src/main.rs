#[macro_use]
extern crate lalrpop_util;
extern crate rustyline;

pub mod ast;
pub mod brim;
pub mod environment;
pub mod eval;
pub mod parse;
pub mod prelude;
pub mod repl;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP

use repl::read_from_prompt;

fn main() {
    read_from_prompt()
}

/*

func name params* = //something

func factorial(n) = if n == 0 { 1 } else { n * factorial(n - 1) };

func factorial(n) = if n == 0 {{ 1 }} else {{ n * factorial(n - 1) }} factorial(4)

Untyped Variant:
func hello a b = a + b;
func hello1 = { let a = 1; let b = 1; a + b }
func hello2 = {
    let a = 2;
    let b = 2;
    a + b;
}

func hello3 a = {
    if a {
        "hello"
    } else {
        "bye"
    }
}

func hello = lambda a b . a + b;

func factorial n = {
    match a {
        0 do 1;
        _ do n * factorial(n - 1);
    }
}

Typed Variant:
hello : int -> int -> int;
func hello (a: int, b: int): int = a + b;

hello1 : int
func hello1(): int = { let a = 1; let b = 1; a + b }
func hello2(): int = {
    let a = 2;
    let b = 2;
    a + b;
}

data Boolean = True | False;

hello3: Boolean -> String;
func hello3 (a: Boolean): String = {
    match a {
        case True do "hello";
        case False do "bye";
    }
}

hello: int -> int -> int;
func hello = lambda (a: int, b: int): int . a + b;

factorial: int -> int;
func factorial n = {
    match a {
        0 do 1;
        _ do n * factorial(n - 1);
    }
}

(* Nil *)
*/
