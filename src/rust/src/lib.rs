use ecolysis_core::*;
use extendr_api::prelude::*;
use transpose::*;

/// Deterministic population viability analysis based on a matrix containing survival and
/// reproduction rates for each age class and a vector counting the starting number of
/// individuals in each age class. The number of generations to calculate must also be included.
/// @export
#[extendr]
fn determ_pva(
    survival_matrix: RMatrix<f64>,
    population_vector: &[f64],
    generations: u32,
) -> RMatrix<f64> {
    let ncol = survival_matrix.nrows();
    let p_vec = population_vector.to_vec();
    let s_mat: Vec<_> = survival_matrix
        .data()
        .to_vec()
        .chunks_exact(ncol)
        .map(|col| col.to_vec())
        .collect();
    let output = PvaDeterministicPopulation::build_from_vectors(p_vec, s_mat)
        .expect("Inputs are not formatted correctly")
        .deterministic_projection(generations)
        .return_numerical_output();
    RMatrix::new_matrix(Vec::len(&output), Vec::len(&output[0]), |r, c| output[r][c])
}

/// Deterministic population viability analysis based on a matrix containing survival and
/// reproduction rates for each age class and a vector counting the starting number of
/// individuals in each age class. The number of generations to calculate must also be included.
/// @export
#[extendr]
fn stoch_pva(
    survival_matrix: RMatrix3D<f64>,
    population_vector: &[f64],
    generations: u32,
    iterations: u32,
) {
    let ncol = survival_matrix.ncols();
    let nsub = survival_matrix.nsub();
    let nrow = survival_matrix.nrows();

    let p_vec = population_vector.to_vec();

    let s_mat: Vec<_> = survival_matrix
        .data()
        .to_vec()
        .chunks_exact(nrow * ncol)
        .map(|sub| {
            let mut sub2 = sub.to_vec();
            let sub0 = sub.to_vec();
            transpose(&sub0, &mut sub2, nrow, ncol);
            sub2.to_vec()
                .chunks_exact(nrow)
                .map(|col| col.to_vec())
                .collect::<Vec<_>>()
        })
        .collect();

    println!("{:?}", s_mat);
    // let output = PvaDeterministicPopulation::build_from_vectors(p_vec, s_mat)
    //    .expect("Inputs are not formatted correctly")
    //    .deterministic_projection(generations)
    //    .return_numerical_output();
    // RMatrix::new_matrix(Vec::len(&output), Vec::len(&output[0]), |r, c| output[r][c])
}

#[cfg(test)]
mod tests {
    use super::*;
    use transpose;
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod recolysis;
    fn determ_pva;
    fn stoch_pva;
}
