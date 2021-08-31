/// Global configuration that lives as long as the application
pub static GLOBAL_CONFIG: GlobalConfig = GlobalConfig {
    network: "http://localhost:7545",
    contract_address: "PUBLICATION CONTRACT ADDRESS",
    encrypted_name: "encrypted",
    key_name: "key",
    decryptor_name: "decryptor",
    decryptor: "https://github.com/yepengding/MyPubDecryptor",
};

/// Global configuration struct
pub struct GlobalConfig {
    pub network: &'static str,
    pub contract_address: &'static str,
    pub encrypted_name: &'static str,
    pub key_name: &'static str,
    pub decryptor_name: &'static str,
    pub decryptor: &'static str,
}