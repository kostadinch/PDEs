use ndarray::Array2;

/// Type of the partial differential equation (PDE).
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]

/// This enum represents the type of the partial differential equation (PDE).
pub enum PdeType {
    Elliptic,
    Parabolic,
    Hyperbolic,
    None,
}

impl PdeType {
    /// # Get Type
    /// This function returns the type of the PDE.
    pub fn get_type(&self) -> PdeType {
        match self {
            PdeType::Elliptic => PdeType::Elliptic,
            PdeType::Parabolic => PdeType::Parabolic,
            PdeType::Hyperbolic => PdeType::Hyperbolic,
            PdeType::None => PdeType::None,
        }
    }
}

/// Elliptic PDE
/// This module contains the implementation of the elliptic PDE.
pub mod elliptic {
    /// # Solve Poisson
    /// This function solves the Poisson equation.
    pub fn solve_poisson_using_finite_differences_method(grid_dimension: 
        (usize, usize), dx: f64, dy: f64, u: &mut ndarray::Array2<f64>, f: &ndarray::Array2<f64>) {
        // Set the tolerance and the maximum number of iterations
        let tolerance = 1e-6;
        let max_iter = 10000;

        // Iterate over the grid
        for iter in 0..max_iter {
            // Initialize the maximum error
            let mut max_error: f64 = 0.0;

            // Iterate over the grid
            for i in 1..grid_dimension.0-1 {
                for j in 1..grid_dimension.1-1 {
                    // Update the value of the function
                    let u_new = 0.25 * (u[[i+1, j]] + u[[i-1, j]] 
                        + u[[i, j+1]] + u[[i, j-1]] - dx * dy * f[[i, j]]);
                    let error = (u_new - u[[i, j]]).abs();
                    max_error = max_error.max(error);
                    u[[i, j]] = u_new;
                }
            }

            // Check if the solution has converged
            if max_error < tolerance {
                println!("Converged after {} iterations", iter);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test the get_type function
    #[test]
    fn test_pde_type_get_type() {
        // Check the type of the PDE
        assert_eq!(PdeType::Elliptic.get_type(), PdeType::Elliptic);
        assert_eq!(PdeType::Parabolic.get_type(), PdeType::Parabolic);
        assert_eq!(PdeType::Hyperbolic.get_type(), PdeType::Hyperbolic);
        assert_eq!(PdeType::None.get_type(), PdeType::None);
    }

    // Test the solve_poisson function
    #[test]
    fn test_solve_poisson() {

        // Define the grid dimension, step sizes, function, and solution
        let grid_dimension = (5, 5);
        let dx = 1.0;
        let dy = 1.0;
        let mut u = Array2::<f64>::zeros(grid_dimension);
        let f = Array2::<f64>::ones(grid_dimension);

        // Solve the Poisson equation
        elliptic::solve_poisson_using_finite_differences_method(grid_dimension, dx, dy, &mut u, &f);

        // Check if the solution has converged by verifying the values in u
        for i in 1..grid_dimension.0-1 {
            for j in 1..grid_dimension.1-1 {
                assert!((u[[i, j]] - 0.25).abs() < 1e-6);
            }
        }
    }
}
