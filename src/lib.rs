use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    // ? == return the result to contents and continue if OK,
    // return Err from function if unsuccessful

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_new_config() {
        let config = get_valid_config("find me");

        assert_eq!(config.filename, "poem.txt");
        assert_eq!(config.query, "find me");
    }

    #[test]
    fn invalid_new_config() {
        let args: Vec<String> = vec![String::from("minigrep"), String::from("cat")];
        let result = Config::new(&args);

        assert!(result.is_err());
    }

    fn get_valid_config(to_search: &str) -> Config {
        let config: Vec<String> = vec![String::from("minigrep"),
                                       to_search.to_string(),
                                       String::from("poem.txt")];

        Config::new(&config).unwrap()
    }
}


