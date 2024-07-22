use rand::Rng;

pub fn generate_random_float() -> f64 {
    rand::thread_rng().gen_range(0.0..100.0)
}
