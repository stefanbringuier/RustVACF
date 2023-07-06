
use ndarray::{Array2};
use crate::modules::readers::{Atom,Frame};

/// Extracts a 2D array of velocities from the provided frames.
///
/// The function takes a vector of `Frame` structs and creates a 2D array where each row
/// corresponds to a particle and each column corresponds to a timestep. The velocity components
/// for each timestep are stored in sets of three columns for vx, vy, vz.
///
/// # Arguments
///
/// * `frames`: A vector of `Frame` structs.
///
/// # Returns
///
/// A 2D array of velocities.
pub fn get_velocities(frames: &Vec<Frame>) -> Array2<f64> {
    let n_particles = frames[0].num_atoms;
    let n_timesteps = frames.len();

    let mut velocities = Array2::zeros((n_particles, 3 * n_timesteps));

    for (t, frame) in frames.iter().enumerate() {
        for (i, atom) in frame.atoms.iter().enumerate() {
            velocities[[i, 3 * t]] = atom.vx;
            velocities[[i, 3 * t + 1]] = atom.vy;
            velocities[[i, 3 * t + 2]] = atom.vz;
        }
    }

    velocities
}
