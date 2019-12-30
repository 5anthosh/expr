use expr::lexer::Lexer;
fn main() {
    let mut a = Lexer::new("1212.121+34");
    println!("{:?}", a.next());
    println!("{:?}", a.next());
    println!("{:?}", a.next());
    println!("{:?}", a.next());
}
