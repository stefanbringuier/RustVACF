use ndarray::{Array2};
use std::io::{self, prelude::*, BufReader};
use RustVACF::modules::readers::{AtomType,Atom,Frame};
use RustVACF::modules::velocities::get_velocities;
use RustVACF::modules::vacf::vacf;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vacf() {
        let frames = vec![
            Frame {
                timestep: 0,
                num_atoms: 1,
                atoms: vec![
                    Atom {
                        id: 1,
                        atom_type: AtomType::Integer(1),
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                        vx: 1.0,
                        vy: 1.0,
                        vz: 1.0,
                    },
                ],
            },
            Frame {
                timestep: 1,
                num_atoms: 1,
                atoms: vec![
                    Atom {
                        id: 1,
                        atom_type: AtomType::Integer(1),
                        x: 2.0,
                        y: 2.0,
                        z: 2.0,
                        vx: 1.0,
                        vy: 1.0,
                        vz: 1.0,
                    },
                ],
            },
        ];

        let velocities = get_velocities(&frames);
        let vacf = vacf(&velocities);
        assert_eq!(vacf[0], 1.0);
    }
}