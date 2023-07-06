#![allow(non_snake_case)]


pub mod modules {
    pub mod readers;
    pub mod velocities;
    pub mod vacf;
}


use std::env;
use crate::modules::readers::read_lammps_dump;
use crate::modules::vacf::vacf;

fn main() {
    // Retrieve the command line arguments
    let args: Vec<String> = env::args().collect();

    // Check that the user provided a filename
    if args.len() < 2 {
        println!("Please provide a filename as a command line argument.");
        return;
    }

    // The first argument is the filename
    let filename = &args[1];

    // Read the file into frames
    let frames = match read_lammps_dump(filename) {
        Ok(frames) => frames,
        Err(err) => {
            println!("Error reading file: {}", err);
            return;
        }
    };

    // Compute the velocity autocorrelation function
    let vacf = vacf(&frames);

    // Print the velocity autocorrelation function
    for (lag, value) in vacf.iter().enumerate() {
        println!("lag {}: {}", lag, value);
    }
}
