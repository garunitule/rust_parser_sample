mod ast;
mod error;
mod lexer;
mod utility;

use crate::ast::{Ast};

fn main() {
    use std::io::{stdin, BufRead, BufReader};

    let stdin = stdin();
    let stdin = stdin.lock();
    let stdin = BufReader::new(stdin);
    let mut lines = stdin.lines();

    loop {
        utility::prompt("> ").unwrap();
        if let Some(Ok(line)) = lines.next() {
            let ast = match line.parse::<Ast>() {
                Ok(ast) => ast,
                Err(e) => {
                    unimplemented!()
                }
            };
            println!("{:?}", ast);
        } else {
            break;
        }
    }
}
