/// Represents the type of the partial differential equation (PDE).
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]

pub enum PDE_Type{
    Elliptic,
    Parabolic,
    Hyperbolic,
    None,
}

impl PDE_Type{
    pub fn get_type(&self) -> PDE_Type{
        match self{
            PDE_Type::Elliptic => PDE_Type::Elliptic,
            PDE_Type::Parabolic => PDE_Type::Parabolic,
            PDE_Type::Hyperbolic => PDE_Type::Hyperbolic,
            PDE_Type::None => PDE_Type::None,
        }

    }
}

pub mod Elliptic{ 
    pub fn poisson<F>(f: F, x0: f64, y0: f64, x_end: f64, step_size: f64) -> Vec<(f64, f64)>
    where F: Fn(f64, f64) -> f64{
        let mut x = x0;
        let mut y = y0;
        let mut result = Vec::new();
        result.push((x, y));
        while x <= x_end{
            result.push((x, y));
            x += step_size;
        }
        result
    }
        

}
