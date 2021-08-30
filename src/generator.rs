use std::env;
use std::error::Error;
use std::process::Command;

use crate::Config;
use crate::config::GLOBAL_CONFIG;

pub fn generate_decryptor(key: String, config: &Config) -> Result<(), Box<dyn Error>> {
    match env::consts::OS {
        "windows" => {
            let current_dir = env::current_dir()?;
            let dir = current_dir.display().to_string();
            let output = Command::new("cmd")
                .args(&["/C", config.decryptor_script.as_str()]).output()?;
        }
        _ => {
            panic!("Current OS is not supported!");
        }
    }
    Ok(())
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
            private_key: "".to_string(),
            decryptor_script: "*/script/build_decryptor.bat".to_string(),
            output_dir: "./".to_string(),
        };
        generate_decryptor(key, &config);
    }
}