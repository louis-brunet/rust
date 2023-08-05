use std::{fmt::Display, io::Write, collections::HashMap};

use super::file_tree::{FileTree, FsNode};

#[derive(Debug)]
pub enum FileTreeParseError {
    UnexpectedEof,
    UnexpectedCharacter(char),
    ChildNotFound(String),
    NotADirectory(String),
}

impl Display for FileTreeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<FileTreeParseError> for String {
    fn from(value: FileTreeParseError) -> Self {
        format!("{:?}", value)
    }
}

pub fn parse_file_tree(input: &str) -> Result<FileTree, FileTreeParseError> {
    let input = parse_literal(input, "$ cd /")?;
    let input = skip_whitespace(input); 
    let input = parse_literal(input, "$ ls")?;
    let input = skip_whitespace(input);

    let mut tree = FileTree::new();
    let input = parse_dir_entry_list(input, &mut tree.root)?;

    println!("parsed 1 level : \n{}", tree);

    parse_command_list_recursive(input, &mut tree.root)?;
    return Ok(tree);
}

fn parse_literal<'a>(remaining_text: &'a str, literal: &str) -> Result<&'a str, FileTreeParseError> {
    let mut input_bytes = remaining_text.bytes();
    let mut size = 0;

    for ch in literal.bytes() {
        match input_bytes.next() {
            Some(input) if input != ch => return Err(FileTreeParseError::UnexpectedCharacter(input.into())),
            Some(_) => (),
            None => return Err(FileTreeParseError::UnexpectedEof),
        };
        size += 1;
    }
    
    return Ok(consume_text(remaining_text, size));
}

fn skip_whitespace(remaining_text: &str) -> &str {
    let whitespace = [b' ', b'\n', b'\r'];
    let mut count = 0;

    for ch in remaining_text.bytes() {
        if !whitespace.contains(&ch) {
            break;
        }
        count += 1;
    } 

    return consume_text(remaining_text, count);
}

fn consume_text(remaining_text: &str, size: usize) -> &str {
    print!("{}", &remaining_text[..size]);
    let _ = std::io::stdout().flush();
    return &remaining_text[size..];
}

fn parse_dir_entry_list<'a>(remaining_text: &'a str, working_directory: &mut FsNode) -> Result<&'a str, FileTreeParseError> {
    let mut input = remaining_text; 
    loop {
        let (chomped_input, name, node) = match input.bytes().next() {
            None => break,
            Some(b'$') => break,
            Some(b'd') => parse_dir_entry_directory(input)?,
            Some(byte) if byte.is_ascii_digit() => parse_dir_entry_file(input)?,
            Some(c) => return Err(FileTreeParseError::UnexpectedCharacter(c.into())),
        };
        input = chomped_input;

        match working_directory {
            FsNode::File(..) => panic!("working directory is a regular file"),
            FsNode::Directory(chilren) => { 
                chilren.entry(name).or_insert(node);
            },
        }

        input = skip_whitespace(input);
    }


    return Ok(input);
}

fn parse_dir_entry_directory(remaining_text: &str,) -> Result<(&str, String, FsNode), FileTreeParseError> {
    let remaining_text = parse_literal(remaining_text, "dir ")?;
    let (remaining_text, dir_name) = parse_identifier(remaining_text)?;

    return Ok((remaining_text, dir_name, FsNode::Directory(HashMap::new())));
}

fn parse_dir_entry_file(remaining_text: &str) -> Result<(&str, String, FsNode), FileTreeParseError> {
    let (remaining_text, size) = parse_uint(remaining_text)?;
    let remaining_text = skip_whitespace(remaining_text);
    let (remaining_text, file_name) = parse_file_name(remaining_text)?;

    return Ok((remaining_text, file_name, FsNode::File(size)));
}

fn parse_identifier(remaining_text: &str) -> Result<(&str, String), FileTreeParseError> {
    let size = remaining_text.bytes().take_while(|ch| ch.is_ascii_alphabetic()).count();

    if size == 0 {
        return Err(match remaining_text.bytes().next() {
            Some(c) => FileTreeParseError::UnexpectedCharacter(c.into()),
            None => FileTreeParseError::UnexpectedEof,
        });
    }

    let identifier = String::from(&remaining_text[..size]);
    let remaining_text = consume_text(remaining_text, size);
    return Ok((remaining_text, identifier));
}

fn parse_command_list_recursive<'a>(remaining_text: &'a str, working_directory: &mut FsNode) -> Result<&'a str, FileTreeParseError> {
    let mut remaining_text = remaining_text;
    loop {
        match remaining_text.bytes().next() {
            None => break,
            Some(_) => (),
        }
        remaining_text = parse_literal(remaining_text, "$ cd ")?;

        if let Some(b'.') = remaining_text.bytes().next() {
            remaining_text = parse_literal(remaining_text, "..")?;
            remaining_text = skip_whitespace(remaining_text);
            break;
        }

        let (remaining, dir_name) = parse_identifier(remaining_text)?;
        remaining_text = remaining;
        let new_working_dir = match working_directory {
            FsNode::Directory(children) => match children.get_mut(&dir_name) {
                Some(child) => child,
                None => return Err(FileTreeParseError::ChildNotFound(dir_name)),
            },
            FsNode::File(_) => return Err(FileTreeParseError::NotADirectory(dir_name)),
        };

        remaining_text = skip_whitespace(remaining_text);
        remaining_text = parse_literal(remaining_text, "$ ls")?;
        remaining_text = skip_whitespace(remaining_text);
        remaining_text = parse_dir_entry_list(remaining_text, new_working_dir)?;

        remaining_text = parse_command_list_recursive(remaining_text, new_working_dir)?;
    }

    return Ok(remaining_text);
}

fn parse_uint(remaining_text: &str) -> Result<(&str, u32), FileTreeParseError> {
    let size = remaining_text.bytes().take_while(|ch| ch.is_ascii_digit()).count();

    if size == 0 {
        return Err(match remaining_text.bytes().next() {
            Some(c) => FileTreeParseError::UnexpectedCharacter(c.into()),
            None => FileTreeParseError::UnexpectedEof,
        });
    }

    let uint = remaining_text[..size].parse().expect("should be only ascii digits");
    let remaining_text = consume_text(remaining_text, size);
    return Ok((remaining_text, uint));
}

fn parse_file_name(remaining_text: &str) -> Result<(&str, String), FileTreeParseError> {
    let size = remaining_text.bytes().take_while(|ch| ch.is_ascii_alphabetic() || *ch == b'.').count();

    if size == 0 {
        return Err(match remaining_text.bytes().next() {
            Some(c) => FileTreeParseError::UnexpectedCharacter(c.into()),
            None => FileTreeParseError::UnexpectedEof,
        });
    }

    let name = &remaining_text[..size];
    let remaining_text = consume_text(remaining_text, size);
    return Ok((remaining_text, name.to_string()));
}

