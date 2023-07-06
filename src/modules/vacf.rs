use ndarray_parallel::prelude::*;
use rayon::prelude::*;
use crate::modules::readers::{Frame};
use crate::modules::velocities::get_velocities;


/// Computes the velocity autocorrelation function (VACF) from a series of molecular dynamics frames.
///
/// The VACF is a measure of the correlation between the velocities of particles at different time intervals.
/// It is computed as the ensemble average of the dot product of the velocity vectors of particles at different
/// times. 
///
/// The VACF calculation is parallelized over the time lags using the Rayon library. Each iteration of the 
/// `into_par_iter().map(|lag| ...)` loop computes the autocorrelation for a particular time lag, and these
/// computations are executed in parallel on different threads. Within each iteration, the loop over `j` steps
/// through the different starting times, and the inner loop over `i` steps through the different particles.
///
/// # Arguments
///
/// * `frames`: A vector of `Frame` structs containing the molecular dynamics trajectory data. Each frame
///     should contain the positions and velocities of each particle at a specific time.
///
/// # Returns
///
/// A vector of floats representing the VACF at different time lags.
///
/// # Example
///
/// ```rust
/// use crate::modules::readers::read_lammps_dump;
/// use crate::modules::vacf::vacf;
///
/// fn main() {
///     let frames = read_lammps_dump("dump.lammps").unwrap();
///     let vacf = vacf(&frames);
///
///     // Do something with the vacf.
/// }
/// ```
pub fn vacf(frames: &Vec<Frame>) -> Vec<f64> {
    // Compute velocities
    let velocities = get_velocities(frames);
    let n_particles = velocities.shape()[0];
    let n_timesteps = velocities.shape()[1] / 3;  // assuming vx, vy, vz for each timestep

    // Compute the autocorrelation
    let vacf: Vec<f64> = (0..n_timesteps).into_par_iter().map(|lag| {
        let mut sum = 0.0;
        for j in 0..(n_timesteps - lag) {
            for i in 0..n_particles {
                let vx1 = velocities[[i, 3 * j]];
                let vy1 = velocities[[i, 3 * j + 1]];
                let vz1 = velocities[[i, 3 * j + 2]];
                let vx2 = velocities[[i, 3 * (j + lag)]];
                let vy2 = velocities[[i, 3 * (j + lag) + 1]];
                let vz2 = velocities[[i, 3 * (j + lag) + 2]];

                let dot_product = vx1 * vx2 + vy1 * vy2 + vz1 * vz2;
                sum += dot_product;
            }
        }
        sum / (n_particles * (n_timesteps - lag)) as f64
    }).collect();

    vacf
}
