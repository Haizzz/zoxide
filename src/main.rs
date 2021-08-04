extern crate clap;
use clap::{App, Arg};
use std::fs;
use zoxide::run;

fn main() {
    let matches = App::new("zoxide")
        .arg(
            Arg::with_name("decompress")
                .short("d")
                .long("decompress")
                .help("decompress a file"),
        )
        .get_matches();
    run(matches);

    let file_content = fs::read("README.md.zst").expect("failed to read file");
    println!("{}", file_content.len());
    println!("{:02x}", file_content[0]);
    println!("{:02x}", file_content[1]);
}
