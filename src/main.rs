mod region;
mod boundary_conditions_type;
mod pde_type;
mod grid;
mod excel_extraction;
pub use crate::excel_extraction::write_matrix_to_csv;
pub use crate::region::Region;
pub use crate::grid::Grid;
pub use crate::boundary_conditions_type::DirichletBoundaryConditions;
pub use crate::pde_type::elliptic;
use std::f64::consts::PI;
use ndarray::Array2;

fn main() {
    // Define the region of interest 
    let x_start = 0.0;
    let x_end = 2.0;
    let y_start = 0.0;
    let y_end = 1.0;
    let region= Region::new(x_start, x_end, y_start, y_end);

    // Define the step size in the x and y directions
    let dx = 0.01;
    let dy = 0.01;

    // Get the grid dimension
    let grid_dimension = region.get_grid_dimensions(dx, dy);

    // Create the grid
    let grid = Grid::create_grid(grid_dimension.0, grid_dimension.1);

    // Get the real points from the grid
    let real_points 
    = grid.get_real_points(region.x_start, region.y_start, dx, dy);
    
    // Define the arrays to store the values of the function and the right-hand side
    let mut u 
    = Array2::<f64>::zeros((grid_dimension.0+1, grid_dimension.1+1));
    let mut f 
    = Array2::<f64>::zeros((grid_dimension.0+1, grid_dimension.1+1));

     // Define the boundary conditions
    let boundary_conditions = DirichletBoundaryConditions {
        up_boundary: Box::new(|x:f64| (PI*x).sin()),
        down_boundary: Box::new(|x:f64| (PI*x).sin()),
        left_boundary: Box::new(|y:f64| 0.0*y),
        right_boundary: Box::new(|y:f64| 0.0*y),
    };

    // Set the values of the function at the boundary
    u= boundary_conditions.get_u_at_boundary(&grid.points, &real_points, 0
        , 0, grid_dimension.0, grid_dimension.1, u);

    
    // Set the values of the right-hand side
    for i in 0..grid_dimension.0+1{
        for j in 0..grid_dimension.1+1{
            f[[i,j]] = -2.0*PI.powi(2)*((PI*real_points[i].0).sin())*((PI*real_points[j].1).sin());
        }
    }

    // Iterative solver (Gauss-Seidel)
    elliptic::solve_poisson_using_finite_differences_method(grid_dimension, dx, dy, &mut u, &f);

    // Reverse the axes of the solution so that it makes physical sense
    u=u.reversed_axes();

    // Write the solution to an Excel file
    if let Err(e) = write_matrix_to_csv(&u, "matrix.csv") {
        eprintln!("Failed to write CSV: {}", e);
    } else {
        println!("Matrix written to matrix.csv");
    }
}