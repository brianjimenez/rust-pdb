

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
