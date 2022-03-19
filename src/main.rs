use clap::{IntoApp, Parser};
use serde::{Deserialize, Serialize};
use ss_uri::SSConfig;

#[derive(Serialize, Deserialize, Debug)]
struct OutputConfig {
    server: String,
    server_port: u16,
    local_port: u16,
    password: String,
    method: String,
}
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    uri: String,
    #[clap(short, long, default_value_t = 1080)]
    port: u16,
}
fn main() {
    let args = Args::parse();

    let config = SSConfig::parse(&args.uri);

    if let Err(e) = config {
        Args::command()
            .error(
                clap::ErrorKind::InvalidValue,
                format!("invalid url : {:?}", e),
            )
            .exit();
    }
    let config = config.unwrap();
    let out = OutputConfig {
        local_port: args.port,
        server: config.host.to_string(),
        server_port: config.port,
        password: config.password,
        method: config.method.to_string(),
    };

    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}
