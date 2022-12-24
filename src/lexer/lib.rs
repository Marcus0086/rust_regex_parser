#[derive(Debug, PartialEq)]
pub enum Token {
    // Single characters
    Char(char),
    AnyChar,
    ZeroOrMore,
    OneOrMore,
    ZeroOrOne,
    GroupStart,
    GroupEnd,
    Alternation,
}
