mod config;
use config::Config;

fn main() 
{
    let config = Config::from_env_args();
    if config.switches.contains(&String::from("--dbg")) { println!("Config data\n{config:#?}\n"); }

    if config.filenames.len() == 0 { config.copy_stdin(); }
}