use atty::Stream;
use clap::Parser;
use std::io::{self, BufRead};
use std::process;
use steganographyrs::options::{extract_options, CliData};
use steganographyrs::steganography;

/// CLI access to the steganography library
fn main() {
    let mut message_from_pipe: Option<String> = None;
    // Piping, the message content is coming from the std in instead of the args.message
    if !atty::is(Stream::Stdin) {
        let mut input_message = io::stdin()
            .lock()
            .lines()
            .fold("".to_string(), |acc, line| acc + &line.unwrap() + "\n");
        input_message.truncate(input_message.len() - 1);
        message_from_pipe = Some(input_message);
    }

    let args = CliData::parse();
    let options = extract_options(args, message_from_pipe);

    match options {
        Ok(steganography_option) => match steganography(steganography_option) {
            Some(s) => {
                print!("{}", s);
            }
            None => { /*Nothing to print. Happen if encrypt or if an error occured*/ }
        },
        Err(error) => panic!("{:?}", error),
    };

    process::exit(0);
}
