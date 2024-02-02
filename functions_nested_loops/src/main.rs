use rand::Rng;
pub mod functions;

fn main() {
	let mut rng = rand::thread_rng();
	let random_ascii: u8 = rng.gen_range(b'A'..=b'z');
	let character: char = random_ascii as char;

	let mut rng2 = rand::thread_rng();
    let n: i8 = rng2.gen_range(0..=std::i8::MAX) - std::i8::MAX / 2;

	// Funtion that prints the alphabet, repeated ten times.
    functions::print_alphabet_x10();

	// Function that checks for lowercase character.
	if functions::_is_lowercase(character) {
		println!("The generated character '{}' is lowercase.", character);
	} else {
		println!("The generated character '{}' is not lowercase.", character);
	}

	// Function that prints the 9 times table, starting with 0.
	functions::nine_times_table();

	// Function that prints all natural numbers from n to 98, followed by a new line.
	functions::print_to_98(n);

	// Function that prints every minute of the day of Jack Bauer, starting from 00:00 to 23:59.
	// currently comment because of length.
	// functions::jack_bauer();
}
