// Ensure the outputs are correct
use matrix_multiplication::matrix;

fn main() {
    let x = matrix!([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

    assert_eq!(x.0, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
