use rust_tut1::{matrix_dot_matrix, matrix_dot_vec, vec_dot_matrix_dot_vec, x_dot_x, Float, Matrix};

use rand::{thread_rng, Rng};
use rand::rngs::ThreadRng;
use std::time::Instant;

fn gen_random_vector(rng: &mut ThreadRng,dim: usize) -> Vec<Float> {
    let vector: Vec<Float> = (0..dim).map(|_| rng.gen_range(-1.0..=1.0)).collect();
    vector
}

fn main() {
    let dim = 3000usize;

    // let vector = vec![1.0; dim];
    // let matrix = vec![vec![1.0;dim];dim];
    let mut rng = thread_rng();
    let vector = gen_random_vector(&mut rng, dim);
 

    let mut matrix: Matrix = Vec::with_capacity(dim);
    for _i in 0..dim {
	matrix.push(Vec::with_capacity(dim))
    }
    
    for i in 0..dim {
	for j in 0..dim {
	    matrix[i].push(rng.gen_range(-1.0..=1.0))
	}
    }

    println!("Benchmarking:");

    let now = Instant::now();
    let inner_product = x_dot_x(&vector);
    println!(
        "x . x : {}\nTime: {}s",
        inner_product,
        now.elapsed().as_millis() as f64 / 1000.0
    );

    let now = Instant::now();
    let result = matrix_dot_vec(&matrix, &vector);
    let done = now.elapsed().as_millis() as f64 / 1000.0;
    println!("Vector length: {}\nTime: {}s", result.len(), done);
}
