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
                eprintln!("Unable to read from command line");
                break;
            }
        }
        let mut evaluator = Evaluator::new(&expression.trim());
        let value = evaluator.eval();
        let value = match value {
            Ok(value) => value,
            Err(e) => {
                eprintln!("{}", e.to_string());
                continue;
            }
        };
        match value {
            Value::Float(val) => println!("{}", val),
            Value::String(string_value) => println!("{}", string_value),
            _ => eprintln!("Unexpected value"),
        };
    }
}
