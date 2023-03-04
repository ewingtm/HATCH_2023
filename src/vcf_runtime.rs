// TODO: Allow for selection of what to encode the nucleotides to, there's a potential chance we need u4... etc.
/// The runtime width we've decided to use for homomorphic encryption.
pub enum BitWidth {
    U2,  // represents all 4 nucleotides, with a maximum of 2^2..
    U3,  // represents all 4 nucleotides, with a maximum of 2^3
    U4,  // 2^4
    U8,  // 2^8
    U12, // 2^12,
    U16, // 2^16.
}
