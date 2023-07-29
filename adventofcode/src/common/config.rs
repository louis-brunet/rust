use std::fs;
use std::io;
use std::io::Read;

#[derive(Debug)]
pub struct FilePathConfig {
    pub file_path: String,
}

#[derive(Debug)]
pub struct FileContentConfig {
    pub content: String,
}

impl FilePathConfig {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<FilePathConfig, &'static str> {
        args.next();

        let file_path = match args.next() {
            None => return Err("need file path argument"),
            Some(path) => path,
        };

        return Ok(FilePathConfig { file_path });
    }
}

impl FileContentConfig {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<FileContentConfig, &'static str> {
        args.next();

        let content: String;
        match args.next() {
            None => { // no file path, use stdin
                println!("No file path provided, using standard input:");
                let mut input = String::new();
                io::stdin().read_to_string(&mut input).map_err(|_| "could not read stdin")?;
                content = input;
            },

            Some(path) => {
                content = fs::read_to_string(path).map_err(|_| "could not read file")?;
            },
        };

        return Ok(FileContentConfig { content });
    }
}
