mod arguments;
mod client_loop;
mod user_input;
use crate::arguments::Arguments;
use crate::client_loop::handle_client_loop;
use crate::user_input::spawn_user_input_thread;
use clap::Parser;
use iris_lib::{connect::ConnectionManager, ircs::IrcServer, types::SERVER_NAME};
use std::net::IpAddr;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let arguments = Arguments::parse();
    println!(
        "Launching {} at {}:{}",
        SERVER_NAME, arguments.ip_address, arguments.port
    );

    let connection_manager = ConnectionManager::launch(arguments.ip_address, arguments.port).await;
    let irc_server = IrcServer::new();

    let shared_connection_manager = Arc::new(Mutex::new(connection_manager));
    let shared_irc_server = Arc::new(Mutex::new(irc_server));

    let (tx, rx) = mpsc::channel();

    // Spawn the user input thread
    let _ = spawn_user_input_thread(tx);

    handle_client_loop(shared_connection_manager, shared_irc_server, &rx).await;
}
