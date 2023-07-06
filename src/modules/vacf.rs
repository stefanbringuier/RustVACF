use ndarray::{Array2, Axis};
use ndarray_parallel::prelude::*;
use rayon::prelude::*;


pub fn vacf(velocities: &Array2<f64>) -> Vec<f64> {
    let n_particles = velocities.shape()[0];
    let n_timesteps = velocities.shape()[1] / 3;  // assuming vx, vy, vz for each timestep

    // Compute the ensemble average
    let mean_velocity = velocities.mean_axis(Axis(0)).unwrap();
    let velocities_centered = velocities - &mean_velocity;

    // Compute the autocorrelation
    let vacf: Vec<f64> = (0..n_timesteps).into_par_iter().map(|lag| {
        let mut sum = 0.0;
        for j in 0..(n_timesteps - lag) {
            for i in 0..n_particles {
                let vx1 = velocities_centered[[i, 3 * j]];
                let vy1 = velocities_centered[[i, 3 * j + 1]];
                let vz1 = velocities_centered[[i, 3 * j + 2]];
                let vx2 = velocities_centered[[i, 3 * (j + lag)]];
                let vy2 = velocities_centered[[i, 3 * (j + lag) + 1]];
                let vz2 = velocities_centered[[i, 3 * (j + lag) + 2]];

                let dot_product = vx1 * vx2 + vy1 * vy2 + vz1 * vz2;
                sum += dot_product;
            }
        }
        sum / (n_particles * (n_timesteps - lag)) as f64
    }).collect();

    vacf
}
