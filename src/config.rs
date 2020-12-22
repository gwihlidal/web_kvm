#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub api_port: u16,
    pub kvm_address: String,
    pub connect_timeout: u64,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            api_port: 5577,
            kvm_address: "192.168.1.10:5000".into(),
            connect_timeout: 1,
        }
    }
}
