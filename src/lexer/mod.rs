mod lib;

struct RegexLexer {
    regex: String,
    pos: usize,
    chars: Vec<char>,
}
