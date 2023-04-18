#[derive()]
pub struct Boundary {
    rangex: Vec<f64>,
    rangey: Vec<f64>,
}

impl Boundary {
    pub fn get_xrange(&self) -> &Vec<f64> {
        &self.rangex
    }

    pub fn get_yrange(&self) -> &Vec<f64> {
        &self.rangey
    }
}
