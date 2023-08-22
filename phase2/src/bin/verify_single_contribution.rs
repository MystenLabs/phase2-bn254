extern crate phase2;
extern crate exitcode;

use phase2::phase2::verify_single_contribution;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: \n<in_old_params.params> <in_new_params.params>");
        std::process::exit(exitcode::USAGE);
    }
    let old_params_filename = &args[1];
    let new_params_filename = &args[2];

    let contribution = verify_single_contribution(old_params_filename, new_params_filename);
    match contribution {
        Ok(_) => println!("Contribution verified"),
        Err(e) => println!("Error occurred: {:?}", e),
    }
}