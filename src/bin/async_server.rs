use std::error::Error;

use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::io::BufWriter;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

async fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    println!("handling stream {:?}", stream);

    let (reader, writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut writer = BufWriter::new(writer);

    let mut message = String::new();

    loop {
        message.clear();
        if reader.read_line(&mut message).await? == 0 {
            break;
        }
        writer
            .write_all(format!("I read and processed the message {}", message).as_bytes())
            .await?;
        writer.flush().await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::spawn(async move {
            if let Err(e) = handle_client(stream).await {
                eprintln!("{}", e);
            }
        });
    }
}
