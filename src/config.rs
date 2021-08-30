/// Global configuration that lives as long as the application
pub static GLOBAL_CONFIG: GlobalConfig = GlobalConfig {
    network: "http://localhost:7545",
    encrypted_name: "encrypted",
    key_name: "key"
};

/// Global configuration struct
pub struct GlobalConfig {
    pub network: &'static str,
    pub encrypted_name: &'static str,
    pub key_name: &'static str
}