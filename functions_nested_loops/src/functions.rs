//  Funtion that prints the alphabet, repeated ten times.
pub fn print_alphabet_x10() {
	let mut line: i32 = 0;

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
	// Char code between 'a' and 'z'?
    return (c >= 'a') && (c <= 'z');
}

// Function that prints every minute of the day of Jack Bauer, starting from 00:00 to 23:59.
pub fn jack_bauer() {
	'outer: for tmin in 0..3{
		for umin in 0..10 {
			for sec in 0..6 {
				for msec in 0..10 {
					println!("{tmin}{umin}:{sec}{msec}");

					if tmin == 2 && umin == 3 && sec == 5 && msec == 9 {
						break 'outer;
					}
				}
			}
		}
	}
}

fn main() {
	jack_bauer();
}