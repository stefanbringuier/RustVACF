use RustVACF::modules::readers::{read_lammps_dump, AtomType};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_lammps_dump() {
        let dump_file = "tests/lammps_dump/dummy.1.dump";
        let result = read_lammps_dump(dump_file);

        // Assert that the result is Ok
        assert!(result.is_ok());

        // Unwrap the result to get the frames
        let frames = result.unwrap();

        // Assert the number of frames
        assert_eq!(frames.len(), 2);

        // Assert the first frame
        let frame1 = &frames[0];
        assert_eq!(frame1.timestep, 0);
        assert_eq!(frame1.num_atoms, 3);
        assert_eq!(frame1.atoms.len(), 3);

        let atom1 = &frame1.atoms[0];
        assert_eq!(atom1.id, 1);
        assert_eq!(atom1.atom_type, AtomType::Integer(1));
        assert_eq!(atom1.x, 0.0);
        assert_eq!(atom1.y, 0.0);
        assert_eq!(atom1.z, 0.0);
        assert_eq!(atom1.vx, 0.0);
        assert_eq!(atom1.vy, 0.0);
        assert_eq!(atom1.vz, 0.0);

        let atom2 = &frame1.atoms[1];
        assert_eq!(atom2.id, 2);
        assert_eq!(atom2.atom_type, AtomType::Integer(2));
        assert_eq!(atom2.x, 1.0);
        assert_eq!(atom2.y, 1.0);
        assert_eq!(atom2.z, 1.0);
        assert_eq!(atom2.vx, 0.0);
        assert_eq!(atom2.vy, 0.0);
        assert_eq!(atom2.vz, 0.0);

        let atom3 = &frame1.atoms[2];
        assert_eq!(atom3.id, 3);
        assert_eq!(atom3.atom_type, AtomType::Integer(1));
        assert_eq!(atom3.x, 2.0);
        assert_eq!(atom3.y, 2.0);
        assert_eq!(atom3.z, 2.0);
        assert_eq!(atom3.vx, 0.0);
        assert_eq!(atom3.vy, 0.0);
        assert_eq!(atom3.vz, 0.0);

        // Assert the second frame
        let frame2 = &frames[1];
        assert_eq!(frame2.timestep, 1);
        assert_eq!(frame2.num_atoms, 2);
        assert_eq!(frame2.atoms.len(), 2);

        let atom4 = &frame2.atoms[0];
        assert_eq!(atom4.id, 1);
        assert_eq!(atom4.atom_type, AtomType::Mass(16.0));
        assert_eq!(atom4.x, 0.0);
        assert_eq!(atom4.y, 0.0);
        assert_eq!(atom4.z, 0.0);
        assert_eq!(atom4.vx, 0.0);
        assert_eq!(atom4.vy, 0.0);
        assert_eq!(atom4.vz, 0.0);

        let atom5 = &frame2.atoms[1];
        assert_eq!(atom5.id, 2);
        assert_eq!(atom5.atom_type, AtomType::Mass(32.0));
        assert_eq!(atom5.x, 1.0);
        assert_eq!(atom5.y, 1.0);
        assert_eq!(atom5.z, 1.0);
        assert_eq!(atom5.vx, 0.0);
        assert_eq!(atom5.vy, 0.0);
        assert_eq!(atom5.vz, 0.0);
    }
}
