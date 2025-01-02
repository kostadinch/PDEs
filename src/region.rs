/// Region struct to define the region of interest for the simulation
pub struct Region {
    pub x_start: f64,
    pub x_end: f64,
    pub y_start: f64,
    pub y_end: f64,
}

impl Region {
    ///# New Region
    /// Creates a new instance of the Region struct.
    pub fn new(x_start: f64, x_end: f64, y_start: f64, y_end: f64) -> Self {
        Region {
            x_start,
            x_end,
            y_start,
            y_end,
        }
    }

    ///# Get Grid Dimensions
    /// Returns the width and height of the grid.
    pub fn get_grid_dimensions(&self, dx: f64, dy: f64) -> (usize, usize) {
        let width = ((self.x_end - self.x_start) / dx).abs() as usize;
        let height = ((self.y_end - self.y_start) / dy).abs() as usize;
        (width, height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test the new function
    #[test]
    fn test_region_creation() {
        // Create a new region
        let region = Region::new(0.0, 10.0, 0.0, 10.0);

        // Check the values of the region
        assert_eq!(region.x_start, 0.0);
        assert_eq!(region.x_end, 10.0);
        assert_eq!(region.y_start, 0.0);
        assert_eq!(region.y_end, 10.0);
    }

    // Test the get_grid_dimensions function
    #[test]
    fn test_get_grid_dimensions() {
        // Create a new region
        let region = Region::new(0.0, 10.0, 0.0, 10.0);
        
        // Get the grid dimensions
        let (width, height) = region.get_grid_dimensions(1.0, 1.0);

        // Check the values
        assert_eq!(width, 10);
        assert_eq!(height, 10);
    }

    // Test the get_grid_dimensions function with non-integer step sizes
    #[test]
    fn test_get_grid_dimensions_non_integer() {
        // Create a new region
        let region = Region::new(0.0, 10.0, 0.0, 10.0);

        // Get the grid dimensions
        let (width, height) = region.get_grid_dimensions(0.5, 0.5);

        // Check the values
        assert_eq!(width, 20);
        assert_eq!(height, 20);
    }

    // Test the get_grid_dimensions function with negative values
    #[test]
    fn test_get_grid_dimensions_negative() {
        // Create a new region
        let region = Region::new(-10.0, 0.0, -10.0, 0.0);

        // Get the grid dimensions
        let (width, height) = region.get_grid_dimensions(1.0, 1.0);

        // Check the values
        assert_eq!(width, 10);
        assert_eq!(height, 10);
    }
}
