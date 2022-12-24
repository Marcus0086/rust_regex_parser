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

    fn explanation(&mut self) -> String {
        let mut result = String::new();
        while let Some(token) = self.next_token() {
            match token {
                Token::Char(chr) => result.push_str(&format!("Match the character '{}' ", chr)),
                Token::AnyChar => result.push_str(&format!("Match any character ")),
                Token::ZeroOrMore => {
                    result.push_str(&format!("Match zero or more of the preceeding character "))
                }
                Token::OneOrMore => todo!(),
                Token::ZeroOrOne => todo!(),
                Token::GroupStart => todo!(),
                Token::GroupEnd => todo!(),
                Token::Alternation => todo!(),
                _ => continue,
            }
        }
        result
    }
}
