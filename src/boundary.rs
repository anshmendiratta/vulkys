#[derive()]
pub struct Boundary {
    rangex: Vec<f32>,
    rangey: Vec<f32>,
}

impl Boundary {
    pub fn get_xrange(&self) -> &Vec<f32> {
        &self.rangex
    }

    pub fn get_yrange(&self) -> &Vec<f32> {
        &self.rangey
    }
}

