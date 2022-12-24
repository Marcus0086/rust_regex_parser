use self::lib::Token;

mod lib;
pub struct RegexLexer {
    regex: String,
    pos: usize,
    chars: Vec<char>,
}

impl RegexLexer {
    pub fn new(regex: &str) -> Self {
        let chars: Vec<char> = regex.chars().collect();
        Self {
            regex: regex.to_string(),
            pos: 0,
            chars,
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        if self.pos >= self.chars.len() {
            return None;
        }

        let chr = self.chars[self.pos];
        self.pos += 1;

        match chr {
            '.' => Some(Token::AnyChar),
            '*' => Some(Token::ZeroOrMore),
            '+' => Some(Token::OneOrMore),
            '?' => Some(Token::ZeroOrOne),
            '(' => Some(Token::GroupStart),
            ')' => Some(Token::GroupEnd),
            '|' => Some(Token::Alternation),
            _ => Some(Token::Char(chr)),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.chars.len() && self.chars[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }

    pub fn explan_regex(&mut self) -> String {
        let mut result = String::new();
        while let Some(token) = self.next_token() {
            match token {
                Token::Char(chr) => result.push_str(&format!("Match a '{}' character. ", chr)),
                Token::AnyChar => result.push_str("Match any character except line breaks. "),
                Token::ZeroOrMore => {
                    result.push_str("Match zero or more of the preceeding token. ")
                }
                Token::OneOrMore => result.push_str("Match one or more of the preceeding token. "),
                Token::ZeroOrOne => {
                    result.push_str("Matches between zero and one of the preceeding token. ")
                }
                Token::GroupStart => result.push_str("Start of a group. "),
                Token::GroupEnd => result.push_str("End of a group. "),
                Token::Alternation => result.push_str(
                    "Matches the expression before of after the | (Acts like a boolean OR). ",
                ),
                _ => continue,
            }
        }
        result.trim().to_owned()
    }
}
