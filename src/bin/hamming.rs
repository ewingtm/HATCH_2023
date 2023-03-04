// A concrete homomorphic example for computing hamming distance.
// This code is an iteratively written example and is not indicative of the
// final project.
use concrete::prelude::*;
use concrete::{generate_keys, set_server_key, ConfigBuilder, FheBool, FheUint2};

use HATCH::{
    toy_dataset::{TOY_VECTOR_A_LEN4, TOY_VECTOR_REF_LEN4},
    Nucleotide,
};

// TODO: Refacto into a single function. It looks like a moron wrote this.

/// Compute the xor of two arrays of FheUint2.
fn compute_xor(a: &[FheUint2], b: &[FheUint2]) -> Vec<FheUint2> {
    let mut out = vec![];

    for (num_a, num_b) in a.iter().zip(b) {
        out.push(num_a ^ num_b);
    }

    return out;
}

/// Compute the xor, then greater than 0 of two arrays of FheUint2. We compute
/// the greater than to strip the data from the xor, N -> {1, 0}.
fn compute_xor_gt(a: &[FheUint2], b: &[FheUint2]) -> Vec<FheUint2> {
    let mut out = vec![];

    for (num_a, num_b) in a.iter().zip(b) {
        let c = num_a ^ num_b;
        let d = c.gt(0);
        out.push(d);
    }

    return out;
}

/// Do both of the above, then sum the arrays. As a result, we have the
/// hamming distance.
fn compute_xor_gt_sum(a: &[FheUint2], b: &[FheUint2]) -> FheUint2 {
    let mut out = vec![];

    for (num_a, num_b) in a.iter().zip(b) {
        let c = num_a ^ num_b;
        let d = c.gt(0);
        out.push(d);
    }

    let mut e = out[0].clone();
    for num in &out[1..] {
        e = e + num;
    }

    return e;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Server setup.
    let config = ConfigBuilder::all_disabled().enable_default_uint2().build();
    let (client_key, server_key) = generate_keys(config);
    set_server_key(server_key);

    println!("REF VECTOR: {:?}", TOY_VECTOR_REF_LEN4);
    println!("A VECTOR:   {:?}", TOY_VECTOR_A_LEN4);

    // Covert each nucleotide into a vector of u8s.
    let clear_ref: Vec<u8> = TOY_VECTOR_REF_LEN4.iter().map(|x| x.into()).collect();
    let clear_toy: Vec<u8> = TOY_VECTOR_A_LEN4.iter().map(|x| x.into()).collect();

    println!("CONVERTED CLEAR_REF: {:?}", clear_ref);
    println!("CONVERTED CLEAR_A_VEC: {:?}", clear_toy);

    // encrypt.
    let enc_ref = clear_ref
        .iter()
        .map(|num| FheUint2::try_encrypt(*num, &client_key).unwrap())
        .collect::<Vec<FheUint2>>();

    let enc_toy = clear_toy
        .iter()
        .map(|num| FheUint2::try_encrypt(*num, &client_key).unwrap())
        .collect::<Vec<FheUint2>>();

    println!("XOR");
    let out = compute_xor(&enc_ref, &enc_toy);
    for value in out {
        let decrypted: u8 = value.decrypt(&client_key);
        print!("{} ", decrypted);
    }
    println!("");

    println!("XOR_GT");
    let out = compute_xor_gt(&enc_ref, &enc_toy);
    for value in out {
        let decrypted: u8 = value.decrypt(&client_key);
        print!("{} ", decrypted);
    }
    println!("");

    println!("HAMMING DISTANCE");
    let value = compute_xor_gt_sum(&enc_ref, &enc_toy);
    let decrypted: u8 = value.decrypt(&client_key);
    println!("{}", decrypted);

    Ok(())
}
