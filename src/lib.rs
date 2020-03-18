use std::fs;
use std::error::Error;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.filename)?;
    println!("\n>>>");
    for (i, line) in search(&config.query, &contents) {
        println!("{}: {}", i, line);
    }
    println!("<<<");

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(query))
        .map(|(i, line)| (i+1, line))
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust:\n\
                        safe, fast, productive.\n\
                        Pick three!";

        assert_eq!(
            vec![(2, "safe, fast, productive.")],
            search(query, contents)
        );
    }
}
