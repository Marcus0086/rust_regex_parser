use lexer::RegexLexer;

mod lexer;
fn main() {
    let regex = "(abc * | bc+ )";
    let mut lexer = RegexLexer::new(regex);
    let result = lexer.explan_regex();
    print!(
        "The explanation for the entered regex {} is {}",
        regex, result
    );
}
