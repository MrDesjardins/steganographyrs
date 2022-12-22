/* use atty::Stream;
use std::io::BufRead; */
use clap::Parser;
use std::process;
use steganographyrs::steganography_lib::function::{add_message_to_image, get_message_from_image};
use steganographyrs::steganography_lib::options::{extract_options, SteganographyOption, CliData};

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

    // if atty::is(Stream::Stdin) {
    //     image_input = Box::new(fs::File::open(args.input_image_path.unwrap()).unwrap());
    // } else {
    //     image_input = Box::new(io::stdin());
    // }

    match options {
        Ok(SteganographyOption) => match Some(SteganographyOption) {
            Some(SteganographyOption::Encrypt { .. }) => {
                add_message_to_image();
            }
            Some(SteganographyOption::Decrypt { .. }) => {
                get_message_from_image();
            }
            None => {}
        },
        Err(error) => panic!("{:?}", error),
    }

    process::exit(0);
}
