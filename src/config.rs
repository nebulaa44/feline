use std::env;

#[derive(Debug)]
pub struct Config
{
    pub filenames: Vec<String>,
    pub switches:  Vec<String>,
}

impl Config
{
    pub fn new() -> Config
    {
        let args = env::args();

        let mut filenames: Vec<String> = vec![];
        let mut switches:  Vec<String> = vec![];

        for (i, arg) in args.enumerate()
        {
            // The first argument is always the binary path.
            // We don't want to print that for obvious reasons.
            if i == 0 { continue; }

            if arg.starts_with("-") && arg != "-" { switches.push(arg); }
            else { filenames.push(arg); }
        }

        Config {filenames, switches}
    }
}