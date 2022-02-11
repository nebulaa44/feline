use std::env;
use std::fs;
use std::io::{self, Read};

#[derive(Debug)]
pub struct Config
{
    pub filenames:    Vec<String>,
    pub switches:     Vec<String>,

    pub debug:        bool,
    pub number_lines: bool,
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

        let number_lines = switches.contains(&"-n".to_string()) || switches.contains(&"--number".to_string());
        let debug = switches.contains(&"--dbg".to_string());

        Config {filenames, switches, debug, number_lines}
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

    pub fn copy_files(&self) -> io::Result<()>
    {
        for i in &self.filenames
        {
            if i == &"-".to_string()
            {
                self.copy_stdin();
            }

            let mut file = fs::File::open(i)?;

            let mut file_contents = String::new();
            file.read_to_string(&mut file_contents).expect("Could not read file!");

            // TODO Add -n support
            println!("{file_contents}");
        }

        Ok(())
    }
}