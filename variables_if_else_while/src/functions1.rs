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
