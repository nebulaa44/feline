use std::env;
use std::io;

mod config;
use config::Config;

fn main() 
{
    let config = Config::new();
    
    if config.filenames.len() == 0 { copy_stdin(); }
}

fn copy_stdin()
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