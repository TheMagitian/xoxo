use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::env;
use std::str;
use xoxo::colours::*;

fn encrypt_xoxo(msg: &mut u64, v: &Vec<u64>) -> u64 {
	for i in 0..v.len() {
		*msg = (*msg) ^ v[i];
		println!("XOR with {} is: {}", v[i], format!("{:05b}", *msg));
	}
	println!("Plaintext as bin:   {CYAN}{}{RESET}", format!("{:05b}", *msg));
	*msg
}

fn binary_to_text(binary: &str) -> String {
    let padded_length = (binary.len() + 4) / 5 * 5; // round up to nearest multiple of 5
    let padded_binary = format!("{:0>width$}", binary, width = padded_length);
    padded_binary
        .as_bytes()
        .chunks(5)
        .filter_map(|chunk| {
            let chunk_str = str::from_utf8(chunk).ok()?;
            let value = u8::from_str_radix(chunk_str, 2).ok()?;
            if value >= 1 && value <= 26 {
                Some((value + b'a' - 1) as char)
            } else {
                None
            }
        })
        .collect()
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 512];
    let mut enc_msg_xoxo: &str = "";
    // read the encrypted message as a binary string
    match stream.read(&mut buffer) {
        Ok(size) => {
            if let Ok(message) = str::from_utf8(&buffer[..size]) {
				enc_msg_xoxo = message;
                // let encrypted_message = u64::from_str_radix(message, 2).unwrap_or(0);
                println!("{BLUE}>>>{RESET} {RED}{RESET} {ITALIC} Encrypted message received from client: {RESET}{YELLOW}{}{RESET}", message);
            } else {
                eprintln!("{RED}  {RESET} {ITALIC}Failed to decode encrypted message.{RESET}");
            }
        }
        Err(e) => eprintln!("Failed to read encrypted message from stream: {}", e),

    }

    let mut primes_buffer = [0u8; 2048]; // arbitrary maximum buffer size
    match stream.read(&mut primes_buffer) {
        Ok(size) => {
            if size > 0 {
                let primes: Vec<u64> = primes_buffer[..size]
                    .chunks_exact(8) // each u64 is 8 bytes
                    .map(|chunk| u64::from_be_bytes(chunk.try_into().unwrap()))
                    .collect();
                
				println!("Numbers received from client: {BOLD}{UNDERLINE}{:?}{RESET}", primes);
				let mut enc_msg_xoxo2 = u64::from_str_radix(enc_msg_xoxo, 2).unwrap();
				let dec_msg_xoxo = encrypt_xoxo(&mut enc_msg_xoxo2, &primes);
				let dec_msg_xoxo_bin = format!("{:b}", dec_msg_xoxo);
				let dec_msg_xoxo_text = binary_to_text(&dec_msg_xoxo_bin);
				
				println!("{GREEN}<<<{RESET}{ITALIC}{RED} {RESET}{ITALIC}  Decrypted message:{RESET} {VIOLET}{}{RESET}", dec_msg_xoxo_text);
            } else {
                eprintln!("{RED}  {RESET} {ITALIC}Error while receiving the primes. {RESET}");
            }
        },
        Err(_) => eprintln!("{RED}  {RESET} {ITALIC}Error while receiving the primes. {RESET}")
    }

}

fn main() {
	let arg: Vec<String> = env::args().collect();
	let port = &arg[1];
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Could not bind to address");

    println!("{BOLD}Server listening on port{RESET} {HIGHLIGHT}{port}{RESET}\n");

	if let Ok((stream, _)) = listener.accept() {
		handle_client(stream);
	} else {
		eprintln!("Error accepting connection.");
	}

	return;
	
}
