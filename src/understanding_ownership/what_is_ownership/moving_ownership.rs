pub fn main() {
    let s1 = String::from("hello");
    // let _s2 = s1;
    let _s2 = s1.clone();

    // !!! ERROR: value is moved to s2
    println!("{}, world!", s1);
}
