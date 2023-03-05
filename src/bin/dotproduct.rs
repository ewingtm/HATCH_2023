// How do you normalize a vector to length 1 without having floating point numbers?
// Perhaps this works: for a Fhe type, (say of two bit-width), the length of the vector to normalize to
// is the maximum length of the vector.

// Or... we have some sort of pre-normalized dv we're already working on.

// TODO: Multiplies take way too long.

use concrete::prelude::*;
use concrete::{generate_keys, set_server_key, ConfigBuilder, FheBool, FheUint4};

// Remember, this rolls over.
fn compute_dot_product_fhe4(a: &[FheUint4], b: &[FheUint4]) -> FheUint4 {
    let mut out = vec![];

    // Multiply all components together.
    for (num_a, num_b) in a.iter().zip(b) {
        out.push(num_a * num_b);
    }

    let mut e = out[0].clone();
    for num in &out[1..] {
        e = e + num;
    }

    return e;
}

fn compute_dot_product(a: &[u8], b: &[u8]) -> u8 {
    let mut sum = 0;

    for (a, b) in a.iter().zip(b) {
        sum += a * b;
    }

    return sum;
}

//
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Server setup.
    let config = ConfigBuilder::all_disabled().enable_default_uint4().build();
    let (client_key, server_key) = generate_keys(config);
    set_server_key(server_key);

    let a4 = [1u8, 2u8, 3u8, 0u8];
    let b4 = [2u8, 1u8, 0u8, 2u8];

    let enc_a4: Vec<FheUint4> = a4
        .iter()
        .map(|num| FheUint4::try_encrypt(*num, &client_key).unwrap())
        .collect();
    let enc_b4: Vec<FheUint4> = b4
        .iter()
        .map(|num| FheUint4::try_encrypt(*num, &client_key).unwrap())
        .collect();

    let enc_output = compute_dot_product_fhe4(&enc_a4, &enc_b4);
    let decrypted: u8 = enc_output.decrypt(&client_key);

    println!("DP_enc: {}", decrypted);
    println!("cleared: {}", compute_dot_product(&a4, &b4));

    Ok(())
}
