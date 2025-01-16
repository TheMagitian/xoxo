use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::env;
use std::str;

fn encrypt_xoxo(msg: &mut u64, v: &Vec<u64>) -> u64 {
	for i in 0..v.len() {
		*msg = (*msg) ^ v[i]
	}
	*msg
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 512];
    let mut enc_msg_xoxo: &str = "";
    // First, read the encrypted message as a binary string
    match stream.read(&mut buffer) {
        Ok(size) => {
            if let Ok(message) = str::from_utf8(&buffer[..size]) {
				enc_msg_xoxo = message;
                // Convert the received binary string back into an integer
                // let encrypted_message = u64::from_str_radix(message, 2).unwrap_or(0);
                // println!(">>> Encrypted message received: {}", encrypted_message);
                println!(">>> Encrypted message received: {}", message);
            } else {
                eprintln!("Failed to decode encrypted message");
            }
        }
        Err(e) => eprintln!("Failed to read encrypted message from stream: {}", e),

    }

    // Read the primes (serialized u64 integers)
    let mut primes_buffer = [0u8; 2048]; // arbitrary maximum buffer size
    match stream.read(&mut primes_buffer) {
        Ok(size) => {
            if size > 0 {
                let primes: Vec<u64> = primes_buffer[..size]
                    .chunks_exact(8) // each u64 is 8 bytes
                    .map(|chunk| u64::from_be_bytes(chunk.try_into().unwrap()))
                    .collect();
                
                println!(">>> Received primes: {:?}", primes);

				let mut enc_msg_xoxo2 = u64::from_str_radix(enc_msg_xoxo, 2).unwrap();
				let dec_msg_xoxo = encrypt_xoxo(&mut enc_msg_xoxo2, &primes);
				let dec_msg_xoxo_bin = format!("{:b}", dec_msg_xoxo);
				println!(">>> Decryted message: {}", dec_msg_xoxo_bin);
            } else {
                eprintln!("Error while receiving the primes.");
            }
        },
        Err(e) => eprintln!("Error while receiving primes: {}", e),
    }

}

fn main() {
	let arg: Vec<String> = env::args().collect();
	let port = &arg[1];
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Could not bind to address");

    println!("Server is listening on port {port}...");

	/*
    // Accept single incoming connection
	match listener.accept() {
		Ok((stream, _)) => { handle_client(stream) },
		Err(e) => eprintln!("Error while accepting connection: {}", e)
	}
	 */

    // Accept incoming connections in a loop
	/*
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Handle each client connection in a new thread
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => eprintln!("Error accepting connection: {}", e),
        }
}
	 */
	if let Ok((stream, _)) = listener.accept() {
		// Handle the client in a single thread
		handle_client(stream);
	} else {
		eprintln!("Error accepting connection.");
	}

	return;
	
}
