#[derive(Clone)]
pub struct CliOptions<'a> {
    pub password: Option<&'a str>,
    pub input_image_path: Option<&'a str>,
    pub output_image_path: Option<&'a str>,
    pub encrypt_mode: Option<bool>,
}


#[derive(Clone)]
pub enum SteganographyOption {
    Encrypt {
        password: String,
        input_image_path: String,
        output_image_path: String,
    },
    Decrypt {
        password: String,
        input_image_path: String,
    }
}

