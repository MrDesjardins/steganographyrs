use atty::Stream;
use clap::Parser;
use std::process;
use std::{fs, io};

use steganographyrs::steganography_lib::function::{add_message_to_image, get_message_from_image};
use steganographyrs::steganography_lib::options::SteganographyOption;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    #[arg(short, long)]
    message: Option<String>,

    #[arg(short, long)]
    password: Option<String>,

    #[arg(short, long)]
    input_image_path: Option<String>,

    #[arg(short, long)]
    output_image_path: Option<String>,

    #[arg(short, long)]
    encrypt_mode: Option<bool>,
}

fn main() {
    let args = Cli::parse();
    let options = extract_options(args);
    /*     let image_input: Box<dyn io::Read>;

       if atty::is(Stream::Stdin) {
           image_input = Box::new(fs::File::open(args.input_image_path.unwrap()).unwrap());
       } else {
           image_input = Box::new(io::stdin());
       }
    */
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

fn extract_options(args: Cli) -> Result<SteganographyOption, String> {
    Ok(match args.encrypt_mode {
        Some(i) => match i {
            true => SteganographyOption::Encrypt {
                message: args
                    .message
                    .unwrap_or_else(|| panic!("Message is required")),
                password: args.password.unwrap_or("default".to_string()),
                input_image_path: args
                    .input_image_path
                    .unwrap_or_else(|| panic!("Input image path")),
                output_image_path: args
                    .output_image_path
                    .unwrap_or_else(|| panic!("Output image path is required")),
            },
            false => SteganographyOption::Decrypt {
                password: args.password.unwrap_or("default".to_string()),
                input_image_path: args
                    .input_image_path
                    .unwrap_or_else(|| panic!("Input image is required")),
            },
        },
        None => panic!("Encrypt mode is required"),
    })
}
