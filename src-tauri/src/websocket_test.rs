#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::net::TcpListener;
    use std::thread::{sleep, spawn};
    use std::time::Duration;
    use tungstenite::{accept, connect, Message, Utf8Bytes};

    #[tokio::test]
    async fn test_websocket_server() {
        let server = TcpListener::bind("127.0.0.1:9001").unwrap();
        for stream in server.incoming() {
            spawn(move || {
                let mut websocket = accept(stream.unwrap()).unwrap();
                loop {
                    let msg = websocket.read().unwrap();

                    // We do not want to send back ping/pong messages.
                    if msg.is_binary() || msg.is_text() {
                        websocket.send(msg).unwrap();
                    }
                }
            });
        }
    }

    #[tokio::test]
    async fn test_message_handling() {
        let url = "ws://localhost:9001"; // WebSocket 服务端地址

        // 建立 WebSocket 连接
        let (mut socket, _response) = match connect(url) {
            Ok(conn) => conn,
            Err(e) => return,
        };

        // 发送消息到 WebSocket 服务端
        let message = Message::Text(Utf8Bytes::from("Hello, Server!".to_string()));
        if let Err(e) = socket.send(message) {
            println!("发送消息失败: {}", e);
            std::io::stdout().flush().unwrap(); // Force flush
            return;
        }

        let handler = spawn(move || {
            loop {
                // 接收来自服务端的消息
                match socket.read() {
                    Ok(msg) => {
                        if let Message::Text(text) = msg {
                            println!("接收到消息: {}", text);
                        } else {
                            println!("接收到非文本消息");
                        }
                        std::io::stdout().flush().unwrap(); // Force flush
                    }
                    Err(e) => {
                        println!("接收消息失败: {}", e);
                        std::io::stdout().flush().unwrap(); // Force flush
                    }
                }

                let message = Message::Text(Utf8Bytes::from("Hello, Server!".to_string()));
                if let Err(e) = socket.send(message) {
                    println!("发送消息失败: {}", e);
                    std::io::stdout().flush().unwrap(); // Force flush
                    return;
                }

                // Sleep for 5 seconds before sending the next message
                sleep(Duration::from_secs(5));
            }
        });

        println!("等待消息处理线程执行");
        std::io::stdout().flush().unwrap(); // Force flush

        // 等待线程执行
        handler.join().unwrap();
    }
}
