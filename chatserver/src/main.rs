use std::sync::{Arc, Mutex};
use std::thread::Thread;
use std::sync::mpsc::channel;
use std::old_io::{TcpListener, TcpStream};
use std::old_io::{Acceptor, Listener};

fn main() {
	println!("Serer Start!");

	let mut q = Arc::new(Mutex::new(Vec::new()));

	let listener = TcpListener::bind("127.0.0.1:80").unwrap();
	let mut acceptor = listener.listen().unwrap();

	for stream in acceptor.incoming() {
		match stream {
			Err(e) => { panic!("Connection Failed"); }
			Ok(stream) => {
				Thread::spawn(move|| {
					// connection succeeded
					handle_client(stream) // function
				});
			}
		}
	}

	drop(acceptor);
}
