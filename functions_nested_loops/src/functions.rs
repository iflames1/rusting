//  Funtion that prints the alphabet, repeated ten times.
pub fn print_alphabet_x10() {
	let mut line = 0;

	// Loop while lines are less than 10 and print the alphabet on each iteration.
	while line < 10 {
		// Iterate through bytes 'a' to 'z'
		for c in b'a'..=b'z' {
			print!("{}", c as char);
		}
		// Move to next line
		print!("\n");

		line += 1;
	}
}

// Function that checks for lowercase character.
pub fn _is_lowercase(c: char) -> bool {
    return (c >= 'a') && (c <= 'z');
}

//fn main() {
//	println!("{}", _is_lowercase('a'))
//}