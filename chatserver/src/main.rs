use std::sync::{Arc, Mutex};
use std::thread::Thread;
use std::sync::mpsc::channel;
use std::old_io::{TcpListener, TcpStream};
use std::old_io::{Acceptor, Listener};
use core::num::cast;

extern crate core;

fn main() {
	println!("Serer Start!");

	let mut q = Arc::new(Mutex::new(Vec::new()));

	let listener = TcpListener::bind("127.0.0.1:2345").unwrap();
	let mut acceptor = listener.listen().unwrap();

	for stream in acceptor.incoming() {
		let strq = match stream {
			Err(e) => { panic!("Connection Failed"); }
			Ok(stream) => {
				Thread::spawn(move|| {
					let mut stream = stream.clone();
					println!("connected");
					stream.write(String::from_str("Connected\n").as_bytes());
					let mut datas = vec!();
					loop {
						let mut buf = [0];
						let result = stream.read(&mut buf).unwrap();
						let mut datas = &mut datas;
						if buf[0] == 13 || buf[0] == 10 {
							println!("{}", String::from_utf8(datas.clone()).unwrap());
							datas.clear();
						}
						datas.push(buf[0]);
					}
				});
			}
		};
		q.lock().unwrap().push(strq);
	}

	drop(acceptor);
}
