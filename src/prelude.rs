use crate::{ast::Ast, environment::Environment};

fn println(args: Vec<Ast>) -> Ast {
    if args.is_empty() {
        println!("\n");
        Ast::Integer(0i64)
    } else {
        for arg in &args {
            match arg {
                Ast::String(string) => println!("stdout> {}", string),
                _ => panic!(),
            }
        }
        Ast::Integer(args.len() as i64)
    }
}

fn equals(args: Vec<Ast>) -> Ast {
    if args.len() == 2 {
        match (&args[0], &args[1]) {
            (Ast::Integer(a), Ast::Integer(b)) => Ast::Boolean(a == b),
            _ => panic!(),
        }
    } else {
        panic!()
    }
}

fn add_primitive_function(prelude: &mut Environment, string: &str, f: fn(Vec<Ast>) -> Ast) {
    prelude.set(String::from(string), Ast::PrimFunction(f));
}

pub fn make_prelude() -> Environment {
    let mut prelude = Environment::new(None);
    add_primitive_function(&mut prelude, "println", println);
    add_primitive_function(&mut prelude, "==", equals);

    prelude
}
