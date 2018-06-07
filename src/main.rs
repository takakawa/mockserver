use std::net::{TcpListener,TcpStream};
use std::thread;
use std::io;
use std::io::{Read,Write};

fn handle_write(mut stream: &TcpStream) -> io::Result<usize>{
	let resp = b"test,test,test";
	match stream.write(resp){
		Ok(o)=>{println!("Sent ok,{}",o);Ok(o)},
		Err(e)=>{println!("Failed to send:{}",e);Err(e)},
	}
}

fn handle_read(mut  stream: &TcpStream) -> io::Result<usize>{
	let mut buf  = [0u8, 4096];
	match stream.read(& mut buf){
		Ok(o)=>{
			let req_str = String::from_utf8_lossy(&buf);
			println!("{}",req_str);
            Ok(o)
		},
		Err(e)=> {
			println!("Unable to read stream: {}",e);
			Err(e)
		},
	}
}
fn handle_client(stream: TcpStream){
	println!("handling tcpstream");
        loop {
			match handle_read(&stream) {
				Ok(o) => {
					handle_write(&stream);
				},
				Err(e) => {
					println!("errr {}",e);
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
