use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

pub type Clients = Arc<Mutex<Vec<TcpStream>>>;

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not bind");
    let clients: Clients = Arc::new(Mutex::new(Vec::new()));

    println!("Server listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client: {}", stream.peer_addr().unwrap());
                let client_stream = stream.try_clone().expect("Failed to clone stream");
                clients.lock().unwrap().push(client_stream);

                let clients_clone = Arc::clone(&clients);
                thread::spawn(move || handle_client(stream, clients_clone));
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}


fn handle_client(mut stream: TcpStream, clients: Clients) {
    let mut buffer = [0; 512];
    let mut username = String::new();

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Client disconnected
                let mut clients_guard = clients.lock().unwrap();
                clients_guard.retain(|client| client.peer_addr() != stream.peer_addr().unwrap());
                break;
            }
            Ok(n) => {
                let message = &buffer[..n];
                // Rest of the message handling
            }
            Err(_) => break,
        }
    }
    println!("Client disconnected: {}", username);
}

