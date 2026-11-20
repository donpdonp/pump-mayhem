use rust_yaml;
use solana_client::transaction_executor;
use std::fs;

fn load_config(filename: &str) -> rust_yaml::value::Value {
    let config_yaml = fs::read_to_string(filename).unwrap();
    let yaml = rust_yaml::Yaml::new();
    return yaml.load_str(&config_yaml).unwrap();
}

fn main() {
    let config = load_config("config.yaml");
    println!("Hello, world");
    let url_str = config.get_str("ws").unwrap().as_str().unwrap();
    let url = url::Url::parse(url_str).unwrap();
    println!("{}", url.path());
    let scks = url.socket_addrs(|| None).unwrap();
    for so in &scks {
        println!("{}", so);
    }
    let _te = transaction_executor::TransactionExecutor::new(scks[0]);
}
