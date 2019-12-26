pub struct Lexer<'a> {
    expr_chars:Vec<char>,
    start:u32,
    current:u32,
}

impl Lexer {
    pub fn new(expr: &str) -> Lexer{
        let a: Vec<char> = expr.chars().collect();
        return Lexer { expr_chars: a, start:0, current: 0}
    }

    fn is_digit(c: char) -> bool {
        return c >= '0' && c <= '9'
    }

    fn is_space(c: char) -> bool {
        return c == '\n' || c == ' ' || c == '\t'
    }
}
