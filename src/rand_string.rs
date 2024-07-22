use rand::Rng;

pub fn generate_random_string() -> String {
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
    let mut rng = rand::thread_rng();
    (0..10).map(|_| chars[rng.gen_range(0..chars.len())]).collect()
}
