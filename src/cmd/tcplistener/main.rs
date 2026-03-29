mod errors;
mod request;
mod headers;
mod body;
mod response;


use tokio::{
    io::{AsyncReadExt, AsyncRead, AsyncWriteExt},
    sync::mpsc,
    net::{TcpListener, TcpStream}
};
use std::error::Error;
use crate::response::ResponseObject;

use request::{
    request_from_reader,
};

use tracing::{
    info, error
};


fn print_type<T>(_: &T) { 
    info!("{:?}", std::any::type_name::<T>());
}

async fn get_lines_channel<R>(mut file: R, tx: mpsc::Sender<String>) -> Result<(), Box<dyn Error>> 
    where R: AsyncRead + Unpin + Send
{
    let mut chunk = vec![0;8];
    let mut number_of_lines = 0;
    let mut curr_line_buffer = Vec::new();
    loop {
        let len = file.read(&mut chunk).await?;
        if len == 0 {
            // End of file
            break;
        }

        for &b in &chunk[..len] {
            if b == b'\n' {
                let curr_string = String::from_utf8_lossy(&curr_line_buffer).to_string();
                // println!("{:?}", curr_string);
                if let Err(_) = tx.send(curr_string).await {
                    info!("Receiver dropped!");
                    return Ok(());
                }
                number_of_lines += 1;
                curr_line_buffer.clear();
            } else {
                curr_line_buffer.push(b);
            }
        }
    }

    info!("The file has {} number of lines", number_of_lines);
    
    Ok(())
} 

async fn handle_connection(mut socket:TcpStream) -> Result<(), Box<dyn Error>> 
{
    let (mut reader, mut writer) = socket.split();
    let response = request_from_reader(&mut reader).await?;

    // let body = serde_json::to_vec(&response.request_line).unwrap();
    let body = "The request was all good".as_bytes();
    // info!("Response: {:?}", body);
    // let total_length =  body.len();

    // info!("total_length = {}", total_length);
    // let header = format!("HTTP/1.1 200 OK\r\nContent-Type: application/text\r\nContent-Length: {}\r\n\r\n", total_length);

    let response = ResponseObject::new();
    let header = response.get_reply_status(200, body);
    
    writer.write_all(header.as_bytes()).await?;
    writer.write_all(&body).await?;
    writer.flush().await?;

    return Ok(());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
                .with_max_level(tracing::Level::DEBUG) // Explicitly set level
                .init();
    // let mut file = File::open("messages.txt").await?;
    // print_type(&file);

    let listener = TcpListener::bind("127.0.0.1:42062").await?;
    let (socket, _) = listener.accept().await?;

    let (_tx, mut rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel(100);

    // print_type(&rx);
    // tokio::spawn(async move {
    //     let _ = get_lines_channel(socket, tx).await;
    // });

    tokio::spawn(async move {
        if let Err(e) = handle_connection(socket).await {
            error!("Error: {:?}", e);
        }
    });

    while let Some(line) = rx.recv().await {
        info!("current line : {}", line);
    }
    
    Ok(())
}
