use std::env;
use std::io;

#[derive(Debug)]
pub struct Config
{
    pub filenames:    Vec<String>,
    pub switches:     Vec<String>,
    pub number_lines: bool
}

impl Config
{
    pub fn from_env_args() -> Config
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

        let number_lines = filenames.contains(&"-n".to_string()) || filenames.contains(&"--number".to_string());

        Config {filenames, switches, number_lines}
    }

    pub fn copy_stdin(&self)
    {
        let mut input_buf = String::new();
        let mut line_num: u16 = 1;

        loop 
        {
            io::stdin()
                .read_line(&mut input_buf)
                .expect("Could not read standard input!");

            if self.number_lines { print!("\t{line_num}  "); }

            // We print! because read_line adds a newline for us.
            print!("{}", input_buf);

            input_buf.clear();
            line_num += 1;
        }
    }
}