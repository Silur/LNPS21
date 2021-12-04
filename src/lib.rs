mod consts;
mod ntt;
use consts::*;
use rand::Rng;
use rand::distributions::{Distribution, Uniform};

pub struct DecryptionSecretKey {
    h1: [u64; KAPPA as usize],
    h2: [u64; KAPPA as usize],
    e1: [u64; (KAPPA + LAMBDA + ALPHA + 5) as usize],
    e2: [u64; (KAPPA + LAMBDA + ALPHA + 5) as usize]
}

impl DecryptionSecretKey {
    fn new<R: Rng>(rng: &mut R) -> Self {
        let range = Uniform::from(0..MU);
        let mut h1 = [0u64; KAPPA as usize];
        let mut h2 = [0u64; KAPPA as usize];
        let mut e1 = [0u64; (KAPPA+LAMBDA+ALPHA+5) as usize];
        let mut e2 = [0u64; (KAPPA+LAMBDA+ALPHA+5) as usize];
        for i in 0..KAPPA as usize {
            h1[i] = range.sample(rng);
            h2[i] = range.sample(rng);
            e1[i] = range.sample(rng);
            e2[i] = range.sample(rng);
        }
        for i in (KAPPA as usize)..(KAPPA+LAMBDA+ALPHA+5) as usize {
            e1[i] = range.sample(rng);
            e2[i] = range.sample(rng);
        }
        Self { h1, h2, e1, e2 }
    }
}

