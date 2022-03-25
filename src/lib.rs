use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Trading... {}", config.command);
    Ok(())
}

pub struct Config {
    pub command: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 2 {
            return Err("too many arguments");
        }
        let command = args[1].clone();

        Ok(Config { command })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
