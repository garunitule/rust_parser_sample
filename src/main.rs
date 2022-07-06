mod lexer;
mod utility;

fn main() {
    use std::io::{stdin, BufRead, BufReader};

    let stdin = stdin();
    let stdin = stdin.lock();
    let stdin = BufReader::new(stdin);
    let mut lines = stdin.lines();

    loop {
        utility::prompt("> ").unwrap();
        if let Some(Ok(line)) = lines.next() {
            let token = lexer::lex(&line);
            println!("{:?}", token);
        } else {
            break;
        }
    }
}
