// declare cargo crates
use std::f64::consts::E;

// declare local code
use super::Run;

// fdtd1d struct
#[derive(Debug, Eq, PartialEq)]
pub struct FDTD1D {
    time_steps: usize,
    nodes: usize,
}

impl Run for FDTD1D {
    fn run(&self) -> Result<(), &'static str> {
        let mut ez: Vec<f64> = vec![0.0; self.nodes];
        let mut hy: Vec<f64> = vec![0.0; self.nodes];
        let eta0 = 377.0;

        // time stepping
        for step in 0..self.time_steps {
            // update fields
            for idx in 0..self.nodes {
                hy[idx] = hy[idx] + (ez[idx + 1] - ez[idx]) / eta0;
                ez[idx] = ez[idx] + (hy[idx] - hy[idx - 1]) * eta0;
            }

            // hardwired source
            ez[0] = E.powi(((step - 30) * (step - 30) / 100).try_into().unwrap());

            println!("ez[50] = {}", ez[50]);
        }

        Ok(())
    }
}

impl FDTD1D {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        // match time_steps
        let time_steps = match args.next() {
            Some(arg) => arg.parse::<usize>().unwrap(),
            None => return Err("number of time steps not provided"),
        };

        // match nodes
        let nodes = match args.next() {
            Some(arg) => arg.parse::<usize>().unwrap(),
            None => return Err("number of nodes not provided"),
        };

        Ok(Self { time_steps, nodes })
    }
}
