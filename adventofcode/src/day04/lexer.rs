use std::fmt::Display;

#[derive(Debug)]
pub enum Token {
    Number(u32),
    Dash,
    Comma,
    NewLine,
    EOF,
}

pub enum LexerError {
    UnexpectedEOF,
    UnknownCharacter(char),
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "{}",
            match self {
                LexerError::UnexpectedEOF => String::from("unexpected EOF"),
                LexerError::UnknownCharacter(c) => format!("unknown character: '{}'", c),
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

    pub fn get_next_token(&mut self) -> Result<(Token, usize, usize), LexerError> {
        let current_char = match self.remaining_text.chars().next() {
            Some(ch) => ch,
            None => return Ok((Token::EOF, self.current_index, self.current_index)),
        };

        let (token, size) = match current_char {
            '-' => (Token::Dash, 1),
            ',' => (Token::Comma, 1),
            '\n' => (Token::NewLine, 1),
            '0'..='9' => tokenize_number(&self.remaining_text)?,
            c => return Err(LexerError::UnknownCharacter(c)),
        };

        let start = self.current_index;
        self.remaining_text = &self.remaining_text[size..];
        self.current_index += size;
        let end = self.current_index;

        return Ok((token, start, end));
    }
}

fn tokenize_number(input: &str) -> Result<(Token, usize), LexerError> {
    let mut size = 0;
    let mut res = 0;

    for ch in input.chars() {
        match ch.to_digit(10) {
            Some(digit) => res = res * 10 + digit,
            None => break,
        }
        size += 1;
    }

    if size == 0 {
        return Err(LexerError::UnexpectedEOF);
    }

    return Ok((Token::Number(res), size));
}
