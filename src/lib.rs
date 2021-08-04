extern crate clap;
use clap::ArgMatches;

pub fn run(args: ArgMatches) -> () {
    let decompress_mode = args.is_present("decompress");

    println!("{}", decompress_mode);

    if decompress_mode {
        decompress()
    } else {
        compress()
    }
}

fn compress() {}

fn decompress() {}
