use std::{env, fs};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::process::Command;

use crate::Config;
use crate::config::GLOBAL_CONFIG;

/// Generate and output decryptor
/// # Arguments
/// * `key` - Generated passphrase
/// * `config` - A config struct reference
pub fn generate_decryptor(key: String, config: &Config) -> Result<(), Box<dyn Error>> {
    match env::consts::OS {
        "windows" => {
            let current_dir = env::current_dir()?;
            let current_dir = current_dir.display().to_string();

            // Fetch MyPubDecryptor from git repository
            println!("Fetching MyPubDecryptor");
            Command::new("git").arg("clone").arg(GLOBAL_CONFIG.decryptor).output()?;

            println!("Overwriting descryptor config file");
            // Overwrite config.rs
            let decryptor_config = generate_decryptor_config_file(DecryptorConfig {
                network: GLOBAL_CONFIG.network.to_string(),
                secret: key,
                decrypted_name: config.filename.to_string(),
            }).unwrap();

            // Write decryptor config file to disk
            let mut decryptor_config_file = File::create("./MyPubDecryptor/src/config.rs")?;
            decryptor_config_file.write_all(decryptor_config.as_bytes())?;
            decryptor_config_file.flush()?;

            println!("Building decryptor");
            // Build MyPubDecryptor
            Command::new("cargo").arg("build").arg("--release").arg(format!("--manifest-path={}\\MyPubDecryptor\\Cargo.toml", current_dir)).output()?;

            // Copy executable to output directory
            fs::copy("./MyPubDecryptor/target/release/mypub-decryptor.exe", format!("{}{}.dec.exe", &config.output_dir, &config.filename))?;

            // Clean
            fs::remove_dir_all("./MyPubDecryptor")?;
        }
        _ => {
            panic!("Current OS is not supported!");
        }
    }
    Ok(())
}

struct DecryptorConfig {
    network: String,
    secret: String,
    decrypted_name: String,
}

fn generate_decryptor_config_file(config: DecryptorConfig) -> Result<String, Box<dyn Error>> {
    let config = format!("pub static GLOBAL_CONFIG: GlobalConfig = GlobalConfig {{\
        network: \"{}\", \
        contract_address: \"{}\", \
        secret: \"{}\", \
        decrypted_name: \"{}\"\
    }};", config.network, GLOBAL_CONFIG.contract_address, config.secret, config.decrypted_name);

    let config_struct = "pub struct GlobalConfig {
        pub network: &'static str,
        pub contract_address: &'static str,
        pub secret: &'static str,
        pub decrypted_name: &'static str,
    }".to_string();

    Ok(format!("{}\r\n{}", config, config_struct))
}

#[cfg(test)]
mod tests {
    use crate::Config;
    use crate::generator::generate_decryptor;

    #[test]
    fn test_generator() {
        let key = "key".to_string();
        let output_dir = "./".to_string();
        let config = Config {
            file_path: "".to_string(),
            filename: "test_file".to_string(),
            private_key: "".to_string(),
            output_dir: "./".to_string(),
        };
        generate_decryptor(key, &config);
    }
}