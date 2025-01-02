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
    /// # Poisson
    /// This function solves the Poisson equation.
    pub fn poisson<F>(_f: F, x0: f64, y0: f64, x_end: f64, step_size: f64) -> Vec<(f64, f64)>
    where
        F: Fn(f64, f64) -> f64,
    {
        let mut x = x0;
        let y = y0;
        let mut result = Vec::new();
        result.push((x, y));
        while x <= x_end {
            result.push((x, y));
            x += step_size;
        }
        result
    }
}
