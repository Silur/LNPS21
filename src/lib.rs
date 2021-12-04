mod consts;
mod ntt;
use consts::*;
use rand::Rng;
use rand::distributions::{Distribution, Uniform};

pub struct DecryptionSecretKey {
    h1: [u64; KAPPA],
    h2: [u64; KAPPA],
    e1: [u64; KAPPA + LAMBDA + ALPHA + 5usize],
    e2: [u64; KAPPA + LAMBDA + ALPHA + 5usize]
}

impl DecryptionSecretKey {
    fn new<R: Rng>(rng: &mut R) -> Self {
        let range = Uniform::from(0..MU);
        let mut h1 = [0u64; KAPPA];
        let mut h2 = [0u64; KAPPA];
        let mut e1 = [0u64; KAPPA+LAMBDA+ALPHA+5usize];
        let mut e2 = [0u64; KAPPA+LAMBDA+ALPHA+5usize];
        for i in 0..KAPPA as usize {
            h1[i] = range.sample(rng);
            h2[i] = range.sample(rng);
            e1[i] = range.sample(rng);
            e2[i] = range.sample(rng);
        }
        for i in KAPPA..KAPPA+LAMBDA+ALPHA+5usize {
            e1[i] = range.sample(rng);
            e2[i] = range.sample(rng);
        }
        Self { h1, h2, e1, e2 }
    }
}

struct CommitmentParams {
    a0: [[u64; (KAPPA+LAMBDA+ALPHA+5)]; KAPPA],
    a1: [[u64; (KAPPA+LAMBDA+ALPHA+5)]; KAPPA],
    a2: [u64; (KAPPA+LAMBDA+ALPHA+5)],
    a3: [u64; (KAPPA+LAMBDA+ALPHA+5)],
    a4: [u64; (KAPPA+LAMBDA+ALPHA+5)],
    t1: [u64; (KAPPA+LAMBDA+ALPHA+5)],
    t2: [u64; (KAPPA+LAMBDA+ALPHA+5)],
}

impl CommitmentParams {
    pub fn new<R: Rng>(rng: &mut R) -> Self {
    let mut a0 = [[0u64; (KAPPA+LAMBDA+ALPHA+5)]; KAPPA];
    let mut a1 = [[0u64; (KAPPA+LAMBDA+ALPHA+5)]; KAPPA];
    let mut a2 = [0u64; (KAPPA+LAMBDA+ALPHA+5)];
    let mut a3 = [0u64; (KAPPA+LAMBDA+ALPHA+5)];
    let mut a4 = [0u64; (KAPPA+LAMBDA+ALPHA+5)];
    let mut t1 = [0u64; (KAPPA+LAMBDA+ALPHA+5)];
    let mut t2 = [0u64; (KAPPA+LAMBDA+ALPHA+5)];
        let range = Uniform::from(0..Q);
        for i in 0..KAPPA {
            for j in 0..KAPPA+LAMBDA+ALPHA+5 {
                a0[i][j] = range.sample(rng);
                a1[i][j] = range.sample(rng);
            }
        }
        for i in 0..KAPPA+LAMBDA+ALPHA+5 {
            a2[i] = range.sample(rng);
            a3[i] = range.sample(rng);
            a4[i] = range.sample(rng);
        }
        // TODO calc t1, t2
        Self { a0, a1, a2, a3, a4, t1, t2 }
    }
}
