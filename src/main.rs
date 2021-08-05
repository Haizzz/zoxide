extern crate clap;
use clap::{App, Arg};
use zoxide::run;

fn main() {
    let matches = App::new("zoxide")
        .arg(
            Arg::with_name("decompress")
                .short("d")
                .long("decompress")
                .takes_value(false)
                .help("decompress a file"),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .index(1)
                .required(true)
                .help("file to operate on"),
        )
        .get_matches();
    run(matches);
}
