pub mod functions1;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let n: i32 = rng.gen_range(0..=std::i32::MAX) - std::i32::MAX / 2;

    println!("Random generated number is: {} \n", n);

    functions1::check_signed(n);
	functions1::last_digit(n);
	functions1::print_alphabet();
	functions1::print_alphabet2();
	functions1::print_numbers();
}
