use model::Model;

#[derive(Debug)]
pub struct Structure {
    pub models: Vec<Model>,
}

impl Structure {
    pub fn new(models: Vec<Model>) -> Structure {
        Structure {
            models,
        }
    }
}

impl Default for Structure {
    fn default() -> Structure {
        Structure {
            models: Vec::new(),
        }
    }
}
