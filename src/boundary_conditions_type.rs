
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
        // for point in points.iter() {
        //     // Check if the point is on the left boundary
            
        //     if point.0 == x_start {
        //         u[[point.0, point.1]] 
        //         = (self.left_boundary)(real_points[point.1].1);
        //     }
        //     // Check if the point is on the right boundary
        //     else if point.0 == x_end {
        //         u[[point.0, point.1]] 
        //         = (self.right_boundary)(real_points[point.1].1);
        //     }
        //     //Check if the point is on the down boundary
        //     else if point.1 == y_start
        //     && point.0 != x_start && point.0 != x_end{
        //         u[[point.0, point.1]] 
        //         = (self.down_boundary)(real_points[point.0].0);
        //     }
        //     // Check if the point is on the up boundary
        //     else if point.1 == y_end
        //     && point.0 != x_start && point.0 != x_end{
        //         u[[point.0, point.1]] 
        //         = (self.up_boundary)(real_points[point.0].0);
        //     }
        // }
        // Return the values of the function at the boundary
        u
    }
} 

