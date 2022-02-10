use std::env;
use std::io;

fn main() 
{
    let args: Vec<String> = env::args().collect();

    // There will always be at least 1 argument: the binary path.
    if args.len() == 1 { copy_stdin(); }
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