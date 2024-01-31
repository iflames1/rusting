// Function to check if a number is positive, negative or zero
pub fn check_signed(number: i32) {
	// Check if number is less than 0
	if number < 0 {
		// Check if number is less than 0
		println!("{number} is negative");
	}
	else if number > 0 {
		// Print number is positive
		println!("{number} is positive");
	}
	else {
		// Print number is zero
		println!("{number} is zero");
	}
}

// Function to find last digit of a number
pub fn last_digit(number: i32) {
	// Get the last digit of number
	let last_digit = number % 10;

	// Determine the value based on the last digit
	let value: &str = if last_digit == 0 {"0"}
		else if last_digit == 0 {"0"}
		else if last_digit > 5 {"greater than 5 and not 0"}
		else {"less than 6 and not 0"};

	// Print the result
	println!("Last digit of {number} is {last_digit} and is {value}");
}

// Function to print lowercase
pub fn print_alphabet() {
	// print lowercase alphabet
	for c in b'a'..=b'z' {
        println!("{}", c as char);
    }

	// move to next line
	print!("\n");
}

// Function to print lowercase and uppercase alphabet
pub fn print_alphabet2() {
	// print lowercase alphabet
	for c in b'a'..=b'z' {
		print!("{}", c as char);
	}

	// print uppercase alphabet
	for c in b'A'..=b'Z' {
		print!("{}", c as char);
	}

	// move to next line
	print!("\n");
}

// Function to print numbers
pub fn print_numbers() {
	let mut digit: u8 = 0;

	// loop while 'digit' is less than 10 and print current position
	while digit < 10 {
		print!("{}", digit);
		digit += 1;
	}
	print!("\n");
}

// Function to print alphabet in reverse
pub fn print_alphabet_reverse() {
	// initialize character to 'z'
	let mut character = b'z';

	// print lowercase alphabet in reverse
	while character >= b'a' {
		print!("{}", character as char);
		character -= 1;
	}

	// move to next line
	print!("\n");
}

// Function to print numbers from 0-9 formatted
pub fn print_numbers_formatted() {
	// Initialize digit variable
	let mut digit: u8 = 0;

	while digit < 10 {
		// print current digit
		print!("{digit}");

		// print comma if digit is not 9
		if digit != 9 {
			print!(", ");
		}

		digit += 1;
	}

	// move to next line
	print!("\n");
}

//fn main() {
//	print_numbers_formatted();
//}
