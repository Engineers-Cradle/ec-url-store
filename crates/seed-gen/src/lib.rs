use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use std::convert::TryInto;

pub fn generate_seed(seed: u32) -> String {
    const ALPHANUMERIC_CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-";
    let seed: u64 = seed.try_into().unwrap();

    let mut rng: Pcg32 = SeedableRng::seed_from_u64(seed);

    (0..8)
        .map(|_| {
            let index = rng.gen::<u64>() % ALPHANUMERIC_CHARS.len() as u64;
            ALPHANUMERIC_CHARS[index as usize] as char
        })
        .collect::<String>()
}