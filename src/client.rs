use std::io::{self, Write, BufRead, BufReader};
use std::net::TcpStream;
use std::thread;

pub fn start_client() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Could not connect to server");

    println!("Connected to the chat server!");
    println!("Please enter your username:");
    
    // Take the username input
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read line");
    let username = username.trim();

    // Send the username to the server (optional)
    stream.write_all(format!("{} has joined the chat\n", username).as_bytes()).expect("Failed to send message");

    let mut stream_clone = stream.try_clone().expect("Failed to clone stream");

    // Thread to read messages from the server
    thread::spawn(move || {
        let reader = BufReader::new(&mut stream_clone);
        for line in reader.lines() {
            match line {
                Ok(msg) => println!("[Chat] {}", msg),
                Err(_) => break,
            }
        }
    });

    // Main thread sends messages
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let msg = line.unwrap();
        if msg.trim() == "/quit" {
            break;
        }
        // Prepend the username to each message
        let message = format!("{}: {}", username, msg);
        if let Err(e) = stream.write_all(format!("{}\n", message).as_bytes()) {
            eprintln!("Failed to send message: {}", e);
            break;
        }
    }

    println!("Disconnected from chat.");
}
