/* use atty::Stream;
use std::io::BufRead; */
use clap::Parser;
use std::process;
use steganographyrs::options::{extract_options, CliData};
use steganographyrs::steganography;

/// CLI access to the steganography library
fn main() {
    /*     let piped_message: String;
    if atty::isnt(Stream::Stdin) {
        let mut input = String::new();
        let all_lines = io::stdin().lock().lines();
        println!("{:?}", all_lines);
    } */

    let args = CliData::parse();
    let options = extract_options(args);
    // let image_input: Box<dyn io::Read>;
    // // https://phrohdoh.com/blog/read-from-file-or-stdin-rust/
    // if atty::is(Stream::Stdin) {
    //     image_input = Box::new(fs::File::open(args.input_image_path.unwrap()).unwrap());
    // } else {
    //     image_input = Box::new(io::stdin());
    // }

    match options {
        Ok(steganography_option) => steganography(steganography_option),
        Err(error) => panic!("{:?}", error),
    };

    process::exit(0);
}
