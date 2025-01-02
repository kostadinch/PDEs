///Grid module
#[derive(Debug)]

///This structs represents the grid.
pub struct Grid {
    pub points: Vec<(usize, usize)>,
    width: usize,
    height: usize,
}

impl Grid {
    ///# New Grid
    /// This function creates a new instance of the grid struct.
    pub fn create_grid(width: usize, height: usize) -> Self {
        let points = (0..height+1)
            .flat_map(|y| (0..width+1).map(move |x| (x, y)))
            .collect();
        Grid {
            points, 
            width, 
            height,
        }
    }

    ///# Print Points
    /// Prints the points of the grid.
    pub fn print_points(&self) {
        let mut current_y = 0;
        for point in &self.points {
            if point.1 != current_y {
                println!();
                current_y = point.1;
            }
            print!("{:?} ", point);
        }
        println!();
    }

    

    ///# Get Real Points
    /// Transforms the points of the grid into x-y coordinates.
    pub fn get_real_points(&self, x_start:f64,y_start:f64,dx:f64,dy:f64) -> Vec<(f64, f64)> {
        self.points.iter().map(|(x, y)| {
            (
                x_start+*x as f64 * dx,
                y_start+*y as f64 * dy,
            )
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test the new function
    #[test]
    fn test_create_grid() {
        // Create a new grid
        let grid = Grid::create_grid(2, 2);

        // Check the width, height, and number of points
        assert_eq!(grid.width, 2);
        assert_eq!(grid.height, 2);
        assert_eq!(grid.points.len(), 9); // (0,0) to (2,2) inclusive
    }

    // Test the print_points function
    #[test]
    fn test_print_points() {
        // Create a new grid
        let grid = Grid::create_grid(2, 2);

        // Call the print_points function to ensure that the systems doesn't panic
        grid.print_points();
    }

    // Test the get_real_points function
    #[test]
    fn test_get_real_points() {
        // Create a new grid
        let grid = Grid::create_grid(2, 2);

        // Get the real points
        let real_points = grid.get_real_points(0.0, 0.0, 1.0, 1.0);

        // Check the real points
        let expected_points = vec![
            (0.0, 0.0), (1.0, 0.0), (2.0, 0.0),
            (0.0, 1.0), (1.0, 1.0), (2.0, 1.0),
            (0.0, 2.0), (1.0, 2.0), (2.0, 2.0),
        ];

        // Check the real points
        assert_eq!(real_points, expected_points);
    }
}
