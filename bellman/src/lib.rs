#![allow(unused_imports)]
#![allow(unused_macros)]
#[macro_use]

extern crate cfg_if;
pub extern crate pairing;
extern crate rand;
extern crate bit_vec;
extern crate byteorder;

#[macro_use]
mod log;

pub mod domain;
pub mod groth16;

#[cfg(feature = "gm17")]
pub mod gm17;
#[cfg(feature = "sonic")]
pub mod sonic;

mod group;
mod source;
mod multiexp;

#[cfg(test)]
mod tests;

/// Return chunk size for each thread given the total size to process.
pub fn get_chunk_size(total: usize) -> usize {
    if total == 0 {
        1
    } else {
        (total - 1) / num_cpus::get() + 1
    }
}

cfg_if! {
    if #[cfg(feature = "multicore")] {
        #[cfg(feature = "wasm")]
        compile_error!("Multicore feature is not yet compatible with wasm target arch");

        pub mod multicore;
        mod worker {
            pub use crate::multicore::*;
        }
    } else {
        pub mod singlecore;
        mod worker {
            pub use crate::singlecore::*;
        }
    }
}

mod cs;
pub use self::cs::*;

use std::str::FromStr;
use std::env;

fn verbose_flag() -> bool {
    option_env!("BELLMAN_VERBOSE").unwrap_or("0") == "1"
}
