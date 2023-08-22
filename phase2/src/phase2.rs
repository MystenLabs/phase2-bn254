use itertools::Itertools;

use std::fs::File;
use std::fs::OpenOptions;

use parameters::*;
use utils::Phase2Error;

/// Make a new contribution with entropy to the last params file (in_params_filename) 
/// and writes to the new params file (out_params_filename).
/// Returns the contribution hash if successful, otherwises returns the error message.
pub fn contribute(
    in_params_filename: &String,
    out_params_filename: &String,
    entropy: &String,
    print_progress: bool,
    progress_update_interval: u32,
) -> Result<[u8; 64], Phase2Error> {
    let disallow_points_at_infinity = false;

    if print_progress {
        println!("starting");
    }
    // Create an RNG based on a mixture of system randomness and user provided randomness
    let mut rng = {
        use byteorder::{ReadBytesExt, BigEndian};
        use blake2::{Blake2b, Digest};
        use rand::{SeedableRng, Rng, OsRng};
        use rand::chacha::ChaChaRng;

        let h = {
            let mut system_rng = OsRng::new().unwrap();
            let mut h = Blake2b::default();

            // Gather 1024 bytes of entropy from the system
            for _ in 0..1024 {
                let r: u8 = system_rng.gen();
                h.input(&[r]);
            }

            // Hash it all up to make a seed
            h.input(&entropy.as_bytes());
            h.result()
        };

        let mut digest = &h[..];

        // Interpret the first 32 bytes of the digest as 8 32-bit words
        let mut seed = [0u32; 8];
        for i in 0..8 {
            seed[i] = digest.read_u32::<BigEndian>()?;
        }

        ChaChaRng::from_seed(&seed)
    };

    let reader = OpenOptions::new()
                            .read(true)
                            .open(in_params_filename)?;
    let mut params = MPCParameters::read(reader, disallow_points_at_infinity, true)?;

    println!("Contributing to {}...", in_params_filename);

    let hash = params.contribute(&mut rng, &progress_update_interval);
    println!("Contribution hash: 0x{:02x}", hash.iter().format(""));

    println!("Writing parameters to {}.", out_params_filename);
    let mut f = File::create(out_params_filename)?;
    params.write(&mut f)?;
    if print_progress {
        println!("New parameters are written to file");
    }
    Ok(hash)
}

/// Verify the last contribution.
/// Returns the hash of the contribution if valid, otherwise returns the error message.
pub fn verify_single_contribution(
    old_params_filename: &String,
    new_params_filename: &String,
) -> Result<[u8; 64], Phase2Error> {
    let disallow_points_at_infinity = false;

    let old_reader = OpenOptions::new()
                                .read(true)
                                .open(old_params_filename)?;
    let old_params = MPCParameters::read(old_reader, disallow_points_at_infinity, true)?;

    let new_reader = OpenOptions::new()
                                .read(true)
                                .open(new_params_filename)?;
    let new_params = MPCParameters::read(new_reader, disallow_points_at_infinity, true)?;

    verify_contribution(&old_params, &new_params)
}