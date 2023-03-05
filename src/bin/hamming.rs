// A concrete homomorphic example for computing hamming distance. The general idea is given two
// sequences A, B compute the distance between them under E[A] & E[B].
use concrete::prelude::*;
use concrete::{generate_keys, set_server_key, ConfigBuilder, FheUint2, FheUint3};

use HATCH::{
    toy_dataset::{TOY_VECTOR_A_LEN4, TOY_VECTOR_A_LEN8, TOY_VECTOR_REF_LEN4, TOY_VECTOR_REF_LEN8},
    Nucleotide,
};

// Compute the xor of all integers, then condense them down to 0 or 1, then sum them all
// together to find the hamming distance.
fn compute_xor_gt_sum_fhe2(a: &[FheUint2], b: &[FheUint2]) -> FheUint2 {
    let mut out = vec![];

    for (num_a, num_b) in a.iter().zip(b) {
        let c = num_a ^ num_b;
        let d = c.gt(0);
        out.push(d);
    }

    // Out only contains numbers within the set {0, 1}.

    // Sum.
    let mut e = out[0].clone();
    for num in &out[1..] {
        e = e + num;
    }

    return e;
}

// Compute the xor of all integers, then condense them down to 0 or 1, then sum them all
// together to find the hamming distance. This time for FheUint3.
fn compute_xor_gt_sum_fhe3(a: &[FheUint3], b: &[FheUint3]) -> FheUint3 {
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

/// Compute the hamming distance between two u8s in fhe2.
fn hamming_distance_fhe2(a: &[u8], b: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    // Server setup.
    let config = ConfigBuilder::all_disabled().enable_default_uint2().build();
    let (client_key, server_key) = generate_keys(config);
    set_server_key(server_key);

    println!("UNENCRYPTED A: {:#?}", a);
    println!("UNENCRYPTED B: {:#?}", b);

    println!("ENCRYPTING");
    // encrypt.
    let enc_ref = a
        .iter()
        .map(|num| FheUint2::try_encrypt(*num, &client_key).unwrap())
        .collect::<Vec<FheUint2>>();

    let enc_toy = b
        .iter()
        .map(|num| FheUint2::try_encrypt(*num, &client_key).unwrap())
        .collect::<Vec<FheUint2>>();

    print!("HAMMING DISTANCE FROM DECRYPTED: ");
    let value = compute_xor_gt_sum_fhe2(&enc_ref, &enc_toy);
    let decrypted: u8 = value.decrypt(&client_key);
    println!("{}", decrypted);

    Ok(())
}

/// Compute the hamming distance between two u8s in fhe2.
fn hamming_distance_fhe3(a: &[u8], b: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    // Server setup.
    let config = ConfigBuilder::all_disabled().enable_default_uint3().build();
    let (client_key, server_key) = generate_keys(config);
    set_server_key(server_key);

    println!("UNENCRYPTED A: {:#?}", a);
    println!("UNENCRYPTED B: {:#?}", b);

    println!("ENCRYPTED");
    // encrypt.
    let enc_ref = a
        .iter()
        .map(|num| FheUint3::try_encrypt(*num, &client_key).unwrap())
        .collect::<Vec<FheUint3>>();

    let enc_toy = b
        .iter()
        .map(|num| FheUint3::try_encrypt(*num, &client_key).unwrap())
        .collect::<Vec<FheUint3>>();

    print!("HAMMING DISTANCE FROM DECRYPTED: ");
    let value = compute_xor_gt_sum_fhe3(&enc_ref, &enc_toy);
    let decrypted: u8 = value.decrypt(&client_key);
    println!("{}", decrypted);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a4: Vec<u8> = TOY_VECTOR_REF_LEN4.iter().map(|x| x.into()).collect();
    let b4: Vec<u8> = TOY_VECTOR_A_LEN4.iter().map(|x| x.into()).collect();

    hamming_distance_fhe2(&a4, &b4)?;

    let a8: Vec<u8> = TOY_VECTOR_REF_LEN8.iter().map(|x| x.into()).collect();
    let b8: Vec<u8> = TOY_VECTOR_A_LEN8.iter().map(|x| x.into()).collect();

    hamming_distance_fhe3(&a8, &b8)?;

    Ok(())
}
