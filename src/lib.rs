pub mod toy_dataset;
use std::fmt;

#[derive(Debug)]
pub enum Nucleotide {
    A,
    T,
    C,
    G,
}

// Nucleotide -> U8.

impl Into<u8> for Nucleotide {
    fn into(self) -> u8 {
        match self {
            Nucleotide::A => 0u8,
            Nucleotide::T => 1u8,
            Nucleotide::C => 2u8,
            Nucleotide::G => 3u8,
        }
    }
}

impl Into<u8> for &Nucleotide {
    fn into(self) -> u8 {
        match self {
            &Nucleotide::A => 0u8,
            &Nucleotide::T => 1u8,
            &Nucleotide::C => 2u8,
            &Nucleotide::G => 3u8,
        }
    }
}

impl fmt::Display for Nucleotide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Nucleotide::A => write!(f, "A"),
            Nucleotide::T => write!(f, "T"),
            Nucleotide::C => write!(f, "C"),
            Nucleotide::G => write!(f, "G"),
        }
    }
}

/// Data related to the spatial locality of the VCF file.
struct Locality {
    chromosome_number: usize,
    position: usize,
}

/// An entry for a specific VCF entry (think a SNP, variation to the reference human genome,
/// whatever).
struct VCFEntry {
    spatial: Locality,
    common_human_genome: Nucleotide,

    // The 'alt' within a VCF serves as the type array
    // for the index of the father/mother pair.
    // alt: [Option<Nucleotide>; 3],

    // The father/mother incidence of the nucleotide.
    rep_father_mother: Vec<[Nucleotide; 2]>,
}
