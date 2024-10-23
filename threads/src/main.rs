use std::sync::{Arc, Mutex};
use std::thread;

#[cfg(test)]
mod tests;

pub fn multiply_matrices(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let a_rows = a.len();
    let a_cols = a[0].len();
    let b_rows = b.len();
    let b_cols = b[0].len();

    if a_cols != b_rows {
        panic!("Number of columns in the first matrix must be equal to the number of rows in the second matrix.");
    }

    let mutex_matrix = Arc::new(Mutex::new(vec![vec![0; b_cols]; a_rows]));
    let shared_a = Arc::new(a);
    let shared_b = Arc::new(b);

    let mut handlers = vec![];

    // One thread for each row of first matrix
    for i in 0..a_rows {
        let mutex_matrix = Arc::clone(&mutex_matrix);
        let a = Arc::clone(&shared_a);
        let b = Arc::clone(&shared_b);

        let handle = thread::spawn(move || {
            for j in 0..b_cols {
                for k in 0..a_cols {
                    let mut mutex_matrix = mutex_matrix.lock().unwrap();
                    mutex_matrix[i][j] += a[i][k] * b[k][j];
                }
            }
        });

        handlers.push(handle);
    }

    for handle in handlers {
        handle.join().unwrap();
    }

    let res = mutex_matrix.lock().unwrap().clone();
    res
}

fn main() {
    // Example https://www.wolframalpha.com/input?i2d=true&i=%7B%7B123%2C45%2C-1%7D%2C%7B2%2C1%2C0%7D%2C%7B-56%2C2%2C3%7D%7D*%7B%7B99%2C88%7D%2C%7B66%2C-77%7D%2C%7B55%2C44%7D%7D

    let matrix_a = vec![vec![123, 45, -1], vec![2, 1, 0], vec![-56, 2, 3]];

    let matrix_b = vec![vec![99, 88], vec![66, -77], vec![55, 44]];

    let result = multiply_matrices(matrix_a, matrix_b);

    for row in result {
        println!("{:?}", row);
    }
}
