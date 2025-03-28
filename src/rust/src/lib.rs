use ecolysis_core::*;
use extendr_api::prelude::*;

/// IDK what this does
/// @export
#[extendr]
fn determ_pva(
    survival_matrix: RMatrix<f64>,
    population_matrix: &[f64],
    generations: u32,
) -> RMatrix<f64> {
    let ncol = survival_matrix.nrows();
    let s_vec = survival_matrix.data();
    let p_vec = population_matrix.to_vec();
    let s_mat: Vec<_> = s_vec
        .to_vec()
        .chunks_exact(ncol)
        .map(|col| col.to_vec())
        .collect();
    let pop = PvaDeterministicPopulation::build_from_vectors(p_vec, s_mat)
        .expect("Inputs are not formatted correctly.");
    let output = pop.deterministic_projection(generations);
    output.print_output();
    let num_out = output.return_numerical_output();
    let flattened = num_out
        .iter()
        .flat_map(|array| array.iter().cloned())
        .collect();
    RMatrix::new_matrix(
        Vec::len(&flattened) / Vec::len(&num_out[0]),
        Vec::len(&num_out[0]),
        |r, c| num_out[r][c],
    )
    //println!("{:?}", s_mat);
    //println!("\n");
    //println!("{:?}", p_vec);
    //println!("\n");
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod recolysis;
    fn hello_world;
    fn determ_pva;
}
