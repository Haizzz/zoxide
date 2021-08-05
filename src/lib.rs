extern crate clap;
use clap::ArgMatches;
use std::fs;

use crate::utils::exit_with_message;

mod utils;

fn read_or_error(path: &str) -> Vec<u8> {
    // given a file path, read the content with error handling
    let file_content = match fs::read(path) {
        Ok(content) => content,
        Err(e) => {
            let err_msg = "Error reading file ".to_owned() + path + ":" + e.to_string().as_str();
            exit_with_message(err_msg.as_str())
        }
    };
    file_content
}

fn compress(path: &str) {}

fn decompress(path: &str) {
    let content = read_or_error(path);
    println!("{}", content.len());
    println!("{:02x}", content[0]);
    println!("{:02x}", content[1]);
}

pub fn run(args: ArgMatches) -> () {
    let decompress_mode = args.is_present("decompress");
    // clap handles required arg so unwrap here is safe
    let file_path = args.value_of("file").unwrap();

    println!("args are: ");
    println!("{}", decompress_mode);
    println!("{}", file_path);
    println!("==========");

    if decompress_mode {
        decompress(file_path)
    } else {
        compress(file_path)
    }
}
