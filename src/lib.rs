extern crate clap;

mod core;
mod utils;

use clap::ArgMatches;
use std::fs;

use crate::core::format::BlockType;
use crate::core::header::decode_header;
use crate::utils::exit_with_message;

// 0xFD2FB528 as u8
const MAGIC_NUMBER: [u8; 4] = [40, 181, 47, 253];

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
    let mut content = read_or_error(path);

    // parse the content of the file
    let magic_number: Vec<u8> = content.drain(0..4).collect();
    for i in 0..4 {
        if MAGIC_NUMBER[i] != magic_number[i] {
            exit_with_message("Corrupted file: magic number does not match");
        }
    }

    let frame_header = decode_header(&mut content);
    println!("remaining content = {:?}", content);
    println!("remaining content = {:?}", content.len());

    // TODO: read blocks and parse them
    let block_header_vec: Vec<u8> = content.drain(0..3).collect();
    println!("block header is: {:?}", block_header_vec);
    let block_header = u32::from(block_header_vec[0])
        | u32::from(block_header_vec[1]) << 8
        | u32::from(block_header_vec[2]) << 16;
    let last_block = block_header & 1 != 0;
    let block_type = match (block_header >> 1) & 3 {
        0 => BlockType::Raw,
        1 => BlockType::Rle,
        2 => BlockType::Compressed,
        3 => BlockType::Reserved,
        _ => exit_with_message("Invalid block type"),
    };
    let block_size = block_header >> 3;

    println!("last block: {:?}", last_block);
    println!("block type: {:?}", block_type);
    println!("block size: {:?}", block_size);

    if (!last_block) {
        // recurse here
    }
    match block_type {
        BlockType::Raw => print!("raw block"),
        BlockType::Rle => print!("rle block"),
        BlockType::Compressed => print!("compressed block"),
        BlockType::Reserved => print!("reserved block"),
    }
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
