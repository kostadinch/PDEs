#[derive(Debug)]

pub struct Grid {
    pub points: Vec<(usize, usize)>,
    width: usize,
    height: usize,
}

impl Grid {
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

    pub fn get_real_points(&self, x_start:f64,y_start:f64,dx:f64,dy:f64) -> Vec<(f64, f64)> {
        self.points.iter().map(|(x, y)| {
            (
                x_start+*x as f64 * dx,
                y_start+*y as f64 * dy,
            )
        }).collect()
    }
}