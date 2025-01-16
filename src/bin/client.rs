// Constants
const KEY: u64 = 987654;

// Imports
use std::io::Write;
use std::net::TcpStream;
use std::env;
mod colours;
use colours::*;

// Functions
fn fill_prime(prime: &mut Vec<u64>, high: u64) {
    let mut ck = vec![true; (high + 1) as usize];
    
    let mut i: u64 = 2;
    while i * i <= high {
        if ck[i as usize] {
            let mut j = i * i;
            while j <= high {
                ck[j as usize] = false;
                j += i;
            }
        }
        i += 1;
    }

    let mut i: u64 = 2;
    while i <= high {
        if ck[i as usize] {
            prime.push(i);
        }
        i += 1;
    }
}

fn segmented_sieve(mut low: u64, high: u64) -> Vec<u64> {
    if low < 2 { low = 2; }

    let range_size = high - low + 1;
    let mut prime = vec![true; range_size as usize];
    let mut chprime = vec![];
    let mut tmp: Vec<u64> = vec![];
    
    // Generate primes up to sqrt(high)
    fill_prime(&mut chprime, (high as f64).sqrt() as u64);


    for p in chprime {
        let mut start = if low % p == 0 { low } else { low + (p - (low % p)) };

        if start == p { start = p * p; }

        let mut j = start;
        while j <= high {
            prime[(j - low) as usize] = false;
            j += p;
        }
    }

    for i in low..=high {
        if prime[(i - low) as usize] { tmp.push(i); }
    }

    tmp
}

fn get_final_list(v: &Vec<u64>) -> Vec<u64> {
	let mut new_primes = vec![];
	for i in (0..v.len()).step_by(89) {
		new_primes.push(v[i]);
	}
	new_primes
}

fn lcs(x: u64) -> u64 {
	let str_x = x.to_string();
	let len = str_x.len();
	let last_digit = str_x.chars().last().unwrap();
	let rest_of_num = &str_x[..len-1];
	let new_num = format!("{}{}", last_digit, rest_of_num);
	new_num.parse::<u64>().unwrap()
}

fn lcs_vector(v: &mut Vec<u64>) -> &mut Vec<u64> {
	for i in 0..v.len() {
		if i%2==0 {
			v[i] = lcs(v[i])
		}
	}
	v
}

fn encrypt_xoxo(msg: &mut u64, v: &Vec<u64>) -> u64 {
	for i in 0..v.len() {
		*msg = (*msg) ^ v[i]
	}
	*msg
}

fn main() {
	let arg: Vec<String> = env::args().collect();
	let port = &arg[1];
	let ptext = &arg[2];
	
	let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).expect("Unable to connect to the server.");

	println!("{BOLD}Client attempting to connect to port {RESET}{HIGHLIGHT}{}{RESET}\n", {port});
	println!("{GREEN}>>>{RESET}{RED}  {RESET} {ITALIC}Message to be encrypted: {RESET}{VIOLET}{}{RESET}", ptext);

	let primes = segmented_sieve(KEY, (105*KEY) as u64/100);
	let mut primes = get_final_list(&primes);
	let lcs_primes: &mut Vec<u64> = lcs_vector(&mut primes);
	// let rcs_primes: &Vec<u64> = rcs_vector(&mut *lcs_primes);
	
	let mut int_ptext = u64::from_str_radix(ptext, 2).unwrap();
	let enc_msg_xoxo = encrypt_xoxo(&mut int_ptext, lcs_primes);
	let enc_msg_xoxo_bin = format!("{:b}", enc_msg_xoxo);
	
	stream.write(enc_msg_xoxo_bin.as_bytes()).expect("Failed to send message to server.");
	println!("{BLUE}<<<{RESET} {RED}{RESET}{ITALIC}  Encrypted message sent to server:{RESET} {YELLOW}{}{RESET}", enc_msg_xoxo_bin);

	let mut primes_bytes = Vec::new();
	for n in &mut *lcs_primes {
		primes_bytes.extend_from_slice(&n.to_be_bytes());
	}

	stream.write_all(&primes_bytes).expect("Failed to send primes to server.");
	// println!(">>> Primes sent to server: {:?}", &lcs_primes);
}
