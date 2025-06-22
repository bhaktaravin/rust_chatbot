// Import the server and client modules
mod server;
mod client;

use std::io;

fn main() {
    println!("Welcome to the chat app!");
    println!("1. Start Server");
    println!("2. Start Client");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice: u32 = choice.trim().parse().unwrap();

    match choice {
        1 => server::start_server(),  // Call the start_server function from the server module
        2 => client::start_client(),  // Call the start_client function from the client module
        _ => println!("Invalid choice. Please choose 1 for Server or 2 for Client."),
    }
}
