use std::path::PathBuf;
use std::process::Command;

use clap::clap_app;
use cbindgen;

fn main() {
    let matches = clap_app!(unreal_rust_compile =>
        (version: "0.1")
        (author: "Elliott Mahler <jointogethe.r@gmail.com>")
        (about: "Runs cargo and cbindgen on a crate. Intended for use with Unreal Engine's build system.")
        (@arg CRATE_DIR: --crate_dir +required +takes_value "Input crate directory")
        (@arg OUTPUT_HEADER_FILE: --output_header_file +required +takes_value "Destination filename for the generated header")
        (@arg CARGO_ARGS: +multiple +last +allow_hyphen_values "Arguments to cargo. Cargo will be run with \"crate_dir\" as the working directory.")
    ).get_matches();

    // pull arguments from the argument parser
    let crate_dir : PathBuf = matches.value_of("CRATE_DIR").expect("crate_dir not provided").into();
    let output_filename : PathBuf = matches.value_of("OUTPUT_HEADER_FILE").expect("output_file not provided").into();
    let cargo_args = matches.values_of("CARGO_ARGS").expect("No cargo args provided");	

	// Build the cargo command from the args
	let compile_result = Command::new("cargo").args(cargo_args).current_dir(&crate_dir).status();

	// If the cargo command completed with errors, return a nonzero status code
	let command_success = match compile_result {
		Ok(status) => status.success(),
		Err(_) => false,
	};
	if !command_success {
		std::process::exit(1);
	}

	// generate the header file data and write it into a vec of bytes
	let generated = cbindgen::generate(&crate_dir).expect("Couldn't generate headers.");
	generated.write_to_file(&output_filename);
}
