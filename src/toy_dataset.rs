// TOY DATA.
use rand::Rng;

use crate::Nucleotide;

/// Our "toy" dataset that is "equivalent" to the Reference HGV..
pub const TOY_VECTOR_REF_LEN4: [Nucleotide; 4] =
    [Nucleotide::A, Nucleotide::T, Nucleotide::C, Nucleotide::G];

pub const TOY_VECTOR_A_LEN4: [Nucleotide; 4] =
    [Nucleotide::A, Nucleotide::T, Nucleotide::C, Nucleotide::A];

pub const TOY_VECTOR_REF_LEN8: [Nucleotide; 8] = [
    Nucleotide::A,
    Nucleotide::T,
    Nucleotide::C,
    Nucleotide::G,
    Nucleotide::A,
    Nucleotide::A,
    Nucleotide::A,
    Nucleotide::A,
];

pub const TOY_VECTOR_A_LEN8: [Nucleotide; 8] = [
    Nucleotide::A,
    Nucleotide::T,
    Nucleotide::C,
    Nucleotide::A,
    Nucleotide::G,
    Nucleotide::G,
    Nucleotide::G,
    Nucleotide::G,
];

/// A distribution to sample from: A, A, A, A, T, T, T, C, C, G.
pub const SAMPLE_DIST: [Nucleotide; 9] = [
    Nucleotide::A,
    Nucleotide::A,
    Nucleotide::A,
    Nucleotide::T,
    Nucleotide::T,
    Nucleotide::T,
    Nucleotide::C,
    Nucleotide::C,
    Nucleotide::G,
];

/// Generate a random nucleotide sequences
fn generate_random<const N: usize, R: Rng>(rng: &mut R) -> [Nucleotide; N] {
    let mut out = [Nucleotide::A; N];
    for i in 0..N {
        let r = rng.gen::<u8>();
        let n: Nucleotide = r.into();
        out[i] = n;
    }

    out
}

/// Genrate a random nucleotide sequence from the distribution.
fn generate_distribution<const N: usize, R: Rng>(rng: &mut R) -> [Nucleotide; N] {
    let mut out = [Nucleotide::A; N];
    for i in 0..N {
        let n: Nucleotide = SAMPLE_DIST[rng.gen::<usize>() % 9];
        out[i] = n;
    }

    out
}
