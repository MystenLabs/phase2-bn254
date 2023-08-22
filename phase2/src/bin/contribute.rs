extern crate rand;
extern crate phase2;
extern crate num_bigint;
extern crate num_traits;
extern crate blake2;
extern crate byteorder;
extern crate exitcode;
extern crate itertools;

use phase2::phase2::contribute;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 && args.len() != 6 {
        println!("Usage: \n<in_params.params> <out_params.params> <in_str_entropy>");
        std::process::exit(exitcode::USAGE);
    }
    if args.len() == 6 && args[4] != "-v" {
        println!("Usage: \n<in_params.params> <out_params.params> <in_str_entropy> -v <progress_interval>");
        std::process::exit(exitcode::USAGE);
    }
    let in_params_filename = &args[1];
    let out_params_filename = &args[2];
    let entropy = &args[3];
    let print_progress = args.len() == 6 && args[4] == "-v";

    let mut progress_update_interval: u32 = 0;
    if print_progress {
        let parsed = args[5].parse::<u32>();
        if !parsed.is_err() {
            progress_update_interval = parsed.unwrap();
        }
    }

    let hash = contribute(in_params_filename, out_params_filename, entropy, print_progress, progress_update_interval);
    match hash {
        Ok(_) => println!("Contributed successfully"),
        Err(e) => println!("Error occurred: {:?}", e),
    }
}
