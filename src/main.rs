use std::net::{TcpListener,TcpStream,Shutdown};
use std::thread;
use std::io;
use std::io::{Read,Write};

fn handle_write(mut stream: &TcpStream) -> io::Result<usize>{
        let resp = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
	stream.write(resp)
}

fn handle_read(mut  stream: &TcpStream) -> io::Result<usize>{
	let mut buf  = [0u8; 4096];
	match stream.read(& mut buf){
		Ok(o)=>{
			let req_str = String::from_utf8_lossy(&buf);
			println!("{}",req_str);
            		Ok(o)
		},
		Err(e)=> {
			println!("Unable to read stream: {}",e);
			stream.shutdown(Shutdown::Both);
			Err(e)
		},
	}
}
fn handle_client(stream: TcpStream){
	println!("handling tcpstream");
        loop {
			match handle_read(&stream) {
				Ok(o) => {
					match handle_write(&stream){
					   Ok(o)=>{println!("Sent ok,{}",o);},
					   Err(e)=>{println!("Failed to send:{}",e);stream.shutdown(Shutdown::Both);break;},
					}
				},
				Err(e) => {
					println!("errr {}",e);
					break;
				}
           }
	}
}
fn main() {
    let listener = TcpListener::bind("localhost:19890").unwrap();
    println!("Listening for conections on port {}",19890);

    for stream in listener.incoming() {
      match stream {
          Ok(conn) => {
			thread::spawn(||{handle_client(conn)});
		},
	  Err(e) => {
		println!("Unable to connect:{}",e);
          }
      }
    }
}
