use clap::Parser;
use std::net::IpAddr;

#[derive(Parser)]
pub struct Arguments {
    #[clap(default_value = "127.0.0.1")]
    pub ip_address: IpAddr,

    #[clap(default_value = "6991")]
    pub port: u16,
}
