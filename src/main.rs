use expr::evaluator::Evaluator;

fn main() {
    let mut evaluator = Evaluator::new("3423+234234");
    println!("{:?}", evaluator.eval());
}
