
/// Represents the type of the boundary condition.
//#[derive(Debug, Clone, PartialEq, Hash, Eq)]

// pub enum BoundaryConditionsType{
//     Dirichlet,
//     Neumann,
//     Periodic,
//     None,
// }

pub struct DirichletBoundaryConditions {
    pub up_boundary: Box<dyn Fn(f64) -> f64>,
    pub down_boundary: Box<dyn Fn(f64) -> f64>,
    pub left_boundary: Box<dyn Fn(f64) -> f64>,
    pub right_boundary: Box<dyn Fn(f64) -> f64>,
}    

impl DirichletBoundaryConditions {
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
    pub fn get_u_at_boundary(&self, real_points: &Vec<(f64, f64)>, x_start: f64
    ,y_start: f64, x_end: f64, y_end: f64) 
    -> Vec<(f64, f64)> {
        let mut result = Vec::new();
        for point in real_points.iter() {
            if point.0 == x_start {
                result.push((point.0, (self.left_boundary)(point.1)));
            }
            else if point.0 == x_end {
                result.push((point.0, (self.right_boundary)(point.1)));
            }
            else if point.1 == y_start && point.0 != x_start && point.0 != x_end {
                result.push(((self.down_boundary)(point.0), point.1));
            }
            else if point.1 == y_end && point.0 != x_start && point.0 != x_end {
                result.push(((self.up_boundary)(point.0), point.1));
            }
        }
        result
    }
} 

