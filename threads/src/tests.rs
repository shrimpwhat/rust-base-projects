use crate::multiply_matrices;

#[test]
fn test_multiply_matrices_basic() {
    let matrix_a = vec![vec![1, 2], vec![3, 4]];
    let matrix_b = vec![vec![2, 0], vec![1, 2]];
    let expected = vec![vec![4, 4], vec![10, 8]];
    assert_eq!(multiply_matrices(matrix_a, matrix_b), expected);
}

#[test]
fn test_multiply_matrices_identity() {
    let matrix_a = vec![vec![1, 0], vec![0, 1]];
    let matrix_b = vec![vec![5, 6], vec![7, 8]];
    let expected = vec![vec![5, 6], vec![7, 8]];
    assert_eq!(multiply_matrices(matrix_a, matrix_b), expected);
}

#[test]
fn test_multiply_matrices_zero() {
    let matrix_a = vec![vec![0, 0], vec![0, 0]];
    let matrix_b = vec![vec![0, 0], vec![0, 0]];
    let expected = vec![vec![0, 0], vec![0, 0]];
    assert_eq!(multiply_matrices(matrix_a, matrix_b), expected);
}

#[test]
fn test_multiply_matrices_non_square() {
    let matrix_a = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let matrix_b = vec![vec![7, 8], vec![9, 10], vec![11, 12]];
    let expected = vec![vec![58, 64], vec![139, 154]];
    assert_eq!(multiply_matrices(matrix_a, matrix_b), expected);
}

#[test]
#[should_panic]
fn invalid_dimensions() {
    let matrix_a = vec![vec![123, 45, -1], vec![2, 1, 0], vec![-56, 2, 3]];

    let matrix_b = vec![vec![99, 88], vec![66, -77]];

    multiply_matrices(matrix_a, matrix_b);
}
