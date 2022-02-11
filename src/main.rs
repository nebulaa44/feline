mod config;
use config::Config;

fn main() 
{
    let config = Config::from_env_args();
    if config.debug { println!("Config data\n{config:#?}\n"); }

    if config.filenames.is_empty() { config.copy_stdin(); }
}