use concrete::prelude::*;
use concrete::{generate_keys, set_server_key, ConfigBuilder, FheUint4};
/// The goal of this file is to utilize the hamming distance to compute an adjaceny matrix.
/// I've created some synthetic data: those that are pulled from weighted distribution are
/// a positive expression of a phenotype. Those that are pulled from the unweighted are not
/// an expression. Another option would be to solidify certain dimensions to certain values
/// (say, [A, T, *, *, C]...), but I've written it this way already.
use rand::Rng;
use HATCH::Nucleotide;

/// The total number of samples (people) to compare.
const SAMPLES: usize = 20;
/// The length of the nucleotide vector we are comparing: corresponds to 16 different places on the pair.
const NUCLEOTIDE_LENGTH: usize = 16;

#[derive(Copy, Clone, Debug)]
struct GenomeWithPhenotypicExpression<const N: usize> {
    pub data: [Nucleotide; N],

    // Whether the person has the disease or not. For this example, if we're weighted, we do,
    // and if we're unweighted, we don't.
    expression: bool,
}

// For this example, the Genome sampled from the distribution would result in the
// the phenotypic expression.
fn build_from_sample<const N: usize, R: Rng>(rng: &mut R) -> GenomeWithPhenotypicExpression<N> {
    let data = HATCH::toy_dataset::generate_distribution::<N, R>(rng);
    GenomeWithPhenotypicExpression {
        data,
        expression: true,
    }
}

// For this example, the Genome sampled from the distribution would result in the
// the phenotypic expression.
fn build_from_random<const N: usize, R: Rng>(rng: &mut R) -> GenomeWithPhenotypicExpression<N> {
    let data = HATCH::toy_dataset::generate_random::<N, R>(rng);
    GenomeWithPhenotypicExpression {
        data,
        expression: false,
    }
}

/// Compute the adjacency matrix with homomorphic encryption, it should return
/// an [NxN] symmetric matrix.
fn homomorphic_adj(samples: &[Vec<u8>]) -> [[u8; SAMPLES]; SAMPLES] {
    // This mirrors the implementaton of "compute_xor_gt_sum_fhe4" with bin/hamming. I'm just
    // going to copy it: I am extremely tired.
    fn hamming(a: &[FheUint4], b: &[FheUint4]) -> FheUint4 {
        let mut out = vec![];
        // Compute the xor, translate down to 0s and 1s.
        for (num_a, num_b) in a.iter().zip(b) {
            let c = num_a ^ num_b;
            let d = c.gt(0);
            out.push(d)
        }

        // Sum the 0s and 1s.
        let mut e = out[0].clone();
        for num in &out[1..] {
            e = e + num;
        }

        return e;
    }

    let config = ConfigBuilder::all_disabled().enable_default_uint4().build();
    let (client_key, server_key) = generate_keys(config);
    set_server_key(server_key);

    // Encode each sample homomorphically.
    let samples_enc: Vec<Vec<FheUint4>> = samples
        .iter()
        .map(|nuc_vector| {
            nuc_vector
                .iter()
                .map(|num| FheUint4::try_encrypt(*num, &client_key).unwrap())
                .collect::<Vec<FheUint4>>()
        })
        .collect();

    // Begin computing the hamming distance for each value. Of note, this is
    // embarassingly parallel. All the way down to the individual vector calculations
    // in the nucleotide.
    let mut adj = vec![];
    for i in 0..SAMPLES {
        let mut tmp = vec![];
        for j in 0..SAMPLES {
            tmp.push(hamming(&samples_enc[i], &samples_enc[j]));
            println!("Computed {},{}", i, j);
        }

        adj.push(tmp);
    }

    // adj is now a matrix of hamming distances, decrypt.
    let mut adj_dec = vec![];
    for i in 0..SAMPLES {
        let mut tmp = vec![];
        for j in 0..SAMPLES {
            let decrypted: u8 = adj[i][j].decrypt(&client_key);
            tmp.push(decrypted);
        }
        adj_dec.push(tmp);
    }

    // I wrote the function return as a [[u8; 20]; 20]. I'm not changing it.
    let mut adj_final = [[0u8; SAMPLES]; SAMPLES];
    for i in 0..SAMPLES {
        for j in 0..SAMPLES {
            adj_final[i][j] = adj_dec[i][j]
        }
    }

    adj_final
}

/// Compute the adjacency matrix without homomorphic encryption, it should return
/// an [NxN] symmetric matrix.
fn reg_adj(samples: &[Vec<u8>]) -> [[u8; SAMPLES]; SAMPLES] {
    // Hamming is the total number of values that aren't alike.
    fn hamming(a: &[u8], b: &[u8]) -> u8 {
        let mut sum = 0;
        for (c, d) in a.iter().zip(b) {
            if c != d {
                sum += 1;
            }
        }

        sum
    }

    let mut adj = [[0u8; SAMPLES]; SAMPLES];
    for i in 0..SAMPLES {
        for j in 0..SAMPLES {
            adj[i][j] = hamming(&samples[i], &samples[j]);
        }
    }

    adj
}

// The goal is to build an adj matrix of all the GenomeWithPhenotypicExpression.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();

    let mut samples: [GenomeWithPhenotypicExpression<NUCLEOTIDE_LENGTH>; SAMPLES] =
        [GenomeWithPhenotypicExpression {
            data: [Nucleotide::A; NUCLEOTIDE_LENGTH],
            expression: true,
        }; SAMPLES];

    // Build the samples that have the phenotypic expression.
    for i in 0..(SAMPLES / 2) {
        samples[i] = build_from_sample(&mut rng);
    }

    // Build the samples that do not have the phenotypic expression.
    for i in 10..SAMPLES {
        samples[i] = build_from_random(&mut rng);
    }

    // TODO: Why can't I make this a Vec<&[u8]>? Convert the data from [Nucleotide] into [u8].
    let samples_u8: Vec<Vec<u8>> = samples
        .iter()
        .map(|pair| {
            pair.data
                .iter()
                .map(|nucleotide| nucleotide.into())
                .collect::<Vec<u8>>()
        }) // Convert each [u8; NUCLEOTIDE]
        .collect();

    println!("### Computing reg.");
    // Compute an adjacency matrix without encryption.
    let adj = reg_adj(&samples_u8);

    // Print out CSV to stdout.
    println!("### 10 WEIGHTED AND 10 UNIFORM SAMPLE");
    for i in 0..SAMPLES {
        for j in 0..SAMPLES {
            print!("{},", adj[i][j]);
        }
        println!("");
    }

    println!("### Computing enc.");
    // Compute it with encryption for fun: if you've gotten this far and you're running this: this computation
    // will probably never finish. This is some massive computation of polynomials (400 total distance calculations
    // between 16 length vectors of Fhe4). My inclination is that this can run entirely in parallel, it operates
    // on seperately encoded Fhe4s. Remember, neural networks were a joke before AlexNet in 2012 which put them on
    // GPUs!
    let enc_dec_adj = homomorphic_adj(&samples_u8);

    Ok(())
}
