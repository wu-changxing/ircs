// src/concurrency.rs
use iris_lib::connect::{ConnectionError, ConnectionRead, ConnectionWrite};
use iris_lib::handlers::UserManager;
use tokio::spawn;

pub fn handle_client_connection(mut conn_read: ConnectionRead, mut conn_write: ConnectionWrite) {
    let mut user_manager = UserManager::new();
    spawn(async move {
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

            println!("Received message: {}", message);

            match user_manager.handle_command(UnparsedMessage(message.clone())) {
                Ok(reply) => match reply {
                    Reply::Welcome(welcome) => {
                        let _ = conn_write.write_message(&welcome.message).await;
                        println!("Sent welcome message: {}", welcome.message);
                    }
                },
                Err(err) => match err {
                    ErrorType::UnknownCommand => {
                        let _ = conn_write.write_message("Error: Unknown command\r\n").await;
                        println!("Error: Unknown command");
                    }
                    ErrorType::NickCollision => {
                        let _ = conn_write
                            .write_message("Error: Nickname already in use\r\n")
                            .await;
                        println!("Error: Nickname already in use");
                    }
                    ErrorType::NoNickNameGiven => {
                        let _ = conn_write
                            .write_message("Error: No nickname given\r\n")
                            .await;
                        println!("Error: No nickname given");
                    }
                },
            }
        }
    });
}
