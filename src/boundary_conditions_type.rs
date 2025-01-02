
use ndarray::Array2;

/// Represents the boundary condition(for now only works for Dirichlet BC).
pub struct DirichletBoundaryConditions {
    pub up_boundary: Box<dyn Fn(f64) -> f64>,
    pub down_boundary: Box<dyn Fn(f64) -> f64>,
    pub left_boundary: Box<dyn Fn(f64) -> f64>,
    pub right_boundary: Box<dyn Fn(f64) -> f64>,
}    

impl DirichletBoundaryConditions {

    ///# New Dirichlet Boundary Conditions
    /// Creates a new instance of the DirichletBoundaryConditions struct.
    pub fn new(
        up_boundary: Box<dyn Fn(f64) -> f64>,
        down_boundary: Box<dyn Fn(f64) -> f64>,
        left_boundary: Box<dyn Fn(f64) -> f64>,
        right_boundary: Box<dyn Fn(f64) -> f64>,
    ) -> Self {
        DirichletBoundaryConditions {
            up_boundary,
            down_boundary,
            left_boundary,
            right_boundary,
        }
    }

    ///# Get U at Boundary
    /// Returns the values of the function at the boundary.
    pub fn get_u_at_boundary(&self, points:&Vec<(usize,usize)> , real_points: &Vec<(f64, f64)>
    , x_start: usize, y_start: usize, x_end:usize, y_end:usize, mut u: Array2<f64>) 
    -> Array2<f64> {
        // Iterate over the points
        // Check if the point is on the boundary and set the value of the function
        for point in points.iter() {
            u[[point.0, y_start]] = (self.down_boundary)(real_points[point.0].0);
            u[[point.0, y_end-1]] = (self.up_boundary)(real_points[point.0].0);
            u[[x_start, point.1]] = (self.left_boundary)(real_points[point.1].1);
            u[[x_end-1, point.1]] = (self.right_boundary)(real_points[point.1].1);
        }

        // Return the values of the function at the boundary
        u
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    // Test the new function
    #[test]
    fn test_new_dirichlet_boundary_conditions() {
        // Define the boundary conditions
        let up = Box::new(|x: f64| x + 1.0);
        let down = Box::new(|x: f64| x + 2.0);
        let left = Box::new(|x: f64| x + 3.0);
        let right = Box::new(|x: f64| x + 4.0);

        // Create the DirichletBoundaryConditions struct
        let bc = DirichletBoundaryConditions::
        new(up, down, left, right);

        // Check if the values are correct
        assert_eq!((bc.up_boundary)(1.0), 2.0);
        assert_eq!((bc.down_boundary)(1.0), 3.0);
        assert_eq!((bc.left_boundary)(1.0), 4.0);
        assert_eq!((bc.right_boundary)(1.0), 5.0);
    }

    // Test the get_u_at_boundary function
    #[test]
    fn test_get_u_at_boundary() {
        // Define the boundary conditions
        let up = Box::new(|x: f64| x + 1.0);
        let down = Box::new(|x: f64| x + 2.0);
        let left = Box::new(|x: f64| x + 3.0);
        let right = Box::new(|x: f64| x + 4.0);

        // Create the DirichletBoundaryConditions struct
        let bc = DirichletBoundaryConditions::
        new(up, down, left, right);

        // Define everything needed for the test
        let points = vec![(0, 0), (1, 1), (2, 2)];
        let real_points = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 2.0)];
        let x_start = 0;
        let y_start = 0;
        let x_end = 3;
        let y_end = 3;
        let f = Array2::<f64>::zeros((3, 3));

        // Call the function
        let u = bc.get_u_at_boundary(&points, &real_points, 
            x_start, y_start, x_end, y_end, f);

        // Check if the values are correct
        assert_eq!(u[[0, 0]], 2.0); // down_boundary at (0, 0)
        assert_eq!(u[[0, 2]], 1.0); // up_boundary at (0, 2)
        assert_eq!(u[[2, 0]], 2.0); // down_boundary at (2, 0)
        assert_eq!(u[[2, 2]], 3.0); // up_boundary at (2, 2)
        assert_eq!(u[[0, 0]], 3.0); // left_boundary at (0, 0)
        assert_eq!(u[[2, 0]], 4.0); // right_boundary at (2, 0)
        assert_eq!(u[[0, 2]], 3.0); // left_boundary at (0, 2)
        assert_eq!(u[[2, 2]], 4.0); // right_boundary at (2, 2)
    }
}

