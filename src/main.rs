use std::net::TcpListener;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1300")?;
    println!("listener bound to thing");
    println!("now listening!");

    loop {
        let (mut tcp_stream, _)  = listener.accept().unwrap();
        let mut buf: [u8; 3] = [0u8; 3];
        
        while tcp_stream.read(&mut buf).unwrap() != 0 {
            // println!("{:?}", buf);
            println!("{:?}", buf);
        }

    }
}
