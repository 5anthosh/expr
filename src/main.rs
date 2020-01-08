use tully::evaluator::Evaluator;
use std::io;
use std::io::Write;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    run(args);
}

fn run(args: Vec<String>) {
    if args.len() == 1 {
        command_line();
        return;
    }
    if args.len() == 2 {
        read_from_file(&args[1]);
        return;
    }
    eprintln!("Usage : Tully [script]")
}

fn command_line() {
    println!("Math expression evaluator");
    let mut evaluator = Evaluator::new();
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        let mut expression = String::new();
        match io::stdin().read_line(&mut expression) {
            Ok(n) => {
                // because of new line character
                if n == 1 {
                    continue;
                }
            }
            _ => {
                eprintln!("Unable to read from command line");
                break;
            }
        }
        evaluator.eval(expression);
    }
}


fn read_from_file(name: &String) {
    let contents = fs::read_to_string(name);
    match contents {
        Ok(source) => {
            Evaluator::new().eval(source);
        }
        Err(e) => eprintln!("Unable to read from file {}", e.to_string())
    };
}