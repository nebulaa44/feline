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
        let mut line_num: u16 = 1;

        let number_lines = self.switches.contains(&"-n".to_string()) || self.switches.contains(&"--number".to_string());

        loop 
        {
            io::stdin()
                .read_line(&mut input_buf)
                .expect("Could not read standard input!");

            if number_lines { print!("\t{line_num}  "); }

            // We print! because read_line adds a newline for us.
            print!("{}", input_buf);

            input_buf.clear();
            line_num += 1;
        }
    }
}