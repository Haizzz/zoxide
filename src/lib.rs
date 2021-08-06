extern crate clap;
use clap::ArgMatches;
use std::fs;

use crate::utils::{bit_at_index, exit_with_message};

mod utils;

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

    // parse frame header
    // TODO: add links to the spec for each of these fields
    let frame_header_descriptor = content.remove(0);
    let frame_content_size_flag = frame_header_descriptor >> 6;
    let single_segment_flag = bit_at_index(frame_header_descriptor, 5);
    let content_checksum_flag = bit_at_index(frame_header_descriptor, 2);
    let dictionary_id_flag = frame_header_descriptor & 3;
    let did_field_size = match dictionary_id_flag {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 4,
        // dictionary_id_flag is a 2 bit flag (0, 1, 2, or 3), the case below would signify corruption
        _ => exit_with_message("Corrupted file: dictionary id flag is corrupted"),
    };

    let fcs_field_size: u8;
    if frame_content_size_flag == 0 && single_segment_flag {
        fcs_field_size = 1;
    } else {
        fcs_field_size = 1 << frame_content_size_flag;
    }

    println!("frame content size flag: {:?}", frame_content_size_flag);
    println!("single segment flag: {:?}", single_segment_flag);
    println!("content checksum: {:?}", content_checksum_flag);
    println!("dictionary id flag: {:?}", dictionary_id_flag);
    println!("did field size: {:?}", did_field_size);
    println!("fcs field size: {:?}", fcs_field_size);

    // TODO: check if the file is smaller than fcs field size
    // TODO: check if reserved or unused bit is set

    // TODO: figure out how to make this not mutable
    let window_size: u8;
    if single_segment_flag {
        // no window descriptor
        // window size is frame content size
        // TODO: parse frame content size
        window_size = 1;
    } else {
        let window_descriptor = content.remove(0);
        let window_log = 10 + (window_descriptor >> 3);
        let window_base = 1 << window_log;
        let window_add = (window_base / 8) * (window_descriptor & 0b111);
        window_size = window_base + window_add;
    }

    println!("window size = {:?}", window_size);
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
