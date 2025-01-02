use std::fs::File;
use std::io::{self, Write};
use ndarray::Array2;
use ndarray::array;
use std::fs;
use std::io::Read;

///# Write Matrix to Excel
/// Writes a matrix to an Excel file.
pub fn write_matrix_to_csv(matrix: &Array2<f64>, filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    // Iterate over the rows of the matrix and write them to the file
    for row in matrix.rows() {
        let line = row.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        writeln!(file, "{}", line)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test the write_matrix_to_csv function
    #[test]
    fn test_write_matrix_to_csv() {
        // Define the matrix and the filename
        let matrix = array![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];
        let filename = "test_output.csv";

        // Call the function
        write_matrix_to_csv(&matrix, filename).expect("Failed to write matrix to CSV");

        // Read the file back and check its contents
        let mut file = File::open(filename).expect("Failed to open test output file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read test output file");

        // Check the contents of the file
        let expected_contents = "1,2,3\n4,5,6\n";
        assert_eq!(contents, expected_contents);

        // Clean up
        fs::remove_file(filename).expect("Failed to delete test output file");
    }

    // Test the write_matrix_to_csv function with an empty matrix
    #[test]
    fn test_write_empty_matrix_to_csv() {
        // Define the matrix and the filename
        let matrix = Array2::<f64>::zeros((0, 0));
        let filename = "test_empty_output.csv";

        // Call the function
        write_matrix_to_csv(&matrix, filename).expect("Failed to write empty matrix to CSV");

        // Read the file back and check its contents
        let mut file = File::open(filename).expect("Failed to open test output file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read test output file");

        let expected_contents = "";
        assert_eq!(contents, expected_contents);

        // Clean up
        fs::remove_file(filename).expect("Failed to delete test output file");
    }
}
