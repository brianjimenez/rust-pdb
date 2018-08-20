use std::ops;

#[derive(Debug)]
pub struct Atom {
    pub name: String,
    pub number: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub occupancy: f32,
    pub bfactor: f32,
}

impl Atom {
    pub fn new(name: String, number: u32, x: f32, y: f32, z: f32, occupancy: f32, bfactor: f32) -> Atom {
        Atom {
            name,
            number,
            x,
            y,
            z,
            occupancy,
            bfactor,
        }
    }
}

impl ops::Sub for Atom {
    type Output = f32;

    fn sub(self, other: Atom) -> f32 {
        (
            (self.x - other.x)*(self.x - other.x) + 
            (self.y - other.y)*(self.y - other.y) + 
            (self.z - other.z)*(self.z - other.z)
        ).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn atom_distance() {
        let a1 = Atom::new("H".to_string(), 1, 2.0, 2.0, 2.0, 0.0, 0.0);
        let a2 = Atom::new("H".to_string(), 2, 0.0, 2.0, 2.0, 0.0, 0.0);
        assert_eq!(a1 - a2, 2.0);
    }
}