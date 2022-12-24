use lexer::RegexLexer;
mod lexer;

pub fn parse_regex(regex: &str) -> String {
    let mut lexer = RegexLexer::new(regex);
    lexer.explan_regex()
}
fn main() {
    let regex = "(abc * | bc+ )";
    let result = parse_regex(regex);
    print!(
        "The explanation for the entered regex {} is: {}",
        regex, result
    );
}
