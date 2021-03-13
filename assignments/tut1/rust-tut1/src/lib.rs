pub type Float = f64;

pub type Matrix = Vec<Vec<Float>>;

fn dot(a: &Vec<Float>, x: &Vec<Float>) -> Result<Float, String> {
    if a.len() != x.len() {
        return Err("Wrong dimensions".to_string());
    }

    let sum = a.iter().zip(x.iter()).map(|(i, j)| i * j).sum();
    Ok(sum)
}

pub fn x_dot_x(x: &Vec<Float>) -> Float {
    dot(x, x).unwrap()
}

pub fn matrix_dot_vec(matrix: &Matrix, x: &Vec<Float>) -> Vec<Float> {
    let y = matrix
        .iter()
        .map(|a| dot(a, x).expect("Error in dot product"))
        .collect();
    y
}

pub fn matrix_dot_matrix(matrix: &Matrix) -> Matrix {
    let n = matrix.len();

    let mut matrix_out = vec![vec![0.0; n]; n];

    for i in 0..n {
        for j in 0..n {
            let col = (0..n).map(|a| matrix[a][j]);
            matrix_out[i][j] = matrix[i]
                .iter()
                .zip(col)
                .map(|(a, b)| {
                    println!("(i:{},j:{}) a:{} b:{}", i, j, a, b);
                    a * b
                })
                .sum();
        }
    }
    matrix_out
}

pub fn vec_dot_matrix_dot_vec(matrix: &Matrix, x: &Vec<Float>) -> Float {
    let vector = matrix_dot_vec(matrix, x);
    dot(&vector, x).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_x_dot_x() {
        let x = vec![1.0, 2.0];
        assert_eq!(x_dot_x(&x), 5.0);
    }

    #[test]
    fn do_matrix_dot_x() {
        let matrix = vec![vec![1.0, 2.0], vec![2.0, 1.0]];
        let x = vec![0.0, 1.0];

        let y = vec![2.0, 1.0];

        assert_eq!(matrix_dot_vec(&matrix, &x), y);
    }

    #[test]
    fn do_matrix_dot_matrix() {
        let a = vec![vec![1.0, 1.0], vec![2.0, 0.0]];
        let y = vec![vec![3.0, 1.0], vec![2.0, 2.0]];

        assert_eq!(matrix_dot_matrix(&a), y);
    }

    #[test]
    fn do_vec_dot_matrix_dot_x() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let x = vec![2.0, 1.0];

        assert_eq!(vec_dot_matrix_dot_vec(&a, &x), 5.0)
    }
}
