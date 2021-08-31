use mypub_encryptor::cipher::encrypt;
use mypub_encryptor::generator::generate_decryptor;
use mypub_encryptor::init_cli;

/// Application entry
fn main() {
    // Get Config from the CLI
    let config = init_cli();

    // Do encryption
    let key = match encrypt(&config) {
        Ok(k) => k,
        Err(e) => {
            eprintln!("{}", e);
            "".to_string()
        }
    };

    if key == "" {
        eprintln!("Failed to encrypt the input file");
        return;
    }

    // Generate decryptor
    match generate_decryptor(key, &config) {
        Ok(_) => println!("Encrypted successfully!"),
        Err(e) => eprintln!("{}", e),
    }
}
