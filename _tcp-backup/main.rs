mod constants;
mod handle_connection;

use constants::{HOST,PORT};
use handle_connection::handle_connection;
use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 网络监听逻辑
    let address = format!("{}:{}", HOST, PORT);
    // 使用 Arc<Mutex<...>> 来包装我们的 HashMap，使其能被多个线程安全地访问
    let storage = Arc::new(Mutex::new(HashMap::<String, String>::new()));
    let listener = TcpListener::bind(&address).expect("TcpListener.bind 失败");
    println!("Server listening on address {}", &address);

    // 接收进入的连接
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Ok(addr) = stream.peer_addr() {
                    println!("New connection from: {}", addr);
                } else {
                    println!("New connection from an unknown address.");
                }
                // 为每个连接创建一个新的 Arc 克隆
                let storage_clone = Arc::clone(&storage); // 通过 Mutex 修改
                // 创建新线程处理连接
                thread::spawn(move || handle_connection(stream, storage_clone));
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
