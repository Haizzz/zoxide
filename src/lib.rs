extern crate clap;
use clap::ArgMatches;

pub fn run(args: ArgMatches) -> () {
    let decompress_mode = match args.value_of("decompress") {
        Some(_) => true,
        None => false,
    };

    if decompress_mode {
        decompress()
    } else {
        compress()
    }
}

fn compress() {}

fn decompress() {}
