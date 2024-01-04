use std::env;
use futures_util::SinkExt;
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    // 从命令行参数获取监听地址和端口
    let args: Vec<String> = env::args().collect();
    let address = args.get(1).map(|a| a.as_str()).unwrap_or("0.0.0.0");
    let port = args.get(2).map(|p| p.as_str()).unwrap_or("11451");

    let addr = format!("{}:{}", address, port);
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("WebSocket Ping Pong Paddle Running In：ws://{}", addr);

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            if let Err(err) = handle_connection(stream).await {
                eprintln!("Handle connect error：{:?}", err);
            }
        });
    }
}

async fn handle_connection(stream: tokio::net::TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut websocket = accept_async(stream).await?;

    while let Some(Ok(message)) = websocket.next().await {
        match message {
            Message::Ping(_) => {
                websocket.send(Message::Pong(vec![])).await?;
            }
            _ => {
                // 对于非标准Ping信息，原样返回信息
                websocket.send(message.clone()).await?;
            }
        }
    }

    Ok(())
}
