use std::io::{ErrorKind,Read,Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

const MSG_SIZE: usize = 32;
const LOCAL: &str = "127.0.0.1:5000";

fn sleep() {
	thread::sleep(::std::time::Duration::from_millis(100));
}
fn main() {
let server = TcpListener::bind(LOCAL).expect("Failed to bind to listener");
server.set_nonblocking(true).expect("failed to set server to non blocking");

let mut clients = vec![];
let (tx,rx) = mpsc::channel::<String>();

loop {
	if let Ok((mut socket, address)) = server.accept() {
		println!("Client Connected");
		let tx = tx.clone();
		clients.push(socket.try_clone().expect("failed to clone socket"));

		thread::spawn( move || {
				let mut buff = vec![0; MSG_SIZE];
				match socket.read_exact(&mut buff){
					Ok(_) => {
						let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>(); 
						println!("{}: {:?}", address, msg);

						let msg = String::from_utf8(msg).expect("invalid utf8 msg");
						tx.send(msg).expect("failed to snd msg to rx");
					},
					Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
					Err(_) => {
						println!("Closing conn");
						//break;
					}
				}
			sleep();
		});
	}
	if let Ok(msg) = rx.try_recv() {
		clients = clients.into_iter().filter_map( |mut client| {
		let mut buff = msg.clone().into_bytes();
		buff.resize(MSG_SIZE,0);	
		client.write_all(&buff).map(|_| client).ok()
	    }).collect::<Vec<_>>();
	}
	sleep();
  }
} 