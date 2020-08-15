use std::net::{TcpListener,TcpStream};//调用标准库
use std::io::{Read,Write};//调用IO读写库

fn main() {
    let listener = TcpListener::bind("0.0.0.0:9999").unwrap();//创建监听器

    for stream in listener.incoming() {//处理客户端输入
        match stream {//模式匹配
            Ok(s) => {
                handle_connection(s);//处理输入
            }
            Err(e) => panic!("encountered IO error: {}", e),//错误处理
        }
    }
}

//获取数据后响应
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];//创建数组存储输入的数据

    stream.read(&mut buffer).unwrap();//输入数据存储在数组中

    stream.write(&buffer).unwrap();//设置响应
    println!("接收数据:{}",String::from_utf8_lossy(&buffer));//打印客户端输入
    stream.flush().unwrap();//清空数据流的缓存
}
