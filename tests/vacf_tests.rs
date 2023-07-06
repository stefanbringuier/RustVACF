use RustVACF::modules::readers::{AtomType,Atom,Frame};
use RustVACF::modules::vacf::vacf;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vacf() {
        // Create 5 atoms with constant velocity
        let atoms: Vec<Atom> = (0..5).map(|i| Atom {
            id: i,
            atom_type: AtomType::Integer(i),
            x: 0.0,
            y: 0.0,
            z: 0.0,
            vx: 1.0,
            vy: 2.0,
            vz: 3.0,
        }).collect();

        // Create 10 frames with these atoms
        let frames: Vec<Frame> = (0..10).map(|i| Frame {
            timestep: i as i32,
            num_atoms: atoms.len(),
            atoms: atoms.clone(),
        }).collect();

        // Compute the VACF
        let vacf = vacf(&frames);

        // The VACF should be constant and equal to the sum of squares of the velocities for all timesteps.
        let expected_vacf = vec![(1.0 * 1.0) + (2.0 * 2.0) + (3.0 * 3.0); 10];
        
        assert_eq!(vacf, expected_vacf);
    }
}
