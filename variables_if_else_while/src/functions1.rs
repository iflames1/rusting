pub fn check_signed(number: i32) {
	if number < 0 {
		println!("{number} is negative");
	}
	else if number > 0 {
		println!("{number} is positive");
	}
	else {
		println!("{number} is zero");
	}
}

pub fn last_digit(number: i32) {
	let last_digit = number % 10;

	let value: &str =
		if last_digit == 0 {"0"}
		else if last_digit == 0 {"0"}
		else if last_digit > 5 {"greater than 5 and not 0"}
		else {"less than 6 and not 0"};

	println!("Last digit of {number} is {last_digit} and is {value}");
}

pub fn print_alphabet() {
	for i in 97u8..=122u8 {
		print!("{}", i as char);
	}
	print!("\n");
	//for c in b'a'..=b'z' {
    //    println!("{}", c as char);
    //}
}

pub fn print_alphabet2() {
	for c in b'a'..=b'z' {
		print!("{}", c as char);
	}
	for c in b'A'..=b'Z' {
		print!("{}", c as char);
	}
	print!("\n");
}

//fn main() {
//	print_alphabet2();
//}
