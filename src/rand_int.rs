use rand::Rng;

pub fn generate_random_int() -> i32 {
    rand::thread_rng().gen_range(0..100)
}
