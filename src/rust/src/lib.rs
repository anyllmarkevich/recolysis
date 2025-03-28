use ecolysis_core::*;
use extendr_api::prelude::*;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

/// IDK what this does
/// @export
#[extendr]
fn determ_pva(survival_matrix: RMatrix<f64>, population_matrix: &[f64], generations: i32) {
    let ncol = survival_matrix.nrows();
    let s_vec = survival_matrix.data();
    let p_vec = population_matrix;
    let s_mat: Vec<_> = s_vec
        .to_vec()
        .chunks_exact(ncol)
        .map(|col| col.to_vec())
        .collect();

    println!("{:?}", s_mat);
    println!("\n");
    println!("{:?}", p_vec);
    println!("\n");
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod recolysis;
    fn hello_world;
    fn determ_pva;
}
