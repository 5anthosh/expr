use std::env;
use std::fs;
use std::io;
use std::io::Write;

use std::thread;
use tully::evaluator::Evaluator;

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
        if let Err(e) = evaluator.eval(&expression) {
            eprintln!("{}", e.to_string());
        }
    }
}

fn read_from_file(name: &String) {
    let contents = fs::read_to_string(name);
    match contents {
        Ok(source) => {
            let handler = thread::spawn(move || {
                if let Err(e) = Evaluator::new().eval(&source) {
                    eprintln!("{}", e.to_string());
                }
            });
            if let Err(e) = handler.join() {
                eprintln!("{:?}", e);
            }
        }
        Err(e) => eprintln!("Unable to read from file {}", e.to_string()),
    };
}
