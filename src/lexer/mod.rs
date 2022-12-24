mod lib;

struct RegexLexer {
    regex: String,
    pos: usize,
    chars: Vec<char>,
}

impl RegexLexer {
    fn new(regex: &str) -> Self {
        let chars: Vec<char> = regex.chars().collect();
        Self {
            regex: regex.to_string(),
            pos: 0,
            chars,
        }
    }
}
