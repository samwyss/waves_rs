#![crate_name = "waves_rs"]
//! A cross-platform, high-performance Rust implementation for various electromagnetic solvers

// local module declarations

pub trait Run {
    /// allows main.rs to run simulation as outlined in Config
    ///
    /// # Arguments
    ///
    /// &self - a reference to a Config enum
    fn run(&self) -> Result<(), &'static str>;
}

/// configuration enum, all simulations are given their own variant
#[derive(Debug, PartialEq, Eq)]
pub enum Config {
    // add simulation variants here
}

impl Config {
    /// Config enum constructor
    ///
    /// # Arguments
    ///
    /// `args` - an iterator containing Strings to be used as arguments
    ///
    /// # Errors
    ///
    /// - no simulation specified
    /// - provided simulation does not match any defined task
    /// - error propagated upward from subsequent function calls
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        //skips the path to the compiled file (first argument passed in)
        args.next();

        // errors if there is no simulation specified
        let sim = match args.next() {
            Some(arg) => arg,
            None => return Err("no simulation specified"),
        };

        // match an all lowercase task to a set of predefined simulations
        match sim.to_lowercase().as_str() {

            // add simulation cases to match against here

            // errors if desired simulation is not defined
            _ => return Err("provided task did not match any defined tasks"),
        }
    }
}

impl Run for Config {
    fn run(&self) -> Result<(), &'static str> {
        // match self against Config variants and run task
        println!("success");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// verifies Config::new() errors if args does contain a simulation
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Errors
    ///
    /// - Config::new() doesn't error if args does not contain a simulation
    #[test]
    fn config_new_no_sim() {
        // args iterator
        let args = [String::from("foo")].into_iter();

        assert!(Config::new(args).is_err());
    }

    /// verifies Config::new() errors if an invalid simulation is requested
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Errors
    ///
    /// - Config::new() doesnt error if an undefined simulation is requested
    #[test]
    fn config_new_invalid_sim() {
        // args iterator
        let args = [String::from("foo"), String::from("bar")].into_iter();
        assert!(Config::new(args).is_err());
    }
}