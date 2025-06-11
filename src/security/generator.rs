use rand::distributions::Alphanumeric;
use rand::{Rng, thread_rng};

pub fn generate_device_secret() -> String {
    let secret: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    secret
}
