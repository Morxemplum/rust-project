use rand::Rng;

fn choose_random_letter() -> char {
	let alphabet : &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
	let mut rng = rand::thread_rng();
	// Generates a random index from 0 to length of alphabet
	let num = rng.gen_range(0, alphabet.len());
	// Gets the character at index num of alphabet and returns it.
	// nth returns an Option enum. To get the actual value, we need to call unwrap()
	alphabet.chars().nth(num).unwrap()
} 

fn is_correct(phrase_one : &str, phrase_two : &str) -> bool { phrase_one.eq(phrase_two) }

fn strip_newline(s : &mut String) {
	if s.ends_with('\n') {
		s.pop();
	}
	// We also need to account for Windows Line Endings, which do CRLF.
	if s.ends_with('\r') {
		s.pop();
	}
}

fn main() {
	// Rust uses back slash as the escape character, just like most other lanaguages
	println!("\nHi, I want to play a game! So I am going to be saying a random phrase that will get longer over time.");
	println!("Your objective is to type in what I say, and you'll get points!");
	println!("Be careful. Get a letter wrong, and it's game over.\n\n"); 
	println!("Let's begin.");

	let input = std::io::stdin();

	let mut playing = true;
	let mut phrase = String::new(); // Instantiates a new empty string object.
	let mut guess = String::new();
	let mut score = 0;
	while playing {
		// Append a new random letter to the string
		phrase.push(choose_random_letter());
		println!("Repeat After Me: {}", phrase);

		guess.clear(); // Clear the string before the read_line.
		println!("Response: ");
		let _bytes = input.read_line(&mut guess).unwrap(); // read_line mutably borrows guess to write to it.
		strip_newline(&mut guess); // Strip newline characters from the input

		if is_correct(&phrase, &guess) {
			score += phrase.len();
			println!("Correct! You just earned {} point(s), and have a total score of {}\n", phrase.len(), score);
		} else {
			playing = false;
			println!("GAME OVER! You got the phrase wrong.");
		}
	}
	println!("You got a total score of {}.\nThanks for playing!", score);
}