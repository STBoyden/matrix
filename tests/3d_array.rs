// Test that compiler will error if a > 2d array is passed as an argument
use matrix_multiplication::matrix;

fn main() {
    let _x = matrix!([[1, 2, 3], [4, [5], 6], [7, 8, 9]]);
}
