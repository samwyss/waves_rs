use std::{env, process};

use waves_rs::{Config, Run};

fn main() {
    // create new config enum
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error in configuration: {err}");
        process::exit(1);
    });

    // run desired simulation
    match config.run() {
        Ok(_) => println!("waves_rs completed simulation successfully and is now exiting"),
        Err(err) => eprintln!("Error running simulation: {err}"),
    }
}
