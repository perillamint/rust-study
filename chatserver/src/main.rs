use std::sync::{Arc, Mutex};
use std::thread::Thread;
use std::sync::mpsc::{channel, Sender};
use std::old_io::{TcpListener, TcpStream};
use std::old_io::{Acceptor, Listener};
use core::num::cast;

extern crate core;

fn main() {
	println!("Server Start!");

	// data Sender List
	let mut txlist: Arc<Mutex<Vec<Sender<String>>>> = Arc::new(Mutex::new(Vec::new()));

	let listener = TcpListener::bind("127.0.0.1:2345").unwrap(); //Server
	let mut acceptor = listener.listen().unwrap();

	for stream in acceptor.incoming() {
		let strq = match stream {
			Err(e) => { panic!("Connection Failed. {:?}", e); }
			Ok(stream) => {

				let mut recv_stream = stream.clone();
				let mut send_stream = stream.clone();

				let read_txlist = txlist.clone();
				let mut write_txlist = txlist.clone();
				
				Thread::spawn(move || { // read from stream 
					println!("connected");
					let mut datas = vec!();
					loop {
						let mut buf = [0];
						let result = recv_stream.read(&mut buf).unwrap();
						let mut datas = &mut datas;
						datas.push(buf[0]);
						if buf[0] == 13 || buf[0] == 10 {						
							//print!("{}", String::from_utf8(datas.clone()).unwrap()); //print to stdout
							for mut tx in read_txlist.lock().unwrap().iter_mut() {
								//print!("{}", &data);
								let data = String::from_utf8(datas.clone());
								if let Some(send_data) = data.ok() {
									tx.send(send_data).unwrap();
								}
							}
							datas.clear();
						}
					}
				});
				Thread::spawn(move || { // write to stream
					send_stream.write(String::from_str("Connected\n").as_bytes());
					let (tx, rx) = channel();
					write_txlist.lock().unwrap().push(tx);
					loop {
						let data = rx.recv().unwrap();
						send_stream.write(data.as_bytes());
					}
				});
			}
		};
	}

	drop(acceptor);
}
