use std::fmt::Display;

#[derive(Debug)]
pub enum Token {
    Number(usize),
    Identifier(char),
    Move,
    From,
    To,
    // Dash,
    // Comma,
    LBracket,
    RBracket,
    Space,
    NewLine,
    EOF,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "{}",
            match self {
                Token::NewLine => '\n'.to_string(),
                tk => format!("{:?} | ", tk),
            }
        );
    }
}

#[derive(Debug)]
pub enum LexerError {
    UnexpectedEOF,
    UnexpectedCharacter(char),
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "{}",
            match self {
                LexerError::UnexpectedEOF => String::from("unexpected EOF"),
                LexerError::UnexpectedCharacter(c) => format!("unexpected character: '{}'", c),
            },
        );
    }
}

pub struct Lexer<'a> {
    current_index: usize,
    remaining_text: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        return Self {
            current_index: 0,
            remaining_text: input,
        };
    }

    fn get_next_token(&mut self) -> Result<(Token, usize, usize), LexerError> {
        let current_char = match self.remaining_text.chars().next() {
            Some(ch) => ch,
            None => return Ok((Token::EOF, self.current_index, self.current_index)),
        };

        let (token, size) = match current_char {
            // '-' => (Token::Dash, 1),
            // ',' => (Token::Comma, 1),
            ' ' => (Token::Space, 1),
            '[' => (Token::LBracket, 1),
            ']' => (Token::RBracket, 1),
            '\n' => (Token::NewLine, 1),
            '0'..='9' => tokenize_number(&self.remaining_text)?,
            c @ 'A'..='Z' => (Token::Identifier(c), 1),
            c => match tokenize_keyword(&self.remaining_text) {
                Some(tk) => tk,
                None => return Err(LexerError::UnexpectedCharacter(c)),
            },
        };

        let start = self.current_index;
        self.remaining_text = &self.remaining_text[size..];
        self.current_index += size;
        let end = self.current_index;

        // print!("{}", token);
        return Ok((token, start, end));
    }
}

impl Iterator for Lexer<'_> {
    type Item = Result<(Token, usize, usize), LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        return Some(self.get_next_token());
    }
}

fn tokenize_keyword(remaining_text: &str) -> Option<(Token, usize)> {
    for (keyword, tk) in [
        ("move", Token::Move),
        ("from", Token::From),
        ("to", Token::To),
    ] {
        match tokenize_string(remaining_text, keyword) {
            Some(size) => return Some((tk, size)),
            None => (),
        }
    }

    return None;
}

fn tokenize_string(remaining_text: &str, expected: &str) -> Option<usize> {
    let mut size = 0;
    let mut input_chars = remaining_text.chars();

    for expected_ch in expected.chars() {
        match input_chars.next() {
            Some(input_ch) if input_ch == expected_ch => (),
            _ => return None,
        }
        size += 1;
    }

    return Some(size);
}

fn tokenize_number(input: &str) -> Result<(Token, usize), LexerError> {
    let mut size = 0;
    let mut res = 0;

    for ch in input.chars() {
        match ch.to_digit(10) {
            Some(digit) => res = res * 10 + digit as usize,
            None => break,
        }
        size += 1;
    }

    if size == 0 {
        return Err(LexerError::UnexpectedEOF);
    }

    return Ok((Token::Number(res), size));
}
