mod region;
mod boundary_conditions_type;
mod pde_type;
mod grid;
pub use crate::region::Region;
pub use crate::grid::Grid;
pub use crate::boundary_conditions_type::DirichletBoundaryConditions;
pub use crate::pde_type::Elliptic;

fn main() {
    let x_start = 3.0;
    let x_end = 7.0;
    let y_start = 4.0;
    let y_end = 7.0;
    let region= Region::new(x_start, x_end, y_start, y_end);

    let dx = 1.0;
    let dy = 1.0;

    let grid_dimension = region.get_grid_dimension(dx, dy);

    let grid = Grid::create_grid(grid_dimension.0, grid_dimension.1);
    let real_points = grid.get_real_points(region.x_start, region.y_start, dx, dy);
    

    let boundary_conditions = DirichletBoundaryConditions {
        up_boundary: Box::new(|x:f64| x),
        down_boundary: Box::new(|x:f64| x),
        left_boundary: Box::new(|y:f64| y),
        right_boundary: Box::new(|y:f64| y),
    };

    let u_at_boundary = boundary_conditions.get_u_at_boundary(
        &real_points, region.x_start, region.y_start, region.x_end, region.y_end);

    {
        let mut current_yy = 0.0;
        for point in &real_points {
            if point.1 != current_yy{
                println!();
                current_yy = point.1;
            }
            print!("{:?} ", point);
        }
        println!();
    let mut current_y = 0.0;
    for point in &u_at_boundary {
        if point.1 != current_y {
            println!();
            current_y = point.1;
        }
            print!("{:?} ", point);
    }
    println!();
    }

    //println!("Up boundary: {}", (boundary_conditions.up_boundary)(1.0));
    //let function_of_x_and_y= |x: f64, y: f64| x*y;
    


}