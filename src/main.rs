use std::env;
use std::io;

fn main() 
{
    let args: Vec<String> = env::args().collect();
    println!("{}", args.len());

    // There will always be at least 1 argument: the binary path.
    if args.len() == 1 { copy_stdin(); }
}

fn copy_stdin()
{
    unimplemented!();
}