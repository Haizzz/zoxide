use std::process;

pub fn exit_with_message(msg: &str) -> ! {
    // print a message to stderr and exit
    eprint!("{}", msg);
    process::exit(1);
}

pub fn bit_at_index(byte: u8, index: u8) -> bool {
    // given a byte, return the bit value at index
    (byte & (1 << index)) != 0
}
