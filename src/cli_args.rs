use super::quiz::Level;
use clap::Clap;
use std::{fs::File, io::Read, str::FromStr};

/// command line arguments
#[derive(Default, Clap, Debug)]
#[clap(version = "0.1.7")]
pub struct Args {
    /// quiz name
    #[clap(long = "name")]
    quiz_name: Option<String>,

    /// quiz id
    #[clap(long = "id")]
    quiz_id: Option<u64>,

    /// random pick one
    #[clap(short, long)]
    random: bool,

    /// interact or not
    #[clap(short)]
    interact: bool,

    /// difficulty of quiz
    #[clap(short, long)]
    level: Option<String>,

    /// show code snippet
    #[clap(short, long = "code")]
    code_snippet: Option<String>,

    /// template string
    #[clap(long = "temp-str")]
    temp_str: Option<String>,

    /// template file
    #[clap(long = "temp-file")]
    temp_file: Option<String>,

    /// token string
    #[clap(long = "token")]
    token: Option<String>,
}

impl Args {
    pub fn name(&self) -> &Option<String> {
        &self.quiz_name
    }

    pub fn if_random(&self) -> bool {
        self.random
    }

    pub fn if_interact(&self) -> bool {
        self.interact
    }

    pub fn quiz_id(&self) -> &Option<u64> {
        &self.quiz_id
    }

    pub fn level(&self) -> Option<Level> {
        match self.level {
            Some(ref s) => Level::from_str(s).map_or_else(
                |e| {
                    println!("{:?}", e.to_string());
                    None
                },
                |a| Some(a),
            ),
            None => None,
        }
    }

    pub fn if_show_code_snippet(&self) -> &Option<String> {
        &self.code_snippet
    }

    /// give template string, if has template file instead of string
    /// use template file prior.
    pub fn template(&self) -> Option<String> {
        if let Some(filepath) = &self.temp_file {
            let mut f = match File::open(filepath) {
                Ok(f) => f,
                Err(e) => panic!("read temp file has issue: {}", e),
            };
            let mut result = String::new();
            f.read_to_string(&mut result).unwrap();
            // update temp_str with file
            Some(result)
        } else {
            self.temp_str.clone()
        }
    }

    pub fn token(&self) -> &Option<String> {
        &self.token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_parse() {
        let a = Args::parse_from(["a", "-l", "e"]);
        dbg!(a);
    }
}
