use std::u16;

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
		// Iterate over minutes, seconds, andiseconds
		for umin in 0..10 {
			for sec in 0..6 {
				for msec in 0..10 {
					// Print time in MM:SS format
					println!("{tmin}{umin}:{sec}{msec}");

					// Stop if time is 23:59
					if tmin == 2 && umin == 3 && sec == 5 && msec == 9 {
						break 'outer;
					}
				}
			}
		}
	}
}

// Function that prints the 9 times table, starting with 0.
pub fn nine_times_table() {
	// Iterate over numbers 1 to 9 and multiply the iteration by 0 to 9
	for num in 0..=9 {
		print!("0, ");

		for mul in 1..=9 {
			// Print the current multiple
			print!("{:2}", num * mul);

			if mul != 9 {
				print!(", ")
			}
		}
		// Move to the next
		print!("\n");
	}
}

// Function to print times table
pub fn print_times_table(number: u16) {
	println!("number is {}", number);

	for num in 0..=number {
		print!("0,");

		for mul in 1..=number {
			print!("{:4}", num * mul);

			if mul != number {
				print!(",")
			}
		}

		print!("\n");
	}
}

// Function that prints all natural numbers from n to 98, followed by a new line.
pub fn print_to_98(mut number: i8) {
	println!("number is {number}");

	if number <= 98 {
		while number < 98 {
			print!("{number}");
			print!(", ");

			number += 1;
		}
	} else {
		while number > 98 {
			print!("{number}");
			print!(", ");

			number -= 1;
		}
	}
	println!("98");
}


//fn main() {
//	print_times_table(12);
//}
