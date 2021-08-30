use mypub_encryptor::cipher::encrypt;
use mypub_encryptor::generator::generate_decryptor;
use mypub_encryptor::init_cli;

/// Application entry
fn main() {
    // Get Config from the CLI
    let config = init_cli();

    // Do encryption
    let key = encrypt(&config).unwrap();
    // match encrypt(&config) {
    //     Ok(passphrase) => println!("Encrypted successfully! \n\
    //     Please securely store your key ({}).", passphrase),
    //     Err(e) => println!("{}", e),
    // };
    match generate_decryptor(key, &config) {
        Ok(_) => println!("Encrypted successfully!"),
        Err(e) => println!("{}", e),
    }
}
