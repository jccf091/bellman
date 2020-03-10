#![allow(unused_imports)]
#![allow(unused_variables)]

use argh::FromArgs;
use bellperson::groth16::Parameters;
use bellperson::{Circuit, ConstraintSystem, SynthesisError};
use ff::{Field, PrimeField};
use paired::Engine;

use std::fs::File;
use std::io::prelude::*;

use std::thread::sleep;
use std::time::{Duration, Instant};

mod dummy;

/// Benchmark arbitrary sized circuits.
#[derive(FromArgs)]
struct Args {
    /// should parameters be generated or loaded from file?
    #[argh(switch)]
    generate_params: bool,
    /// how many constraints to generate?
    #[argh(option)]
    constraints: usize,
}

fn main() {
    env_logger::init();
    use bellperson::groth16::{
        create_random_proof, create_random_proof_batch, generate_random_parameters,
        prepare_verifying_key, verify_proof, Proof,
    };
    use paired::bls12_381::{Bls12, Fr};
    use rand::thread_rng;

    let args: Args = argh::from_env();
    let rng = &mut thread_rng();

    let parameters_path = "parameters.dat";

    let constraints = args.constraints;

    println!("Using {} constraints", constraints);

    // Create parameters for our circuit
    let params = if !args.generate_params {
        println!("Loading parameters");

        let param_file = File::open(parameters_path).expect("Unable to open parameters file!");
        Parameters::<Bls12>::read(param_file, false /* false for better performance*/)
            .expect("Unable to read parameters file!")
    } else {
        println!("Generating parameters");

        let c = dummy::DummyDemo::<Bls12> {
            constraints,
            xx: None,
        };

        let p = generate_random_parameters(c, rng).unwrap();
        let param_file = File::create(parameters_path).expect("Unable to create parameters file!");
        p.write(param_file)
            .expect("Unable to write parameters file!");
        p
    };

    // Prepare the verification key (for proof verification)
    let pvk = prepare_verifying_key(&params.vk);

    // Create an instance of circuit
    let c = dummy::DummyDemo::<Bls12> {
        constraints,
        xx: Fr::from_str("3"),
    };

    const SAMPLES: usize = 10;

    println!("Creating {} proofs...", SAMPLES);

    for _ in 0..SAMPLES {
        let now = Instant::now();

        // Create a groth16 proof with our parameters.
        let proof = create_random_proof(c.clone(), &params, rng).unwrap();
        println!(
            "Single proof gen finished in {}s and {}ms",
            now.elapsed().as_secs(),
            now.elapsed().subsec_nanos() / 1000000
        );

        println!("{}", verify_proof(&pvk, &proof, &[]).unwrap());
    }

    println!("Batch proof");
    let now = Instant::now();

    let proof = create_random_proof_batch(vec![c; SAMPLES], &params, rng).unwrap();

    println!(
        "Total proof batch gen finished in {}s and {}ms for {} proofs",
        now.elapsed().as_secs(),
        now.elapsed().subsec_nanos() / 1000000,
        SAMPLES
    );
}