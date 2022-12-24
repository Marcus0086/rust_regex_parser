mod lexer;

use lexer::RegexLexer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_regex(regex: &str) -> String {
    let mut lexer = RegexLexer::new(regex);
    lexer.explan_regex()
}
