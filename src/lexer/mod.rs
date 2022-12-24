use self::lib::Token;

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

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        if self.pos > self.chars.len() {
            return None;
        }

        let chr = self.chars[self.pos];
        self.pos += 1;

        match chr {
            '.' => Some(Token::AnyChar),
            _ => Some(Token::Char(chr)),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.chars.len() && self.chars[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }
}
