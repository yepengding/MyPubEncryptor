use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

use secrecy::Secret;
use sha3::{Digest, Sha3_256};

use crate::Config;
use crate::config::GLOBAL_CONFIG;
use crate::utils::get_uuid;

/// Encrypt the input file and output to disk
/// # Arguments
/// * `config` - A config struct reference
pub fn encrypt(config: &Config) -> Result<String, Box<dyn Error>> {
    // Read file
    let mut file = File::open(&config.file_path)?;
    let mut file_bytes = Vec::new();
    let _ = file.read_to_end(&mut file_bytes);

    let plaintext = &file_bytes[..];

    // Generate passphrase (i.e., key)
    let mut hasher = Sha3_256::new();
    let passphrase_components = format!("{}/{}/{}", GLOBAL_CONFIG.network, get_uuid(), &config.private_key);
    hasher.update(passphrase_components.as_bytes());
    let result = hasher.finalize();
    let passphrase = format!("{:x}", result);

    // Calculated encrypted file
    let encrypted = {
        let encryptor = age::Encryptor::with_user_passphrase(Secret::new(passphrase.to_owned()));

        let mut encrypted = vec![];
        let mut writer = encryptor.wrap_output(&mut encrypted)?;
        writer.write_all(plaintext)?;
        writer.finish()?;

        encrypted
    };

    // Write encrypted file to disk
    let mut encrypted_file = File::create(format!("{}{}", &config.output_dir, GLOBAL_CONFIG.encrypted_name))?;
    encrypted_file.write_all(&encrypted)?;
    encrypted_file.flush()?;

    // Write passphrase to disk
    let mut passphrase_file = File::create(format!("{}{}", &config.output_dir, GLOBAL_CONFIG.key_name))?;
    passphrase_file.write_all(passphrase.as_bytes())?;
    encrypted_file.flush()?;

    Ok(passphrase)
}

#[cfg(test)]
mod tests {
    use std::fs::{File, remove_file};
    use std::io::{Read, Write};

    use secrecy::Secret;

    use crate::cipher::encrypt;
    use crate::Config;
    use crate::config::GLOBAL_CONFIG;

    #[test]
    fn encryption_correct() {
        let plaintext = b"Hello, encryption!";
        let passphrase = "passphrase here";

        // Calculate encrypted text
        let encrypted = {
            let encryptor = age::Encryptor::with_user_passphrase(Secret::new(passphrase.to_owned()));

            let mut encrypted = vec![];
            let mut writer = encryptor.wrap_output(&mut encrypted).unwrap();
            writer.write_all(plaintext).unwrap();
            writer.finish().unwrap();

            encrypted
        };

        // Calculate decrypted text
        let decrypted = {
            let decryptor = match age::Decryptor::new(&encrypted[..]).unwrap() {
                age::Decryptor::Passphrase(d) => d,
                _ => unreachable!(),
            };

            let mut decrypted = vec![];
            let mut reader = decryptor.decrypt(&Secret::new(passphrase.to_owned()), None).unwrap();
            let _ = reader.read_to_end(&mut decrypted);

            decrypted
        };

        // Decrypted text is equal to the plaintext
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn file_encryption_correct() {
        // Simulate a configuration
        let config = Config {
            file_path: String::from("./Cargo.toml"),
            private_key: String::from("a private key"),
            decryptor_script: String::from(""),
            output_dir: String::from("./"),
        };

        // Encrypt the input file
        let passphrase = encrypt(&config).unwrap();

        // Read origin file
        let mut origin_file = File::open(&config.file_path).unwrap();
        let mut origin_file_bytes = Vec::new();
        let _ = origin_file.read_to_end(&mut origin_file_bytes);
        let origin = &origin_file_bytes[..];

        // Read encrypted file
        let mut encrypted_file = File::open(format!("{}{}", &config.output_dir, GLOBAL_CONFIG.encrypted_name)).unwrap();
        let mut encrypted_file_bytes = Vec::new();
        let _ = encrypted_file.read_to_end(&mut encrypted_file_bytes);
        let encrypted = &encrypted_file_bytes[..];

        // Calculate decrypted file
        let decrypted = {
            let decryptor = match age::Decryptor::new(&encrypted[..]).unwrap() {
                age::Decryptor::Passphrase(d) => d,
                _ => unreachable!(),
            };

            let mut decrypted = vec![];
            let mut reader = decryptor.decrypt(&Secret::new(passphrase), None).unwrap();
            let _ = reader.read_to_end(&mut decrypted);

            decrypted
        };

        // Decrypted file is equal to the origin
        assert_eq!(decrypted, origin);

        // Clear outputs
        remove_file(format!("{}{}", &config.output_dir, GLOBAL_CONFIG.encrypted_name)).unwrap();
        remove_file(format!("{}{}", &config.output_dir, GLOBAL_CONFIG.key_name)).unwrap();
    }
}