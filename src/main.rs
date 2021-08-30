use mypub_encryptor::cipher::encrypt;
use mypub_encryptor::init_cli;

/// Application entry
fn main() {
    // Get Config from the CLI
    let config = init_cli();

    // Do encryption
    match encrypt(&config) {
        Ok(passphrase) => println!("Encrypted successfully! \n\
        Please securely store your key ({}).", passphrase),
        Err(e) => println!("{}", e),
    };
}
