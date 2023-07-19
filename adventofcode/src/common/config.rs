use std::fs;

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

        let content = match args.next() {
            None => return Err("need file path argument"),
            Some(path) => fs::read_to_string(path),
        };

        return content
            .map(|str| FileContentConfig { content: str })
            .map_err(|_| "could not read file");
    }
}
