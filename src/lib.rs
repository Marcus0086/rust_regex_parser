mod lexer;

use lexer::RegexLexer;

#[no_mangle]
pub fn parse_regex(regex: &str) -> String {
    let mut lexer = RegexLexer::new(regex);
    lexer.explan_regex()
}
