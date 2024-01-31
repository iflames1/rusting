pub mod functions1;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let n: i32 = rng.gen_range(0..=std::i32::MAX) - std::i32::MAX / 2;

    println!("Random generated number is: {} \n", n);

	// Function to check if a number is positive, negative or zero
    functions1::check_signed(n);

	// Function to find last digit of a number
	functions1::last_digit(n);

	// Function to print lowercase
	functions1::print_alphabet();

	// Function to print lowercase and uppercase alphabet
	functions1::print_alphabet2();

	// Function to print numbers
	functions1::print_numbers();

	// Function to print alphabet in reverse
	functions1::print_alphabet_reverse();

	// Function to print numbers from 0-9 formatted with commas
	functions1::print_numbers_formatted();

	// Funtion that prints all possible different combinations of two digits
	functions1::print_comb();
}
