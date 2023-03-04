// TOY DATA.

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
