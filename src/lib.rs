extern crate clap;
use clap::ArgMatches;
use std::fs;

pub fn run(args: ArgMatches) -> () {
    let decompress_mode = args.is_present("decompress");
    // clap handles required arg so unwrap here is safe
    let file_path = args.value_of("file").unwrap();

    println!("{}", decompress_mode);
    println!("{}", file_path);

    if decompress_mode {
        decompress()
    } else {
        compress()
    }

    let file_content = fs::read("README.md.zst").expect("failed to read file");
    println!("{}", file_content.len());
    println!("{:02x}", file_content[0]);
    println!("{:02x}", file_content[1]);
}

fn compress() {}

fn decompress() {}
