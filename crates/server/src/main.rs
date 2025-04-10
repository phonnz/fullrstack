use tokio::net::{TcpListener, TcpStream};
use futures::{StreamExt, SinkExt};
use tokio_tungstenite::accept_async;
use tracing::{info, error};

mod db;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Get database URL from environment or use default
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://fullrstack:fullrstack@localhost:5432/fullrstack".to_string());

    if let Some(arg) = std::env::args().nth(1) {
        if arg == "migrate" {
            println!("Running database migrations...");
            db::run_migrations(&database_url).await?;
            println!("Migrations completed successfully!");
            return Ok(());
        }
    }

    // Create the TCP listener
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    info!("WebSocket server listening on ws://{}", addr);

    // Accept connections
    while let Ok((stream, addr)) = listener.accept().await {
        info!("New connection from: {}", addr);
        tokio::spawn(handle_connection(stream));
    }

    Ok(())
}

async fn handle_connection(stream: TcpStream) {
    match accept_async(stream).await {
        Ok(ws_stream) => {
            let (mut write, mut read) = ws_stream.split();
            
            // Echo incoming messages (for testing)
            while let Some(message) = read.next().await {
                match message {
                    Ok(msg) => {
                        if let Err(e) = write.send(msg).await {
                            error!("Error sending message: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        error!("Error receiving message: {}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            error!("Error during WebSocket handshake: {}", e);
        }
    }
} 