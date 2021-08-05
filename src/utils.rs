use std::process;

pub fn exit_with_message(msg: &str) -> ! {
    // print a message to stderr and exit
    eprint!("{}", msg);
    process::exit(1);
}
