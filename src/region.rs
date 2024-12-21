pub struct Region {
    pub x_start: f64,
    pub x_end: f64,
    pub y_start: f64,
    pub y_end: f64,
}

impl Region {
    pub fn new(x_start: f64, x_end: f64, y_start: f64, y_end: f64) -> Self {
        Region {
            x_start,
            x_end,
            y_start,
            y_end,
        }
    }
    pub fn get_grid_dimension(&self, dx: f64, dy: f64) -> (usize, usize) {
        let width = ((self.x_end - self.x_start) / dx).abs() as usize;
        let height = ((self.y_end - self.y_start) / dy).abs() as usize;
        (width, height)
    }
}