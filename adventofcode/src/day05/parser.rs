use std::{collections::VecDeque, fmt::Display};

use crate::day05::lexer::Token;

use super::{
    config::{Crate, Instruction},
    lexer::{Lexer, LexerError},
};

pub enum ParserError {
    Lexer(LexerError),
    UnexpectedToken(Token),
    InvalidStackIndex(usize),
}

impl From<LexerError> for ParserError {
    fn from(err: LexerError) -> Self {
        return ParserError::Lexer(err);
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "{}",
            match self {
                ParserError::Lexer(lerr) => format!("lexer error: {}", lerr),
                ParserError::UnexpectedToken(tk) => format!("unexpected token: {:?}", tk),
                ParserError::InvalidStackIndex(index) => format!("invalid stack index: {}", index),
            }
        );
    }
}

pub fn parse_part1(
    lexer: &mut Lexer,
) -> Result<(Vec<VecDeque<Crate>>, Vec<Instruction>), ParserError> {
    let stacks = parse_stacks(lexer)?;

    match get_next_token(lexer)? {
        (Token::NewLine, ..) => (),
        (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
    }

    let instructions = parse_instruction_list(lexer, stacks.len())?;

    // match get_next_token(lexer)? {
    //     (Token::NewLine, ..) => (),
    //     (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
    // }

    return Ok((stacks, instructions));
}

fn parse_stacks(lexer: &mut Lexer) -> Result<Vec<VecDeque<Crate>>, ParserError> {
    // back -> bottom of the stack
    // front -> top of the stack
    let mut stacks: Vec<VecDeque<Crate>> = Vec::new();

    // parse first line of crates
    loop {
        let mut new_stack = VecDeque::new();    
        match parse_crate(lexer)? {
            Some(cr) => new_stack.push_back(cr),
            None => (),
        }
        stacks.push(new_stack);

        match get_next_token(lexer)? {
            (Token::NewLine, ..) => break,
            (Token::Space, ..) => (),
            (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
        }
    }
    let stack_count = stacks.len();

    // continue parsing crates, or break at the line with stack labels
    // after the first stack label
    loop {
        match get_next_token(lexer)? {
            (Token::Space, ..) => match get_next_token(lexer)? {
                (Token::Number(_), ..) => match get_next_token(lexer)? {
                    (Token::Space, ..) => break,
                    (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
                },

                (Token::Space, ..) => match get_next_token(lexer)? {
                    (Token::Space, ..) => (),
                    (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
                },

                (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
            },

            (Token::LBracket, ..) => match get_next_token(lexer)? {
                (Token::Identifier(ch), ..) => {
                    stacks[0].push_back(Crate::new(ch));

                    match get_next_token(lexer)? {
                        (Token::RBracket, ..) => (),
                        (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
                    }
                }
                (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
            },

            (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
        }

        for i in 1..stack_count {
            match get_next_token(lexer)? {
                (Token::Space, ..) => (),
                (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
            }

            match parse_crate(lexer)? {
                Some(cr) => stacks[i].push_back(cr),
                None => (),
            };
        }

        match get_next_token(lexer)? {
            (Token::NewLine, ..) => (),
            (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
        }
    }

    // finish parsing the stack labels line

    for _ in 1..stack_count {
        match get_next_token(lexer)? {
            (Token::Space, ..) => (),
            (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
        }
        match get_next_token(lexer)? {
            (Token::Space, ..) => match get_next_token(lexer)? {
                (Token::Number(_), ..) => match get_next_token(lexer)? {
                    (Token::Space, ..) => (),
                    (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
                },

                (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
            },

            (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
        }
    }

    match get_next_token(lexer)? {
        (Token::NewLine, ..) => (),
        (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
    }

    return Ok(stacks);
}

fn parse_instruction_list(
    lexer: &mut Lexer,
    stack_count: usize,
) -> Result<Vec<Instruction>, ParserError> {
    let mut instructions: Vec<Instruction> = vec![];

    loop {
        match get_next_token(lexer)? {
            // (Token::EOF, ..) => break,
            (Token::NewLine, ..) => break,
            (Token::Move, ..) => (),
            (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
        }

        match get_next_token(lexer)? {
            (Token::Space, ..) => (),
            (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
        }

        let move_count = match get_next_token(lexer)? {
            (Token::Number(n), ..) => n,
            (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
        };

        match get_next_token(lexer)? {
            (Token::Space, ..) => (),
            (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
        }

        match get_next_token(lexer)? {
            (Token::From, ..) => (),
            (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
        }

        match get_next_token(lexer)? {
            (Token::Space, ..) => (),
            (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
        }

        let from_index = match get_next_token(lexer)? {
            (Token::Number(n), ..) => n - 1,
            (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
        };

        if from_index >= stack_count {
            return Err(ParserError::InvalidStackIndex(from_index));
        }

        match get_next_token(lexer)? {
            (Token::Space, ..) => (),
            (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
        }

        match get_next_token(lexer)? {
            (Token::To, ..) => (),
            (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
        }

        match get_next_token(lexer)? {
            (Token::Space, ..) => (),
            (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
        }

        let to_index = match get_next_token(lexer)? {
            (Token::Number(n), ..) => n - 1,
            (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
        };

        if to_index >= stack_count {
            return Err(ParserError::InvalidStackIndex(to_index));
        }

        match get_next_token(lexer)? {
            (Token::NewLine, ..) => (),
            (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
        }

        instructions.push(Instruction::new(move_count, from_index, to_index));
    }

    return Ok(instructions);
}

fn parse_crate(lexer: &mut Lexer) -> Result<Option<Crate>, ParserError> {
    match get_next_token(lexer)? {
        (Token::Space, ..) => match get_next_token(lexer)? {
            (Token::Space, ..) => match get_next_token(lexer)? {
                (Token::Space, ..) => return Ok(None),

                (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
            },

            (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
        },

        (Token::LBracket, ..) => match get_next_token(lexer)? {
            (Token::Identifier(id), ..) => match get_next_token(lexer)? {
                (Token::RBracket, ..) => return Ok(Some(Crate::new(id))),

                (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
            },

            (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
        },

        (tk, ..) => return Err(ParserError::UnexpectedToken(tk)),
    }
}

fn get_next_token(lexer: &mut Lexer<'_>) -> Result<(Token, usize, usize), ParserError> {
    return match lexer.next() {
        Some(tk) => tk.map_err(|err| err.into()),
        None => panic!("no more tokens"), // (ParserError::UnexpectedToken(Token::EOF)),
    };
}
