use rand::Rng;

fn mains() {
    let mut rng = rand::thread_rng();
    let n: i32 = rng.gen_range(0..=std::i32::MAX) - std::i32::MAX / 2;

    // Your code goes here

    println!("{}", n);
}
