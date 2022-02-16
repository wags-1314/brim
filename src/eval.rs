use crate::{ast::{Ast, Operation}, environment::Environment};

pub fn eval(ast: Ast, mut env: Environment) -> (Ast, Environment) {
    match ast {
        int @ Ast::Integer(_) => (int, env),
        Ast::Symbol(string) => (env.get(&string).unwrap_or_else(|| panic!()), env),
        prim @ Ast::PrimFunction(_) => (prim, env),
        def @ Ast::DefFunction { params: _, body: _ } => (def, env),
        string @ Ast::String(_) => (string, env),
        boolean @ Ast::Boolean(_) => (boolean, env),
        Ast::Nil => todo!(),
        Ast::Apply { func, args } => {
            let (func, mut env) = eval(*func, env);
            let mut _args: Vec<Ast> = vec![];
            for arg in args {
                let (arg, env_) = eval(arg, env);
                _args.push(arg);
                env = env_;
            };
            let args = _args;
            match func {
                Ast::PrimFunction(f) => (f(args), env),
                Ast::DefFunction { params, body } => {
                    let mut new_env = Environment::new(Some(Box::new(env)));
                    new_env.set_list(params, args);
                    eval(*body, new_env)
                }
                _ => panic!(),
            }
        }
        Ast::Func { name, params, body } => {
            let func = Ast::DefFunction { params, body };
            env.set(name, func.clone());
            (func, env)
        },
        Ast::Begin(mut declarations) => {
            if declarations.len() == 0 {
                panic!()
            } else {
                let mut new_env = Environment::new(Some(Box::new(env)));
                let last = declarations.pop().unwrap();
                for declaration in declarations {
                    let (_, env) = eval(declaration, new_env);
                    new_env = env;
                }

                eval(last, new_env)
            }
        }
        Ast::If(pred, consq , alt) => {
            let (pred, env) = eval(*pred, env);
            if pred.is_true() {
                eval(*consq, env)
            } else if let Some(alt) = alt {
                eval(*alt, env)
            } else {
                (Ast::Boolean(false), env)
            }
        },
        Ast::BinOp { operation, lhs, rhs } => eval_binop(operation, lhs, rhs, env),
    }
}

fn eval_binop(operation: Operation, lhs: Box<Ast>, rhs: Box<Ast>, env: Environment) -> (Ast, Environment) {
    use Operation::*;
    use Ast::Integer;

    let (lhs, env_1) = eval(*lhs, env);
    let (rhs, env_2) = eval(*rhs, env_1);

    match (operation, lhs, rhs) {
        (Plus, Integer(a), Integer(b)) => (Integer(a + b), env_2),
        (Minus, Integer(a), Integer(b)) => (Integer(a - b), env_2),
        (Multiply, Integer(a), Integer(b)) => (Integer(a * b), env_2),
        (Divide, Integer(a), Integer(b)) => (Integer(a / b), env_2),
        _ => panic!()
    }
}
