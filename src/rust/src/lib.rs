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
fn determ_pva(
    survival_matrix: RMatrix<f64>,
    population_matrix: RMatrix<f64>,
    generations: i32,
) -> &'static str {
    let s_mat = survival_matrix.data();
    let p_mat = population_matrix.data();
    println!("\n");
    println!("{:?}", survival_matrix.data());
    println!("\n");
    "hello world"
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod recolysis;
    fn hello_world;
    fn determ_pva;
}
