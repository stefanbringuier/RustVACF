use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::f64;

// Enum to represent the different types for the 'type' attribute
#[derive(Debug, PartialEq, Clone)]
pub enum AtomType {
    Integer(i32),
    Mass(f64),
    Element(String),
}

// Struct to hold atom data
#[derive(Debug,Clone)]
pub struct Atom {
    pub id: i32,
    pub atom_type: AtomType,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
}


// Struct to hold frame data
#[derive(Debug)]
pub struct Frame {
    pub timestep: i32,
    pub num_atoms: usize,
    pub atoms: Vec<Atom>,
}


/// Reads a LAMMPS dump file and returns a vector of `Atom` structs.
///
/// The `Atom` struct has the following fields:
///
/// * `id`: The atom ID.
/// * `atom_type`: The atom type.
/// * `x`: The x-coordinate of the atom.
/// * `y`: The y-coordinate of the atom.
/// * `z`: The z-coordinate of the atom.
/// * `vx`: The x-velocity of the atom.
/// * `vy`: The y-velocity of the atom.
/// * `vz`: The z-velocity of the atom.
///
/// The function will return an error if the file does not contain the expected format.
///
/// # Arguments
///
/// * `filename`: The path to the LAMMPS dump file.
///
/// # Returns
///
/// A vector of frames with field having vector of `Atom` structs.
///
/// # Example
///
/// ```rust
/// use lammps_dump::read_lammps_dump;
///
/// fn main() {
///     let frames = read_lammps_dump("dump.lammps").unwrap();
///
///     // Do something with the atoms.
/// }
/// ```
pub fn read_lammps_dump(filename: &str) -> io::Result<Vec<Frame>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut frames: Vec<Frame> = Vec::new();
    let mut current_frame: Option<Frame> = None;

    // Use an iterator to read lines
    let mut lines = reader.lines();

    while let Some(Ok(line)) = lines.next() {
        if line.starts_with("ITEM: TIMESTEP") {
            if let Some(Ok(timestep_line)) = lines.next() {
                let timestep = timestep_line.trim().parse::<i32>()
                    .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid timestep value"))?;

                let frame = Frame {
                    timestep,
                    num_atoms: 0,
                    atoms: Vec::new(),
                };

                current_frame = Some(frame);
            }
        } else if line.starts_with("ITEM: NUMBER OF ATOMS") {
            if let Some(Ok(num_atoms_line)) = lines.next() {
                let num_atoms = num_atoms_line.trim().parse::<usize>()
                    .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid number of atoms value"))?;

                if let Some(ref mut frame) = current_frame {
                    frame.num_atoms = num_atoms;
                }
            }
        } else if line.starts_with("ITEM: ATOMS") {
            // Split to get type
            let atom_type_label = line.split_whitespace().nth(3)
            .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Invalid atom type"))?;
        

            let atom_type = determine_atom_type(atom_type_label)?;

            for _ in 0..current_frame.as_ref().unwrap().num_atoms {
                if let Some(Ok(atom_line)) = lines.next() {
                    let data: Vec<&str> = atom_line.split_whitespace().collect();
                    if data.len() != 8 {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Invalid atom data",
                        ));
                    }

                    let atom = Atom {
                        id: data[0].parse::<i32>().unwrap(),
                        atom_type: parse_atom_data(&atom_type, data[1]),
                        x: data[2].parse::<f64>().unwrap(),
                        y: data[3].parse::<f64>().unwrap(),
                        z: data[4].parse::<f64>().unwrap(),
                        vx: data[5].parse::<f64>().unwrap(),
                        vy: data[6].parse::<f64>().unwrap(),
                        vz: data[7].parse::<f64>().unwrap(),
                    };

                    if let Some(ref mut frame) = current_frame {
                        frame.atoms.push(atom);
                    }
                }
            }

            // At the end of an ATOMS section, we push the current frame to frames vector
            if let Some(frame) = current_frame.take() {
                frames.push(frame);
            }
        }
    }

    Ok(frames)
}


fn parse_atom_data(atom_type: &AtomType, data: &str) -> AtomType {
    match atom_type {
        AtomType::Integer(_) => {
            if let Ok(int_data) = data.parse::<i32>() {
                AtomType::Integer(int_data)
            } else {
                panic!("Data could not be parsed into an Integer")
            }
        }
        AtomType::Mass(_) => {
            if let Ok(float_data) = data.parse::<f64>() {
                AtomType::Mass(float_data)
            } else {
                panic!("Data could not be parsed into a Mass")
            }
        }
        AtomType::Element(_) => AtomType::Element(data.to_string()),
    }
}

/// Would be used to validate header for type/mass/element
fn determine_atom_type(atom_type_label: &str) -> io::Result<AtomType> {
    match atom_type_label {
        "type" => Ok(AtomType::Integer(0)),
        "mass" => Ok(AtomType::Mass(0.0)),
        _ => Ok(AtomType::Element(atom_type_label.to_string())),
    }
}
