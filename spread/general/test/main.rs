use std::env;
use std::process;

fn main() {
	let args: Vec<String> = env::args().collect();
	
	if args.len() > 1 {
		println!("Hello from Rust: {}", args[1]);
	} else {
		println!("Hello from Rust!");
	}
	process::exit(0);
}
