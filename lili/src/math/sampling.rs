/// Computes the balance heuristic for two distributions
///
/// This is a heuristic to weight the importance of distributions for
/// multiple importance sampling.
fn balance_heuristic(nf: u32, f_pdf: f32, ng: u32, g_pdf: f32) -> f32 {
    ((nf as f32) * f_pdf) / ((nf as f32) * f_pdf + (ng as f32) * g_pdf)
}

/// Computes the power heuristic for two distributions
///
/// This is a heuristic to weight the importance of distributions for
/// multiple importance sampling.
///
/// Beta is chosen as 2
fn power_heuristic(nf: u32, f_pdf: f32, ng: u32, g_pdf: f32) -> f32 {
    let f = (nf as f32) * f_pdf;
    let g = (ng as f32) * g_pdf;

    (f * f) / ((f * f) + (g * g))
}
