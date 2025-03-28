use ecolysis_core::*;
use extendr_api::prelude::*;

/// Deterministic population viability analysis based on a matrix containing survival and
/// reproduction rates for each age class and a vector counting the starting number of
/// individuals in each age class. The number of generations to calculate must also be included.
/// @export
#[extendr]
fn determ_pva(
    survival_matrix: RMatrix<f64>,
    population_matrix: &[f64],
    generations: u32,
) -> RMatrix<f64> {
    let ncol = survival_matrix.nrows();
    let p_vec = population_matrix.to_vec();
    let s_mat: Vec<_> = survival_matrix
        .data()
        .to_vec()
        .chunks_exact(ncol)
        .map(|col| col.to_vec())
        .collect();
    let output = PvaDeterministicPopulation::build_from_vectors(p_vec, s_mat)
        .expect("Inputs are not formatted correctly.")
        .deterministic_projection(generations)
        .return_numerical_output();
    RMatrix::new_matrix(Vec::len(&output), Vec::len(&output[0]), |r, c| output[r][c])
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod recolysis;
    fn determ_pva;
}
