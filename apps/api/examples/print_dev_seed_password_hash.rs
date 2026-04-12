//! Prints Argon2id PHC for `DevSeed!ChangeMe` using [`PasswordConfig::from_env`] defaults.
//! Paste output into `seeds/dev_seed_comprehensive.sql` if you change `PasswordConfig` defaults.
//! Run: `cargo run --example print_dev_seed_password_hash` from `apps/api`.

use api::{hash_password, PasswordConfig};

fn main() {
    let config = PasswordConfig::from_env();
    let hash = hash_password("DevSeed!ChangeMe", &config).expect("hash");
    println!("{hash}");
}
