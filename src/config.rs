use std::env;
use std::io;

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

    pub fn copy_stdin(&self)
    {
        let mut input_buf = String::new();

        loop 
        {
            io::stdin()
                .read_line(&mut input_buf)
                .expect("Could not read standard input!");

            // We print! because read_line adds a newline for us.
            print!("{}", input_buf);

            input_buf.clear();
        }
    }
}