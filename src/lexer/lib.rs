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

    // Keywords
    Dot,
    Star,
    Plus,
    Question,
    LeftParen,
    RightParen,
    Pipe,

    // Character classes
    CharClassStart,
    CharClassEnd,
    CharClassInvert,
    CharRange,

    // Anchors
    Anchor_start,
    anchor_end,
    anchor_word_boundary,

    // Special characters
    Escape,
    Octal,
    Hex,
    Unicode,
}
