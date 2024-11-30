pub fn main() {
    for number in 1..5 {
        println!("{}", number);
    }

    for number in 1..=5 {
        println!("{}", number);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
