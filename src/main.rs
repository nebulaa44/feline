mod config;
use config::Config;

fn main() 
{
    let config = Config::from_env_args();
    if config.switches.contains(&String::from("--dbg")) { println!("Config data\n{config:#?}\n"); }

    if config.filenames.is_empty() { config.copy_stdin(); }
}