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
    //let population = ecolysis_core::populations::PvaDeterministicPopulation::build_from_vectors(
    //    p_mat.to_vec(),
    //    s_mat.to_vec().into_iter().map(|item| item.to_vec).collect(),
    // Take the input dimensions
    let dims = survival_matrix.dim();

    // Get the number of rows and columns
    let nrows = dims[0] as usize;
    let ncols = dims[1] as usize;

    // Create a new vector to hold the transposed data
    let mut transposed_data = Vec::with_capacity(nrows * ncols);

    // Transpose the matrix by swapping rows and columns
    for row in 0..nrows {
        for col in 0..ncols {
            // Get a reference to the element at position (row, col)
            let value = &survival_matrix[[col, row]];
            // Collect the reference for the transposed matrix
            transposed_data.push(value);
        }
    }
    println!("{:?}", transposed_data);
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
