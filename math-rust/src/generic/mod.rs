pub trait Mathematics {
    fn vec_add(&self, a: &[f32], b: &[f32]) -> Vec<f32>;
    fn vec_div(&self, a: &[f32], b: &[f32]) -> Vec<f32>;
    fn vec_mul(&self, a: &[f32], b: &[f32]) -> Vec<f32>;
    fn vec_sub(&self, a: &[f32], b: &[f32]) -> Vec<f32>;
}