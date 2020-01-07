use expr::evaluator::Evaluator;
use expr::value::Value;
use std::io;
use std::io::Write;

fn main() {
    println!("Math expression evaluator");
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
                println!("Unable to read from command line");
                break;
            }
        }
        let mut evaluator = Evaluator::new(&expression.trim());
        let value = evaluator.eval();
        match value {
            Value::Float(val) => println!("{}", val),
            _ => panic!("Unexpected value"),
        };
    }
}
