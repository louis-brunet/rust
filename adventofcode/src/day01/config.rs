#[derive(Debug)]
pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let file_path = match args.next() {
            None => return Err("need file path argument"),
            Some(path) => path,
        };

        return Ok(Config {
            file_path
        })
    }
}
