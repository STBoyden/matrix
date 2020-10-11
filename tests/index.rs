// Check that indexing works
use matrix_multiplication::matrix;

fn main() {
    let x = matrix!([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    assert_eq!(x[[2, 1]], 6);
    assert_eq!(x[[0, 0]], 1);
}
