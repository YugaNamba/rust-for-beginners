use std::env;
use std::error::Error;

use minigrep;

mod settings;
use settings::Settings;

fn main() -> Result<(), Box<dyn Error>> {
    let settings = Settings::new()?;
    println!("{:?}", settings);

    let args: Vec<String> = env::args().collect();
    let config = minigrep::Config::new(args)?;

    println!("Searching for {:?}", config);

    minigrep::run(config)?;

    Ok(())
}
