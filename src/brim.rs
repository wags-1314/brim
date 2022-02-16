use crate::{parse::parse, eval::eval, environment::Environment};

pub fn run(source: &str, env: Environment) -> (String, Environment) {
    let ast = parse(source).unwrap();

    println!("AST{{ {} }}", ast);

    let (res, env_) = eval(ast, env);

    (format!("{}", res), env_)
}