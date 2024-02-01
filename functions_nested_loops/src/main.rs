use rand::Rng;
pub mod functions;

fn main() {
	let mut rng = rand::thread_rng();
	let random_ascii: u8 = rng.gen_range(b'A'..=b'z');
	let character: char = random_ascii as char;

	// Funtion that prints the alphabet, repeated ten times.
    functions::print_alphabet_x10();

	// Function that checks for lowercase character.
	if functions::_is_lowercase(character) {
		println!("The generated character '{}' is lowercase.", character);
	} else {
		println!("The generated character '{}' is not lowercase.", character);
	}

	// Function that prints every minute of the day of Jack Bauer, starting from 00:00 to 23:59.
	// currently comment because of length.
	// functions::jack_bauer();
}
