use rand::Rng;

fn main() {
	let mut rng = rand::thread_rng();
	let alphabet : &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
	let num = rng.gen_range(0, alphabet.len());
	println!("Chosen letter: {}", &alphabet[num..num+1]);
}