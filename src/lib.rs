use clap::{App, Arg};

mod config;
pub mod cipher;
pub mod utils;

/// Config struct that holds encryption information
#[derive(Debug)]
pub struct Config {
    pub file_path: String,
    pub private_key: String,
    pub output_dir: String,
}

/// Initialize CLI
pub fn init_cli() -> Config {
    let matches = App::new("MyPub Encryptor")
        .version("0.1.0")
        .author("Yepeng Ding <yepengding@g.ecc.u-tokyo.ac.jp>")
        .about("Encryption module of MyPub Cipher Suite")
        .arg(Arg::new("FILE")
            .about("Sets an input file path")
            .required(true)
            .index(1))
        .arg(Arg::new("private_key")
            .short('k')
            .long("key")
            .value_name("PRIVATE KEY")
            .about("Sets your private key")
            .takes_value(true)
            .required(true))
        .arg(Arg::new("output_dir")
            .short('o')
            .long("output")
            .value_name("OUTPUT DIR")
            .about("Sets your output directory")
            .takes_value(true))
        .get_matches();

    let mut config = Config {
        file_path: String::from(""),
        private_key: String::from(""),
        output_dir: String::from("./"),
    };

    if let Some(file_path) = matches.value_of("FILE") {
        config.file_path = String::from(file_path);
    }

    if let Some(private_key) = matches.value_of("private_key") {
        config.private_key = String::from(private_key);
    }

    config
}