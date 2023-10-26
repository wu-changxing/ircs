use iris_lib::{
    connect::{ConnectionError, ConnectionManager, ConnectionRead, ConnectionWrite},
    ircs::{client::Client, IrcCommand, IrcMessage, IrcServer, WriteMessage},
};
use std::sync::mpsc::Receiver;
use tokio::task::JoinHandle;

use std::sync::Arc;

use tokio::sync::Mutex;

async fn handle_client(
    mut conn_read: ConnectionRead,
    conn_write: ConnectionWrite,
    irc_server: Arc<Mutex<IrcServer>>,
) {
    println!("New connection from {}", conn_read.id());

    let mut client = Client {
        nick: String::new(),
        realname: None,
        channels: Vec::new(),
    };
    let mut registered = false;

    let conn_write = Arc::new(Mutex::new(conn_write));

    loop {
        println!("Waiting for message...");
        let message = match conn_read.read_message().await {
            Ok(message) => message,
            Err(ConnectionError::ConnectionLost | ConnectionError::ConnectionClosed) => {
                println!("Lost connection.");
                break;
            }
            Err(_) => {
                println!("Invalid message received... ignoring message.");
                continue;
            }
        };
        println!(
            "[{}] Received message from {}: {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            conn_read.id(),
            message
        );
        let irc_message = IrcMessage::parse(&message);
        if let Some(irc_message) = irc_message {
            let mut conn_write = Arc::clone(&conn_write);
            match irc_message.command {
                IrcCommand::NICK => {
                    if !registered {
                        let mut irc_server = irc_server.lock().await;
                        irc_server
                            .handle_nick_command(irc_message, &mut conn_write, &mut client)
                            .await;
                    }
                }

                IrcCommand::USER => {
                    if !registered && !client.nick.is_empty() {
                        let mut irc_server = irc_server.lock().await;
                        irc_server
                            .handle_user_command(irc_message, &mut conn_write, &mut client)
                            .await;
                        if !client.nick.is_empty() && client.realname.is_some() {
                            registered = true;
                            irc_server.welcome_client(&client, &mut conn_write).await;
                            irc_server.add_connection(client.nick.clone(), conn_write);
                        }
                    }
                }
                IrcCommand::PING => {
                    let irc_server = irc_server.lock().await;
                    irc_server
                        .handle_ping_command(irc_message, &mut conn_write)
                        .await;
                }
                // handle privmsg
                IrcCommand::PRIVMSG => {
                    if registered {
                        let mut irc_server = irc_server.lock().await;
                        let from_nick = client.nick.clone();
                        irc_server
                            .handle_privmsg_command(irc_message, &mut conn_write, &from_nick)
                            .await;
                        print!("PRIVMSG is be handled well: ");
                        drop(irc_server);
                    }
                }
                IrcCommand::QUIT => {
                    let mut irc_server = irc_server.lock().await;
                    irc_server
                        .handle_quit_command(irc_message, &mut conn_write)
                        .await;
                    break;
                }
                IrcCommand::JOIN => {
                    if registered {
                        let mut irc_server = irc_server.lock().await;
                        irc_server
                            .handle_join_command(irc_message, &mut conn_write, &mut client)
                            .await;
                    }
                }
                IrcCommand::PART => {
                    if registered {
                        let mut irc_server = irc_server.lock().await;
                        irc_server
                            .handle_part_command(irc_message, &mut conn_write, &mut client)
                            .await;
                    }
                }
                _ => {
                    println!("Unhandled command");
                }
            }
        } else {
            println!("Invalid IRC message");
        }
    }
}

pub async fn handle_client_loop(
    connection_manager: Arc<Mutex<ConnectionManager>>,
    irc_server: Arc<Mutex<IrcServer>>,
    rx: &Receiver<String>,
) {
    let mut tasks = Vec::<JoinHandle<()>>::new();

    loop {
        let (conn_read, conn_write) = connection_manager
            .lock()
            .await
            .accept_new_connection()
            .await;
        println!(
            "[{}] New connection accepted: {:?}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            conn_write.socket_addr()
        );

        let server_clone = irc_server.clone();
        let task = tokio::spawn(async move {
            handle_client(conn_read, conn_write, server_clone).await;
        });

        tasks.push(task);

        // Periodically clean up finished tasks
        tasks.retain(|t| !t.is_finished());

        // Check for any incoming messages from other parts of the system
        match rx.try_recv() {
            Ok(message) => {
                println!(
                    "Received message from other parts of the system: {}",
                    message
                );
                // Handle the message as needed
            }
            Err(_) => {
                // No messages received, continue listening for new clients
            }
        }
    }
}
