mod cli;
use clap::Parser;
use std::{env, fs, path};
fn main() -> std::io::Result<()> {
    let current_path = env::current_dir().unwrap();

    let cli = cli::Cli::parse();

    if let Some(config_path) = cli.config.as_deref() {
        println!("current Path is {:?}", current_path);
        println!("config Path is {:?}", config_path);
        let mut abs_config_path = path::PathBuf::from(config_path);
        
        if config_path.is_absolute() {
            abs_config_path = fs::canonicalize(config_path).unwrap();
            println!("1 config Absolute Path is {:?}", abs_config_path);
        } else {
            abs_config_path = current_path.join(config_path);
            abs_config_path = fs::canonicalize(config_path).unwrap();
            println!("2 config Absolute Path is {:?}", abs_config_path);
        };

        match abs_config_path.try_exists() {
            Ok(status) => {
                println!("config is exists");
            }
            Err(err) => {
                println!("reading config has Error {:?}", err);
            }
        };

        if abs_config_path.is_file() {
            println!("config is file");
        } else {
            println!("config is dir");
        }
    }
    // dynamic
}
