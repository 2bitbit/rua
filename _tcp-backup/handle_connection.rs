use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

pub fn handle_connection(stream: TcpStream, db: Arc<Mutex<HashMap<String, String>>>) {
    // 为写入操作克隆一个新的 stream 句柄：向操作系统内核请求：“请为我指向的这个底层网络连接，再创建一个新的文件描述符（句柄）。”
    let mut stream_writer = stream
        .try_clone()
        .expect("为 writer 进行 clone stream 失败");

    // BufReader 从原始的 stream 句柄进行读取。
    // 它持有对 stream 的不可变借用。
    let mut stream_reader = BufReader::new(&stream);

    // TcpStream 不是一个简单的内存结构，它是一个操作系统资源（套接字 Socket）的句柄 (Handle)。
    // 数据竞争的安全性由操作系统内核来保证
    // 操作系统内核的网络协议栈本身就是被设计为可以被多个线程（甚至多个进程）安全地并发访问的。内核内部有自己的读写缓冲区和锁机制，来管理来自同一个连接的数据流。

    loop {
        let mut line = String::new();
        // reader 使用对 stream 的不可变借用
        match stream_reader.read_line(&mut line) {
            Ok(0) => {
                // 收到0字节，意味着客户端关闭了连接
                println!("Connection closed by client.");
                break;
            }
            Ok(_) => {
                let parts: Vec<&str> = line.trim().split_whitespace().collect();
                if parts.is_empty() {
                    continue;
                }
                let command = parts[0];
                let response = match command {
                    "GET" if parts.len() == 2 => {
                        let key = parts[1];
                        // 锁住互斥锁以读取数据
                        let db_guard = db.lock().unwrap();
                        db_guard
                            .get(key)
                            .cloned()
                            .unwrap_or_else(|| "Key not found".to_string())
                    }
                    "SET" if parts.len() == 3 => {
                        let key = parts[1].to_string();
                        let value = parts[2].to_string();
                        // 锁住互斥锁以写入数据
                        let mut db_guard = db.lock().unwrap();
                        db_guard.insert(key, value);
                        "OK".to_string()
                    }
                    _ => "Invalid command".to_string(),
                };

                // 使用独立的 writer 句柄进行写入，这需要对 writer 的可变借用。
                // 这与 reader 对 stream 的不可变借用不再冲突。
                let stream_write_result = stream_writer.write_all((response + "\n").as_bytes()); // 必须是 bytes (&[u8])，因为网络传输的根本媒介就是字节流。
                if stream_write_result.is_err() {
                    break; // 写入失败，很可能连接已断开，退出循环
                }
            }
            Err(_) => {
                eprintln!("Error reading from connection");
                break;
            }
        }
    }
}
