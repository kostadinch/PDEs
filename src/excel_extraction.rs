use std::fs::File;
use std::io::{self, Write};
use ndarray::{Array2};

///# Write Matrix to Excel
/// Writes a matrix to an Excel file.
pub fn write_matrix_to_csv(matrix: &Array2<f64>, filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    for row in matrix.rows() {
        let line = row.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        writeln!(file, "{}", line)?;
    }
    Ok(())
}